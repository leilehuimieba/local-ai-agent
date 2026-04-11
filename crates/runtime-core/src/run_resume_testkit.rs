#[cfg(test)]
pub(crate) mod testkit {
    use crate::checkpoint::RunCheckpoint;
    use crate::contracts::{
        ModelRef, ProviderRef, RunRequest, RunResult, RuntimeRunResponse, WorkspaceRef,
    };
    use crate::run_resume_event_testkit::testkit::{
        sample_confirmation_boundary_event, sample_event, sample_execution_boundary_event,
        sample_verification_event,
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
        let ids = sample_result_ids();
        let payload = sample_result_payload();
        RunResult {
            request_id: ids.0,
            run_id: ids.1,
            session_id: ids.2,
            trace_id: ids.3,
            kind: "run_result".to_string(),
            source: "runtime".to_string(),
            status: payload.0,
            final_answer: payload.1,
            summary: payload.2,
            error: None,
            memory_write_summary: None,
            final_stage: "Finish".to_string(),
            checkpoint_id: Some("cp-1".to_string()),
            resumable: Some(true),
        }
    }

    fn sample_result_ids() -> (String, String, String, String) {
        (
            "request-1".to_string(),
            "run-1".to_string(),
            "session-1".to_string(),
            "trace-1".to_string(),
        )
    }

    fn sample_result_payload() -> (String, String, String) {
        (
            "failed".to_string(),
            "temporary failure".to_string(),
            "temporary failure".to_string(),
        )
    }

    fn sample_checkpoint_without_events(reason: &str, handoff_path: &str) -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint(reason, handoff_path);
        checkpoint.response.events.clear();
        checkpoint
    }

}
