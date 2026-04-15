use runtime_core::{lifecycle_mapping_snapshot, RunEvent};
use serde_json::json;

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

fn main() {
    let snapshot = lifecycle_mapping_snapshot(&lifecycle_events());
    println!(
        "{}",
        serde_json::to_string_pretty(&snapshot).expect("snapshot should serialize")
    );
}
