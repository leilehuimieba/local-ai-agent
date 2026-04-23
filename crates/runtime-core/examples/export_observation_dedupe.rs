use runtime_core::{RunEvent, dedupe_lifecycle_observations, lifecycle_mapping_snapshot};
use serde_json::json;

fn sample_event(event_type: &str, stage: &str, sequence: u32, timestamp: &str) -> RunEvent {
    serde_json::from_value(json!({
        "event_id": format!("event-{sequence}"),
        "event_type": event_type,
        "trace_id": "trace-m1",
        "session_id": "session-m1",
        "run_id": "run-m1",
        "sequence": sequence,
        "timestamp": timestamp,
        "stage": stage,
        "summary": format!("{event_type} sample")
    }))
    .expect("sample event must be valid")
}

fn sample_events() -> Vec<RunEvent> {
    vec![
        sample_event("run_started", "Analyze", 1, "1713000000000"),
        sample_event("run_started", "Analyze", 2, "1713000000100"),
        sample_event("analysis_ready", "Analyze", 3, "1713000000200"),
        sample_event("plan_ready", "Plan", 4, "1713000000300"),
        sample_event("action_completed", "Observe", 5, "1713000000400"),
        sample_event("verification_completed", "Verify", 6, "1713000000500"),
        sample_event("run_finished", "Finish", 7, "1713000000600"),
    ]
}

fn main() {
    let mapped = lifecycle_mapping_snapshot(&sample_events());
    let dedupe = dedupe_lifecycle_observations(&mapped.mapped_items);
    println!(
        "{}",
        serde_json::to_string_pretty(&dedupe).expect("dedupe report should serialize")
    );
}
