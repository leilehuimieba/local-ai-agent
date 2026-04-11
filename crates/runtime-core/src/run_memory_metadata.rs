use std::collections::BTreeMap;

pub(crate) fn append_memory_governance_metadata(
    metadata: &mut BTreeMap<String, String>,
    outcome: &crate::memory_router::MemoryWriteOutcome,
) {
    metadata.insert("memory_kind".to_string(), outcome.record_type.clone());
    append_governance_audit(metadata, outcome);
}

fn append_governance_audit(
    metadata: &mut BTreeMap<String, String>,
    outcome: &crate::memory_router::MemoryWriteOutcome,
) {
    metadata.insert(
        "governance_status".to_string(),
        outcome.audit.governance_status.clone(),
    );
    metadata.insert(
        "memory_action".to_string(),
        outcome.audit.memory_action.clone(),
    );
    metadata.insert(
        "governance_version".to_string(),
        outcome.audit.governance_version.clone(),
    );
    metadata.insert(
        "governance_reason".to_string(),
        outcome.audit.governance_reason.clone(),
    );
    metadata.insert(
        "governance_source".to_string(),
        outcome.audit.governance_source.clone(),
    );
    metadata.insert(
        "governance_at".to_string(),
        outcome.audit.governance_at.clone(),
    );
    metadata.insert(
        "source_event_type".to_string(),
        outcome.audit.source_event_type.clone(),
    );
    metadata.insert(
        "source_artifact_path".to_string(),
        outcome.audit.source_artifact_path.clone(),
    );
    metadata.insert(
        "archive_reason".to_string(),
        outcome.audit.archive_reason.clone(),
    );
}
