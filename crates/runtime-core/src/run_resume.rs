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
    session.short_term.recent_observation = resume_recent_observation(checkpoint);
    session.short_term.recent_tool_result = resume_recent_tool_result(checkpoint);
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
    let boundary = resume_execution_boundary(checkpoint);
    let hint = resume_recovery_hint(checkpoint);
    let with_action = if action.is_empty() {
        base
    } else {
        format!("{base}；继续动作：{action}")
    };
    let with_boundary = if boundary.is_empty() {
        with_action
    } else {
        format!("{with_action}；恢复边界：{boundary}")
    };
    if hint.is_empty() {
        with_boundary
    } else {
        format!("{with_boundary}；恢复提示：{hint}")
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
    let tool = event
        .metadata
        .get("tool_display_name")
        .cloned()
        .unwrap_or_default();
    let task = event
        .metadata
        .get("task_title")
        .cloned()
        .unwrap_or_default();
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

fn resume_execution_boundary(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(execution_boundary_from_event)
        .or_else(|| confirmation_boundary_from_events(checkpoint))
        .unwrap_or_default()
}

fn execution_boundary_from_event(event: &crate::contracts::RunEvent) -> Option<String> {
    if !is_execution_boundary_event(event) {
        return None;
    }
    let mut parts = vec![format!("阶段={}", event.stage), format!("事件={}", event.event_type)];
    if let Some(step) = event.metadata.get("next_step").filter(|step| !step.is_empty()) {
        parts.push(format!("下一步={step}"));
    }
    Some(parts.join("，"))
}

fn is_execution_boundary_event(event: &crate::contracts::RunEvent) -> bool {
    matches!(
        event.event_type.as_str(),
        "action_requested" | "action_completed" | "verification_completed" | "run_failed"
    )
}

fn confirmation_boundary_from_events(checkpoint: &RunCheckpoint) -> Option<String> {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find(|event| event.event_type == "confirmation_required")
        .map(|event| {
            let step = event
                .metadata
                .get("next_step")
                .cloned()
                .unwrap_or_else(|| "等待用户确认后再继续".to_string());
            format!("阶段={}，事件={}，下一步={}", event.stage, event.event_type, step)
        })
}

fn resume_recovery_hint(checkpoint: &RunCheckpoint) -> String {
    let failed = checkpoint
        .response
        .events
        .iter()
        .rev()
        .find(|event| event.event_type == "run_failed")
        .and_then(|event| event.metadata.get("failure_recovery_hint").cloned());
    if failed.is_some() {
        return failed.unwrap_or_default();
    }
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find(|event| event.event_type == "verification_completed")
        .and_then(|event| event.metadata.get("verification_summary").cloned())
        .unwrap_or_default()
}

fn resume_recent_tool_result(checkpoint: &RunCheckpoint) -> String {
    let verification = resume_verification_summary(checkpoint);
    if verification.is_empty() {
        checkpoint.response.result.summary.clone()
    } else {
        format!("验证快照：{verification}")
    }
}

fn resume_recent_observation(checkpoint: &RunCheckpoint) -> String {
    let answer = checkpoint.response.result.final_answer.clone();
    let artifact = resume_artifact_path(checkpoint);
    if artifact.is_empty() {
        return answer;
    }
    if answer.is_empty() {
        return format!("恢复到产物：{artifact}");
    }
    format!("{answer}；产物：{artifact}")
}

fn resume_verification_summary(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(|event| event.verification_snapshot.as_ref())
        .map(|snapshot| snapshot.summary.clone())
        .filter(|summary| !summary.is_empty())
        .unwrap_or_default()
}

fn resume_artifact_path(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(|event| {
            event
                .metadata
                .get("artifact_path")
                .cloned()
                .filter(|path| !path.is_empty())
                .or_else(|| artifact_from_verification_snapshot(event))
        })
        .unwrap_or_default()
}

fn artifact_from_verification_snapshot(event: &crate::contracts::RunEvent) -> Option<String> {
    event.verification_snapshot.as_ref().and_then(|snapshot| {
        snapshot
            .evidence
            .iter()
            .find_map(|line| line.strip_prefix("artifact=").map(str::to_string))
    })
}

#[cfg(test)]
mod tests {
    use super::apply_resume_checkpoint;
    use crate::checkpoint::RunCheckpoint;
    use crate::contracts::{
        ModelRef, ProviderRef, RunEvent, RunRequest, RunResult, RuntimeRunResponse,
        VerificationSnapshot, WorkspaceRef,
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
        assert_eq!(
            session.short_term.handoff_artifact_path,
            "D:/repo/handoff.json"
        );
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
        assert!(
            session
                .short_term
                .current_plan
                .contains("继续动作：执行命令")
        );
        assert!(
            session
                .short_term
                .current_plan
                .contains("执行命令: Write-Error")
        );
    }

    #[test]
    fn appends_recovery_hint_into_resume_plan() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(
            session
                .short_term
                .current_plan
                .contains("恢复提示：建议先检查命令语法")
        );
    }

    #[test]
    fn appends_execution_boundary_into_resume_plan() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint_with_execution_boundary();
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(
            session
                .short_term
                .current_plan
                .contains("恢复边界：阶段=Execute，事件=action_completed，下一步=进入验证阶段")
        );
    }

    #[test]
    fn restores_verification_snapshot_into_short_term_memory() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint_with_verification_snapshot();
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(
            session
                .short_term
                .recent_tool_result
                .contains("验证快照：验证通过并产生产物")
        );
        assert!(
            session
                .short_term
                .recent_observation
                .contains("D:/repo/verify/report.txt")
        );
    }

    #[test]
    fn appends_confirmation_boundary_when_no_execution_event() {
        let request = sample_request("after_confirmation");
        let checkpoint = sample_checkpoint_with_confirmation_boundary();
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(
            session
                .short_term
                .current_plan
                .contains("恢复边界：阶段=PausedForConfirmation，事件=confirmation_required")
        );
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

    fn sample_checkpoint_with_verification_snapshot() -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        checkpoint.response.events.push(sample_verification_event());
        checkpoint
    }

    fn sample_checkpoint_with_execution_boundary() -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        checkpoint.response.events.push(sample_execution_boundary_event());
        checkpoint
    }

    fn sample_checkpoint_with_confirmation_boundary() -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint_without_events("confirmation_required", "");
        checkpoint
            .response
            .events
            .push(sample_confirmation_boundary_event());
        checkpoint
    }

    fn sample_checkpoint_without_events(reason: &str, handoff_path: &str) -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint(reason, handoff_path);
        checkpoint.response.events.clear();
        checkpoint
    }

    fn sample_event(handoff_path: &str) -> RunEvent {
        let mut metadata = BTreeMap::new();
        metadata.insert("tool_name".to_string(), "run_command".to_string());
        metadata.insert("tool_display_name".to_string(), "执行命令".to_string());
        metadata.insert(
            "task_title".to_string(),
            "执行命令: Write-Error 'stage-b retry acceptance'; ex...".to_string(),
        );
        metadata.insert(
            "failure_recovery_hint".to_string(),
            "建议先检查命令语法、依赖和当前环境，再决定是否重试。".to_string(),
        );
        if !handoff_path.is_empty() {
            metadata.insert(
                "handoff_artifact_path".to_string(),
                handoff_path.to_string(),
            );
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

    fn sample_verification_event() -> RunEvent {
        let mut event = sample_event("");
        event.event_id = "event-2".to_string();
        event.event_type = "verification_completed".to_string();
        event.stage = "Verify".to_string();
        event.summary = "verification passed".to_string();
        event.metadata.insert(
            "artifact_path".to_string(),
            "D:/repo/verify/report.txt".to_string(),
        );
        event.verification_snapshot = Some(VerificationSnapshot {
            code: "verified".to_string(),
            summary: "验证通过并产生产物".to_string(),
            passed: true,
            policy: "inspect_command_result".to_string(),
            evidence: vec![
                "summary=ok".to_string(),
                "artifact=D:/repo/verify/report.txt".to_string(),
            ],
        });
        event
    }

    fn sample_execution_boundary_event() -> RunEvent {
        let mut event = sample_event("");
        event.event_id = "event-3".to_string();
        event.event_type = "action_completed".to_string();
        event.stage = "Execute".to_string();
        event
            .metadata
            .insert("next_step".to_string(), "进入验证阶段".to_string());
        event
    }

    fn sample_confirmation_boundary_event() -> RunEvent {
        let mut event = sample_event("");
        event.event_id = "event-4".to_string();
        event.event_type = "confirmation_required".to_string();
        event.stage = "PausedForConfirmation".to_string();
        event
            .metadata
            .insert("next_step".to_string(), "等待用户确认后再继续".to_string());
        event
    }
}
