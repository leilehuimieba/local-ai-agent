use runtime_core::{
    observation_queue_health, run_observation_queue_flow, ModelRef, ProviderRef, RunEvent,
    RunRequest, WorkspaceRef,
};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn sample_event(event_type: &str, stage: &str, sequence: u32) -> RunEvent {
    serde_json::from_value(json!({
        "event_id": format!("event-{sequence}"),
        "event_type": event_type,
        "trace_id": "trace-m2",
        "session_id": "session-m2",
        "run_id": "run-m2",
        "sequence": sequence,
        "timestamp": "1713000000000",
        "stage": stage,
        "summary": format!("{event_type} sample")
    }))
    .expect("sample event must be valid")
}

fn sample_events() -> Vec<RunEvent> {
    vec![
        sample_event("run_started", "Analyze", 1),
        sample_event("analysis_ready", "Analyze", 2),
        sample_event("plan_ready", "Plan", 3),
        sample_event("action_completed", "Observe", 4),
        sample_event("verification_completed", "Verify", 5),
        sample_event("run_finished", "Finish", 6),
    ]
}

fn sample_request() -> RunRequest {
    let workspace_id = format!("workspace-m2-health-{}", unix_millis());
    RunRequest {
        request_id: "mem-12-request".to_string(),
        run_id: "run-m2".to_string(),
        session_id: "session-m2".to_string(),
        trace_id: "trace-m2".to_string(),
        user_input: "health report".to_string(),
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
    let _ = run_observation_queue_flow(&request, &sample_events());
    let report = observation_queue_health(&request);
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("health report should serialize")
    );
}
