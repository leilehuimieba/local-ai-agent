use crate::contracts::RunRequest;
use crate::memory::search_memory_entries;
use crate::sqlite_store::list_current_memory_object_entries_limited_sqlite;
use crate::memory_views::{SystemViewSummary, select_system_view_summaries};
use crate::text::summarize_text;

#[derive(Clone, Debug)]
pub(crate) struct MemoryDigest {
    pub summary: String,
    pub has_system_views: bool,
    pub has_current_objects: bool,
    pub current_object_count: usize,
}

pub(crate) fn recall_memory_digest(
    request: &RunRequest,
    query: &str,
    limit: usize,
) -> MemoryDigest {
    let object_entries = list_current_memory_object_entries_limited_sqlite(request, limit);
    let entries = search_memory_entries(request, query, limit);
    let system_views = select_system_view_summaries(request, query, limit);
    MemoryDigest {
        summary: digest_summary(&system_views, &object_entries, &entries),
        has_system_views: !system_views.is_empty(),
        has_current_objects: !object_entries.is_empty(),
        current_object_count: object_entries.len(),
    }
}

fn digest_summary(
    system_views: &[SystemViewSummary],
    object_entries: &[crate::memory::MemoryEntry],
    entries: &[crate::memory::MemoryEntry],
) -> String {
    if system_views.is_empty() && object_entries.is_empty() && entries.is_empty() {
        return "当前没有命中相关长期记忆。".to_string();
    }
    let mut lines = system_views
        .iter()
        .map(system_view_line)
        .collect::<Vec<_>>();
    lines.extend(object_entries.iter().map(memory_object_line));
    lines.extend(entries.iter().map(memory_line));
    summarize_text(&lines.join(" || "))
}

fn memory_object_line(entry: &crate::memory::MemoryEntry) -> String {
    format!(
        "[object] {} | URI={} | 别名={} | 优先级={} | 更新时间={}",
        entry.summary,
        entry.source,
        summarize_text(&entry.source_artifact_path),
        entry.priority,
        memory_updated_at(entry),
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

fn system_view_line(view: &SystemViewSummary) -> String {
    format!("[system] {} | {}", view.uri, view.summary)
}

fn memory_reason(entry: &crate::memory::MemoryEntry) -> &'static str {
    if entry.source_type == "seed" {
        "基线记忆优先"
    } else if entry.source_type == "memory_object_current" {
        "current memory object 命中"
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use crate::memory::MemoryEntry;
    use crate::sqlite_store::write_memory_entry_sqlite;
    use std::collections::BTreeMap;

    #[test]
    fn recall_digest_includes_system_view_lines() {
        let digest = recall_memory_digest(&sample_request(), "规则", 3);
        assert!(digest.summary.contains("system://"));
    }

    #[test]
    fn recall_digest_surfaces_current_memory_object_block() {
        let request = sample_request();
        write_memory_entry_sqlite(&request, &sample_entry("memory-object-1", "对象摘要")).unwrap();
        let digest = recall_memory_digest(&request, "对象摘要", 3);
        let object_entries = list_current_memory_object_entries_limited_sqlite(&request, 3);
        assert!(!object_entries.is_empty());
        assert!(memory_object_line(&object_entries[0]).contains("[object]"));
        assert!(digest.has_current_objects);
        assert_eq!(digest.current_object_count, object_entries.len());
        assert!(digest.summary.contains("对象摘要"));
    }

    fn sample_request() -> RunRequest {
        let root = std::env::temp_dir().join(format!(
            "memory-recall-{}",
            crate::events::timestamp_now()
        ));
        std::fs::create_dir_all(&root).unwrap();
        RunRequest {
            request_id: "request-test".to_string(),
            run_id: "run-test".to_string(),
            session_id: "session-test".to_string(),
            trace_id: "trace-test".to_string(),
            user_input: "当前项目规则".to_string(),
            mode: "standard".to_string(),
            model_ref: ModelRef {
                provider_id: "p".to_string(),
                model_id: "m".to_string(),
                display_name: "model".to_string(),
            },
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-test".to_string(),
                name: "workspace".to_string(),
                root_path: root.display().to_string(),
                is_active: true,
            },
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }

    fn sample_entry(id: &str, summary: &str) -> MemoryEntry {
        MemoryEntry {
            id: id.to_string(),
            kind: "project_rule".to_string(),
            title: "rule-object".to_string(),
            summary: summary.to_string(),
            content: format!("content-{summary}"),
            scope: "workspace".to_string(),
            workspace_id: "workspace-test".to_string(),
            session_id: "session-test".to_string(),
            source_run_id: "run-test".to_string(),
            source: "run:run-test".to_string(),
            source_type: "runtime".to_string(),
            source_title: "rule-object".to_string(),
            source_event_type: "run_finished".to_string(),
            source_artifact_path: String::new(),
            governance_version: "v1".to_string(),
            governance_reason: "测试".to_string(),
            governance_source: "test".to_string(),
            governance_at: "1".to_string(),
            archive_reason: String::new(),
            verified: true,
            priority: 12,
            archived: false,
            archived_at: String::new(),
            created_at: "1001".to_string(),
            updated_at: "1001".to_string(),
            timestamp: "1001".to_string(),
        }
    }
}
