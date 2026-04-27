use crate::contracts::RunRequest;
use crate::observation::types::{ObservationRecord, DEDUPE_WINDOW_MILLIS};
use std::hash::{DefaultHasher, Hash, Hasher};

pub(crate) fn dedupe_key(record: &ObservationRecord) -> String {
    format!(
        "{}:{}:{}",
        record.event_type,
        content_hash(record),
        time_window_bucket(record)
    )
}

pub(crate) fn content_hash(record: &ObservationRecord) -> u64 {
    let content = format!(
        "{}|{}|{}|{}|{}",
        record.summary, record.stage, record.tool_name, record.artifact_ref, record.run_id
    );
    hash_text(&content)
}

pub(crate) fn hash_text(value: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

pub(crate) fn time_window_bucket(record: &ObservationRecord) -> u128 {
    let millis = parse_timestamp_millis(&record.event_timestamp);
    millis / DEDUPE_WINDOW_MILLIS
}

pub(crate) fn parse_timestamp_millis(value: &str) -> u128 {
    value.trim().parse::<u128>().unwrap_or_default()
}

pub(crate) fn forced_failure(request: &RunRequest, key: &str) -> bool {
    request
        .context_hints
        .get(key)
        .is_some_and(|value| value == "true")
}

