use serde::{Deserialize, Serialize};

pub(crate) const LIFECYCLE_TARGET_EVENTS: [&str; 6] = [
    "run_started",
    "analysis_ready",
    "plan_ready",
    "action_completed",
    "verification_completed",
    "run_finished",
];
pub(crate) const DEDUPE_WINDOW_MILLIS: u128 = 300000;

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
pub(crate) struct PersistenceOutcome {
    pub(crate) written_count: usize,
    pub(crate) errors: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StoredObservationRow {
    pub(crate) id: i64,
    pub(crate) event_type: String,
    pub(crate) observation_kind: String,
    pub(crate) stage: String,
    pub(crate) summary: String,
    pub(crate) tool_name: String,
    pub(crate) artifact_ref: String,
    pub(crate) trace_id: String,
    pub(crate) run_id: String,
    pub(crate) session_id: String,
    pub(crate) created_at: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ScoredObservationRow {
    pub(crate) row: StoredObservationRow,
    pub(crate) source_weight: f64,
    pub(crate) freshness_score: f64,
    pub(crate) keyword_score: f64,
    pub(crate) total_score: f64,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PrivacyApplyStats {
    pub(crate) redacted_count: usize,
    pub(crate) private_skipped_count: usize,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PrivacyApplyResult {
    pub(crate) records: Vec<ObservationRecord>,
    pub(crate) stats: PrivacyApplyStats,
}
