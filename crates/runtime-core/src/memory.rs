use crate::contracts::RunRequest;
use crate::memory_schema::{StructuredMemoryEntry, canonical_kind};
use crate::paths::{long_term_memory_file_path, memory_file_path, memory_tombstone_file_path};
use crate::sqlite_store::{list_memory_entries_sqlite, write_memory_entry_sqlite};
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
    let record = structured_memory_entry(entry);
    write_memory_entry_sqlite(request, entry)?;
    append_jsonl(memory_file_path(request), &record)?;
    append_jsonl(long_term_memory_file_path(request), &record)
}

pub(crate) fn search_memory_entries(
    request: &RunRequest,
    query: &str,
    limit: usize,
) -> Vec<MemoryEntry> {
    let query_text = query.trim();
    let mut scored = score_memory_entries(request, query_text);
    sort_memory_entries(&mut scored);
    scored
        .into_iter()
        .map(|(_, entry)| entry)
        .take(limit)
        .collect()
}

fn to_memory_entry(entry: StructuredMemoryEntry) -> MemoryEntry {
    let kind = canonical_kind(&entry.memory_type);
    let timestamp = entry_timestamp(&entry);
    let summary = fallback_text(&entry.summary, &entry.content, "无摘要");
    let source_title = fallback_text(&entry.source_title, &entry.title, &summary);
    let created_at = fallback_time(&entry.created_at, &entry.timestamp, &timestamp);
    let updated_at = fallback_time(&entry.updated_at, &created_at, &timestamp);
    let archived_at = archived_at(&entry);
    let source_run_id = entry.source_run_id.clone();
    MemoryEntry {
        id: entry.id,
        kind: kind.clone(),
        title: fallback_text(&entry.title, &summary, &kind),
        summary: summary.clone(),
        content: entry.content,
        scope: entry.scope,
        workspace_id: entry.workspace_id,
        session_id: entry.session_id,
        source_run_id: source_run_id.clone(),
        source: fallback_text(&entry.source, &source_run_id, "runtime"),
        source_type: fallback_text(&entry.source_type, "", "runtime"),
        source_title,
        source_event_type: entry.source_event_type,
        source_artifact_path: entry.source_artifact_path,
        verified: entry.verified,
        priority: entry.priority,
        archived: entry.archived,
        archived_at,
        created_at,
        updated_at,
        timestamp,
    }
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
    let sqlite_entries = list_memory_entries_sqlite(request);
    if sqlite_entries.is_empty() {
        entries.extend(read_structured_entries(&memory_file_path(request)));
        entries.extend(read_structured_entries(&long_term_memory_file_path(
            request,
        )));
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

fn score_memory_entry(
    request: &RunRequest,
    query_text: &str,
    entry: MemoryEntry,
) -> Option<(i32, MemoryEntry)> {
    let mut score = base_memory_score(query_text, &entry);
    score += memory_source_priority(&entry);
    if entry.workspace_id == request.workspace_ref.workspace_id {
        score += 8;
    }
    if entry.session_id == request.session_id {
        score += 4;
    }
    (score > 0).then_some((score, entry))
}

fn base_memory_score(query_text: &str, entry: &MemoryEntry) -> i32 {
    let haystack = format!("{} {} {}", entry.kind, entry.summary, entry.content);
    let mut score = score_text(query_text, &haystack);
    if query_text.is_empty() {
        score += 1;
    }
    score
}

fn sort_memory_entries(scored: &mut [(i32, MemoryEntry)]) {
    scored.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| right.1.timestamp.cmp(&left.1.timestamp))
    });
}

fn dedupe_memory_entries(entries: Vec<MemoryEntry>) -> Vec<MemoryEntry> {
    let mut seen = std::collections::BTreeSet::new();
    entries
        .into_iter()
        .filter(|entry| seen.insert(memory_key(entry)))
        .collect()
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
    MemoryEntry {
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
        verified: true,
        priority: 100,
        archived: false,
        archived_at: String::new(),
        created_at: "9999990000000".to_string(),
        updated_at: "9999990000000".to_string(),
        timestamp: "9999990000000".to_string(),
    }
}

fn should_skip_memory_entry(query_text: &str, entry: &MemoryEntry) -> bool {
    is_recursive_memory(entry)
        || is_path_only_memory(entry)
        || is_low_value_runtime_memory(entry)
        || is_test_memory_noise(query_text, entry)
}

fn is_recursive_memory(entry: &MemoryEntry) -> bool {
    entry.summary.contains("文件：run:") || entry.content.contains("文件：run:")
}

fn is_path_only_memory(entry: &MemoryEntry) -> bool {
    looks_like_path_only(&entry.content) || looks_like_path_only(&entry.summary)
}

fn is_low_value_runtime_memory(entry: &MemoryEntry) -> bool {
    is_runtime_project_answer_memory(entry)
        || is_runtime_tool_trace_memory(entry)
        || is_runtime_fallback_memory(entry)
}

fn is_test_memory_noise(query_text: &str, entry: &MemoryEntry) -> bool {
    is_project_query(query_text) && is_test_doc_memory(entry) && !is_seed_memory(entry)
}

fn is_project_query(query_text: &str) -> bool {
    query_text.contains("项目") || query_text.contains("说明") || query_text.contains("做什么")
}

fn is_test_doc_memory(entry: &MemoryEntry) -> bool {
    entry.summary.contains("docs\\07-test")
        || entry.summary.contains("docs/07-test")
        || entry.content.contains("docs\\07-test")
        || entry.content.contains("docs/07-test")
}

fn is_seed_memory(entry: &MemoryEntry) -> bool {
    entry.source_run_id.starts_with("seed:")
}

fn should_archive_memory_entry(entry: &MemoryEntry) -> bool {
    !is_seed_memory(entry) && is_low_value_runtime_memory(entry)
}

fn is_runtime_project_answer_memory(entry: &MemoryEntry) -> bool {
    let project_answer = entry.kind == "project_knowledge" || entry.kind == "workspace_summary";
    let runtime_source = entry.source_type == "runtime";
    let generated = entry.title.contains("项目说明")
        || entry
            .summary
            .contains("已基于项目文档片段完成一次项目说明回答");
    project_answer && runtime_source && generated
}

fn is_runtime_tool_trace_memory(entry: &MemoryEntry) -> bool {
    let trace_title = entry.title.contains("导出知识到思源")
        || entry.title.contains("检索思源笔记")
        || entry.title.contains("读取思源正文")
        || entry.title.contains("复用已存在思源知识");
    let trace_summary = entry.summary.contains("知识已导出到思源目录")
        || entry.summary.contains("已返回思源笔记摘要")
        || entry.summary.contains("思源正文读取成功")
        || entry.summary.contains("命中已存在思源导出");
    entry.kind == "lesson_learned"
        && entry.source_type == "runtime"
        && (trace_title || trace_summary)
}

fn is_runtime_fallback_memory(entry: &MemoryEntry) -> bool {
    entry.kind == "lesson_learned"
        && entry.source_type == "runtime"
        && (is_garbled_reply(&entry.content) || is_capability_fallback(&entry.content))
}

fn is_garbled_reply(content: &str) -> bool {
    content.contains("显示为乱码")
        || content.contains("无法识别为有效的文字或指令")
        || content.contains("无法准确识别您想要表达的意思")
}

fn is_capability_fallback(content: &str) -> bool {
    content.contains("无法打开你的计算机")
        || content.contains("无法控制你的计算机硬件")
        || content.contains("如果你有工作区内的文件管理")
}

fn looks_like_path_only(value: &str) -> bool {
    let text = value.trim();
    (text.contains(":\\") || text.contains(":/"))
        && !text.contains('。')
        && !text.contains('，')
        && !text.contains(' ')
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
