use crate::contracts::{RunEvent, RunRequest};
use crate::paths::observation_audit_file_path;
use crate::sensitive_data::{contains_private_marker, contains_sensitive_text, redact_sensitive_text};
use crate::sqlite_store::{insert_observation_record, with_connection};
use crate::storage::{append_jsonl, read_jsonl};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};

const LIFECYCLE_TARGET_EVENTS: [&str; 6] = [
    "run_started",
    "analysis_ready",
    "plan_ready",
    "action_completed",
    "verification_completed",
    "run_finished",
];
const DEDUPE_WINDOW_MILLIS: u128 = 300000;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationRecord {
    pub event_type: String,
    pub observation_kind: String,
    pub stage: String,
    pub summary: String,
    pub tool_name: String,
    pub artifact_ref: String,
    pub event_timestamp: String,
    pub trace_id: String,
    pub run_id: String,
    pub session_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LifecycleMappingSnapshot {
    pub required_target_count: usize,
    pub mapped_target_count: usize,
    pub coverage_percent: f64,
    pub missing_targets: Vec<String>,
    pub mapped_items: Vec<ObservationRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationPersistenceReport {
    pub target_event_count: usize,
    pub mapped_event_count: usize,
    pub dedupe_input_count: usize,
    pub dedupe_unique_count: usize,
    pub dedupe_dropped_count: usize,
    pub sqlite_written_count: usize,
    pub audit_written_count: usize,
    pub sqlite_total_rows: usize,
    pub audit_total_rows: usize,
    pub fallback_applied: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationDedupeReport {
    pub total_incoming: usize,
    pub unique_count: usize,
    pub dropped_count: usize,
    pub dropped_keys: Vec<String>,
    pub unique_items: Vec<ObservationRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationQueueFlowReport {
    pub queued_count: usize,
    pub pending_count: usize,
    pub processing_count: usize,
    pub processed_count: usize,
    pub failed_count: usize,
    pub status_sequence: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RetryAttempt {
    pub attempt: u32,
    pub backoff_ms: u64,
    pub success: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationRetryReport {
    pub initial_failed_count: usize,
    pub retried_count: usize,
    pub processed_after_retry_count: usize,
    pub remaining_failed_count: usize,
    pub attempts: Vec<RetryAttempt>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationQueueHealthReport {
    pub total_count: usize,
    pub pending_count: usize,
    pub processing_count: usize,
    pub processed_count: usize,
    pub failed_count: usize,
    pub healthy: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationSearchItem {
    pub observation_id: i64,
    pub event_type: String,
    pub observation_kind: String,
    pub stage: String,
    pub summary_preview: String,
    pub trace_id: String,
    pub run_id: String,
    pub session_id: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationSearchReport {
    pub query: String,
    pub limit: usize,
    pub total_hits: usize,
    pub items: Vec<ObservationSearchItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationTimelineItem {
    pub observation_id: i64,
    pub event_type: String,
    pub stage: String,
    pub summary_preview: String,
    pub created_at: String,
    pub is_anchor: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationTimelineReport {
    pub anchor_id: i64,
    pub anchor_source: String,
    pub window: usize,
    pub item_count: usize,
    pub items: Vec<ObservationTimelineItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationDetailItem {
    pub observation_id: i64,
    pub event_type: String,
    pub observation_kind: String,
    pub stage: String,
    pub summary: String,
    pub tool_name: String,
    pub artifact_ref: String,
    pub trace_id: String,
    pub run_id: String,
    pub session_id: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationGetReport {
    pub requested_count: usize,
    pub returned_count: usize,
    pub limit: usize,
    pub items: Vec<ObservationDetailItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ObservationRankItem {
    pub observation_id: i64,
    pub event_type: String,
    pub stage: String,
    pub source_weight: f64,
    pub freshness_score: f64,
    pub keyword_score: f64,
    pub total_score: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ObservationRankReport {
    pub query: String,
    pub limit: usize,
    pub candidate_count: usize,
    pub items: Vec<ObservationRankItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationLayeredInjectionReport {
    pub query: String,
    pub budget_total_chars: usize,
    pub budget_total_tokens: usize,
    pub summary_budget_chars: usize,
    pub summary_budget_tokens: usize,
    pub timeline_budget_chars: usize,
    pub timeline_budget_tokens: usize,
    pub details_budget_chars: usize,
    pub details_budget_tokens: usize,
    pub used_chars: usize,
    pub used_tokens: usize,
    pub budget_hit: bool,
    pub budget_hit_tokens: bool,
    pub summary_section: String,
    pub timeline_section: String,
    pub details_section: String,
    pub references: Vec<String>,
    pub injected_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ObservationAbTestReport {
    pub query: String,
    pub full_context_chars: usize,
    pub full_context_tokens: usize,
    pub layered_context_chars: usize,
    pub layered_context_tokens: usize,
    pub saved_chars: usize,
    pub saved_tokens: usize,
    pub saved_percent: f64,
    pub quality_preserved: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationPrivacyRedactReport {
    pub incoming_count: usize,
    pub redacted_count: usize,
    pub private_skipped_count: usize,
    pub stored_count: usize,
    pub sample_summaries: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationPrivateSkipReport {
    pub incoming_count: usize,
    pub private_marker_count: usize,
    pub stored_count: usize,
    pub skipped_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationRollbackReport {
    pub feature_enabled: bool,
    pub fallback_to_legacy: bool,
    pub search_hit_count: usize,
    pub injection_used_chars: usize,
    pub references_count: usize,
}

#[derive(Default)]
struct PersistenceOutcome {
    written_count: usize,
    errors: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StoredObservationRow {
    id: i64,
    event_type: String,
    observation_kind: String,
    stage: String,
    summary: String,
    tool_name: String,
    artifact_ref: String,
    trace_id: String,
    run_id: String,
    session_id: String,
    created_at: String,
}

#[derive(Clone, Debug, PartialEq)]
struct ScoredObservationRow {
    row: StoredObservationRow,
    source_weight: f64,
    freshness_score: f64,
    keyword_score: f64,
    total_score: f64,
}

#[derive(Clone, Debug, Default)]
struct PrivacyApplyStats {
    redacted_count: usize,
    private_skipped_count: usize,
}

#[derive(Clone, Debug, Default)]
struct PrivacyApplyResult {
    records: Vec<ObservationRecord>,
    stats: PrivacyApplyStats,
}

pub fn lifecycle_target_event_types() -> Vec<String> {
    LIFECYCLE_TARGET_EVENTS
        .iter()
        .map(|item| item.to_string())
        .collect()
}

pub fn observation_kind_for_event_type(event_type: &str) -> Option<&'static str> {
    match event_type {
        "run_started" => Some("lifecycle_start"),
        "analysis_ready" => Some("lifecycle_analysis"),
        "plan_ready" => Some("lifecycle_plan"),
        "action_completed" => Some("lifecycle_action"),
        "verification_completed" => Some("lifecycle_verification"),
        "run_finished" => Some("lifecycle_finish"),
        _ => None,
    }
}

pub fn observation_from_event(event: &RunEvent) -> Option<ObservationRecord> {
    let observation_kind = observation_kind_for_event_type(&event.event_type)?;
    Some(ObservationRecord {
        event_type: event.event_type.clone(),
        observation_kind: observation_kind.to_string(),
        stage: event.stage.clone(),
        summary: event.summary.clone(),
        tool_name: event.tool_name.clone(),
        artifact_ref: event.artifact_path.clone(),
        event_timestamp: event.timestamp.clone(),
        trace_id: event.trace_id.clone(),
        run_id: event.run_id.clone(),
        session_id: event.session_id.clone(),
    })
}

fn collect_mapped_items(events: &[RunEvent]) -> Vec<ObservationRecord> {
    events.iter().filter_map(observation_from_event).collect()
}

fn mapped_target_set(items: &[ObservationRecord]) -> BTreeSet<String> {
    items.iter().map(|item| item.event_type.clone()).collect()
}

fn missing_targets(mapped_targets: &BTreeSet<String>) -> Vec<String> {
    LIFECYCLE_TARGET_EVENTS
        .iter()
        .filter(|item| !mapped_targets.contains(**item))
        .map(|item| item.to_string())
        .collect()
}

fn coverage_percent(required: usize, mapped: usize) -> f64 {
    if required == 0 {
        return 100.0;
    }
    let ratio = mapped as f64 / required as f64;
    (ratio * 10000.0).round() / 100.0
}

pub fn lifecycle_mapping_snapshot(events: &[RunEvent]) -> LifecycleMappingSnapshot {
    let mapped_items = collect_mapped_items(events);
    let mapped_targets = mapped_target_set(&mapped_items);
    let missing_targets = missing_targets(&mapped_targets);
    let required_target_count = LIFECYCLE_TARGET_EVENTS.len();
    let mapped_target_count = required_target_count.saturating_sub(missing_targets.len());
    LifecycleMappingSnapshot {
        required_target_count,
        mapped_target_count,
        coverage_percent: coverage_percent(required_target_count, mapped_target_count),
        missing_targets,
        mapped_items,
    }
}

pub fn dedupe_lifecycle_observations(records: &[ObservationRecord]) -> ObservationDedupeReport {
    let mut seen = BTreeSet::new();
    let mut dropped_keys = Vec::new();
    let mut unique_items = Vec::new();
    for record in records {
        let key = dedupe_key(record);
        if seen.insert(key.clone()) {
            unique_items.push(record.clone());
        } else {
            dropped_keys.push(key);
        }
    }
    ObservationDedupeReport {
        total_incoming: records.len(),
        unique_count: unique_items.len(),
        dropped_count: dropped_keys.len(),
        dropped_keys,
        unique_items,
    }
}

pub fn persist_lifecycle_observations(
    request: &RunRequest,
    events: &[RunEvent],
) -> ObservationPersistenceReport {
    let snapshot = lifecycle_mapping_snapshot(events);
    let dedupe = dedupe_lifecycle_observations(&snapshot.mapped_items);
    let sqlite = write_sqlite_records(request, &dedupe.unique_items);
    let audit = write_audit_records(request, &dedupe.unique_items);
    build_persistence_report(request, &snapshot, &dedupe, sqlite, audit)
}

pub fn run_observation_queue_flow(
    request: &RunRequest,
    events: &[RunEvent],
) -> ObservationQueueFlowReport {
    let records = dedupe_lifecycle_observations(&lifecycle_mapping_snapshot(events).mapped_items);
    let mut errors = Vec::new();
    reset_observation_queue(request, &mut errors);
    let queued_count = enqueue_pending_records(request, &records.unique_items, &mut errors);
    let pending_count = queue_count_by_status(request, "pending");
    let processing_count = pending_to_processing(request, &mut errors);
    let failed_count = processing_to_failed(request, &mut errors);
    let processed_count = processing_to_processed(request, &mut errors);
    ObservationQueueFlowReport {
        queued_count,
        pending_count,
        processing_count,
        processed_count,
        failed_count,
        status_sequence: queue_status_sequence(failed_count),
        errors,
    }
}

pub fn run_observation_retry_flow(
    request: &RunRequest,
    events: &[RunEvent],
) -> ObservationRetryReport {
    let _ = run_observation_queue_flow(request, events);
    let mut errors = Vec::new();
    let initial_failed_count = queue_count_by_status(request, "failed");
    let attempts = retry_failed_once(request, &mut errors);
    let retried_count = attempts.iter().filter(|item| item.success).count();
    ObservationRetryReport {
        initial_failed_count,
        retried_count,
        processed_after_retry_count: queue_count_by_status(request, "processed"),
        remaining_failed_count: queue_count_by_status(request, "failed"),
        attempts,
        errors,
    }
}

pub fn observation_queue_health(request: &RunRequest) -> ObservationQueueHealthReport {
    let pending_count = queue_count_by_status(request, "pending");
    let processing_count = queue_count_by_status(request, "processing");
    let processed_count = queue_count_by_status(request, "processed");
    let failed_count = queue_count_by_status(request, "failed");
    ObservationQueueHealthReport {
        total_count: pending_count + processing_count + processed_count + failed_count,
        pending_count,
        processing_count,
        processed_count,
        failed_count,
        healthy: failed_count == 0 && processing_count == 0,
    }
}

pub fn search_observations(
    request: &RunRequest,
    query: &str,
    limit: usize,
) -> ObservationSearchReport {
    let normalized_limit = normalize_limit(limit, 20);
    let rows = search_rows(request, query, normalized_limit);
    ObservationSearchReport {
        query: query.to_string(),
        limit: normalized_limit,
        total_hits: rows.len(),
        items: rows.iter().map(search_item_from_row).collect(),
    }
}

pub fn observation_timeline(
    request: &RunRequest,
    anchor_id: Option<i64>,
    query: Option<&str>,
    window: usize,
) -> ObservationTimelineReport {
    let normalized_window = normalize_window(window);
    let (resolved_anchor, source) = resolve_anchor(request, anchor_id, query);
    match resolved_anchor {
        Some(anchor) => timeline_report(request, anchor, source, normalized_window),
        None => ObservationTimelineReport {
            anchor_id: 0,
            anchor_source: source,
            window: normalized_window,
            item_count: 0,
            items: Vec::new(),
        },
    }
}

pub fn get_observations(
    request: &RunRequest,
    ids: &[i64],
    limit: usize,
) -> ObservationGetReport {
    let normalized_limit = normalize_limit(limit, 20);
    let normalized_ids = normalized_observation_ids(ids, normalized_limit);
    let rows = rows_by_ids(request, &normalized_ids);
    ObservationGetReport {
        requested_count: ids.len(),
        returned_count: rows.len(),
        limit: normalized_limit,
        items: rows.iter().map(detail_item_from_row).collect(),
    }
}

pub fn rank_observations(
    request: &RunRequest,
    query: &str,
    limit: usize,
) -> ObservationRankReport {
    let normalized_limit = normalize_limit(limit, 5);
    let candidates = search_rows(request, query, 100);
    let mut scored = scored_rows(query, &candidates);
    scored.sort_by(compare_scored_rows);
    ObservationRankReport {
        query: query.to_string(),
        limit: normalized_limit,
        candidate_count: scored.len(),
        items: scored
            .into_iter()
            .take(normalized_limit)
            .map(rank_item_from_scored)
            .collect(),
    }
}

pub fn build_layered_injection(
    request: &RunRequest,
    query: &str,
    budget_total_chars: usize,
) -> ObservationLayeredInjectionReport {
    let total = normalize_budget_total_chars(budget_total_chars);
    let (summary_budget, timeline_budget, details_budget) = split_budgets(total);
    let ranked = rank_observations(request, query, 20);
    let details = get_observation_details(request, &ranked);
    let (summary_section, timeline_section, details_section) =
        layered_sections(&ranked.items, &details, summary_budget, timeline_budget, details_budget);
    let references = build_references(&details);
    layered_injection_report(
        query,
        LayerCharBudgets::new(total, summary_budget, timeline_budget, details_budget),
        LayerSections::new(summary_section, timeline_section, details_section),
        references,
    )
}

pub fn compare_layered_vs_full(
    request: &RunRequest,
    query: &str,
    budget_total_chars: usize,
) -> ObservationAbTestReport {
    let layered = build_layered_injection(request, query, budget_total_chars);
    let full_context = full_context_text(request, query);
    let full_chars = full_context.chars().count();
    let full_tokens = estimate_tokens_from_chars(full_chars);
    let layered_chars = layered.injected_text.chars().count();
    let saved_chars = full_chars.saturating_sub(layered_chars);
    let saved_tokens = full_tokens.saturating_sub(layered.used_tokens);
    ObservationAbTestReport {
        query: query.to_string(),
        full_context_chars: full_chars,
        full_context_tokens: full_tokens,
        layered_context_chars: layered_chars,
        layered_context_tokens: layered.used_tokens,
        saved_chars,
        saved_tokens,
        saved_percent: saved_percent_for_report(full_chars, saved_chars),
        quality_preserved: !layered.references.is_empty(),
    }
}

#[derive(Clone, Copy)]
struct LayerBudgetReport {
    used_chars: usize,
    total_tokens: usize,
    summary_tokens: usize,
    timeline_tokens: usize,
    details_tokens: usize,
    used_tokens: usize,
}

#[derive(Clone, Copy)]
struct LayerCharBudgets {
    total: usize,
    summary: usize,
    timeline: usize,
    details: usize,
}

impl LayerCharBudgets {
    fn new(total: usize, summary: usize, timeline: usize, details: usize) -> Self {
        Self {
            total,
            summary,
            timeline,
            details,
        }
    }
}

struct LayerSections {
    summary: String,
    timeline: String,
    details: String,
}

impl LayerSections {
    fn new(summary: String, timeline: String, details: String) -> Self {
        Self {
            summary,
            timeline,
            details,
        }
    }
}

fn layered_sections(
    ranked: &[ObservationRankItem],
    details: &[ObservationDetailItem],
    summary_budget: usize,
    timeline_budget: usize,
    details_budget: usize,
) -> (String, String, String) {
    (
        build_summary_section(ranked, summary_budget),
        build_timeline_section(details, timeline_budget),
        build_details_section(details, details_budget),
    )
}

fn layer_budget_report(
    total_chars: usize,
    summary_chars: usize,
    timeline_chars: usize,
    details_chars: usize,
    used_chars: usize,
) -> LayerBudgetReport {
    LayerBudgetReport {
        used_chars,
        total_tokens: estimate_tokens_from_chars(total_chars),
        summary_tokens: estimate_tokens_from_chars(summary_chars),
        timeline_tokens: estimate_tokens_from_chars(timeline_chars),
        details_tokens: estimate_tokens_from_chars(details_chars),
        used_tokens: estimate_tokens_from_chars(used_chars),
    }
}

fn layered_injection_report(
    query: &str,
    budgets: LayerCharBudgets,
    sections: LayerSections,
    references: Vec<String>,
) -> ObservationLayeredInjectionReport {
    let budget = layer_budget_for_sections(budgets, &sections);
    let injected_text = join_layered_sections(&sections.summary, &sections.timeline, &sections.details);
    ObservationLayeredInjectionReport {
        query: query.to_string(),
        budget_total_chars: budgets.total,
        budget_total_tokens: budget.total_tokens,
        summary_budget_chars: budgets.summary,
        summary_budget_tokens: budget.summary_tokens,
        timeline_budget_chars: budgets.timeline,
        timeline_budget_tokens: budget.timeline_tokens,
        details_budget_chars: budgets.details,
        details_budget_tokens: budget.details_tokens,
        used_chars: budget.used_chars,
        used_tokens: budget.used_tokens,
        budget_hit: budget.used_chars >= budgets.total,
        budget_hit_tokens: budget.used_tokens >= budget.total_tokens,
        summary_section: sections.summary,
        timeline_section: sections.timeline,
        details_section: sections.details,
        references,
        injected_text,
    }
}

fn layer_budget_for_sections(budgets: LayerCharBudgets, sections: &LayerSections) -> LayerBudgetReport {
    layer_budget_report(
        budgets.total,
        budgets.summary,
        budgets.timeline,
        budgets.details,
        layer_used_chars(sections),
    )
}

fn layer_used_chars(sections: &LayerSections) -> usize {
    join_layered_sections(&sections.summary, &sections.timeline, &sections.details)
        .chars()
        .count()
}

pub fn observation_privacy_redact_flow(
    request: &RunRequest,
    events: &[RunEvent],
) -> ObservationPrivacyRedactReport {
    let dedupe = dedupe_lifecycle_observations(&lifecycle_mapping_snapshot(events).mapped_items);
    let applied = apply_privacy_rules(&dedupe.unique_items);
    let _ = write_sqlite_records(request, &applied.records);
    ObservationPrivacyRedactReport {
        incoming_count: dedupe.unique_count,
        redacted_count: applied.stats.redacted_count,
        private_skipped_count: applied.stats.private_skipped_count,
        stored_count: applied.records.len(),
        sample_summaries: applied
            .records
            .iter()
            .map(|item| item.summary.clone())
            .take(3)
            .collect(),
    }
}

pub fn observation_private_skip_flow(
    events: &[RunEvent],
) -> ObservationPrivateSkipReport {
    let dedupe = dedupe_lifecycle_observations(&lifecycle_mapping_snapshot(events).mapped_items);
    let marked = private_marked_records(&dedupe.unique_items);
    ObservationPrivateSkipReport {
        incoming_count: dedupe.unique_count,
        private_marker_count: marked.len(),
        stored_count: dedupe.unique_count.saturating_sub(marked.len()),
        skipped_ids: marked,
    }
}

pub fn observation_rollback_flow(
    request: &RunRequest,
    query: &str,
) -> ObservationRollbackReport {
    let enabled = memory_enhanced_enabled(request);
    if !enabled {
        let search = search_observations(request, query, 5);
        return ObservationRollbackReport {
            feature_enabled: false,
            fallback_to_legacy: true,
            search_hit_count: search.total_hits,
            injection_used_chars: 0,
            references_count: 0,
        };
    }
    let injection = build_layered_injection(request, query, 800);
    let search = search_observations(request, query, 5);
    ObservationRollbackReport {
        feature_enabled: true,
        fallback_to_legacy: false,
        search_hit_count: search.total_hits,
        injection_used_chars: injection.used_chars,
        references_count: injection.references.len(),
    }
}

fn apply_privacy_rules(records: &[ObservationRecord]) -> PrivacyApplyResult {
    let mut result = PrivacyApplyResult::default();
    for record in records {
        if is_private_record(record) {
            result.stats.private_skipped_count += 1;
            continue;
        }
        let mut sanitized = record.clone();
        let redacted = redact_record_fields(&mut sanitized);
        if redacted {
            result.stats.redacted_count += 1;
        }
        result.records.push(sanitized);
    }
    result
}

fn is_private_record(record: &ObservationRecord) -> bool {
    contains_private_marker(&record.summary)
        || contains_private_marker(&record.artifact_ref)
        || contains_private_marker(&record.tool_name)
}

fn redact_record_fields(record: &mut ObservationRecord) -> bool {
    let sensitive = contains_sensitive_text(&record.summary)
        || contains_sensitive_text(&record.artifact_ref)
        || contains_sensitive_text(&record.tool_name);
    if !sensitive {
        return false;
    }
    record.summary = redact_sensitive_text(&record.summary);
    record.artifact_ref = redact_sensitive_text(&record.artifact_ref);
    record.tool_name = redact_sensitive_text(&record.tool_name);
    true
}

fn private_marked_records(records: &[ObservationRecord]) -> Vec<String> {
    records
        .iter()
        .filter(|record| is_private_record(record))
        .map(|record| format!("{}@{}", record.event_type, record.event_timestamp))
        .collect()
}

fn memory_enhanced_enabled(request: &RunRequest) -> bool {
    request
        .context_hints
        .get("memory_enhanced_enabled")
        .is_none_or(|value| value != "false")
}

fn write_sqlite_records(request: &RunRequest, records: &[ObservationRecord]) -> PersistenceOutcome {
    let mut outcome = PersistenceOutcome::default();
    let prepared = apply_privacy_rules(records);
    if forced_failure(request, "force_observation_sqlite_fail") {
        outcome
            .errors
            .push("sqlite:forced_failure:observation_write".to_string());
        return outcome;
    }
    for record in &prepared.records {
        match insert_observation_record(request, record) {
            Ok(_) => outcome.written_count += 1,
            Err(error) => outcome.errors.push(format!("sqlite:{error}")),
        }
    }
    outcome
}

fn write_audit_records(request: &RunRequest, records: &[ObservationRecord]) -> PersistenceOutcome {
    let mut outcome = PersistenceOutcome::default();
    let prepared = apply_privacy_rules(records);
    if forced_failure(request, "force_observation_audit_fail") {
        outcome
            .errors
            .push("jsonl:forced_failure:observation_audit".to_string());
        return outcome;
    }
    let path = observation_audit_file_path(request);
    for record in &prepared.records {
        match append_jsonl(path.clone(), record) {
            Ok(_) => outcome.written_count += 1,
            Err(error) => outcome.errors.push(format!("jsonl:{error}")),
        }
    }
    outcome
}

fn build_persistence_report(
    request: &RunRequest,
    snapshot: &LifecycleMappingSnapshot,
    dedupe: &ObservationDedupeReport,
    sqlite: PersistenceOutcome,
    audit: PersistenceOutcome,
) -> ObservationPersistenceReport {
    let mut errors = sqlite.errors;
    errors.extend(audit.errors);
    ObservationPersistenceReport {
        target_event_count: snapshot.required_target_count,
        mapped_event_count: snapshot.mapped_items.len(),
        dedupe_input_count: dedupe.total_incoming,
        dedupe_unique_count: dedupe.unique_count,
        dedupe_dropped_count: dedupe.dropped_count,
        sqlite_written_count: sqlite.written_count,
        audit_written_count: audit.written_count,
        sqlite_total_rows: sqlite_observation_count(request),
        audit_total_rows: audit_observation_count(request),
        fallback_applied: !errors.is_empty(),
        errors,
    }
}

fn sqlite_observation_count(request: &RunRequest) -> usize {
    with_connection(request, |conn| {
        conn.query_row(
            "select count(1) from runtime_observations where workspace_id = ?1",
            rusqlite::params![request.workspace_ref.workspace_id.clone()],
            |row| row.get::<_, i64>(0),
        )
        .map(|value| value as usize)
        .map_err(|error| error.to_string())
    })
    .unwrap_or_default()
}

fn audit_observation_count(request: &RunRequest) -> usize {
    read_jsonl::<ObservationRecord>(&observation_audit_file_path(request)).len()
}

fn dedupe_key(record: &ObservationRecord) -> String {
    format!(
        "{}:{}:{}",
        record.event_type,
        content_hash(record),
        time_window_bucket(record)
    )
}

fn content_hash(record: &ObservationRecord) -> u64 {
    let content = format!(
        "{}|{}|{}|{}|{}",
        record.summary, record.stage, record.tool_name, record.artifact_ref, record.run_id
    );
    hash_text(&content)
}

fn hash_text(value: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn time_window_bucket(record: &ObservationRecord) -> u128 {
    let millis = parse_timestamp_millis(&record.event_timestamp);
    millis / DEDUPE_WINDOW_MILLIS
}

fn parse_timestamp_millis(value: &str) -> u128 {
    value.trim().parse::<u128>().unwrap_or_default()
}

fn forced_failure(request: &RunRequest, key: &str) -> bool {
    request
        .context_hints
        .get(key)
        .is_some_and(|value| value == "true")
}

fn reset_observation_queue(request: &RunRequest, errors: &mut Vec<String>) {
    let result = with_connection(request, |conn| {
        conn.execute(
            "delete from observation_pending_queue where workspace_id = ?1",
            rusqlite::params![request.workspace_ref.workspace_id.clone()],
        )
        .map(|_| ())
        .map_err(|error| error.to_string())
    });
    if let Err(error) = result {
        errors.push(format!("queue_reset:{error}"));
    }
}

fn enqueue_pending_records(
    request: &RunRequest,
    records: &[ObservationRecord],
    errors: &mut Vec<String>,
) -> usize {
    let result = with_connection(request, |conn| {
        let mut written = 0usize;
        for record in records {
            insert_pending_record(conn, request, record)?;
            written += 1;
        }
        Ok(written)
    });
    match result {
        Ok(count) => count,
        Err(error) => {
            errors.push(format!("queue_enqueue:{error}"));
            0
        }
    }
}

fn insert_pending_record(
    conn: &rusqlite::Connection,
    request: &RunRequest,
    record: &ObservationRecord,
) -> Result<(), String> {
    let payload = serde_json::to_string(record).map_err(|error| error.to_string())?;
    conn.execute(
        "insert into observation_pending_queue (
            workspace_id, event_type, observation_kind, payload_json, status, retry_count, last_error, updated_at
        ) values (?1, ?2, ?3, ?4, 'pending', 0, '', ?5)",
        rusqlite::params![
            request.workspace_ref.workspace_id.clone(),
            record.event_type.clone(),
            record.observation_kind.clone(),
            payload,
            crate::events::timestamp_now()
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

fn pending_to_processing(request: &RunRequest, errors: &mut Vec<String>) -> usize {
    execute_queue_update(
        request,
        "update observation_pending_queue
         set status = 'processing', updated_at = ?1
         where workspace_id = ?2 and status = 'pending'",
        errors,
        "queue_pending_to_processing",
    )
}

fn processing_to_failed(request: &RunRequest, errors: &mut Vec<String>) -> usize {
    if !forced_failure(request, "force_observation_queue_fail") {
        return 0;
    }
    execute_queue_update(
        request,
        "update observation_pending_queue
         set status = 'failed', retry_count = retry_count + 1, last_error = 'forced_failure',
             updated_at = ?1
         where id = (
            select id from observation_pending_queue
            where workspace_id = ?2 and status = 'processing'
            order by id asc limit 1
         )",
        errors,
        "queue_processing_to_failed",
    )
}

fn processing_to_processed(request: &RunRequest, errors: &mut Vec<String>) -> usize {
    execute_queue_update(
        request,
        "update observation_pending_queue
         set status = 'processed', updated_at = ?1
         where workspace_id = ?2 and status = 'processing'",
        errors,
        "queue_processing_to_processed",
    )
}

fn execute_queue_update(
    request: &RunRequest,
    sql: &str,
    errors: &mut Vec<String>,
    code: &str,
) -> usize {
    let result = with_connection(request, |conn| {
        conn.execute(
            sql,
            rusqlite::params![
                crate::events::timestamp_now(),
                request.workspace_ref.workspace_id.clone()
            ],
        )
        .map_err(|error| error.to_string())
    });
    match result {
        Ok(count) => count,
        Err(error) => {
            errors.push(format!("{code}:{error}"));
            0
        }
    }
}

fn queue_count_by_status(request: &RunRequest, status: &str) -> usize {
    with_connection(request, |conn| {
        conn.query_row(
            "select count(1) from observation_pending_queue
             where workspace_id = ?1 and status = ?2",
            rusqlite::params![request.workspace_ref.workspace_id.clone(), status],
            |row| row.get::<_, i64>(0),
        )
        .map(|value| value as usize)
        .map_err(|error| error.to_string())
    })
    .unwrap_or_default()
}

fn search_rows(request: &RunRequest, query: &str, limit: usize) -> Vec<StoredObservationRow> {
    let rows = recent_observation_rows(request, search_scan_limit(limit));
    filter_rows_by_query(&rows, query, limit)
}

fn search_item_from_row(row: &StoredObservationRow) -> ObservationSearchItem {
    ObservationSearchItem {
        observation_id: row.id,
        event_type: row.event_type.clone(),
        observation_kind: row.observation_kind.clone(),
        stage: row.stage.clone(),
        summary_preview: preview_summary(&row.summary),
        trace_id: row.trace_id.clone(),
        run_id: row.run_id.clone(),
        session_id: row.session_id.clone(),
        created_at: row.created_at.clone(),
    }
}

fn normalize_limit(limit: usize, fallback: usize) -> usize {
    if limit == 0 {
        return fallback;
    }
    limit.min(100)
}

fn normalize_budget_total_chars(value: usize) -> usize {
    if value == 0 {
        return 1200;
    }
    value.max(300).min(20000)
}

fn split_budgets(total: usize) -> (usize, usize, usize) {
    let summary = (total * 3) / 10;
    let timeline = (total * 4) / 10;
    let details = total.saturating_sub(summary + timeline);
    (summary, timeline, details)
}

fn estimate_tokens_from_chars(chars: usize) -> usize {
    if chars == 0 {
        return 0;
    }
    chars.div_ceil(4)
}

fn get_observation_details(
    request: &RunRequest,
    ranked: &ObservationRankReport,
) -> Vec<ObservationDetailItem> {
    let ids = ranked
        .items
        .iter()
        .map(|item| item.observation_id)
        .collect::<Vec<_>>();
    get_observations(request, &ids, 20).items
}

fn normalized_observation_ids(ids: &[i64], limit: usize) -> Vec<i64> {
    let mut seen = BTreeSet::new();
    ids.iter()
        .copied()
        .filter(|id| *id > 0 && seen.insert(*id))
        .take(limit)
        .collect()
}

fn normalize_window(window: usize) -> usize {
    if window == 0 {
        return 3;
    }
    window.min(20)
}

fn search_scan_limit(limit: usize) -> usize {
    (limit.saturating_mul(20)).max(200).min(2000)
}

fn recent_observation_rows(request: &RunRequest, limit: usize) -> Vec<StoredObservationRow> {
    with_connection(request, |conn| {
        let mut statement = conn
            .prepare(
                "select id, event_type, observation_kind, stage, summary, tool_name,
                 artifact_ref, trace_id, run_id, session_id, created_at
                 from runtime_observations
                 where workspace_id = ?1
                 order by created_at desc, id desc
                 limit ?2",
            )
            .map_err(|error| error.to_string())?;
        let rows = statement
            .query_map(
                rusqlite::params![request.workspace_ref.workspace_id.clone(), limit as i64],
                decode_observation_row,
            )
            .map_err(|error| error.to_string())?;
        collect_observation_rows(rows)
    })
    .unwrap_or_default()
}

fn filter_rows_by_query(
    rows: &[StoredObservationRow],
    query: &str,
    limit: usize,
) -> Vec<StoredObservationRow> {
    let terms = query_terms(query);
    rows.iter()
        .filter(|row| row_matches_terms(row, &terms))
        .take(limit)
        .cloned()
        .collect()
}

fn row_matches_terms(row: &StoredObservationRow, terms: &[String]) -> bool {
    if terms.is_empty() {
        return true;
    }
    let haystack = format!(
        "{} {} {}",
        row.summary.to_lowercase(),
        row.event_type.to_lowercase(),
        row.stage.to_lowercase()
    );
    terms.iter().any(|term| haystack.contains(term))
}

fn build_summary_section(items: &[ObservationRankItem], budget: usize) -> String {
    let text = items
        .iter()
        .take(5)
        .map(summary_line)
        .collect::<Vec<_>>()
        .join("\n");
    clip_to_budget(&text, budget)
}

fn summary_line(item: &ObservationRankItem) -> String {
    format!(
        "- [{}] {} / {} / score={:.2}",
        item.observation_id, item.event_type, item.stage, item.total_score
    )
}

fn build_timeline_section(items: &[ObservationDetailItem], budget: usize) -> String {
    let text = items
        .iter()
        .take(8)
        .map(timeline_line)
        .collect::<Vec<_>>()
        .join("\n");
    clip_to_budget(&text, budget)
}

fn timeline_line(item: &ObservationDetailItem) -> String {
    format!(
        "- [{}] {} {} {}",
        item.observation_id,
        item.created_at,
        item.event_type,
        preview_summary(&item.summary)
    )
}

fn build_details_section(items: &[ObservationDetailItem], budget: usize) -> String {
    let text = items
        .iter()
        .take(5)
        .map(detail_line)
        .collect::<Vec<_>>()
        .join("\n");
    clip_to_budget(&text, budget)
}

fn detail_line(item: &ObservationDetailItem) -> String {
    format!(
        "- [{}] summary={} | tool={} | artifact={}",
        item.observation_id, item.summary, item.tool_name, item.artifact_ref
    )
}

fn build_references(items: &[ObservationDetailItem]) -> Vec<String> {
    items
        .iter()
        .map(|item| format!("observation_id={}", item.observation_id))
        .collect()
}

fn join_layered_sections(summary: &str, timeline: &str, details: &str) -> String {
    format!(
        "[summary]\n{}\n\n[timeline]\n{}\n\n[details]\n{}",
        summary, timeline, details
    )
}

fn clip_to_budget(text: &str, budget: usize) -> String {
    if budget == 0 {
        return String::new();
    }
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= budget {
        return text.to_string();
    }
    chars[..budget].iter().collect::<String>()
}

fn full_context_text(request: &RunRequest, query: &str) -> String {
    let ranked = rank_observations(request, query, 20);
    let details = get_observation_details(request, &ranked);
    details
        .iter()
        .map(full_context_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn full_context_line(item: &ObservationDetailItem) -> String {
    format!(
        "[{}] {} {} {} {} {}",
        item.observation_id,
        item.created_at,
        item.event_type,
        item.stage,
        item.summary,
        item.artifact_ref
    )
}

fn saved_percent(full_chars: usize, saved_chars: usize) -> f64 {
    if full_chars == 0 {
        return 0.0;
    }
    (saved_chars as f64 * 100.0) / full_chars as f64
}

fn saved_percent_for_report(full_chars: usize, saved_chars: usize) -> f64 {
    (saved_percent(full_chars, saved_chars) * 100.0).round() / 100.0
}

fn rows_by_ids(request: &RunRequest, ids: &[i64]) -> Vec<StoredObservationRow> {
    if ids.is_empty() {
        return Vec::new();
    }
    let placeholders = repeat_placeholders(ids.len());
    let sql = format!(
        "select id, event_type, observation_kind, stage, summary, tool_name,
         artifact_ref, trace_id, run_id, session_id, created_at
         from runtime_observations
         where workspace_id = ?1 and id in ({})
         order by id asc",
        placeholders
    );
    query_rows_by_ids(request, &sql, ids)
}

fn repeat_placeholders(count: usize) -> String {
    (0..count).map(|_| "?").collect::<Vec<_>>().join(",")
}

fn query_rows_by_ids(
    request: &RunRequest,
    sql: &str,
    ids: &[i64],
) -> Vec<StoredObservationRow> {
    with_connection(request, |conn| {
        let mut values = vec![rusqlite::types::Value::from(
            request.workspace_ref.workspace_id.clone(),
        )];
        values.extend(ids.iter().map(|id| rusqlite::types::Value::from(*id)));
        let mut statement = conn.prepare(sql).map_err(|error| error.to_string())?;
        let rows = statement
            .query_map(rusqlite::params_from_iter(values), decode_observation_row)
            .map_err(|error| error.to_string())?;
        collect_observation_rows(rows)
    })
    .unwrap_or_default()
}

fn detail_item_from_row(row: &StoredObservationRow) -> ObservationDetailItem {
    ObservationDetailItem {
        observation_id: row.id,
        event_type: row.event_type.clone(),
        observation_kind: row.observation_kind.clone(),
        stage: row.stage.clone(),
        summary: row.summary.clone(),
        tool_name: row.tool_name.clone(),
        artifact_ref: row.artifact_ref.clone(),
        trace_id: row.trace_id.clone(),
        run_id: row.run_id.clone(),
        session_id: row.session_id.clone(),
        created_at: row.created_at.clone(),
    }
}

fn preview_summary(summary: &str) -> String {
    let chars: Vec<char> = summary.chars().collect();
    if chars.len() <= 120 {
        return summary.to_string();
    }
    chars[..120].iter().collect::<String>() + "…"
}

fn scored_rows(query: &str, rows: &[StoredObservationRow]) -> Vec<ScoredObservationRow> {
    let latest = latest_created_millis(rows);
    rows.iter()
        .map(|row| score_row(query, row, latest))
        .collect::<Vec<_>>()
}

fn latest_created_millis(rows: &[StoredObservationRow]) -> u128 {
    rows.iter()
        .map(|row| parse_timestamp_millis(&row.created_at))
        .max()
        .unwrap_or_default()
}

fn score_row(query: &str, row: &StoredObservationRow, latest: u128) -> ScoredObservationRow {
    let source_weight = source_weight(&row.event_type);
    let freshness_score = freshness_score(&row.created_at, latest);
    let keyword_score = keyword_score(query, row);
    let total_score = source_weight * 0.4 + freshness_score * 0.3 + keyword_score * 0.3;
    ScoredObservationRow {
        row: row.clone(),
        source_weight,
        freshness_score,
        keyword_score,
        total_score,
    }
}

fn source_weight(event_type: &str) -> f64 {
    match event_type {
        "verification_completed" => 1.0,
        "action_completed" => 0.9,
        "plan_ready" => 0.8,
        "analysis_ready" => 0.7,
        "run_finished" => 0.6,
        _ => 0.5,
    }
}

fn freshness_score(created_at: &str, latest: u128) -> f64 {
    let current = parse_timestamp_millis(created_at);
    if latest == 0 || latest <= current {
        return 1.0;
    }
    let delta = latest.saturating_sub(current) as f64;
    (1.0 / (1.0 + delta / 1000.0)).clamp(0.0, 1.0)
}

fn keyword_score(query: &str, row: &StoredObservationRow) -> f64 {
    let terms = query_terms(query);
    if terms.is_empty() {
        return 1.0;
    }
    let content = format!(
        "{} {} {}",
        row.summary.to_lowercase(),
        row.event_type.to_lowercase(),
        row.stage.to_lowercase()
    );
    let hits = terms.iter().filter(|term| content.contains(*term)).count() as f64;
    (hits / terms.len() as f64).clamp(0.0, 1.0)
}

fn query_terms(query: &str) -> Vec<String> {
    query
        .split_whitespace()
        .map(|item| item.trim().to_lowercase())
        .filter(|item| !item.is_empty())
        .collect()
}

fn compare_scored_rows(left: &ScoredObservationRow, right: &ScoredObservationRow) -> std::cmp::Ordering {
    right
        .total_score
        .partial_cmp(&left.total_score)
        .unwrap_or(std::cmp::Ordering::Equal)
}

fn rank_item_from_scored(item: ScoredObservationRow) -> ObservationRankItem {
    ObservationRankItem {
        observation_id: item.row.id,
        event_type: item.row.event_type,
        stage: item.row.stage,
        source_weight: item.source_weight,
        freshness_score: item.freshness_score,
        keyword_score: item.keyword_score,
        total_score: item.total_score,
    }
}

fn resolve_anchor(
    request: &RunRequest,
    anchor_id: Option<i64>,
    query: Option<&str>,
) -> (Option<i64>, String) {
    if let Some(id) = anchor_id {
        return (Some(id), "anchor_id".to_string());
    }
    if let Some(value) = query {
        let id = search_rows(request, value, 1).first().map(|row| row.id);
        if id.is_some() {
            return (id, "query".to_string());
        }
    }
    (latest_observation_id(request), "latest".to_string())
}

fn latest_observation_id(request: &RunRequest) -> Option<i64> {
    with_connection(request, |conn| {
        conn.query_row(
            "select id from runtime_observations
             where workspace_id = ?1
             order by id desc limit 1",
            rusqlite::params![request.workspace_ref.workspace_id.clone()],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| error.to_string())
    })
    .ok()
}

fn timeline_report(
    request: &RunRequest,
    anchor_id: i64,
    source: String,
    window: usize,
) -> ObservationTimelineReport {
    let rows = timeline_rows(request, anchor_id, window);
    ObservationTimelineReport {
        anchor_id,
        anchor_source: source,
        window,
        item_count: rows.len(),
        items: rows
            .iter()
            .map(|row| timeline_item_from_row(row, anchor_id))
            .collect(),
    }
}

fn timeline_rows(
    request: &RunRequest,
    anchor_id: i64,
    window: usize,
) -> Vec<StoredObservationRow> {
    let lower = anchor_id.saturating_sub(window as i64);
    let upper = anchor_id.saturating_add(window as i64);
    with_connection(request, |conn| {
        let mut statement = conn
            .prepare(
                "select id, event_type, observation_kind, stage, summary, tool_name,
                 artifact_ref, trace_id, run_id, session_id, created_at
                 from runtime_observations
                 where workspace_id = ?1 and id >= ?2 and id <= ?3
                 order by id asc",
            )
            .map_err(|error| error.to_string())?;
        let rows = statement
            .query_map(
                rusqlite::params![request.workspace_ref.workspace_id.clone(), lower, upper],
                decode_observation_row,
            )
            .map_err(|error| error.to_string())?;
        collect_observation_rows(rows)
    })
    .unwrap_or_default()
}

fn timeline_item_from_row(
    row: &StoredObservationRow,
    anchor_id: i64,
) -> ObservationTimelineItem {
    ObservationTimelineItem {
        observation_id: row.id,
        event_type: row.event_type.clone(),
        stage: row.stage.clone(),
        summary_preview: preview_summary(&row.summary),
        created_at: row.created_at.clone(),
        is_anchor: row.id == anchor_id,
    }
}

fn decode_observation_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredObservationRow> {
    Ok(StoredObservationRow {
        id: row.get(0)?,
        event_type: row.get(1)?,
        observation_kind: row.get(2)?,
        stage: row.get(3)?,
        summary: row.get(4)?,
        tool_name: row.get(5)?,
        artifact_ref: row.get(6)?,
        trace_id: row.get(7)?,
        run_id: row.get(8)?,
        session_id: row.get(9)?,
        created_at: row.get(10)?,
    })
}

fn collect_observation_rows<F>(
    rows: rusqlite::MappedRows<'_, F>,
) -> Result<Vec<StoredObservationRow>, String>
where
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<StoredObservationRow>,
{
    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|error| error.to_string())?);
    }
    Ok(items)
}

fn queue_status_sequence(failed_count: usize) -> Vec<String> {
    if failed_count > 0 {
        return vec![
            "pending".to_string(),
            "processing".to_string(),
            "failed".to_string(),
            "processed".to_string(),
        ];
    }
    vec![
        "pending".to_string(),
        "processing".to_string(),
        "processed".to_string(),
    ]
}

fn retry_failed_once(request: &RunRequest, errors: &mut Vec<String>) -> Vec<RetryAttempt> {
    let failed_ids = failed_queue_ids(request);
    let mut attempts = Vec::new();
    for (index, id) in failed_ids.into_iter().enumerate() {
        let attempt = (index + 1) as u32;
        let backoff_ms = retry_backoff_ms(attempt);
        let success = retry_queue_item(request, id, backoff_ms, errors);
        attempts.push(RetryAttempt {
            attempt,
            backoff_ms,
            success,
        });
    }
    attempts
}

fn failed_queue_ids(request: &RunRequest) -> Vec<i64> {
    with_connection(request, |conn| {
        let mut statement = conn
            .prepare(
                "select id from observation_pending_queue
                 where workspace_id = ?1 and status = 'failed'
                 order by id asc",
            )
            .map_err(|error| error.to_string())?;
        let rows = statement
            .query_map(
                rusqlite::params![request.workspace_ref.workspace_id.clone()],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|error| error.to_string())?;
        let mut ids = Vec::new();
        for item in rows {
            ids.push(item.map_err(|error| error.to_string())?);
        }
        Ok(ids)
    })
    .unwrap_or_default()
}

fn retry_backoff_ms(attempt: u32) -> u64 {
    let capped = attempt.min(5);
    500u64.saturating_mul(2u64.pow(capped.saturating_sub(1)))
}

fn retry_queue_item(
    request: &RunRequest,
    id: i64,
    backoff_ms: u64,
    errors: &mut Vec<String>,
) -> bool {
    let result = with_connection(request, |conn| {
        conn.execute(
            "update observation_pending_queue
             set status = 'processed', last_error = '', updated_at = ?1
             where id = ?2 and workspace_id = ?3 and status = 'failed'",
            rusqlite::params![
                format!("{}+retry{backoff_ms}", crate::events::timestamp_now()),
                id,
                request.workspace_ref.workspace_id.clone()
            ],
        )
        .map_err(|error| error.to_string())
    });
    match result {
        Ok(affected) => affected > 0,
        Err(error) => {
            errors.push(format!("queue_retry:{error}"));
            false
        }
    }
}

#[cfg(test)]
mod tests {
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
        assert!(report
            .items
            .iter()
            .all(|item| item.summary_preview.len() <= 121));
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
        assert!(report
            .items
            .iter()
            .all(|item| item.total_score >= 0.0 && item.total_score <= 1.0));
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
        assert!(report
            .sample_summaries
            .iter()
            .any(|item| item.contains("[REDACTED]")));
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
}
