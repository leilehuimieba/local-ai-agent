use runtime_core::{
    ModelRef, ProviderRef, RunEvent, RunRequest, WorkspaceRef, build_layered_injection,
    compare_layered_vs_full, get_observations, persist_lifecycle_observations, rank_observations,
};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn sample_event(event_type: &str, stage: &str, sequence: u32, summary: &str) -> RunEvent {
    serde_json::from_value(json!({
        "event_id": format!("event-{sequence}"),
        "event_type": event_type,
        "trace_id": "trace-m4",
        "session_id": "session-m4",
        "run_id": "run-m4",
        "sequence": sequence,
        "timestamp": format!("{}", 1713000000000u128 + sequence as u128),
        "stage": stage,
        "summary": summary
    }))
    .expect("sample event must be valid")
}

fn sample_events() -> Vec<RunEvent> {
    vec![
        sample_event(
            "run_started",
            "Analyze",
            1,
            &long_summary("start runtime loop"),
        ),
        sample_event(
            "analysis_ready",
            "Analyze",
            2,
            &long_summary("analysis for injection verification budget"),
        ),
        sample_event(
            "plan_ready",
            "Plan",
            3,
            &long_summary("plan layered injection verification"),
        ),
        sample_event(
            "action_completed",
            "Observe",
            4,
            &long_summary("action writes detailed artifact path for injection"),
        ),
        sample_event(
            "verification_completed",
            "Verify",
            5,
            &long_summary("verification keeps quality baseline for injection"),
        ),
        sample_event(
            "run_finished",
            "Finish",
            6,
            &long_summary("finish with references"),
        ),
    ]
}

fn long_summary(seed: &str) -> String {
    (0..8)
        .map(|index| format!("{seed}-segment-{index}"))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn sample_request() -> RunRequest {
    let workspace_id = format!("workspace-m4-injection-{}", unix_millis());
    RunRequest {
        request_id: "mem-18-request".to_string(),
        run_id: "run-m4".to_string(),
        session_id: "session-m4".to_string(),
        trace_id: "trace-m4".to_string(),
        user_input: "inject runtime memory".to_string(),
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
    let budget_total = 800usize;
    let _ = persist_lifecycle_observations(&request, &sample_events());
    let injection = build_layered_injection(&request, "injection verification", budget_total);
    let ab = ab_with_full_query(
        &request,
        "injection verification",
        "injection",
        budget_total,
    );
    let report = json!({
        "injection": injection,
        "ab_test": ab
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("injection report should serialize")
    );
}

fn ab_with_full_query(
    request: &RunRequest,
    layered_query: &str,
    full_query: &str,
    budget_total: usize,
) -> serde_json::Value {
    let layered = compare_layered_vs_full(request, layered_query, budget_total);
    let full = full_context_chars(request, layered_query, full_query);
    let layered_chars = layered.layered_context_chars;
    let saved_chars = full.saturating_sub(layered_chars);
    let saved_percent = if full == 0 {
        0.0
    } else {
        (saved_chars as f64 * 100.0) / full as f64
    };
    json!({
        "query": layered_query,
        "full_query": full_query,
        "full_context_chars": full,
        "layered_context_chars": layered_chars,
        "saved_chars": saved_chars,
        "saved_percent": saved_percent,
        "quality_preserved": layered.quality_preserved
    })
}

fn full_context_chars(request: &RunRequest, primary_query: &str, fallback_query: &str) -> usize {
    let ranked = rank_observations(request, primary_query, 20);
    let active = if ranked.items.is_empty() {
        rank_observations(request, fallback_query, 20)
    } else {
        ranked
    };
    let ids = active
        .items
        .iter()
        .map(|item| item.observation_id)
        .collect::<Vec<_>>();
    get_observations(request, &ids, 20)
        .items
        .iter()
        .map(|item| item.summary.len() + item.artifact_ref.len() + item.event_type.len())
        .sum()
}
