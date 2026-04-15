use runtime_core::{
    persist_lifecycle_observations, ModelRef, ProviderRef, RunEvent, RunRequest, WorkspaceRef,
};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn sample_event(event_type: &str, stage: &str, sequence: u32) -> RunEvent {
    serde_json::from_value(json!({
        "event_id": format!("event-{sequence}"),
        "event_type": event_type,
        "trace_id": "trace-m1",
        "session_id": "session-m1",
        "run_id": "run-m1",
        "sequence": sequence,
        "timestamp": "1713000000000",
        "stage": stage,
        "summary": format!("{event_type} sample")
    }))
    .expect("sample event must be valid")
}

fn lifecycle_events() -> Vec<RunEvent> {
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
    let workspace_id = format!("workspace-m1-fallback-{}", unix_millis());
    RunRequest {
        request_id: "mem-08-request".to_string(),
        run_id: "run-m1".to_string(),
        session_id: "session-m1".to_string(),
        trace_id: "trace-m1".to_string(),
        user_input: "memory observation fallback export".to_string(),
        mode: "standard".to_string(),
        model_ref: ModelRef {
            provider_id: "local".to_string(),
            model_id: "test-model".to_string(),
            display_name: "test-model".to_string(),
        },
        provider_ref: ProviderRef::default(),
        workspace_ref: workspace_ref_for(&workspace_id),
        context_hints: forced_sqlite_fail_hints(),
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

fn forced_sqlite_fail_hints() -> BTreeMap<String, String> {
    let mut hints = BTreeMap::new();
    hints.insert(
        "force_observation_sqlite_fail".to_string(),
        "true".to_string(),
    );
    hints
}

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

fn main() {
    let report = persist_lifecycle_observations(&sample_request(), &lifecycle_events());
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("report should serialize")
    );
}
