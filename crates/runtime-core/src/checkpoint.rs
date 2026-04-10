use crate::contracts::{RunEvent, RunRequest, RuntimeRunResponse};
use crate::events::make_event;
use crate::sqlite_store::{load_runtime_checkpoint_sqlite, write_runtime_checkpoint_sqlite};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct RunCheckpoint {
    pub checkpoint_id: String,
    pub run_id: String,
    pub session_id: String,
    pub trace_id: String,
    pub workspace_id: String,
    pub status: String,
    pub final_stage: String,
    pub resumable: bool,
    pub resume_reason: String,
    pub resume_stage: String,
    pub event_count: u32,
    pub request: RunRequest,
    pub response: RuntimeRunResponse,
    pub created_at: String,
}

pub(crate) fn with_runtime_checkpoint(
    request: &RunRequest,
    response: RuntimeRunResponse,
) -> RuntimeRunResponse {
    let original = response.clone();
    let checkpoint_id = checkpoint_id(request);
    let enriched = checkpoint_response(request, response, &checkpoint_id);
    let checkpoint = checkpoint_record(request, &enriched, &checkpoint_id);
    persist_checkpoint(request, checkpoint, original, enriched)
}

pub(crate) fn checkpoint_resume_event(request: &RunRequest) -> Option<RunEvent> {
    let checkpoint_id = request.resume_from_checkpoint_id.trim();
    if checkpoint_id.is_empty() {
        return None;
    }
    match load_runtime_checkpoint_sqlite(request, checkpoint_id) {
        Ok(Some(checkpoint)) if resume_matches(request, &checkpoint) => {
            Some(resumed_event(request, &checkpoint))
        }
        Ok(Some(_)) => Some(skipped_resume_event(request, checkpoint_id, "checkpoint 与当前恢复请求不匹配，已按普通重试继续。")),
        Ok(None) => Some(skipped_resume_event(request, checkpoint_id, "未找到对应 checkpoint，已按普通重试继续。")),
        Err(error) => Some(skipped_resume_event(request, checkpoint_id, &format!("读取 checkpoint 失败：{error}"))),
    }
}

pub(crate) fn with_checkpoint_resume_event(
    mut response: RuntimeRunResponse,
    event: Option<RunEvent>,
) -> RuntimeRunResponse {
    let Some(event) = event else {
        return response;
    };
    let index = resume_insert_index(&response.events);
    response.events.insert(index, event);
    resequence_events(&mut response.events);
    response
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn load_runtime_checkpoint(
    request: &RunRequest,
    checkpoint_id: &str,
) -> Result<Option<RunCheckpoint>, String> {
    load_runtime_checkpoint_sqlite(request, checkpoint_id)
}

fn persist_checkpoint(
    request: &RunRequest,
    checkpoint: RunCheckpoint,
    original: RuntimeRunResponse,
    enriched: RuntimeRunResponse,
) -> RuntimeRunResponse {
    match write_runtime_checkpoint_sqlite(request, &checkpoint) {
        Ok(()) => enriched,
        Err(_) => original,
    }
}

fn checkpoint_response(
    request: &RunRequest,
    mut response: RuntimeRunResponse,
    checkpoint_id: &str,
) -> RuntimeRunResponse {
    let resumable = checkpoint_resume_profile(&response).0;
    response.result.checkpoint_id = Some(checkpoint_id.to_string());
    response.result.resumable = Some(resumable);
    insert_checkpoint_event(request, &mut response.events, checkpoint_id, &response.result);
    response
}

fn insert_checkpoint_event(
    request: &RunRequest,
    events: &mut Vec<RunEvent>,
    checkpoint_id: &str,
    result: &crate::contracts::RunResult,
) {
    let event = checkpoint_event(request, checkpoint_id, result, checkpoint_event_count(events));
    let index = terminal_event_index(events).unwrap_or(events.len());
    events.insert(index, event);
    resequence_events(events);
}

fn checkpoint_event(
    request: &RunRequest,
    checkpoint_id: &str,
    result: &crate::contracts::RunResult,
    event_count: u32,
) -> RunEvent {
    make_event(
        request,
        0,
        "checkpoint_written",
        &result.final_stage,
        "已写入运行检查点",
        &format!("本次运行快照已写入 checkpoint：{checkpoint_id}"),
        checkpoint_metadata(checkpoint_id, result, event_count),
    )
}

fn checkpoint_metadata(
    checkpoint_id: &str,
    result: &crate::contracts::RunResult,
    event_count: u32,
) -> BTreeMap<String, String> {
    let mut metadata = BTreeMap::new();
    metadata.insert("checkpoint_id".to_string(), checkpoint_id.to_string());
    metadata.insert("checkpoint_status".to_string(), result.status.clone());
    metadata.insert("checkpoint_stage".to_string(), result.final_stage.clone());
    metadata.insert("checkpoint_event_count".to_string(), event_count.to_string());
    metadata.insert("checkpoint_written".to_string(), "true".to_string());
    metadata.insert("result_summary".to_string(), result.summary.clone());
    metadata
}

fn checkpoint_record(
    request: &RunRequest,
    response: &RuntimeRunResponse,
    checkpoint_id: &str,
) -> RunCheckpoint {
    let resume = checkpoint_resume_profile(response);
    RunCheckpoint {
        checkpoint_id: checkpoint_id.to_string(),
        run_id: request.run_id.clone(),
        session_id: request.session_id.clone(),
        trace_id: request.trace_id.clone(),
        workspace_id: request.workspace_ref.workspace_id.clone(),
        status: response.result.status.clone(),
        final_stage: response.result.final_stage.clone(),
        resumable: resume.0,
        resume_reason: resume.1,
        resume_stage: resume.2,
        event_count: checkpoint_event_count(&response.events),
        request: redacted_request(request),
        response: response.clone(),
        created_at: timestamp_now(),
    }
}

fn resume_matches(request: &RunRequest, checkpoint: &RunCheckpoint) -> bool {
    let same_scope = checkpoint.run_id == request.run_id
        && checkpoint.session_id == request.session_id
        && checkpoint.workspace_id == request.workspace_ref.workspace_id;
    match resume_strategy(request).as_str() {
        "after_confirmation" => same_scope && checkpoint_ready_for_confirmation(request, checkpoint),
        _ => same_scope,
    }
}

fn checkpoint_ready_for_confirmation(request: &RunRequest, checkpoint: &RunCheckpoint) -> bool {
    checkpoint.resumable
        && checkpoint.resume_reason == "confirmation_required"
        && request
            .confirmation_decision
            .as_ref()
            .is_some_and(|decision| decision.decision == "approve")
}

fn resume_strategy(request: &RunRequest) -> String {
    if request.resume_strategy.trim().is_empty() {
        "after_confirmation".to_string()
    } else {
        request.resume_strategy.clone()
    }
}

fn resumed_event(request: &RunRequest, checkpoint: &RunCheckpoint) -> RunEvent {
    make_event(
        request,
        0,
        "checkpoint_resumed",
        &checkpoint.resume_stage,
        "已从 checkpoint 恢复运行",
        &format!("已读取 checkpoint：{}，继续当前任务。", checkpoint.checkpoint_id),
        resume_metadata(
            &checkpoint.checkpoint_id,
            "resumed",
            &checkpoint.resume_stage,
            &checkpoint.resume_reason,
        ),
    )
}

fn skipped_resume_event(request: &RunRequest, checkpoint_id: &str, reason: &str) -> RunEvent {
    make_event(
        request,
        0,
        "checkpoint_resume_skipped",
        "Analyze",
        "checkpoint 恢复已跳过",
        reason,
        resume_metadata(checkpoint_id, "skipped", "Analyze", "resume_skipped"),
    )
}

fn resume_metadata(
    checkpoint_id: &str,
    status: &str,
    stage: &str,
    reason: &str,
) -> BTreeMap<String, String> {
    let mut metadata = BTreeMap::new();
    metadata.insert("checkpoint_id".to_string(), checkpoint_id.to_string());
    metadata.insert("checkpoint_resume_status".to_string(), status.to_string());
    metadata.insert("checkpoint_stage".to_string(), stage.to_string());
    metadata.insert("checkpoint_resume_reason".to_string(), reason.to_string());
    metadata
}

fn checkpoint_resume_profile(response: &RuntimeRunResponse) -> (bool, String, String) {
    if response.result.status == "awaiting_confirmation" {
        return (
            true,
            "confirmation_required".to_string(),
            "PausedForConfirmation".to_string(),
        );
    }
    let retryable = response
        .result
        .error
        .as_ref()
        .is_some_and(|error| error.retryable);
    if retryable {
        return (true, "retryable_failure".to_string(), "Execute".to_string());
    }
    (false, "none".to_string(), response.result.final_stage.clone())
}

fn redacted_request(request: &RunRequest) -> RunRequest {
    let mut redacted = request.clone();
    redacted.provider_ref.api_key.clear();
    redacted
}

fn checkpoint_event_count(events: &[RunEvent]) -> u32 {
    events.len() as u32 + 1
}

fn terminal_event_index(events: &[RunEvent]) -> Option<usize> {
    events
        .iter()
        .position(|event| is_terminal_event(&event.event_type))
}

fn resume_insert_index(events: &[RunEvent]) -> usize {
    events
        .iter()
        .position(|event| event.event_type == "run_started")
        .map(|index| index + 1)
        .unwrap_or(0)
}

fn is_terminal_event(event_type: &str) -> bool {
    event_type == "run_finished" || event_type == "run_failed"
}

fn resequence_events(events: &mut [RunEvent]) {
    for (index, event) in events.iter_mut().enumerate() {
        event.sequence = index as u32 + 1;
    }
}

fn checkpoint_id(request: &RunRequest) -> String {
    format!("{}-{}", request.run_id, unix_millis())
}

fn timestamp_now() -> String {
    unix_millis().to_string()
}

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_millis())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{
        checkpoint_resume_event, load_runtime_checkpoint, with_checkpoint_resume_event,
        with_runtime_checkpoint,
    };
    use crate::contracts::{
        ErrorInfo, ModelRef, ProviderRef, RunRequest, RunResult, RuntimeRunResponse, WorkspaceRef,
    };
    use std::collections::BTreeMap;
    use std::fs;

    #[test]
    fn writes_checkpoint_and_inserts_event() {
        let root = test_root("writes_checkpoint_and_inserts_event");
        let request = sample_request(&root);
        let response = with_runtime_checkpoint(&request, sample_response(&request));
        let checkpoint_id = response.result.checkpoint_id.clone().unwrap_or_default();
        let loaded = load_runtime_checkpoint(&request, &checkpoint_id).unwrap_or(None);
        assert!(!checkpoint_id.is_empty());
        assert!(response.events.iter().any(|item| item.event_type == "checkpoint_written"));
        assert_eq!(response.events.last().map(|item| item.event_type.as_str()), Some("run_finished"));
        assert!(loaded.is_some());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resumes_from_matching_confirmation_checkpoint() {
        let root = test_root("resumes_from_matching_confirmation_checkpoint");
        let request = sample_request(&root);
        let response = with_runtime_checkpoint(&request, awaiting_response(&request));
        let mut resumed = request.clone();
        resumed.resume_from_checkpoint_id = response.result.checkpoint_id.unwrap_or_default();
        resumed.resume_strategy = "after_confirmation".to_string();
        resumed.confirmation_decision = Some(approve_decision(&request.run_id));
        let event = checkpoint_resume_event(&resumed);
        let response = with_checkpoint_resume_event(sample_response(&resumed), event);
        assert!(response.events.iter().any(|item| item.event_type == "checkpoint_resumed"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn marks_retryable_failure_checkpoint_as_resumable() {
        let root = test_root("marks_retryable_failure_checkpoint_as_resumable");
        let request = sample_request(&root);
        let response = with_runtime_checkpoint(&request, retryable_failure_response(&request));
        let checkpoint_id = response.result.checkpoint_id.clone().unwrap_or_default();
        let loaded = load_runtime_checkpoint(&request, &checkpoint_id).unwrap_or(None);
        assert!(loaded.as_ref().is_some_and(|item| item.resumable));
        assert!(loaded.as_ref().is_some_and(|item| item.resume_reason == "retryable_failure"));
        assert!(loaded.as_ref().is_some_and(|item| item.resume_stage == "Execute"));
        let _ = fs::remove_dir_all(root);
    }

    fn test_root(case_name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("runtime-core-{case_name}"));
        let _ = fs::remove_dir_all(&root);
        let _ = fs::create_dir_all(root.join("workspace"));
        root
    }

    fn sample_request(root: &std::path::Path) -> RunRequest {
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "test".to_string(),
            mode: "standard".to_string(),
            model_ref: ModelRef {
                provider_id: "provider".to_string(),
                model_id: "model".to_string(),
                display_name: "Model".to_string(),
            },
            provider_ref: ProviderRef {
                provider_id: "provider".to_string(),
                display_name: "Provider".to_string(),
                base_url: "https://example.com".to_string(),
                chat_completions_path: "/chat".to_string(),
                models_path: "/models".to_string(),
                api_key: "secret".to_string(),
            },
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-1".to_string(),
                name: "Workspace".to_string(),
                root_path: root.join("workspace").display().to_string(),
                is_active: true,
            },
            context_hints: BTreeMap::from([("repo_root".to_string(), root.display().to_string())]),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }

    fn sample_response(request: &RunRequest) -> RuntimeRunResponse {
        RuntimeRunResponse {
            events: vec![
                sample_event(request, 1, "run_started", "Analyze"),
                sample_event(request, 2, "run_finished", "Finish"),
            ],
            result: RunResult {
                request_id: request.request_id.clone(),
                run_id: request.run_id.clone(),
                session_id: request.session_id.clone(),
                trace_id: request.trace_id.clone(),
                kind: "run_result".to_string(),
                source: "runtime".to_string(),
                status: "completed".to_string(),
                final_answer: "ok".to_string(),
                summary: "ok".to_string(),
                error: None,
                memory_write_summary: None,
                final_stage: "Finish".to_string(),
                checkpoint_id: None,
                resumable: None,
            },
            confirmation_request: None,
        }
    }

    fn awaiting_response(request: &RunRequest) -> RuntimeRunResponse {
        let mut response = sample_response(request);
        response.result.status = "awaiting_confirmation".to_string();
        response.result.final_stage = "PausedForConfirmation".to_string();
        response.events[1].event_type = "confirmation_required".to_string();
        response.events[1].stage = "PausedForConfirmation".to_string();
        response
    }

    fn approve_decision(run_id: &str) -> crate::contracts::ConfirmationDecision {
        crate::contracts::ConfirmationDecision {
            confirmation_id: format!("confirm-risk-{run_id}"),
            run_id: run_id.to_string(),
            decision: "approve".to_string(),
            note: String::new(),
            remember: false,
        }
    }

    fn retryable_failure_response(request: &RunRequest) -> RuntimeRunResponse {
        RuntimeRunResponse {
            events: vec![
                sample_event(request, 1, "run_started", "Analyze"),
                sample_event(request, 2, "run_failed", "Finish"),
                sample_event(request, 3, "run_finished", "Finish"),
            ],
            result: RunResult {
                request_id: request.request_id.clone(),
                run_id: request.run_id.clone(),
                session_id: request.session_id.clone(),
                trace_id: request.trace_id.clone(),
                kind: "run_result".to_string(),
                source: "runtime".to_string(),
                status: "failed".to_string(),
                final_answer: "temporary failure".to_string(),
                summary: "temporary failure".to_string(),
                error: Some(ErrorInfo {
                    error_code: "action_execution_failed".to_string(),
                    message: "temporary failure".to_string(),
                    summary: "temporary failure".to_string(),
                    retryable: true,
                    source: "runtime".to_string(),
                    stage: "Finish".to_string(),
                    metadata: BTreeMap::new(),
                }),
                memory_write_summary: None,
                final_stage: "Finish".to_string(),
                checkpoint_id: None,
                resumable: None,
            },
            confirmation_request: None,
        }
    }

    fn sample_event(
        request: &RunRequest,
        sequence: u32,
        event_type: &str,
        stage: &str,
    ) -> crate::contracts::RunEvent {
        crate::contracts::RunEvent {
            event_id: format!("{}-{sequence}", request.run_id),
            kind: "run_event".to_string(),
            source: "runtime".to_string(),
            record_type: String::new(),
            source_type: String::new(),
            agent_id: "primary".to_string(),
            agent_label: "主智能体".to_string(),
            event_type: event_type.to_string(),
            trace_id: request.trace_id.clone(),
            session_id: request.session_id.clone(),
            run_id: request.run_id.clone(),
            sequence,
            timestamp: "1".to_string(),
            stage: stage.to_string(),
            summary: event_type.to_string(),
            detail: String::new(),
            tool_name: String::new(),
            tool_display_name: String::new(),
            tool_category: String::new(),
            output_kind: String::new(),
            result_summary: String::new(),
            artifact_path: String::new(),
            risk_level: String::new(),
            confirmation_id: String::new(),
            final_answer: String::new(),
            completion_status: String::new(),
            completion_reason: String::new(),
            verification_summary: String::new(),
            checkpoint_written: false,
            context_snapshot: None,
            tool_call_snapshot: None,
            verification_snapshot: None,
            metadata: BTreeMap::new(),
        }
    }
}
