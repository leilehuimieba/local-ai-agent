use crate::context_policy::ContextAssemblyPolicy;
use crate::contracts::RunRequest;
use crate::memory::search_memory_entries;
use crate::memory_router::{MemoryRouteSelection, select_memory_route};
use crate::memory_views::{SystemViewSummary, select_system_view_summaries};
use crate::sqlite_store::list_current_memory_object_entries_limited_sqlite;
use crate::text::summarize_text;

const SYSTEM_LAYER: &str = "system views";
const OBJECT_LAYER: &str = "current memory object";
const HISTORY_LAYER: &str = "history entries";

#[derive(Clone, Debug)]
pub(crate) struct MemoryDigest {
    pub summary: String,
    pub has_system_views: bool,
    pub has_current_objects: bool,
    pub current_object_count: usize,
    pub memory_route: String,
    pub selected_layers: Vec<String>,
    pub match_reason: String,
    pub reuse_confidence: String,
    pub skipped_layers: Vec<String>,
}

pub(crate) fn recall_memory_digest(request: &RunRequest, query: &str, limit: usize) -> MemoryDigest {
    recall_memory_digest_with_policy(request, query, limit, None)
}

pub(crate) fn recall_memory_digest_with_policy(
    request: &RunRequest,
    query: &str,
    limit: usize,
    policy: Option<&ContextAssemblyPolicy>,
) -> MemoryDigest {
    let route = select_memory_route(request, query, policy);
    let objects = list_current_memory_object_entries_limited_sqlite(request, limit);
    let entries = search_memory_entries(request, query, limit);
    let views = select_system_view_summaries(request, query, limit);
    build_memory_digest(&route, &views, &objects, &entries)
}

fn build_memory_digest(
    route: &MemoryRouteSelection,
    system_views: &[SystemViewSummary],
    object_entries: &[crate::memory::MemoryEntry],
    entries: &[crate::memory::MemoryEntry],
) -> MemoryDigest {
    let selected = selected_digest_lines(route, system_views, object_entries, entries);
    let fallback = fallback_digest_lines(system_views, object_entries, entries);
    MemoryDigest {
        summary: digest_summary(route, &selected, &fallback),
        has_system_views: !system_views.is_empty(),
        has_current_objects: !object_entries.is_empty(),
        current_object_count: object_entries.len(),
        memory_route: route.route.clone(),
        selected_layers: route.selected_layers.clone(),
        match_reason: route.match_reason.clone(),
        reuse_confidence: route.reuse_confidence.clone(),
        skipped_layers: route.skipped_layers.clone(),
    }
}

fn selected_digest_lines(
    route: &MemoryRouteSelection,
    system_views: &[SystemViewSummary],
    object_entries: &[crate::memory::MemoryEntry],
    entries: &[crate::memory::MemoryEntry],
) -> Vec<String> {
    let mut lines = Vec::new();
    push_layer_lines(
        &mut lines,
        wants_layer(route, SYSTEM_LAYER),
        system_views.iter().map(system_view_line),
    );
    push_layer_lines(
        &mut lines,
        wants_layer(route, OBJECT_LAYER),
        object_entries.iter().map(memory_object_line),
    );
    push_layer_lines(
        &mut lines,
        wants_layer(route, HISTORY_LAYER),
        entries.iter().map(memory_line),
    );
    lines.into_iter().take(3).collect()
}

fn fallback_digest_lines(
    system_views: &[SystemViewSummary],
    object_entries: &[crate::memory::MemoryEntry],
    entries: &[crate::memory::MemoryEntry],
) -> Vec<String> {
    let mut lines = system_views.iter().map(system_view_line).collect::<Vec<_>>();
    lines.extend(object_entries.iter().map(memory_object_line));
    lines.extend(entries.iter().map(memory_line));
    lines.into_iter().take(3).collect()
}

fn push_layer_lines<I>(lines: &mut Vec<String>, enabled: bool, values: I)
where
    I: Iterator<Item = String>,
{
    if enabled {
        lines.extend(values);
    }
}

fn wants_layer(route: &MemoryRouteSelection, layer: &str) -> bool {
    route.selected_layers.iter().any(|item| item == layer)
}

fn digest_summary(route: &MemoryRouteSelection, selected: &[String], fallback: &[String]) -> String {
    if selected.is_empty() && fallback.is_empty() {
        return "当前没有命中相关长期记忆。".to_string();
    }
    let lines = preferred_lines(selected, fallback);
    summarize_text(&format!(
        "记忆路由：{}；选中层：{}；命中原因：{}；复用置信度：{}；摘要：{}",
        route.route,
        route.selected_layers.join(" + "),
        route.match_reason,
        route.reuse_confidence,
        lines.join(" || ")
    ))
}

fn preferred_lines<'a>(selected: &'a [String], fallback: &'a [String]) -> &'a [String] {
    if selected.is_empty() { fallback } else { selected }
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
    use crate::context_policy::ContextAssemblyPolicy;
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use crate::memory::MemoryEntry;
    use crate::sqlite_store::write_memory_entry_sqlite;
    use std::collections::BTreeMap;

    #[test]
    fn recall_digest_includes_system_view_lines() {
        let digest = recall_memory_digest(&sample_request("当前项目规则"), "规则", 3);
        assert_eq!(digest.memory_route, "ask_route");
        assert!(digest.summary.contains("system://"));
    }

    #[test]
    fn recall_digest_surfaces_current_memory_object_block() {
        let request = sample_request("对象摘要");
        write_memory_entry_sqlite(&request, &sample_entry("memory-object-1", "对象摘要")).unwrap();
        let digest = recall_memory_digest(&request, "对象摘要", 3);
        assert!(digest.selected_layers.contains(&OBJECT_LAYER.to_string()));
        assert!(digest.has_current_objects);
        assert_eq!(digest.current_object_count, 1);
        assert!(digest.summary.contains("对象摘要"));
    }

    #[test]
    fn recall_digest_prefers_repair_route_with_policy() {
        let request = sample_request("为什么上次失败");
        write_memory_entry_sqlite(&request, &sample_entry("memory-object-1", "temporary failure")).unwrap();
        let digest = recall_memory_digest_with_policy(&request, "为什么上次失败", 3, Some(&repair_policy()));
        assert_eq!(digest.memory_route, "repair_route");
        assert_eq!(digest.reuse_confidence, "high");
        assert!(digest.match_reason.contains("恢复或失败处理"));
    }

    fn sample_request(user_input: &str) -> RunRequest {
        let root = std::env::temp_dir().join(format!("memory-recall-{}", crate::events::timestamp_now()));
        std::fs::create_dir_all(&root).unwrap();
        RunRequest {
            request_id: "request-test".to_string(),
            run_id: "run-test".to_string(),
            session_id: "session-test".to_string(),
            trace_id: "trace-test".to_string(),
            user_input: user_input.to_string(),
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

    fn repair_policy() -> ContextAssemblyPolicy {
        ContextAssemblyPolicy {
            profile: "repair_profile".to_string(),
            prompt_profile: "agent_resolve".to_string(),
            include_session: true,
            include_memory: true,
            include_knowledge: false,
            include_tool_preview: true,
            skill_injection_enabled: true,
            max_skill_level: "level1:index-summary".to_string(),
            phase_label: "repair".to_string(),
            selection_reason: "test".to_string(),
            prefer_artifact_context: true,
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
            memory_write_layer: String::new(),
            memory_write_decision: String::new(),
            memory_write_reason: String::new(),
            memory_duplicate_strategy: String::new(),
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
