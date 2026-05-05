#[cfg(test)]
mod tests {
    use crate::capabilities::resolve_tool;
    use crate::planner::PlannedAction;
    use crate::query_engine::{RuntimeEnvelope, RuntimeRunState, bootstrap_run};
    use crate::query_engine_testkit::testkit::{sample_repo_context, sample_session};
    use crate::risk::assess_risk;
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use crate::run_risk_flow::handle_risk_outcome;
    use crate::skill_catalog::SkillCatalog;
    use crate::tool_registry::ToolCall;
    use std::collections::BTreeMap;

    #[test]
    fn writes_permission_metadata_when_confirmation_required() {
        let request = sample_request("standard", "cmd: rm test.txt");
        let state = bootstrap_run(&request);
        let mut events = Vec::new();
        let mut sequence = 1;
        let response =
            handle_risk_outcome(&request, &state, &mut events, &mut sequence).expect("should require confirmation");
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
            plan.metadata.get("permission_rule_layer").map(String::as_str),
            Some("high_risk_guard")
        );
        assert_eq!(
            plan.metadata.get("confirmation_chain_step").map(String::as_str),
            Some("required")
        );
        let result_error = response.result.error.expect("error info");
        assert_eq!(
            result_error.metadata.get("permission_decision").map(String::as_str),
            Some("require_confirmation")
        );
        assert_eq!(
            result_error.metadata.get("permission_rule_layer").map(String::as_str),
            Some("high_risk_guard")
        );
    }

    #[test]
    fn confirmation_event_includes_tool_arguments_json() {
        let request = sample_request("standard", "cmd: rm test.txt");
        let state = bootstrap_run(&request);
        let mut events = Vec::new();
        let mut sequence = 1;
        let response =
            handle_risk_outcome(&request, &state, &mut events, &mut sequence).expect("confirmation");
        let event = response.events.iter().find(|item| item.event_type == "confirmation_required").expect("event");
        assert_eq!(event.metadata.get("tool_name").map(String::as_str), Some("run_command"));
        assert_eq!(
            event.metadata.get("tool_arguments_json").map(String::as_str),
            Some(r#"{"command":"rm test.txt"}"#)
        );
    }

    #[test]
    fn patch_confirmation_carries_preview_payload() {
        let request = patch_request();
        let action = PlannedAction::ApplyPatch {
            diff: patch_diff().to_string(),
            dry_run: false,
        };
        let state = patch_state(&request, action);
        let mut events = Vec::new();
        let mut sequence = 1;
        let response =
            handle_risk_outcome(&request, &state, &mut events, &mut sequence).expect("confirmation");
        let confirmation = response.confirmation_request.expect("confirmation request");
        assert_eq!(confirmation.tool_name, "workspace_apply_patch");
        assert!(confirmation.tool_arguments_json.contains(r#""dry_run":false"#));
        assert!(confirmation.patch_preview_report_json.contains(r#""dry_run": true"#));
        let event = response
            .events
            .iter()
            .find(|item| item.event_type == "confirmation_required")
            .expect("event");
        assert_eq!(
            event.metadata.get("patch_preview_report_json").map(String::as_str),
            Some(confirmation.patch_preview_report_json.as_str())
        );
    }

    #[test]
    fn writes_permission_metadata_when_blocked_by_mode() {
        let request = sample_request("observe", "cmd: echo write > file.txt");
        let state = bootstrap_run(&request);
        let mut events = Vec::new();
        let mut sequence = 1;
        let response = handle_risk_outcome(&request, &state, &mut events, &mut sequence).expect("blocked");
        assert_eq!(response.result.status, "failed");
        let verify = response
            .events
            .iter()
            .find(|item| item.event_type == "verification_completed")
            .expect("verification event");
        assert_eq!(
            verify.metadata.get("permission_decision").map(String::as_str),
            Some("blocked")
        );
        assert_eq!(
            verify.metadata.get("permission_rule_layer").map(String::as_str),
            Some("mode_guard")
        );
        assert_eq!(
            verify.metadata.get("confirmation_chain_step").map(String::as_str),
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

    fn patch_request() -> RunRequest {
        let root = std::env::temp_dir().join(format!("risk-patch-{}", unique_id()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("sample.txt"), "hello\nold\nend").unwrap();
        let mut request = sample_request("standard", "agent: please patch sample.txt");
        request.workspace_ref.root_path = root.display().to_string();
        request
    }

    fn patch_state(request: &RunRequest, action: PlannedAction) -> RuntimeRunState {
        let base = bootstrap_run(request);
        let tool_call = ToolCall {
            spec: resolve_tool(&action),
            action: action.clone(),
        };
        RuntimeRunState {
            envelope: RuntimeEnvelope {
                request: request.clone(),
                session_context: sample_session(),
                repo_context: sample_repo_context(),
                skill_catalog: SkillCatalog::default(),
                context_envelope: base.envelope.context_envelope,
                visible_tools: base.envelope.visible_tools,
            },
            action: action.clone(),
            tool_call,
            task_title: "应用代码补丁".to_string(),
            analysis_detail: "patch risk test".to_string(),
            risk_outcome: assess_risk(request, &action),
            tool_trace: None,
            verification_report: None,
        }
    }

    fn patch_diff() -> &'static str {
        "--- a/sample.txt\n+++ b/sample.txt\n@@ -1,3 +1,3 @@\n hello\n-old\n+new\n end"
    }

    fn unique_id() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
