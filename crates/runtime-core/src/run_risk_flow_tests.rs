#[cfg(test)]
mod tests {
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use crate::query_engine::bootstrap_run;
    use crate::run_risk_flow::handle_risk_outcome;
    use std::collections::BTreeMap;

    #[test]
    fn writes_permission_metadata_when_confirmation_required() {
        let request = sample_request("standard", "cmd: rm test.txt");
        let state = bootstrap_run(&request);
        let mut events = Vec::new();
        let mut sequence = 1;
        let response = handle_risk_outcome(&request, &state, &mut events, &mut sequence)
            .expect("should require confirmation");
        assert_eq!(response.result.status, "awaiting_confirmation");
        let plan = response
            .events
            .iter()
            .find(|item| item.event_type == "plan_ready")
            .expect("plan event");
        assert_eq!(
            plan.metadata.get("permission_decision").map(String::as_str),
            Some("require_confirmation")
        );
        assert_eq!(
            plan.metadata
                .get("permission_rule_layer")
                .map(String::as_str),
            Some("high_risk_guard")
        );
        assert_eq!(
            plan.metadata
                .get("confirmation_chain_step")
                .map(String::as_str),
            Some("required")
        );
        let result_error = response.result.error.expect("error info");
        assert_eq!(
            result_error
                .metadata
                .get("permission_decision")
                .map(String::as_str),
            Some("require_confirmation")
        );
        assert_eq!(
            result_error
                .metadata
                .get("permission_rule_layer")
                .map(String::as_str),
            Some("high_risk_guard")
        );
    }

    #[test]
    fn writes_permission_metadata_when_blocked_by_mode() {
        let request = sample_request("observe", "cmd: echo write > file.txt");
        let state = bootstrap_run(&request);
        let mut events = Vec::new();
        let mut sequence = 1;
        let response =
            handle_risk_outcome(&request, &state, &mut events, &mut sequence).expect("blocked");
        assert_eq!(response.result.status, "failed");
        let verify = response
            .events
            .iter()
            .find(|item| item.event_type == "verification_completed")
            .expect("verification event");
        assert_eq!(
            verify
                .metadata
                .get("permission_decision")
                .map(String::as_str),
            Some("blocked")
        );
        assert_eq!(
            verify
                .metadata
                .get("permission_rule_layer")
                .map(String::as_str),
            Some("mode_guard")
        );
        assert_eq!(
            verify
                .metadata
                .get("confirmation_chain_step")
                .map(String::as_str),
            Some("rule_blocked")
        );
    }

    fn sample_request(mode: &str, user_input: &str) -> RunRequest {
        let mut request = base_request();
        request.mode = mode.to_string();
        request.user_input = user_input.to_string();
        request
    }

    fn base_request() -> RunRequest {
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: String::new(),
            mode: String::new(),
            model_ref: sample_model_ref(),
            provider_ref: ProviderRef::default(),
            workspace_ref: sample_workspace_ref(),
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
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
}
