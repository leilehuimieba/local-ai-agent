use crate::observation::types::{ObservationRecord, PrivacyApplyResult};
use crate::sensitive_data::{
    contains_private_marker, contains_sensitive_text, redact_sensitive_text,
};

pub(crate) fn apply_privacy_rules(records: &[ObservationRecord]) -> PrivacyApplyResult {
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

pub(crate) fn is_private_record(record: &ObservationRecord) -> bool {
    contains_private_marker(&record.summary)
        || contains_private_marker(&record.artifact_ref)
        || contains_private_marker(&record.tool_name)
}

pub(crate) fn redact_record_fields(record: &mut ObservationRecord) -> bool {
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

pub(crate) fn private_marked_records(records: &[ObservationRecord]) -> Vec<String> {
    records
        .iter()
        .filter(|record| is_private_record(record))
        .map(|record| format!("{}@{}", record.event_type, record.event_timestamp))
        .collect()
}
