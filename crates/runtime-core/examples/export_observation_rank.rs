use runtime_core::{
    persist_lifecycle_observations, rank_observations, ModelRef, ProviderRef, RunEvent,
    RunRequest, WorkspaceRef,
};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn sample_event(event_type: &str, stage: &str, sequence: u32, summary: &str) -> RunEvent {
    serde_json::from_value(json!({
        "event_id": format!("event-{sequence}"),
        "event_type": event_type,
        "trace_id": "trace-m3",
        "session_id": "session-m3",
        "run_id": "run-m3",
        "sequence": sequence,
        "timestamp": format!("{}", 1713000000000u128 + sequence as u128),
        "stage": stage,
        "summary": summary
    }))
    .expect("sample event must be valid")
}

fn sample_events() -> Vec<RunEvent> {
    vec![
        sample_event("run_started", "Analyze", 1, "rank baseline"),
        sample_event("analysis_ready", "Analyze", 2, "rank keyword analysis"),
        sample_event("plan_ready", "Plan", 3, "rank plan"),
        sample_event("action_completed", "Observe", 4, "rank action completed"),
        sample_event("verification_completed", "Verify", 5, "rank verification completed"),
        sample_event("run_finished", "Finish", 6, "rank finish"),
    ]
}

fn sample_request() -> RunRequest {
    let workspace_id = format!("workspace-m3-rank-{}", unix_millis());
    RunRequest {
        request_id: "mem-16-request".to_string(),
        run_id: "run-m3".to_string(),
        session_id: "session-m3".to_string(),
        trace_id: "trace-m3".to_string(),
        user_input: "rank observations".to_string(),
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
    let _ = persist_lifecycle_observations(&request, &sample_events());
    let report = rank_observations(&request, "rank verification", 5);
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("rank report should serialize")
    );
}
