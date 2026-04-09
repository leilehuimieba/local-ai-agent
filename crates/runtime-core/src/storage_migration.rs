use crate::contracts::RunRequest;
use crate::knowledge::is_recursive_record;
use crate::knowledge_store::KnowledgeRecord;
use crate::memory::{MemoryEntry, normalized_memory_entry};
use crate::memory_schema::canonical_kind_for_record;
use crate::paths::{knowledge_base_file_path, long_term_memory_file_path, memory_file_path};
use crate::sqlite_store::{
    insert_knowledge_record, insert_memory_entry, knowledge_count, memory_count,
};
use crate::storage::{overwrite_jsonl, read_jsonl};
use rusqlite::Connection;

pub(crate) fn ensure_workspace_imported(
    request: &RunRequest,
    conn: &Connection,
) -> Result<(), String> {
    import_memory_if_needed(request, conn)?;
    import_knowledge_if_needed(request, conn)?;
    compact_legacy_files(request)
}

fn import_memory_if_needed(request: &RunRequest, conn: &Connection) -> Result<(), String> {
    if memory_count(conn, &request.workspace_ref.workspace_id)? > 0 {
        return Ok(());
    }
    for entry in legacy_memory_entries(request) {
        insert_memory_entry(conn, &entry)?;
    }
    Ok(())
}

fn import_knowledge_if_needed(request: &RunRequest, conn: &Connection) -> Result<(), String> {
    if knowledge_count(conn, &request.workspace_ref.workspace_id)? > 0 {
        return Ok(());
    }
    for record in legacy_knowledge_records(request) {
        insert_knowledge_record(conn, &record)?;
    }
    Ok(())
}

fn legacy_memory_entries(request: &RunRequest) -> Vec<MemoryEntry> {
    let mut entries = Vec::new();
    entries.extend(read_legacy_memory(memory_file_path(request)));
    entries.extend(read_legacy_memory(long_term_memory_file_path(request)));
    dedupe_memory(entries)
}

fn legacy_knowledge_records(request: &RunRequest) -> Vec<KnowledgeRecord> {
    read_jsonl::<KnowledgeRecord>(&knowledge_base_file_path(request))
        .into_iter()
        .map(normalize_knowledge_record)
        .filter(|record| !is_recursive_record(record))
        .collect::<Vec<_>>()
        .into_iter()
        .filter(dedupe_knowledge())
        .collect()
}

fn compact_legacy_files(request: &RunRequest) -> Result<(), String> {
    compact_memory_file(memory_file_path(request))?;
    compact_memory_file(long_term_memory_file_path(request))?;
    compact_knowledge_file(knowledge_base_file_path(request))
}

fn compact_memory_file(path: std::path::PathBuf) -> Result<(), String> {
    let items = read_legacy_memory(path.clone());
    let entries = dedupe_memory(items);
    overwrite_jsonl(path, &entries)
}

fn compact_knowledge_file(path: std::path::PathBuf) -> Result<(), String> {
    let entries = read_jsonl::<KnowledgeRecord>(&path)
        .into_iter()
        .map(normalize_knowledge_record)
        .filter(|record| !is_recursive_record(record))
        .collect::<Vec<_>>()
        .into_iter()
        .filter(dedupe_knowledge())
        .collect::<Vec<_>>();
    overwrite_jsonl(path, &entries)
}

fn read_legacy_memory(path: std::path::PathBuf) -> Vec<MemoryEntry> {
    read_jsonl::<crate::memory_schema::StructuredMemoryEntry>(&path)
        .into_iter()
        .map(legacy_memory_entry)
        .collect()
}

fn legacy_memory_entry(entry: crate::memory_schema::StructuredMemoryEntry) -> MemoryEntry {
    let source_title = default_text(&entry.source_title, &entry.title, &entry.summary);
    let governance_version = entry.governance_version.clone();
    let governance_reason = default_text(
        &entry.governance_reason,
        "",
        "历史 JSONL 记忆已导入 SQLite 并补齐审计字段。",
    );
    let governance_source = default_text(&entry.governance_source, "", "storage_migration");
    let governance_at = default_text(&entry.governance_at, &entry.updated_at, &entry.timestamp);
    let archive_reason = archive_reason(&entry);
    let archived_at = archived_at(&entry);
    normalized_memory_entry(&MemoryEntry {
        id: entry.id,
        kind: canonical_kind_for_record(&entry.memory_type, &entry.title, &entry.summary),
        title: entry.title,
        summary: entry.summary,
        content: entry.content,
        scope: entry.scope,
        workspace_id: entry.workspace_id,
        session_id: entry.session_id,
        source_run_id: entry.source_run_id.clone(),
        source: default_text(&entry.source, &entry.source_run_id, "runtime"),
        source_type: default_text(&entry.source_type, "", "runtime"),
        source_title,
        source_event_type: entry.source_event_type,
        source_artifact_path: entry.source_artifact_path,
        governance_version,
        governance_reason,
        governance_source,
        governance_at,
        archive_reason,
        verified: entry.verified,
        priority: entry.priority,
        archived: entry.archived,
        archived_at,
        created_at: entry.created_at,
        updated_at: entry.updated_at,
        timestamp: entry.timestamp,
    })
}

fn normalize_knowledge_record(record: KnowledgeRecord) -> KnowledgeRecord {
    KnowledgeRecord {
        source_type: default_text(&record.source_type, "", "runtime"),
        priority: record.priority,
        archived: record.archived,
        ..record
    }
}

fn dedupe_memory(entries: Vec<MemoryEntry>) -> Vec<MemoryEntry> {
    let mut seen = std::collections::BTreeSet::new();
    entries
        .into_iter()
        .filter(|entry| !should_drop_memory(entry))
        .filter(|entry| seen.insert(memory_key(entry)))
        .collect()
}

fn default_text(primary: &str, secondary: &str, fallback: &str) -> String {
    if !primary.trim().is_empty() {
        return primary.to_string();
    }
    if !secondary.trim().is_empty() {
        return secondary.to_string();
    }
    fallback.to_string()
}

fn archived_at(entry: &crate::memory_schema::StructuredMemoryEntry) -> String {
    if entry.archived {
        return default_text(&entry.archived_at, &entry.updated_at, &entry.timestamp);
    }
    String::new()
}

fn archive_reason(entry: &crate::memory_schema::StructuredMemoryEntry) -> String {
    if !entry.archived {
        return String::new();
    }
    default_text(
        &entry.archive_reason,
        &entry.governance_reason,
        "历史记录在迁移前已标记归档。",
    )
}

fn dedupe_knowledge() -> impl FnMut(&KnowledgeRecord) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    move |record| !should_drop_knowledge(record) && seen.insert(knowledge_key(record))
}

fn memory_key(entry: &MemoryEntry) -> String {
    format!(
        "{}|{}|{}|{}",
        entry.workspace_id, entry.kind, entry.title, entry.summary
    )
}

fn knowledge_key(record: &KnowledgeRecord) -> String {
    format!(
        "{}|{}|{}|{}",
        record.workspace_id, record.title, record.summary, record.source_type
    )
}

fn should_drop_memory(entry: &MemoryEntry) -> bool {
    is_runtime_project_answer_memory(entry)
        || is_runtime_tool_trace_memory(entry)
        || is_runtime_fallback_memory(entry)
        || is_low_value_runtime_lesson(entry)
        || is_legacy_preference_noise(entry)
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

fn is_low_value_runtime_lesson(entry: &MemoryEntry) -> bool {
    let generic_context = entry.kind == "lesson_learned"
        && entry.source_type == "runtime"
        && entry.title.contains("基于会话压缩摘要继续回答。")
        && entry.summary.contains("已从最近")
        && entry.summary.contains("完成一次模型回答");
    let tool_trace = entry.kind == "lesson_learned"
        && entry.source_type == "runtime"
        && (entry.title.contains("读取文件：")
            || entry.title.contains("执行命令：")
            || entry.summary.contains("文件读取成功")
            || entry.summary.contains("命令执行成功"));
    generic_context || tool_trace
}

fn is_legacy_preference_noise(entry: &MemoryEntry) -> bool {
    entry.kind == "preference" && entry.title.trim().is_empty() && !entry.verified
}

fn should_drop_knowledge(record: &KnowledgeRecord) -> bool {
    let runtime_generated = record.source_type == "runtime" && record.source.starts_with("run:");
    let project_answer = record.title.contains("项目说明")
        || record
            .summary
            .contains("已基于项目文档片段完成一次项目说明回答");
    runtime_generated && project_answer
}
