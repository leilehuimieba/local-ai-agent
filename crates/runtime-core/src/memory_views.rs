use crate::contracts::RunRequest;
use crate::knowledge_store::search_knowledge_records;
use crate::memory::{MemoryEntry, search_memory_entries};
use crate::observation::ObservationRecord;
use crate::paths::observation_audit_file_path;
use crate::storage::read_jsonl;
use crate::text::{score_text, summarize_text};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SystemViewSummary {
    pub uri: String,
    pub summary: String,
}

pub(crate) fn select_system_view_summaries(
    request: &RunRequest,
    query: &str,
    limit: usize,
) -> Vec<SystemViewSummary> {
    let mut ranked = base_system_views(request)
        .into_iter()
        .enumerate()
        .map(|(index, view)| (score_view(query, &view), index, view))
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(&right.1)));
    ranked
        .into_iter()
        .map(|(_, _, view)| view)
        .take(limit)
        .collect()
}

fn base_system_views(request: &RunRequest) -> Vec<SystemViewSummary> {
    vec![
        boot_view(request),
        recent_view(request),
        index_view(request),
        rules_view(request),
        workspace_view(request),
    ]
}

fn score_view(query: &str, view: &SystemViewSummary) -> i32 {
    if query.trim().is_empty() {
        return 1;
    }
    score_text(query, &format!("{} {}", view.uri, view.summary))
}

fn boot_view(request: &RunRequest) -> SystemViewSummary {
    system_view(
        "system://boot",
        summarize_memory_entries(
            important_memories(request),
            3,
            "当前启动没有命中高优先级记忆。",
        ),
    )
}

fn recent_view(request: &RunRequest) -> SystemViewSummary {
    let memory = summarize_memory_entries(important_memories(request), 2, "当前没有近期记忆。");
    let knowledge = summarize_knowledge(request);
    let observation = summarize_observations(request);
    system_view(
        "system://recent",
        summarize_text(&format!("{memory} || {knowledge} || {observation}")),
    )
}

fn index_view(request: &RunRequest) -> SystemViewSummary {
    let entries = important_memories(request);
    let kinds = entries
        .iter()
        .map(|entry| entry.kind.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let summary = if kinds.is_empty() {
        "当前没有可索引的长期记忆类型。".to_string()
    } else {
        format!("已索引长期记忆类型：{}", kinds.join("、"))
    };
    system_view("system://index", summary)
}

fn rules_view(request: &RunRequest) -> SystemViewSummary {
    let entries = important_memories(request)
        .into_iter()
        .filter(is_rule_like)
        .collect::<Vec<_>>();
    system_view(
        "system://rules",
        summarize_memory_entries(entries, 3, "当前没有命中显式规则记忆。"),
    )
}

fn workspace_view(request: &RunRequest) -> SystemViewSummary {
    let memory_count = important_memories(request).len();
    let knowledge_count = search_knowledge_records(request).len();
    let summary = format!(
        "工作区={}；长期记忆候选={}；知识条目={}。",
        request.workspace_ref.workspace_id, memory_count, knowledge_count
    );
    system_view(
        &format!("system://workspace/{}", request.workspace_ref.workspace_id),
        summary,
    )
}

fn important_memories(request: &RunRequest) -> Vec<MemoryEntry> {
    search_memory_entries(request, "", 8)
}

fn summarize_memory_entries(entries: Vec<MemoryEntry>, limit: usize, fallback: &str) -> String {
    let lines = entries
        .into_iter()
        .take(limit)
        .map(|entry| format!("[{}] {}", entry.kind, entry.summary))
        .collect::<Vec<_>>();
    if lines.is_empty() {
        fallback.to_string()
    } else {
        summarize_text(&lines.join(" || "))
    }
}

fn summarize_knowledge(request: &RunRequest) -> String {
    let mut items = search_knowledge_records(request);
    items.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));
    let lines = items
        .into_iter()
        .take(2)
        .map(|item| item.summary)
        .collect::<Vec<_>>();
    if lines.is_empty() {
        "当前没有近期知识条目。".to_string()
    } else {
        summarize_text(&lines.join(" || "))
    }
}

fn summarize_observations(request: &RunRequest) -> String {
    let mut items = read_jsonl::<ObservationRecord>(&observation_audit_file_path(request));
    items.sort_by(|left, right| right.event_timestamp.cmp(&left.event_timestamp));
    let lines = items
        .into_iter()
        .take(2)
        .map(|item| item.summary)
        .collect::<Vec<_>>();
    if lines.is_empty() {
        "当前没有近期观察锚点。".to_string()
    } else {
        summarize_text(&lines.join(" || "))
    }
}

fn system_view(uri: &str, summary: String) -> SystemViewSummary {
    SystemViewSummary {
        uri: uri.to_string(),
        summary,
    }
}

fn is_rule_like(entry: &MemoryEntry) -> bool {
    entry.kind == "project_rule" || entry.kind == "preference" || entry.summary.contains("规则")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use std::collections::BTreeMap;

    #[test]
    fn builds_five_system_views() {
        let views = select_system_view_summaries(&sample_request(), "", 5);
        let uris = views.into_iter().map(|view| view.uri).collect::<Vec<_>>();
        assert!(uris.contains(&"system://boot".to_string()));
        assert!(uris.contains(&"system://recent".to_string()));
        assert!(uris.contains(&"system://index".to_string()));
        assert!(uris.contains(&"system://rules".to_string()));
        assert!(
            uris.iter()
                .any(|uri| uri.starts_with("system://workspace/"))
        );
    }

    fn sample_request() -> RunRequest {
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
                root_path: "D:/repo".to_string(),
                is_active: true,
            },
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }
}
