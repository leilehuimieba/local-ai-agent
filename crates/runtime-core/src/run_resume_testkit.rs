#[cfg(test)]
pub(crate) mod testkit {
    use crate::checkpoint::RunCheckpoint;
    use crate::contracts::{
        ModelRef, ProviderRef, RunEvent, RunRequest, RunResult, RuntimeRunResponse,
        VerificationSnapshot, WorkspaceRef,
    };
    use std::collections::BTreeMap;

    pub(crate) fn sample_request(strategy: &str) -> RunRequest {
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "retry task".to_string(),
            mode: "standard".to_string(),
            model_ref: sample_model_ref(),
            provider_ref: ProviderRef::default(),
            workspace_ref: sample_workspace_ref(),
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: "cp-1".to_string(),
            resume_strategy: strategy.to_string(),
            confirmation_decision: None,
        }
    }

    pub(crate) fn sample_checkpoint(reason: &str, handoff_path: &str) -> RunCheckpoint {
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

    pub(crate) fn sample_response(handoff_path: &str) -> RuntimeRunResponse {
        RuntimeRunResponse {
            events: vec![sample_event(handoff_path)],
            result: sample_result(),
            confirmation_request: None,
        }
    }

    pub(crate) fn sample_checkpoint_with_verification_snapshot() -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        checkpoint.response.events.push(sample_verification_event());
        checkpoint
    }

    pub(crate) fn sample_checkpoint_with_execution_boundary() -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        checkpoint
            .response
            .events
            .push(sample_execution_boundary_event());
        checkpoint
    }

    pub(crate) fn sample_checkpoint_with_confirmation_boundary() -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint_without_events("confirmation_required", "");
        checkpoint
            .response
            .events
            .push(sample_confirmation_boundary_event());
        checkpoint
    }

    fn sample_model_ref() -> ModelRef {
        ModelRef {
            provider_id: "provider".to_string(),
            model_id: "model".to_string(),
            display_name: "Model".to_string(),
        }
    }

    fn sample_workspace_ref() -> WorkspaceRef {
        WorkspaceRef {
            workspace_id: "workspace-1".to_string(),
            name: "Workspace".to_string(),
            root_path: "D:/repo".to_string(),
            is_active: true,
        }
    }

    fn sample_result() -> RunResult {
        RunResult {
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
        }
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
