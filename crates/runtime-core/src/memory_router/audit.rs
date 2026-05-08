use super::MemoryAuditTrail;
use crate::events::timestamp_now;
use crate::memory::MemoryEntry;
use crate::memory_schema::MEMORY_GOVERNANCE_VERSION;
use crate::text::summarize_text;

pub(super) fn working_memory_audit() -> MemoryAuditTrail {
    MemoryAuditTrail {
        governance_status: "written".to_string(),
        memory_action: "write".to_string(),
        governance_version: MEMORY_GOVERNANCE_VERSION.to_string(),
        governance_reason: "短期工作记忆已按当前运行时状态同步落盘。".to_string(),
        governance_source: "runtime_working_memory".to_string(),
        governance_at: timestamp_now(),
        memory_write_layer: "working_only".to_string(),
        memory_write_decision: "accepted".to_string(),
        memory_write_reason: "当前状态仅保留 working_only 短期留痕。".to_string(),
        memory_duplicate_strategy: "none".to_string(),
        source_event_type: "memory_written".to_string(),
        source_artifact_path: String::new(),
        archive_reason: String::new(),
    }
}

pub(super) fn written_audit(entry: &MemoryEntry) -> MemoryAuditTrail {
    MemoryAuditTrail {
        governance_status: "written".to_string(),
        memory_action: "write".to_string(),
        governance_version: entry.governance_version.clone(),
        governance_reason: entry.governance_reason.clone(),
        governance_source: entry.governance_source.clone(),
        governance_at: entry.governance_at.clone(),
        memory_write_layer: entry.memory_write_layer.clone(),
        memory_write_decision: entry.memory_write_decision.clone(),
        memory_write_reason: entry.memory_write_reason.clone(),
        memory_duplicate_strategy: entry.memory_duplicate_strategy.clone(),
        source_event_type: entry.source_event_type.clone(),
        source_artifact_path: entry.source_artifact_path.clone(),
        archive_reason: entry.archive_reason.clone(),
    }
}

pub(super) fn skipped_audit(event_type: &str, source: &str, reason: &str) -> MemoryAuditTrail {
    MemoryAuditTrail {
        governance_status: "skipped".to_string(),
        memory_action: "skip".to_string(),
        governance_version: MEMORY_GOVERNANCE_VERSION.to_string(),
        governance_reason: summarize_text(reason),
        governance_source: source.to_string(),
        governance_at: timestamp_now(),
        memory_write_layer: String::new(),
        memory_write_decision: "rejected".to_string(),
        memory_write_reason: summarize_text(reason),
        memory_duplicate_strategy: "none".to_string(),
        source_event_type: event_type.to_string(),
        source_artifact_path: String::new(),
        archive_reason: String::new(),
    }
}

pub(super) fn skipped_entry_audit(entry: &MemoryEntry, event_type: &str, reason: &str) -> MemoryAuditTrail {
    let source = audit_source(entry);
    let mut audit = skipped_audit(event_type, source, reason);
    audit.memory_write_layer = entry.memory_write_layer.clone();
    audit.memory_write_decision = entry.memory_write_decision.clone();
    audit.memory_write_reason = entry.memory_write_reason.clone();
    audit.memory_duplicate_strategy = entry.memory_duplicate_strategy.clone();
    audit.source_artifact_path = entry.source_artifact_path.clone();
    audit
}

fn audit_source(entry: &MemoryEntry) -> &str {
    if entry.governance_source.is_empty() {
        "runtime_skip_guard"
    } else {
        entry.governance_source.as_str()
    }
}
