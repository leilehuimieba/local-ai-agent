use crate::contracts::RunRequest;
use crate::memory::search_memory_entries;
use crate::text::summarize_text;

#[derive(Clone, Debug)]
pub(crate) struct MemoryDigest {
    pub summary: String,
}

pub(crate) fn recall_memory_digest(
    request: &RunRequest,
    query: &str,
    limit: usize,
) -> MemoryDigest {
    let entries = search_memory_entries(request, query, limit);
    MemoryDigest {
        summary: digest_summary(&entries),
    }
}

fn digest_summary(entries: &[crate::memory::MemoryEntry]) -> String {
    if entries.is_empty() {
        return "当前没有命中相关长期记忆。".to_string();
    }
    summarize_text(
        &entries
            .iter()
            .map(memory_line)
            .collect::<Vec<_>>()
            .join(" || "),
    )
}

fn memory_line(entry: &crate::memory::MemoryEntry) -> String {
    format!(
        "[{}] {} | 来源={} | 类型={} | 理由={} | 优先级={} | 更新时间={}",
        entry.kind,
        entry.summary,
        entry.source,
        entry.source_type,
        memory_reason(entry),
        entry.priority,
        memory_updated_at(entry),
    )
}

fn memory_reason(entry: &crate::memory::MemoryEntry) -> &'static str {
    if entry.source_type == "seed" {
        "基线记忆优先"
    } else if entry.source.contains("README") || entry.source.contains("docs/06-development") {
        "高价值文档命中"
    } else {
        "按当前输入相关性召回"
    }
}

fn memory_updated_at(entry: &crate::memory::MemoryEntry) -> &str {
    if entry.updated_at.is_empty() {
        &entry.timestamp
    } else {
        &entry.updated_at
    }
}
