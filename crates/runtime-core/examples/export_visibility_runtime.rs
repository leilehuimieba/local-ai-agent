use runtime_core::{
    ModelRef, ProviderRef, RunRequest, WorkspaceRef, simulate_run_with_runtime_events,
};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|it| it.as_millis())
        .unwrap_or_default()
}

fn sample_request() -> RunRequest {
    let stamp = unix_millis();
    RunRequest {
        request_id: format!("h01-request-{stamp}"),
        run_id: format!("h01-run-{stamp}"),
        session_id: format!("h01-session-{stamp}"),
        trace_id: format!("h01-trace-{stamp}"),
        user_input: "cmd: Get-Date".to_string(),
        mode: "standard".to_string(),
        model_ref: ModelRef {
            provider_id: "local".to_string(),
            model_id: "test-model".to_string(),
            display_name: "test-model".to_string(),
        },
        provider_ref: ProviderRef::default(),
        workspace_ref: WorkspaceRef {
            workspace_id: format!("h01-workspace-{stamp}"),
            name: "h01-workspace".to_string(),
            root_path: ".".to_string(),
            is_active: true,
        },
        context_hints: BTreeMap::new(),
        resume_from_checkpoint_id: String::new(),
        resume_strategy: String::new(),
        confirmation_decision: None,
    }
}

fn coverage(events: &[runtime_core::RunEvent], key: &str) -> usize {
    events
        .iter()
        .filter(|event| {
            event
                .metadata
                .get(key)
                .map(|v| !v.is_empty())
                .unwrap_or(false)
        })
        .count()
}

fn main() {
    let request = sample_request();
    let response = simulate_run_with_runtime_events(&request);
    let total = response.events.len();
    let report = json!({
      "checked_at": unix_millis(),
      "status": if total > 0 { "passed" } else { "failed" },
      "summary": {
        "event_count": total,
        "activity_state_coverage": coverage(&response.events, "activity_state"),
        "heartbeat_at_coverage": coverage(&response.events, "heartbeat_at"),
        "task_title_coverage": coverage(&response.events, "task_title"),
        "next_action_hint_coverage": coverage(&response.events, "next_action_hint"),
        "failure_route_coverage": coverage(&response.events, "failure_route"),
      },
      "events": response.events
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
    );
}
