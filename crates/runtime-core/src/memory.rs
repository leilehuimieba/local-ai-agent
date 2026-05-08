mod policy;

pub(crate) use self::policy::normalized_memory_entry;
use self::policy::{is_seed_memory, should_archive_memory_entry, should_skip_memory_entry};
use crate::contracts::RunRequest;
use crate::memory_schema::{StructuredMemoryEntry, canonical_kind};
use crate::paths::{long_term_memory_file_path, memory_file_path, memory_tombstone_file_path};
use crate::sqlite_store::{
    list_current_memory_object_entries_sqlite, list_memory_entries_sqlite, write_memory_entry_sqlite,
};
use crate::storage::{append_jsonl, read_jsonl};
use crate::text::score_text;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct MemoryEntry {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub scope: String,
    pub workspace_id: String,
    pub session_id: String,
    pub source_run_id: String,
    pub source: String,
    pub source_type: String,
    pub source_title: String,
    pub source_event_type: String,
    pub source_artifact_path: String,
    pub governance_version: String,
    pub governance_reason: String,
    pub governance_source: String,
    pub governance_at: String,
    pub archive_reason: String,
    pub memory_write_layer: String,
    pub memory_write_decision: String,
    pub memory_write_reason: String,
    pub memory_duplicate_strategy: String,
    pub verified: bool,
    pub priority: i32,
    pub archived: bool,
    pub archived_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MemoryTombstone {
    memory_id: String,
}

pub(crate) fn append_memory_entry(request: &RunRequest, entry: &MemoryEntry) -> Result<(), String> {
    if should_archive_memory_entry(entry) {
        return Err("命中低价值运行时记忆治理规则，跳过写入。".to_string());
    }
    let normalized = normalized_memory_entry(entry);
    let record = structured_memory_entry(&normalized);
    write_memory_entry_sqlite(request, &normalized)?;
    append_jsonl(memory_file_path(request), &record)?;
    append_jsonl(long_term_memory_file_path(request), &record)
}

pub(crate) fn search_memory_entries(request: &RunRequest, query: &str, limit: usize) -> Vec<MemoryEntry> {
    let query_text = query.trim();
    let mut scored = score_memory_entries(request, query_text);
    sort_memory_entries(&mut scored);
    scored.into_iter().map(|(_, entry)| entry).take(limit).collect()
}

fn to_memory_entry(entry: StructuredMemoryEntry) -> MemoryEntry {
    let kind = canonical_kind(&entry.memory_type);
    let timestamp = entry_timestamp(&entry);
    let summary = fallback_text(&entry.summary, &entry.content, "无摘要");
    let source_title = fallback_text(&entry.source_title, &entry.title, &summary);
    let created_at = fallback_time(&entry.created_at, &entry.timestamp, &timestamp);
    let updated_at = fallback_time(&entry.updated_at, &created_at, &timestamp);
    let archived_at = archived_at(&entry);
    normalized_memory_entry(&MemoryEntry {
        id: entry.id,
        kind: kind.clone(),
        title: fallback_text(&entry.title, &summary, &kind),
        summary: summary.clone(),
        content: entry.content,
        scope: entry.scope,
        workspace_id: entry.workspace_id,
        session_id: entry.session_id,
        source_run_id: entry.source_run_id.clone(),
        source: fallback_text(&entry.source, &entry.source_run_id, "runtime"),
        source_type: fallback_text(&entry.source_type, "", "runtime"),
        source_title,
        source_event_type: entry.source_event_type,
        source_artifact_path: entry.source_artifact_path,
        governance_version: entry.governance_version,
        governance_reason: entry.governance_reason,
        governance_source: entry.governance_source,
        governance_at: entry.governance_at,
        archive_reason: entry.archive_reason,
        memory_write_layer: entry.memory_write_layer,
        memory_write_decision: entry.memory_write_decision,
        memory_write_reason: entry.memory_write_reason,
        memory_duplicate_strategy: entry.memory_duplicate_strategy,
        verified: entry.verified,
        priority: entry.priority,
        archived: entry.archived,
        archived_at,
        created_at,
        updated_at,
        timestamp,
    })
}

fn structured_memory_entry(entry: &MemoryEntry) -> StructuredMemoryEntry {
    StructuredMemoryEntry {
        id: entry.id.clone(),
        memory_type: canonical_kind(&entry.kind),
        title: entry.title.clone(),
        summary: entry.summary.clone(),
        content: entry.content.clone(),
        workspace_id: entry.workspace_id.clone(),
        source_run_id: entry.source_run_id.clone(),
        source: entry.source.clone(),
        source_type: entry.source_type.clone(),
        source_title: entry.source_title.clone(),
        source_event_type: entry.source_event_type.clone(),
        source_artifact_path: entry.source_artifact_path.clone(),
        governance_version: entry.governance_version.clone(),
        governance_reason: entry.governance_reason.clone(),
        governance_source: entry.governance_source.clone(),
        governance_at: entry.governance_at.clone(),
        archive_reason: entry.archive_reason.clone(),
        memory_write_layer: entry.memory_write_layer.clone(),
        memory_write_decision: entry.memory_write_decision.clone(),
        memory_write_reason: entry.memory_write_reason.clone(),
        memory_duplicate_strategy: entry.memory_duplicate_strategy.clone(),
        verified: entry.verified,
        priority: entry.priority,
        archived: entry.archived,
        archived_at: entry.archived_at.clone(),
        created_at: entry.created_at.clone(),
        updated_at: entry.updated_at.clone(),
        scope: entry.scope.clone(),
        session_id: entry.session_id.clone(),
        timestamp: entry.timestamp.clone(),
    }
}

fn all_memory_entries(request: &RunRequest) -> Vec<MemoryEntry> {
    let mut entries = seed_memory_entries(request);
    entries.extend(list_current_memory_object_entries_sqlite(request));
    let sqlite_entries = list_memory_entries_sqlite(request);
    if sqlite_entries.is_empty() {
        entries.extend(read_structured_entries(&memory_file_path(request)));
        entries.extend(read_structured_entries(&long_term_memory_file_path(request)));
    } else {
        entries.extend(sqlite_entries);
    }
    entries
}

fn read_structured_entries(path: &std::path::Path) -> Vec<MemoryEntry> {
    read_jsonl::<StructuredMemoryEntry>(path)
        .into_iter()
        .map(to_memory_entry)
        .collect()
}

fn score_memory_entries(request: &RunRequest, query_text: &str) -> Vec<(i32, MemoryEntry)> {
    let deleted = deleted_memory_ids(request);
    dedupe_memory_entries(all_memory_entries(request))
        .into_iter()
        .filter(|entry| !deleted.contains(&entry.id))
        .filter(|entry| !entry.archived)
        .filter(|entry| !should_skip_memory_entry(query_text, entry))
        .filter_map(|entry| score_memory_entry(request, query_text, entry))
        .collect()
}

fn deleted_memory_ids(request: &RunRequest) -> BTreeSet<String> {
    read_jsonl::<MemoryTombstone>(&memory_tombstone_file_path(request))
        .into_iter()
        .map(|item| item.memory_id)
        .collect()
}

fn score_memory_entry(request: &RunRequest, query_text: &str, entry: MemoryEntry) -> Option<(i32, MemoryEntry)> {
    let mut score = base_memory_score(query_text, &entry);
    score += memory_source_priority(&entry);
    score += memory_priority_bonus(entry.priority);
    score += memory_object_bonus(query_text, &entry);
    if entry.workspace_id == request.workspace_ref.workspace_id {
        score += 8;
    }
    if entry.session_id == request.session_id {
        score += 4;
    }
    (score > 0).then_some((score, entry))
}

fn base_memory_score(query_text: &str, entry: &MemoryEntry) -> i32 {
    let haystack = format!(
        "{} {} {} {} {} {}",
        entry.kind, entry.title, entry.summary, entry.content, entry.source, entry.source_artifact_path
    );
    let mut score = score_text(query_text, &haystack);
    if query_text.is_empty() {
        score += 1;
    }
    score
}

fn memory_priority_bonus(priority: i32) -> i32 {
    priority.clamp(-40, 120) / 2
}

fn memory_object_bonus(query_text: &str, entry: &MemoryEntry) -> i32 {
    if entry.source_type != "memory_object_current" {
        return 0;
    }
    18 + score_text(query_text, &entry.source) + score_text(query_text, &entry.source_artifact_path)
}

fn sort_memory_entries(scored: &mut [(i32, MemoryEntry)]) {
    scored.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| memory_object_rank(&right.1).cmp(&memory_object_rank(&left.1)))
            .then_with(|| right.1.timestamp.cmp(&left.1.timestamp))
    });
}

fn dedupe_memory_entries(entries: Vec<MemoryEntry>) -> Vec<MemoryEntry> {
    let mut seen = std::collections::BTreeSet::new();
    entries
        .into_iter()
        .filter(|entry| seen.insert(dedupe_memory_key(entry)))
        .collect()
}

fn memory_object_rank(entry: &MemoryEntry) -> i32 {
    if entry.source_type == "memory_object_current" {
        1
    } else {
        0
    }
}

fn dedupe_memory_key(entry: &MemoryEntry) -> String {
    if entry.source_type == "memory_object_current" {
        return format!("object://{}", entry.source);
    }
    memory_key(entry)
}

fn entry_timestamp(entry: &StructuredMemoryEntry) -> String {
    fallback_time(&entry.timestamp, &entry.updated_at, &entry.created_at)
}

fn seed_memory_entries(request: &RunRequest) -> Vec<MemoryEntry> {
    vec![
        build_seed_memory(
            request,
            "project-entry",
            "project_rule",
            "项目主口径与执行入口基线",
            "优先使用 docs/README.md 与 docs/06-development 的当前有效文档理解项目；docs/07-test 只作为验收参考，不作为项目说明主依据。",
            "当前项目是本地智能体。运行时主入口优先参考 docs/README.md、docs/06-development/智能体框架主干开发任务书_V1.md、docs/06-development/本地记忆与知识沉淀需求文档_V1.md、docs/06-development/本地记忆与知识沉淀开发任务书_V1.md。",
        ),
        build_seed_memory(
            request,
            "memory-policy",
            "project_rule",
            "记忆基线与召回边界",
            "长期记忆只保留跨任务可复用、已验证、可结构化的结论；README 与开发文档优先，07-test 不应成为长期记忆主基线。",
            "本地记忆与知识沉淀需求文档要求长期记忆服务跨任务复用，只按需召回，不允许把日志、验收记录、一次性过程当成长期记忆主输入。",
        ),
    ]
}

fn memory_source_priority(entry: &MemoryEntry) -> i32 {
    if is_seed_memory(entry) {
        return 140;
    }
    doc_path_priority(&entry.summary) + doc_path_priority(&entry.content)
}

fn doc_path_priority(text: &str) -> i32 {
    let value = text.replace('\\', "/").to_lowercase();
    if value.contains("/readme.md") || value.contains("readme.md") {
        return 56;
    }
    if value.contains("/docs/06-development/") {
        return 42;
    }
    if value.contains("/docs/02-architecture/") || value.contains("/docs/03-runtime/") {
        return 24;
    }
    if value.contains("/docs/07-test/") {
        return -72;
    }
    0
}

fn build_seed_memory(
    request: &RunRequest,
    id_suffix: &str,
    kind: &str,
    title: &str,
    summary: &str,
    content: &str,
) -> MemoryEntry {
    normalized_memory_entry(&MemoryEntry {
        id: format!("seed-{id_suffix}"),
        kind: kind.to_string(),
        title: title.to_string(),
        summary: summary.to_string(),
        content: content.to_string(),
        scope: request.workspace_ref.name.clone(),
        workspace_id: request.workspace_ref.workspace_id.clone(),
        session_id: "seed".to_string(),
        source_run_id: format!("seed:{id_suffix}"),
        source: "docs/README.md".to_string(),
        source_type: "seed".to_string(),
        source_title: title.to_string(),
        source_event_type: String::new(),
        source_artifact_path: "docs/README.md".to_string(),
        governance_version: String::new(),
        governance_reason: String::new(),
        governance_source: String::new(),
        governance_at: String::new(),
        archive_reason: String::new(),
        memory_write_layer: String::new(),
        memory_write_decision: String::new(),
        memory_write_reason: String::new(),
        memory_duplicate_strategy: String::new(),
        verified: true,
        priority: 100,
        archived: false,
        archived_at: String::new(),
        created_at: "9999990000000".to_string(),
        updated_at: "9999990000000".to_string(),
        timestamp: "9999990000000".to_string(),
    })
}

fn memory_key(entry: &MemoryEntry) -> String {
    format!(
        "{}|{}|{}|{}",
        entry.workspace_id, entry.kind, entry.title, entry.summary
    )
}

fn fallback_text(primary: &str, secondary: &str, default: &str) -> String {
    let value = primary.trim();
    if !value.is_empty() {
        return value.to_string();
    }
    let value = secondary.trim();
    if !value.is_empty() {
        return value.to_string();
    }
    default.to_string()
}

fn fallback_time(primary: &str, secondary: &str, default: &str) -> String {
    let value = primary.trim();
    if !value.is_empty() {
        return value.to_string();
    }
    let value = secondary.trim();
    if !value.is_empty() {
        return value.to_string();
    }
    default.to_string()
}

fn archived_at(entry: &StructuredMemoryEntry) -> String {
    if entry.archived {
        return fallback_time(&entry.archived_at, &entry.updated_at, &entry.timestamp);
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use crate::events::timestamp_now;
    use crate::sqlite_store::write_memory_entry_sqlite;
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicU64, Ordering};

    static MEMORY_TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn dedupe_keeps_first_entry_per_memory_key() {
        let first = sample_entry("memory-1", "lesson_learned", "同一条记忆", 20);
        let mut duplicate = sample_entry("memory-2", "lesson_learned", "同一条记忆", 80);
        duplicate.timestamp = "2".to_string();
        let third = sample_entry("memory-3", "lesson_learned", "另一条记忆", 20);
        let items = dedupe_memory_entries(vec![first.clone(), duplicate, third.clone()]);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, first.id);
        assert_eq!(items[1].id, third.id);
    }

    #[test]
    fn dedupe_keeps_distinct_memory_object_identity() {
        let mut object = sample_entry("version-1", "project_rule", "同一条记忆", 20);
        object.source_type = "memory_object_current".to_string();
        object.source = "memory://workspace-test/project-rule/rule-object".to_string();
        let legacy = sample_entry("memory-legacy", "project_rule", "同一条记忆", 20);
        let items = dedupe_memory_entries(vec![object.clone(), legacy]);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].source_type, "memory_object_current");
    }

    #[test]
    fn score_prefers_higher_priority_for_same_query() {
        let request = sample_request();
        let high = sample_entry("memory-high", "preference", "统一中文输出", 80);
        let low = sample_entry("memory-low", "lesson_learned", "统一中文输出", 60);
        let high_score = score_memory_entry(&request, "中文输出", high).unwrap().0;
        let low_score = score_memory_entry(&request, "中文输出", low).unwrap().0;
        assert!(high_score > low_score);
    }

    #[test]
    fn append_blocks_low_value_runtime_fallback_memory() {
        let request = sample_request();
        let mut entry = sample_entry("memory-archive", "lesson_learned", "低价值回退", 0);
        entry.source_type = "runtime".to_string();
        entry.content = "无法打开你的计算机，请使用本地文件管理。".to_string();
        assert!(should_archive_memory_entry(&entry));
        let result = append_memory_entry(&request, &entry);
        assert!(result.is_err());
    }

    #[test]
    fn search_includes_current_memory_object_for_duplicate_entry() {
        let request = sample_request();
        let entry = ascii_entry("memory-object-1", "alphaobjectcurrent");
        write_memory_entry_sqlite(&request, &entry).unwrap();
        let hits = search_memory_entries(&request, "alphaobjectcurrent", 3);
        assert!(hits.iter().any(|item| item.source_type == "memory_object_current"));
    }

    #[test]
    fn search_can_hit_current_memory_object_uri() {
        let request = sample_request();
        let entry = ascii_entry("memory-object-2", "uri summary");
        write_memory_entry_sqlite(&request, &entry).unwrap();
        let hits = search_memory_entries(&request, "memory://workspace-test/project-rule/rule-object", 1);
        assert_eq!(hits[0].source_type, "memory_object_current");
        assert_eq!(hits[0].title, "rule-object");
    }

    #[test]
    fn sort_prefers_current_memory_object_on_same_score() {
        let mut scored = vec![
            (50, sample_entry("legacy", "project_rule", "同分旧记录", 10)),
            (50, {
                let mut entry = sample_entry("current", "project_rule", "同分对象", 10);
                entry.source_type = "memory_object_current".to_string();
                entry.source = "memory://workspace-test/project-rule/rule-object".to_string();
                entry
            }),
        ];
        sort_memory_entries(&mut scored);
        assert_eq!(scored[0].1.source_type, "memory_object_current");
    }

    #[test]
    fn normalized_entry_backfills_write_governance_fields() {
        let entry = normalized_memory_entry(&sample_entry("memory-write", "workspace_summary", "项目状态", 0));
        assert_eq!(entry.memory_write_layer, "working_only");
        assert_eq!(entry.memory_write_decision, "rejected");
        assert!(!entry.memory_write_reason.is_empty());
        assert_eq!(entry.memory_duplicate_strategy, "none");
    }

    fn sample_request() -> RunRequest {
        let seq = MEMORY_TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let root = std::env::temp_dir().join(format!("memory-search-{}-{}", timestamp_now(), seq));
        std::fs::create_dir_all(&root).unwrap();
        RunRequest {
            request_id: "request-test".to_string(),
            run_id: "run-test".to_string(),
            session_id: "session-test".to_string(),
            trace_id: "trace-test".to_string(),
            user_input: "test".to_string(),
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

    fn ascii_entry(id: &str, summary: &str) -> MemoryEntry {
        let mut item = sample_entry(id, "project_rule", summary, 12);
        item.title = "rule-object".to_string();
        item.content = format!("content-{summary}");
        item
    }

    fn sample_entry(id: &str, kind: &str, summary: &str, priority: i32) -> MemoryEntry {
        MemoryEntry {
            id: id.to_string(),
            kind: kind.to_string(),
            title: summary.to_string(),
            summary: summary.to_string(),
            content: summary.to_string(),
            scope: "workspace".to_string(),
            workspace_id: "workspace-test".to_string(),
            session_id: "session-test".to_string(),
            source_run_id: "run-test".to_string(),
            source: "run:run-test".to_string(),
            source_type: "runtime".to_string(),
            source_title: summary.to_string(),
            source_event_type: "run_finished".to_string(),
            source_artifact_path: String::new(),
            governance_version: String::new(),
            governance_reason: String::new(),
            governance_source: String::new(),
            governance_at: "1".to_string(),
            archive_reason: String::new(),
            memory_write_layer: String::new(),
            memory_write_decision: String::new(),
            memory_write_reason: String::new(),
            memory_duplicate_strategy: String::new(),
            verified: true,
            priority,
            archived: false,
            archived_at: String::new(),
            created_at: "1".to_string(),
            updated_at: "1".to_string(),
            timestamp: "1".to_string(),
        }
    }
}
