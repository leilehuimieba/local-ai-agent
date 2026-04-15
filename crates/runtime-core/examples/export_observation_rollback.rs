use runtime_core::{
    observation_rollback_flow, persist_lifecycle_observations, ModelRef, ProviderRef, RunEvent,
    RunRequest, WorkspaceRef,
};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn sample_event(event_type: &str, stage: &str, sequence: u32, summary: &str) -> RunEvent {
    serde_json::from_value(json!({
        "event_id": format!("event-{sequence}"),
        "event_type": event_type,
        "trace_id": "trace-m5",
        "session_id": "session-m5",
        "run_id": "run-m5",
        "sequence": sequence,
        "timestamp": format!("{}", 1713000000000u128 + sequence as u128),
        "stage": stage,
        "summary": summary
    }))
    .expect("sample event must be valid")
}

fn sample_events() -> Vec<RunEvent> {
    vec![
        sample_event("run_started", "Analyze", 1, "start"),
        sample_event("analysis_ready", "Analyze", 2, "analysis rollback"),
        sample_event("plan_ready", "Plan", 3, "plan rollback"),
        sample_event("action_completed", "Observe", 4, "action rollback"),
        sample_event("verification_completed", "Verify", 5, "verify rollback"),
        sample_event("run_finished", "Finish", 6, "finish rollback"),
    ]
}

fn sample_request(enabled: bool) -> RunRequest {
    let workspace_id = format!("workspace-m5-rollback-{}", unix_millis());
    let mut context_hints = BTreeMap::new();
    if !enabled {
        context_hints.insert("memory_enhanced_enabled".to_string(), "false".to_string());
    }
    RunRequest {
        request_id: "mem-24-request".to_string(),
        run_id: "run-m5".to_string(),
        session_id: "session-m5".to_string(),
        trace_id: "trace-m5".to_string(),
        user_input: "rollback memory enhanced".to_string(),
        mode: "standard".to_string(),
        model_ref: ModelRef {
            provider_id: "local".to_string(),
            model_id: "test-model".to_string(),
            display_name: "test-model".to_string(),
        },
        provider_ref: ProviderRef::default(),
        workspace_ref: workspace_ref_for(&workspace_id),
        context_hints,
        resume_from_checkpoint_id: String::new(),
        resume_strategy: String::new(),
        confirmation_decision: None,
    }
}

fn workspace_ref_for(workspace_id: &str) -> WorkspaceRef {
    WorkspaceRef {
        workspace_id: workspace_id.to_string(),
        name: workspace_id.to_string(),
        root_path: ".".to_string(),
        is_active: true,
    }
}

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

fn main() {
    let enabled_request = sample_request(true);
    let _ = persist_lifecycle_observations(&enabled_request, &sample_events());
    let enabled_report = observation_rollback_flow(&enabled_request, "rollback");

    let disabled_request = sample_request(false);
    let _ = persist_lifecycle_observations(&disabled_request, &sample_events());
    let disabled_report = observation_rollback_flow(&disabled_request, "rollback");

    let report = json!({
        "enabled": enabled_report,
        "disabled": disabled_report
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("rollback report should serialize")
    );
}
