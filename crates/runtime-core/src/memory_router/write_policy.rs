use super::{MemoryWriteOutcome, skipped_memory_outcome};
use crate::memory::MemoryEntry;

pub(super) fn auto_memory_layer(entry: &MemoryEntry) -> &'static str {
    match entry.kind.as_str() {
        "workspace_summary" => "working_only",
        "lesson_learned" | "task_outcome" => "episodic_memory",
        _ => "semantic_or_procedural_memory",
    }
}

pub(super) fn auto_memory_reason(layer: &str) -> &'static str {
    match layer {
        "episodic_memory" => "当前结果形成了可复用过程经验，进入 episodic_memory。",
        _ => "当前结果具备跨任务复用价值，进入 semantic_or_procedural_memory。",
    }
}

pub(super) fn govern_entry(entry: &mut MemoryEntry, layer: &str, decision: &str, reason: &str, duplicate: &str) {
    entry.memory_write_layer = layer.to_string();
    entry.memory_write_decision = decision.to_string();
    entry.memory_write_reason = reason.to_string();
    entry.memory_duplicate_strategy = duplicate.to_string();
}

pub(super) fn reject_entry_outcome(entry: &mut MemoryEntry, layer: &str, reason: &str) -> MemoryWriteOutcome {
    govern_entry(entry, layer, "rejected", reason, "none");
    skipped_memory_outcome(layer, entry, reason)
}

pub(super) fn duplicate_entry_outcome(entry: &mut MemoryEntry, reason: &str) -> MemoryWriteOutcome {
    let layer = entry.memory_write_layer.clone();
    govern_entry(entry, &layer, "duplicate_skipped", reason, "same_kind_title_summary");
    skipped_memory_outcome(&layer, entry, reason)
}

pub(super) fn append_error_outcome(entry: &MemoryEntry, reason: &str) -> MemoryWriteOutcome {
    let mut rejected = entry.clone();
    let layer = rejected.memory_write_layer.clone();
    govern_entry(&mut rejected, &layer, "rejected", reason, "none");
    skipped_memory_outcome(&layer, &rejected, reason)
}
