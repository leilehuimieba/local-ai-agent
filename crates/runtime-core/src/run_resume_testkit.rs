#[cfg(test)]
pub(crate) mod testkit {
    use crate::checkpoint::RunCheckpoint;
    use crate::contracts::{
        ModelRef, ProviderRef, RunEvent, RunRequest, RunResult, RuntimeRunResponse, WorkspaceRef,
    };
    use crate::run_resume_event_testkit::testkit::{
        sample_confirmation_boundary_event, sample_event, sample_execution_boundary_event,
        sample_verification_event,
    };
    use std::collections::BTreeMap;

    pub(crate) fn sample_request(strategy: &str) -> RunRequest {
        let ids = sample_run_ids();
        RunRequest {
            request_id: ids.0,
            run_id: ids.1,
            session_id: ids.2,
            trace_id: ids.3,
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
        let ids = sample_checkpoint_ids();
        let meta = sample_checkpoint_meta(reason);
        RunCheckpoint {
            checkpoint_id: ids.0,
            run_id: ids.1,
            session_id: ids.2,
            trace_id: ids.3,
            workspace_id: ids.4,
            status: meta.0,
            final_stage: meta.1,
            resumable: meta.2,
            resume_reason: meta.3,
            resume_stage: meta.4,
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
        sample_retry_checkpoint_with_event(sample_verification_event())
    }

    pub(crate) fn sample_checkpoint_with_execution_boundary() -> RunCheckpoint {
        sample_retry_checkpoint_with_event(sample_execution_boundary_event())
    }

    pub(crate) fn sample_checkpoint_with_confirmation_boundary() -> RunCheckpoint {
        sample_checkpoint_with_event(
            sample_checkpoint_without_events("confirmation_required", ""),
            sample_confirmation_boundary_event(),
        )
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
        let result_meta = sample_result_meta();
        RunResult {
            request_id: ids.0,
            run_id: ids.1,
            session_id: ids.2,
            trace_id: ids.3,
            kind: result_meta.0,
            source: result_meta.1,
            status: payload.0,
            final_answer: payload.1,
            summary: payload.2,
            error: None,
            memory_write_summary: None,
            final_stage: result_meta.2,
            checkpoint_id: Some(result_meta.3),
            resumable: Some(result_meta.4),
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

    fn sample_result_meta() -> (String, String, String, String, bool) {
        (
            "run_result".to_string(),
            "runtime".to_string(),
            "Finish".to_string(),
            "cp-1".to_string(),
            true,
        )
    }

    fn sample_checkpoint_ids() -> (String, String, String, String, String) {
        (
            "cp-1".to_string(),
            "run-1".to_string(),
            "session-1".to_string(),
            "trace-1".to_string(),
            "workspace-1".to_string(),
        )
    }

    fn sample_checkpoint_meta(reason: &str) -> (String, String, bool, String, String) {
        (
            "failed".to_string(),
            "Finish".to_string(),
            true,
            reason.to_string(),
            "Execute".to_string(),
        )
    }

    fn sample_checkpoint_without_events(reason: &str, handoff_path: &str) -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint(reason, handoff_path);
        checkpoint.response.events.clear();
        checkpoint
    }

    fn sample_retry_checkpoint_with_event(event: RunEvent) -> RunCheckpoint {
        sample_checkpoint_with_event(
            sample_checkpoint("retryable_failure", "D:/repo/handoff.json"),
            event,
        )
    }

    fn sample_checkpoint_with_event(
        mut checkpoint: RunCheckpoint,
        event: RunEvent,
    ) -> RunCheckpoint {
        checkpoint.response.events.push(event);
        checkpoint
    }

    fn sample_run_ids() -> (String, String, String, String) {
        (
            "request-1".to_string(),
            "run-1".to_string(),
            "session-1".to_string(),
            "trace-1".to_string(),
        )
    }
}
