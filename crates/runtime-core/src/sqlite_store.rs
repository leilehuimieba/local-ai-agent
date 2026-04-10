use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::knowledge_store::KnowledgeRecord;
use crate::memory::{MemoryEntry, normalized_memory_entry};
use crate::paths::sqlite_db_path;
use crate::storage_migration::ensure_workspace_imported;
use rusqlite::{Connection, params};
use serde_json::{from_str, to_string};
use std::collections::BTreeSet;
use std::fs;

pub(crate) fn write_memory_entry_sqlite(
    request: &RunRequest,
    entry: &MemoryEntry,
) -> Result<(), String> {
    with_connection(request, |conn| insert_memory_entry(conn, entry))
}

pub(crate) fn list_memory_entries_sqlite(request: &RunRequest) -> Vec<MemoryEntry> {
    with_connection(request, |conn| load_memory_entries(conn, request)).unwrap_or_default()
}

pub(crate) fn write_knowledge_record_sqlite(
    request: &RunRequest,
    record: &KnowledgeRecord,
) -> Result<(), String> {
    with_connection(request, |conn| insert_knowledge_record(conn, record))
}

pub(crate) fn list_knowledge_records_sqlite(request: &RunRequest) -> Vec<KnowledgeRecord> {
    with_connection(request, |conn| load_knowledge_records(conn, request)).unwrap_or_default()
}

pub(crate) fn write_runtime_checkpoint_sqlite(
    request: &RunRequest,
    checkpoint: &RunCheckpoint,
) -> Result<(), String> {
    with_connection(request, |conn| insert_runtime_checkpoint(conn, checkpoint))
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn load_runtime_checkpoint_sqlite(
    request: &RunRequest,
    checkpoint_id: &str,
) -> Result<Option<RunCheckpoint>, String> {
    with_connection(request, |conn| select_runtime_checkpoint(conn, checkpoint_id))
}

pub(crate) fn with_connection<T, F>(request: &RunRequest, f: F) -> Result<T, String>
where
    F: FnOnce(&Connection) -> Result<T, String>,
{
    let path = sqlite_db_path(request);
    create_parent_dir(&path)?;
    let conn = Connection::open(path).map_err(|error| error.to_string())?;
    init_schema(&conn)?;
    ensure_workspace_imported(request, &conn)?;
    cleanup_workspace_records(&conn, &request.workspace_ref.workspace_id)?;
    f(&conn)
}

pub(crate) fn insert_memory_entry(conn: &Connection, entry: &MemoryEntry) -> Result<(), String> {
    conn.execute(
        "insert or ignore into long_term_memory (
            id, workspace_id, memory_type, title, summary, content, source, source_run_id, source_type,
            source_title, source_event_type, source_artifact_path, governance_version, governance_reason,
            governance_source, governance_at, archive_reason, verified, priority, archived, archived_at,
            created_at, updated_at, scope, session_id, timestamp
        ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)",
        params![
            entry.id, entry.workspace_id, entry.kind, entry.title, entry.summary, entry.content,
            entry.source, entry.source_run_id, entry.source_type, entry.source_title,
            entry.source_event_type, entry.source_artifact_path, entry.governance_version,
            entry.governance_reason, entry.governance_source, entry.governance_at,
            entry.archive_reason, bool_flag(entry.verified), entry.priority,
            bool_flag(entry.archived), entry.archived_at, entry.created_at, entry.updated_at,
            entry.scope, entry.session_id, entry.timestamp
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

pub(crate) fn insert_knowledge_record(
    conn: &Connection,
    record: &KnowledgeRecord,
) -> Result<(), String> {
    conn.execute(
        "insert or ignore into knowledge_base (
            id, workspace_id, knowledge_type, title, summary, content, tags, source,
            source_type, verified, priority, archived, created_at, updated_at
        ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            record.id,
            record.workspace_id,
            record.knowledge_type,
            record.title,
            record.summary,
            record.content,
            encode_tags(&record.tags),
            record.source,
            record.source_type,
            bool_flag(record.verified),
            record.priority,
            bool_flag(record.archived),
            record.created_at,
            record.updated_at
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

pub(crate) fn memory_count(conn: &Connection, workspace_id: &str) -> Result<i64, String> {
    count_by_workspace(conn, "long_term_memory", workspace_id)
}

pub(crate) fn knowledge_count(conn: &Connection, workspace_id: &str) -> Result<i64, String> {
    count_by_workspace(conn, "knowledge_base", workspace_id)
}

fn load_memory_entries(
    conn: &Connection,
    request: &RunRequest,
) -> Result<Vec<MemoryEntry>, String> {
    let mut statement = conn
        .prepare(
            "select id, memory_type, title, summary, content, scope, workspace_id, session_id,
             source_run_id, source, source_type, source_title, source_event_type, source_artifact_path,
             governance_version, governance_reason, governance_source, governance_at, archive_reason,
             verified, priority, archived, archived_at, created_at, updated_at, timestamp
             from long_term_memory where workspace_id = ?1 order by priority desc, updated_at desc",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(
            params![request.workspace_ref.workspace_id.clone()],
            map_memory_entry,
        )
        .map_err(|error| error.to_string())?;
    collect_rows(rows)
}

fn load_knowledge_records(
    conn: &Connection,
    request: &RunRequest,
) -> Result<Vec<KnowledgeRecord>, String> {
    let mut statement = conn
        .prepare(
            "select id, knowledge_type, title, summary, content, tags, source, source_type,
             verified, workspace_id, priority, archived, created_at, updated_at
             from knowledge_base where workspace_id = ?1 order by priority desc, updated_at desc",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(
            params![request.workspace_ref.workspace_id.clone()],
            map_knowledge_record,
        )
        .map_err(|error| error.to_string())?;
    collect_rows(rows)
}

fn create_parent_dir(path: &std::path::Path) -> Result<(), String> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    fs::create_dir_all(parent).map_err(|error| error.to_string())
}

fn init_schema(conn: &Connection) -> Result<(), String> {
    for statement in SCHEMA_STATEMENTS {
        conn.execute(statement, [])
            .map_err(|error| error.to_string())?;
    }
    run_memory_migrations(conn)?;
    backfill_memory_governance(conn)
}

fn cleanup_workspace_records(conn: &Connection, workspace_id: &str) -> Result<(), String> {
    cleanup_memory_records(conn, workspace_id)?;
    cleanup_knowledge_records(conn, workspace_id)
}

fn cleanup_memory_records(conn: &Connection, workspace_id: &str) -> Result<(), String> {
    let items = load_memory_entries_for_workspace(conn, workspace_id)?;
    let stale = duplicate_memory_ids(items);
    delete_records(conn, "long_term_memory", &stale)
}

fn cleanup_knowledge_records(conn: &Connection, workspace_id: &str) -> Result<(), String> {
    let items = load_knowledge_records_for_workspace(conn, workspace_id)?;
    let stale = duplicate_knowledge_ids(items);
    delete_records(conn, "knowledge_base", &stale)
}

fn map_memory_entry(row: &rusqlite::Row<'_>) -> rusqlite::Result<MemoryEntry> {
    Ok(MemoryEntry {
        id: row.get(0)?,
        kind: row.get(1)?,
        title: row.get(2)?,
        summary: row.get(3)?,
        content: row.get(4)?,
        scope: row.get(5)?,
        workspace_id: row.get(6)?,
        session_id: row.get(7)?,
        source_run_id: row.get(8)?,
        source: row.get(9)?,
        source_type: row.get(10)?,
        source_title: row.get(11)?,
        source_event_type: row.get(12)?,
        source_artifact_path: row.get(13)?,
        governance_version: row.get(14)?,
        governance_reason: row.get(15)?,
        governance_source: row.get(16)?,
        governance_at: row.get(17)?,
        archive_reason: row.get(18)?,
        verified: row.get::<_, i32>(19)? != 0,
        priority: row.get(20)?,
        archived: row.get::<_, i32>(21)? != 0,
        archived_at: row.get(22)?,
        created_at: row.get(23)?,
        updated_at: row.get(24)?,
        timestamp: row.get(25)?,
    })
}

fn run_memory_migrations(conn: &Connection) -> Result<(), String> {
    for statement in MEMORY_MIGRATIONS {
        apply_memory_migration(conn, statement)?;
    }
    Ok(())
}

fn backfill_memory_governance(conn: &Connection) -> Result<(), String> {
    for entry in pending_governance_entries(conn)? {
        let normalized = normalized_memory_entry(&entry);
        if governance_changed(&entry, &normalized) {
            update_memory_governance(conn, &normalized)?;
        }
    }
    Ok(())
}

fn apply_memory_migration(conn: &Connection, statement: &str) -> Result<(), String> {
    match conn.execute(statement, []) {
        Ok(_) => Ok(()),
        Err(error) if error.to_string().contains("duplicate column name") => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

fn map_knowledge_record(row: &rusqlite::Row<'_>) -> rusqlite::Result<KnowledgeRecord> {
    Ok(KnowledgeRecord {
        id: row.get(0)?,
        knowledge_type: row.get(1)?,
        title: row.get(2)?,
        summary: row.get(3)?,
        content: row.get(4)?,
        tags: decode_tags(row.get::<_, String>(5)?),
        source: row.get(6)?,
        source_type: row.get(7)?,
        verified: row.get::<_, i32>(8)? != 0,
        workspace_id: row.get(9)?,
        priority: row.get(10)?,
        archived: row.get::<_, i32>(11)? != 0,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
    })
}

fn collect_rows<T, F>(rows: rusqlite::MappedRows<'_, F>) -> Result<Vec<T>, String>
where
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
{
    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|error| error.to_string())?);
    }
    Ok(items)
}

fn pending_governance_entries(conn: &Connection) -> Result<Vec<MemoryEntry>, String> {
    let sql = "select id, memory_type, title, summary, content, scope, workspace_id, session_id,
               source_run_id, source, source_type, source_title, source_event_type, source_artifact_path,
               governance_version, governance_reason, governance_source, governance_at, archive_reason,
               verified, priority, archived, archived_at, created_at, updated_at, timestamp
               from long_term_memory
               where trim(governance_version) = '' or trim(governance_reason) = ''
               or trim(governance_source) = '' or trim(governance_at) = ''
               or (archived != 0 and trim(archive_reason) = '')";
    let mut statement = conn.prepare(sql).map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], map_memory_entry)
        .map_err(|error| error.to_string())?;
    collect_rows(rows)
}

fn load_memory_entries_for_workspace(
    conn: &Connection,
    workspace_id: &str,
) -> Result<Vec<MemoryEntry>, String> {
    let request = workspace_request(workspace_id);
    load_memory_entries(conn, &request)
}

fn load_knowledge_records_for_workspace(
    conn: &Connection,
    workspace_id: &str,
) -> Result<Vec<KnowledgeRecord>, String> {
    let request = workspace_request(workspace_id);
    load_knowledge_records(conn, &request)
}

fn workspace_request(workspace_id: &str) -> RunRequest {
    RunRequest {
        request_id: String::new(),
        run_id: String::new(),
        session_id: String::new(),
        trace_id: String::new(),
        user_input: String::new(),
        mode: String::new(),
        model_ref: crate::contracts::ModelRef {
            provider_id: String::new(),
            model_id: String::new(),
            display_name: String::new(),
        },
        provider_ref: Default::default(),
        workspace_ref: crate::contracts::WorkspaceRef {
            workspace_id: workspace_id.to_string(),
            name: String::new(),
            root_path: String::new(),
            is_active: true,
        },
        context_hints: Default::default(),
        resume_from_checkpoint_id: String::new(),
        resume_strategy: String::new(),
        confirmation_decision: None,
    }
}

fn duplicate_memory_ids(items: Vec<MemoryEntry>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    items
        .into_iter()
        .filter_map(|item| stale_memory_id(&mut seen, item))
        .collect()
}

fn governance_changed(current: &MemoryEntry, normalized: &MemoryEntry) -> bool {
    current.governance_version != normalized.governance_version
        || current.governance_reason != normalized.governance_reason
        || current.governance_source != normalized.governance_source
        || current.governance_at != normalized.governance_at
        || current.archive_reason != normalized.archive_reason
}

fn duplicate_knowledge_ids(items: Vec<KnowledgeRecord>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    items
        .into_iter()
        .filter_map(|item| stale_knowledge_id(&mut seen, item))
        .collect()
}

fn stale_knowledge_id(seen: &mut BTreeSet<String>, item: KnowledgeRecord) -> Option<String> {
    let key = knowledge_key(&item);
    let recursive = item.source.starts_with("run:")
        && (item.summary.contains("文件：run:") || item.content.contains("文件：run:"));
    (recursive || is_runtime_generated_knowledge(&item) || !seen.insert(key)).then_some(item.id)
}

fn update_memory_governance(conn: &Connection, entry: &MemoryEntry) -> Result<(), String> {
    conn.execute(
        "update long_term_memory
         set governance_version = ?1, governance_reason = ?2, governance_source = ?3,
             governance_at = ?4, archive_reason = ?5
         where id = ?6",
        params![
            entry.governance_version,
            entry.governance_reason,
            entry.governance_source,
            entry.governance_at,
            entry.archive_reason,
            entry.id
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

fn stale_memory_id(seen: &mut BTreeSet<String>, item: MemoryEntry) -> Option<String> {
    let key = memory_key(&item);
    (is_runtime_generated_memory(&item) || !seen.insert(key)).then_some(item.id)
}

fn delete_records(conn: &Connection, table: &str, ids: &[String]) -> Result<(), String> {
    for id in ids {
        let sql = format!("delete from {table} where id = ?1");
        conn.execute(&sql, params![id])
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn memory_key(item: &MemoryEntry) -> String {
    format!(
        "{}|{}|{}|{}",
        item.workspace_id, item.kind, item.title, item.summary
    )
}

fn knowledge_key(item: &KnowledgeRecord) -> String {
    format!(
        "{}|{}|{}|{}",
        item.workspace_id, item.title, item.summary, item.source_type
    )
}

fn count_by_workspace(conn: &Connection, table: &str, workspace_id: &str) -> Result<i64, String> {
    let sql = format!("select count(1) from {table} where workspace_id = ?1");
    conn.query_row(&sql, params![workspace_id], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn encode_tags(tags: &[String]) -> String {
    serde_json::to_string(tags).unwrap_or_else(|_| "[]".to_string())
}

fn decode_tags(value: String) -> Vec<String> {
    serde_json::from_str(&value).unwrap_or_default()
}

fn bool_flag(value: bool) -> i32 {
    if value { 1 } else { 0 }
}

fn is_runtime_generated_memory(item: &MemoryEntry) -> bool {
    let project_answer = item.kind == "project_knowledge" || item.kind == "workspace_summary";
    let generated_answer = item.title.contains("项目说明")
        || item
            .summary
            .contains("已基于项目文档片段完成一次项目说明回答");
    let tool_trace = item.kind == "lesson_learned"
        && (item.title.contains("导出知识到思源")
            || item.title.contains("检索思源笔记")
            || item.title.contains("读取思源正文")
            || item.title.contains("复用已存在思源知识")
            || item.summary.contains("知识已导出到思源目录")
            || item.summary.contains("已返回思源笔记摘要")
            || item.summary.contains("思源正文读取成功")
            || item.summary.contains("命中已存在思源导出"));
    let fallback = item.kind == "lesson_learned"
        && (is_garbled_reply(&item.content) || is_capability_fallback(&item.content));
    item.source_type == "runtime"
        && (project_answer && generated_answer
            || tool_trace
            || fallback
            || is_low_value_runtime_lesson(item)
            || is_legacy_preference_noise(item))
}

fn is_runtime_generated_knowledge(item: &KnowledgeRecord) -> bool {
    let project_answer = item.title.contains("项目说明")
        || item
            .summary
            .contains("已基于项目文档片段完成一次项目说明回答");
    item.source_type == "runtime" && item.source.starts_with("run:") && project_answer
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

fn is_low_value_runtime_lesson(item: &MemoryEntry) -> bool {
    let generic_context = item.kind == "lesson_learned"
        && item.title.contains("基于会话压缩摘要继续回答。")
        && item.summary.contains("已从最近")
        && item.summary.contains("完成一次模型回答");
    let tool_trace = item.kind == "lesson_learned"
        && (item.title.contains("读取文件：")
            || item.title.contains("执行命令：")
            || item.summary.contains("文件读取成功")
            || item.summary.contains("命令执行成功"));
    generic_context || tool_trace
}

fn is_legacy_preference_noise(item: &MemoryEntry) -> bool {
    item.kind == "preference" && item.title.trim().is_empty() && !item.verified
}

const SCHEMA_STATEMENTS: [&str; 10] = [
    "create table if not exists long_term_memory (
        id text primary key,
        workspace_id text not null,
        memory_type text not null,
        title text not null,
        summary text not null,
        content text not null,
        source text not null,
        source_run_id text not null,
        source_type text not null,
        source_title text not null default '',
        source_event_type text not null default '',
        source_artifact_path text not null default '',
        governance_version text not null default '',
        governance_reason text not null default '',
        governance_source text not null default '',
        governance_at text not null default '',
        archive_reason text not null default '',
        verified integer not null default 0,
        priority integer not null default 0,
        archived integer not null default 0,
        archived_at text not null default '',
        created_at text not null,
        updated_at text not null,
        scope text not null,
        session_id text not null,
        timestamp text not null
    )",
    "create index if not exists idx_memory_workspace_type on long_term_memory (workspace_id, memory_type)",
    "create index if not exists idx_memory_workspace_updated on long_term_memory (workspace_id, updated_at)",
    "create index if not exists idx_memory_workspace_priority on long_term_memory (workspace_id, priority)",
    "create table if not exists knowledge_base (
        id text primary key,
        workspace_id text not null,
        knowledge_type text not null,
        title text not null,
        summary text not null,
        content text not null,
        tags text not null,
        source text not null,
        source_type text not null,
        verified integer not null default 0,
        priority integer not null default 0,
        archived integer not null default 0,
        created_at text not null,
        updated_at text not null
    )",
    "create index if not exists idx_knowledge_workspace_type on knowledge_base (workspace_id, knowledge_type)",
    "create index if not exists idx_knowledge_workspace_source on knowledge_base (workspace_id, source_type)",
    "create index if not exists idx_knowledge_workspace_updated on knowledge_base (workspace_id, updated_at)",
    "create table if not exists runtime_checkpoints (
        checkpoint_id text primary key,
        run_id text not null,
        session_id text not null,
        trace_id text not null,
        workspace_id text not null,
        status text not null,
        final_stage text not null,
        resumable integer not null default 0,
        resume_reason text not null default '',
        resume_stage text not null default '',
        event_count integer not null default 0,
        request_payload text not null,
        response_payload text not null,
        created_at text not null
    )",
    "create index if not exists idx_checkpoint_run on runtime_checkpoints (run_id, created_at)",
];

const MEMORY_MIGRATIONS: [&str; 11] = [
    "alter table long_term_memory add column source_title text not null default ''",
    "alter table long_term_memory add column source_event_type text not null default ''",
    "alter table long_term_memory add column source_artifact_path text not null default ''",
    "alter table long_term_memory add column governance_version text not null default ''",
    "alter table long_term_memory add column governance_reason text not null default ''",
    "alter table long_term_memory add column governance_source text not null default ''",
    "alter table long_term_memory add column governance_at text not null default ''",
    "alter table long_term_memory add column archive_reason text not null default ''",
    "alter table long_term_memory add column archived_at text not null default ''",
    "alter table runtime_checkpoints add column resume_reason text not null default ''",
    "alter table runtime_checkpoints add column resume_stage text not null default ''",
];

fn insert_runtime_checkpoint(
    conn: &Connection,
    checkpoint: &RunCheckpoint,
) -> Result<(), String> {
    let request_payload = to_string(&checkpoint.request).map_err(|error| error.to_string())?;
    let response_payload = to_string(&checkpoint.response).map_err(|error| error.to_string())?;
    conn.execute(
        "insert or replace into runtime_checkpoints (
            checkpoint_id, run_id, session_id, trace_id, workspace_id, status, final_stage,
            resumable, resume_reason, resume_stage, event_count, request_payload, response_payload, created_at
        ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            checkpoint.checkpoint_id, checkpoint.run_id, checkpoint.session_id, checkpoint.trace_id,
            checkpoint.workspace_id, checkpoint.status, checkpoint.final_stage,
            bool_flag(checkpoint.resumable), checkpoint.resume_reason, checkpoint.resume_stage,
            checkpoint.event_count, request_payload, response_payload, checkpoint.created_at
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

#[cfg_attr(not(test), allow(dead_code))]
fn select_runtime_checkpoint(
    conn: &Connection,
    checkpoint_id: &str,
) -> Result<Option<RunCheckpoint>, String> {
    let mut statement = conn
        .prepare(
            "select checkpoint_id, run_id, session_id, trace_id, workspace_id, status, final_stage,
             resumable, resume_reason, resume_stage, event_count, request_payload, response_payload, created_at
             from runtime_checkpoints where checkpoint_id = ?1",
        )
        .map_err(|error| error.to_string())?;
    let mut rows = statement
        .query(params![checkpoint_id])
        .map_err(|error| error.to_string())?;
    match rows.next().map_err(|error| error.to_string())? {
        Some(row) => decode_runtime_checkpoint(row).map(Some),
        None => Ok(None),
    }
}

#[cfg_attr(not(test), allow(dead_code))]
fn decode_runtime_checkpoint(row: &rusqlite::Row<'_>) -> Result<RunCheckpoint, String> {
    let request_payload: String = row.get(11).map_err(|error| error.to_string())?;
    let response_payload: String = row.get(12).map_err(|error| error.to_string())?;
    Ok(RunCheckpoint {
        checkpoint_id: row.get(0).map_err(|error| error.to_string())?,
        run_id: row.get(1).map_err(|error| error.to_string())?,
        session_id: row.get(2).map_err(|error| error.to_string())?,
        trace_id: row.get(3).map_err(|error| error.to_string())?,
        workspace_id: row.get(4).map_err(|error| error.to_string())?,
        status: row.get(5).map_err(|error| error.to_string())?,
        final_stage: row.get(6).map_err(|error| error.to_string())?,
        resumable: row.get::<_, i32>(7).map_err(|error| error.to_string())? != 0,
        resume_reason: row.get(8).map_err(|error| error.to_string())?,
        resume_stage: row.get(9).map_err(|error| error.to_string())?,
        event_count: row.get(10).map_err(|error| error.to_string())?,
        request: from_str(&request_payload).map_err(|error| error.to_string())?,
        response: from_str(&response_payload).map_err(|error| error.to_string())?,
        created_at: row.get(13).map_err(|error| error.to_string())?,
    })
}
