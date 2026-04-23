use runtime_core::{RunEvent, observation_private_skip_flow};
use serde_json::json;

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
        sample_event("run_started", "Analyze", 1, "normal start"),
        sample_event("analysis_ready", "Analyze", 2, "[PRIVATE] hidden"),
        sample_event("plan_ready", "Plan", 3, "privacy:private internal"),
        sample_event("action_completed", "Observe", 4, "private=true should skip"),
        sample_event("verification_completed", "Verify", 5, "normal verify"),
        sample_event("run_finished", "Finish", 6, "finish"),
    ]
}

fn main() {
    let report = observation_private_skip_flow(&sample_events());
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("private skip report should serialize")
    );
}
