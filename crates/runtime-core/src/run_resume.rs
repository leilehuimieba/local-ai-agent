use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::session::SessionMemory;

pub(crate) fn apply_resume_checkpoint(
    session: &mut SessionMemory,
    checkpoint: Option<&RunCheckpoint>,
    request: &RunRequest,
) {
    let Some(checkpoint) = checkpoint else {
        return;
    };
    session.short_term.current_plan = resume_plan(checkpoint);
    session.short_term.current_phase = resume_phase(checkpoint);
    session.short_term.last_run_status = checkpoint.status.clone();
    session.short_term.recent_observation = checkpoint.response.result.final_answer.clone();
    session.short_term.recent_tool_result = checkpoint.response.result.summary.clone();
    session.short_term.handoff_artifact_path = resume_handoff_artifact_path(checkpoint);
    clear_resume_confirmation_state(session, checkpoint, request);
}

fn clear_resume_confirmation_state(
    session: &mut SessionMemory,
    checkpoint: &RunCheckpoint,
    request: &RunRequest,
) {
    if checkpoint.resume_reason == "confirmation_required" {
        session.short_term.pending_confirmation.clear();
        session.short_term.open_issue.clear();
        return;
    }
    if request.resume_strategy == "retry_failure" {
        session.short_term.pending_confirmation.clear();
        session.short_term.open_issue = checkpoint.response.result.summary.clone();
    }
}

fn resume_plan(checkpoint: &RunCheckpoint) -> String {
    let base = format!(
        "从 checkpoint 恢复：{} -> {}",
        checkpoint.resume_reason, checkpoint.resume_stage
    );
    let action = resume_action_hint(checkpoint);
    if action.is_empty() {
        base
    } else {
        format!("{base}；继续动作：{action}")
    }
}

fn resume_phase(checkpoint: &RunCheckpoint) -> String {
    if checkpoint.resume_reason == "confirmation_required" {
        "confirmation_resume".to_string()
    } else {
        "recovery".to_string()
    }
}

fn resume_handoff_artifact_path(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(|event| event.metadata.get("handoff_artifact_path").cloned())
        .unwrap_or_default()
}

fn resume_action_hint(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(action_hint_from_event)
        .unwrap_or_default()
}

fn action_hint_from_event(event: &crate::contracts::RunEvent) -> Option<String> {
    let tool = event.metadata.get("tool_display_name").cloned().unwrap_or_default();
    let task = event.metadata.get("task_title").cloned().unwrap_or_default();
    let name = event.metadata.get("tool_name").cloned().unwrap_or_default();
    if !tool.is_empty() && !task.is_empty() {
        return Some(format!("{tool} ({task})"));
    }
    if !tool.is_empty() {
        return Some(tool);
    }
    if !task.is_empty() {
        return Some(task);
    }
    if !name.is_empty() {
        return Some(name);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::apply_resume_checkpoint;
    use crate::checkpoint::RunCheckpoint;
    use crate::contracts::{
        ModelRef, ProviderRef, RunEvent, RunRequest, RunResult, RuntimeRunResponse, WorkspaceRef,
    };
    use crate::session::SessionMemory;
    use std::collections::BTreeMap;

    #[test]
    fn restores_handoff_artifact_path_for_retry_recovery() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert_eq!(session.short_term.current_phase, "recovery");
        assert_eq!(session.short_term.handoff_artifact_path, "D:/repo/handoff.json");
    }

    #[test]
    fn leaves_handoff_artifact_path_empty_when_checkpoint_has_none() {
        let request = sample_request("after_confirmation");
        let checkpoint = sample_checkpoint("confirmation_required", "");
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(session.short_term.handoff_artifact_path.is_empty());
    }

    #[test]
    fn appends_last_action_hint_into_resume_plan() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(session.short_term.current_plan.contains("继续动作：执行命令"));
        assert!(session.short_term.current_plan.contains("执行命令: Write-Error"));
    }

    fn sample_request(strategy: &str) -> RunRequest {
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "retry task".to_string(),
            mode: "standard".to_string(),
            model_ref: ModelRef {
                provider_id: "provider".to_string(),
                model_id: "model".to_string(),
                display_name: "Model".to_string(),
            },
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-1".to_string(),
                name: "Workspace".to_string(),
                root_path: "D:/repo".to_string(),
                is_active: true,
            },
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: "cp-1".to_string(),
            resume_strategy: strategy.to_string(),
            confirmation_decision: None,
        }
    }

    fn sample_checkpoint(reason: &str, handoff_path: &str) -> RunCheckpoint {
        RunCheckpoint {
            checkpoint_id: "cp-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            workspace_id: "workspace-1".to_string(),
            status: "failed".to_string(),
            final_stage: "Finish".to_string(),
            resumable: true,
            resume_reason: reason.to_string(),
            resume_stage: "Execute".to_string(),
            event_count: 2,
            request: sample_request(reason),
            response: sample_response(handoff_path),
            created_at: "1".to_string(),
        }
    }

    fn sample_response(handoff_path: &str) -> RuntimeRunResponse {
        RuntimeRunResponse {
            events: vec![sample_event(handoff_path)],
            result: RunResult {
                request_id: "request-1".to_string(),
                run_id: "run-1".to_string(),
                session_id: "session-1".to_string(),
                trace_id: "trace-1".to_string(),
                kind: "run_result".to_string(),
                source: "runtime".to_string(),
                status: "failed".to_string(),
                final_answer: "temporary failure".to_string(),
                summary: "temporary failure".to_string(),
                error: None,
                memory_write_summary: None,
                final_stage: "Finish".to_string(),
                checkpoint_id: Some("cp-1".to_string()),
                resumable: Some(true),
            },
            confirmation_request: None,
        }
    }

    fn sample_event(handoff_path: &str) -> RunEvent {
        let mut metadata = BTreeMap::new();
        metadata.insert("tool_name".to_string(), "run_command".to_string());
        metadata.insert("tool_display_name".to_string(), "执行命令".to_string());
        metadata.insert(
            "task_title".to_string(),
            "执行命令: Write-Error 'stage-b retry acceptance'; ex...".to_string(),
        );
        if !handoff_path.is_empty() {
            metadata.insert("handoff_artifact_path".to_string(), handoff_path.to_string());
        }
        RunEvent {
            event_id: "event-1".to_string(),
            kind: "run_event".to_string(),
            source: "runtime".to_string(),
            record_type: String::new(),
            source_type: String::new(),
            agent_id: "primary".to_string(),
            agent_label: "主智能体".to_string(),
            event_type: "run_failed".to_string(),
            trace_id: "trace-1".to_string(),
            session_id: "session-1".to_string(),
            run_id: "run-1".to_string(),
            sequence: 1,
            timestamp: "1".to_string(),
            stage: "Failed".to_string(),
            summary: "failed".to_string(),
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
            metadata,
        }
    }
}
