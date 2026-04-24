use super::*;
use serde_json::json;

fn sample_event(event_type: &str, stage: &str) -> RunEvent {
    serde_json::from_value(json!({
        "event_id": format!("event-{event_type}"),
        "event_type": event_type,
        "session_id": "session-1",
        "run_id": "run-1",
        "sequence": 1,
        "timestamp": "1000",
        "stage": stage,
        "summary": format!("{event_type} summary")
    }))
    .expect("sample event should be valid")
}

#[test]
fn should_map_lifecycle_event_kind() {
    assert_eq!(
        observation_kind_for_event_type("run_started"),
        Some("lifecycle_start")
    );
    assert_eq!(
        observation_kind_for_event_type("analysis_ready"),
        Some("lifecycle_analysis")
    );
    assert_eq!(observation_kind_for_event_type("unknown_event"), None);
}

#[test]
fn should_generate_full_coverage_snapshot() {
    let events = vec![
        sample_event("run_started", "Analyze"),
        sample_event("analysis_ready", "Analyze"),
        sample_event("plan_ready", "Plan"),
        sample_event("action_completed", "Observe"),
        sample_event("verification_completed", "Verify"),
        sample_event("run_finished", "Finish"),
    ];
    let snapshot = lifecycle_mapping_snapshot(&events);
    assert_eq!(snapshot.required_target_count, 6);
    assert_eq!(snapshot.mapped_target_count, 6);
    assert_eq!(snapshot.coverage_percent, 100.0);
    assert!(snapshot.missing_targets.is_empty());
}

#[test]
fn should_drop_duplicate_observations_in_time_window() {
    let events = vec![
        sample_event("run_started", "Analyze"),
        sample_event("run_started", "Analyze"),
        sample_event("analysis_ready", "Analyze"),
    ];
    let mapped = lifecycle_mapping_snapshot(&events);
    let dedupe = dedupe_lifecycle_observations(&mapped.mapped_items);
    assert_eq!(dedupe.total_incoming, 3);
    assert_eq!(dedupe.unique_count, 2);
    assert_eq!(dedupe.dropped_count, 1);
}

fn sample_request() -> RunRequest {
    RunRequest {
        request_id: "request-test".to_string(),
        run_id: "run-test".to_string(),
        session_id: "session-test".to_string(),
        trace_id: "trace-test".to_string(),
        user_input: "test".to_string(),
        mode: "standard".to_string(),
        model_ref: crate::contracts::ModelRef {
            provider_id: "local".to_string(),
            model_id: "test-model".to_string(),
            display_name: "test-model".to_string(),
        },
        provider_ref: crate::contracts::ProviderRef::default(),
        workspace_ref: crate::contracts::WorkspaceRef {
            workspace_id: format!("workspace-test-{}", crate::events::timestamp_now()),
            name: "workspace-test".to_string(),
            root_path: ".".to_string(),
            is_active: true,
        },
        context_hints: Default::default(),
        resume_from_checkpoint_id: String::new(),
        resume_strategy: String::new(),
        confirmation_decision: None,
    }
}

fn sample_lifecycle_events() -> Vec<RunEvent> {
    vec![
        sample_event("run_started", "Analyze"),
        sample_event("analysis_ready", "Analyze"),
        sample_event("plan_ready", "Plan"),
        sample_event("action_completed", "Observe"),
        sample_event("verification_completed", "Verify"),
        sample_event("run_finished", "Finish"),
    ]
}

fn sample_observation_ids(request: &RunRequest, query: &str) -> Vec<i64> {
    search_observations(request, query, 5)
        .items
        .into_iter()
        .map(|item| item.observation_id)
        .collect()
}

#[test]
fn should_search_observations_with_lightweight_fields() {
    let request = sample_request();
    let _ = persist_lifecycle_observations(&request, &sample_lifecycle_events());
    let report = search_observations(&request, "analysis", 5);
    assert!(report.total_hits >= 1);
    assert!(
        report
            .items
            .iter()
            .all(|item| item.summary_preview.len() <= 121)
    );
}

#[test]
fn should_build_timeline_from_query_anchor() {
    let request = sample_request();
    let _ = persist_lifecycle_observations(&request, &sample_lifecycle_events());
    let report = observation_timeline(&request, None, Some("plan"), 2);
    assert!(report.item_count >= 1);
    assert!(report.items.iter().any(|item| item.is_anchor));
}

#[test]
fn should_get_observations_by_ids() {
    let request = sample_request();
    let _ = persist_lifecycle_observations(&request, &sample_lifecycle_events());
    let ids = sample_observation_ids(&request, "analysis");
    let report = get_observations(&request, &ids, 3);
    assert!(report.returned_count >= 1);
    assert!(report.items.iter().all(|item| !item.summary.is_empty()));
}

#[test]
fn should_rank_observations_with_scoring_breakdown() {
    let request = sample_request();
    let _ = persist_lifecycle_observations(&request, &sample_lifecycle_events());
    let report = rank_observations(&request, "verification", 5);
    assert!(report.candidate_count >= 1);
    assert!(
        report
            .items
            .iter()
            .all(|item| item.total_score >= 0.0 && item.total_score <= 1.0)
    );
}

#[test]
fn should_redact_sensitive_observation_fields() {
    let request = sample_request();
    let events = vec![
        sample_event("run_started", "Analyze"),
        serde_json::from_value(json!({
            "event_id": "event-sensitive",
            "event_type": "analysis_ready",
            "session_id": "session-1",
            "run_id": "run-1",
            "sequence": 2,
            "timestamp": "1001",
            "stage": "Analyze",
            "summary": "token=sk-test-123"
        }))
        .expect("sensitive event should parse"),
    ];
    let report = observation_privacy_redact_flow(&request, &events);
    assert!(report.redacted_count >= 1);
    assert!(
        report
            .sample_summaries
            .iter()
            .any(|item| item.contains("[REDACTED]"))
    );
}

#[test]
fn should_skip_private_observations() {
    let events = vec![
        sample_event("run_started", "Analyze"),
        serde_json::from_value(json!({
            "event_id": "event-private",
            "event_type": "analysis_ready",
            "session_id": "session-1",
            "run_id": "run-1",
            "sequence": 2,
            "timestamp": "1001",
            "stage": "Analyze",
            "summary": "[PRIVATE] secret"
        }))
        .expect("private event should parse"),
    ];
    let report = observation_private_skip_flow(&events);
    assert!(report.private_marker_count >= 1);
    assert!(report.stored_count < report.incoming_count);
}

#[test]
fn should_rollback_to_legacy_when_feature_disabled() {
    let mut request = sample_request();
    request
        .context_hints
        .insert("memory_enhanced_enabled".to_string(), "false".to_string());
    let _ = persist_lifecycle_observations(&request, &sample_lifecycle_events());
    let report = observation_rollback_flow(&request, "analysis");
    assert!(report.fallback_to_legacy);
    assert!(!report.feature_enabled);
}

#[test]
fn should_resolve_budget_from_context_budget_tokens() {
    let mut request = sample_request();
    request
        .context_hints
        .insert("context_budget_tokens".to_string(), "512000".to_string());
    assert_eq!(resolve_observation_budget_chars(&request, 1200), 2_048_000);
}

#[test]
fn should_resolve_budget_from_codex_context_tokens_when_primary_missing() {
    let mut request = sample_request();
    request
        .context_hints
        .insert("codex_context_tokens".to_string(), "512000".to_string());
    assert_eq!(resolve_observation_budget_chars(&request, 300), 2_048_000);
}
