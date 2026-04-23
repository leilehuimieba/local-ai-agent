use runtime_core::{
    ModelRef, ProviderRef, RunEvent, RunRequest, WorkspaceRef, observation_privacy_redact_flow,
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
        sample_event("analysis_ready", "Analyze", 2, "api_key=sk-test-123"),
        sample_event("plan_ready", "Plan", 3, "contact user.demo@example.com"),
        sample_event("action_completed", "Observe", 4, "phone 13800138000"),
        sample_event("verification_completed", "Verify", 5, "normal verify"),
        sample_event("run_finished", "Finish", 6, "finish"),
    ]
}

fn sample_request() -> RunRequest {
    let workspace_id = format!("workspace-m5-redact-{}", unix_millis());
    RunRequest {
        request_id: "mem-22-request".to_string(),
        run_id: "run-m5".to_string(),
        session_id: "session-m5".to_string(),
        trace_id: "trace-m5".to_string(),
        user_input: "privacy redact".to_string(),
        mode: "standard".to_string(),
        model_ref: ModelRef {
            provider_id: "local".to_string(),
            model_id: "test-model".to_string(),
            display_name: "test-model".to_string(),
        },
        provider_ref: ProviderRef::default(),
        workspace_ref: workspace_ref_for(&workspace_id),
        context_hints: BTreeMap::new(),
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
    let request = sample_request();
    let report = observation_privacy_redact_flow(&request, &sample_events());
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("redact report should serialize")
    );
}
