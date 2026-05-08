use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::knowledge_store::KnowledgeRecord;
use crate::memory::{MemoryEntry, normalized_memory_entry};
use crate::memory_object_store::{MemoryObjectRollbackResult, MemoryObjectVersion};
use crate::observation::ObservationRecord;
use crate::paths::sqlite_db_path;
use crate::storage_migration::ensure_workspace_imported;
use rusqlite::{Connection, params};
use std::collections::BTreeSet;
use std::fs;
pub mod checkpoint;
mod cleanup_rules;
pub mod memory_object;
mod schema;
use self::cleanup_rules::{is_runtime_generated_knowledge, is_runtime_generated_memory};
use self::schema::{apply_schema, run_memory_migrations};
pub(crate) use checkpoint::*;
pub(crate) use memory_object::*;

pub(crate) fn write_memory_entry_sqlite(request: &RunRequest, entry: &MemoryEntry) -> Result<(), String> {
    with_connection(request, |conn| {
        insert_memory_entry(conn, entry)?;
        upsert_memory_object_version(conn, entry)?;
        Ok(())
    })
}

pub(crate) fn list_memory_entries_sqlite(request: &RunRequest) -> Vec<MemoryEntry> {
    with_connection(request, |conn| load_memory_entries(conn, request)).unwrap_or_default()
}

pub(crate) fn list_current_memory_object_entries_sqlite(request: &RunRequest) -> Vec<MemoryEntry> {
    with_connection(request, |conn| load_current_memory_object_entries(conn, request)).unwrap_or_default()
}

pub(crate) fn list_current_memory_object_entries_limited_sqlite(
    request: &RunRequest,
    limit: usize,
) -> Vec<MemoryEntry> {
    with_connection(request, |conn| {
        load_current_memory_object_entries_limited(conn, request, limit)
    })
    .unwrap_or_default()
}

#[allow(dead_code)]
pub(crate) fn sync_memory_object_entry_sqlite(
    request: &RunRequest,
    entry: &MemoryEntry,
) -> Result<MemoryObjectVersion, String> {
    with_connection(request, |conn| upsert_memory_object_version(conn, entry))
}

#[allow(dead_code)]
pub(crate) fn list_memory_object_versions_sqlite(request: &RunRequest, object_id: &str) -> Vec<MemoryObjectVersion> {
    with_connection(request, |conn| load_memory_object_versions(conn, object_id)).unwrap_or_default()
}

#[allow(dead_code)]
pub(crate) fn list_memory_object_aliases_sqlite(request: &RunRequest, object_id: &str) -> Vec<String> {
    with_connection(request, |conn| load_memory_object_aliases(conn, object_id)).unwrap_or_default()
}

#[allow(dead_code)]
pub(crate) fn rollback_memory_object_sqlite(
    request: &RunRequest,
    object_id: &str,
    target_version_id: &str,
) -> Result<MemoryObjectRollbackResult, String> {
    with_connection(request, |conn| {
        rollback_memory_object_conn(conn, request, object_id, target_version_id)
    })
}

pub(crate) fn write_knowledge_record_sqlite(request: &RunRequest, record: &KnowledgeRecord) -> Result<(), String> {
    with_connection(request, |conn| insert_knowledge_record(conn, record))
}

pub(crate) fn list_knowledge_records_sqlite(request: &RunRequest) -> Vec<KnowledgeRecord> {
    with_connection(request, |conn| load_knowledge_records(conn, request)).unwrap_or_default()
}

pub(crate) fn write_runtime_checkpoint_sqlite(request: &RunRequest, checkpoint: &RunCheckpoint) -> Result<(), String> {
    with_connection(request, |conn| insert_runtime_checkpoint(conn, checkpoint))
}

pub(crate) fn insert_observation_record(request: &RunRequest, record: &ObservationRecord) -> Result<(), String> {
    with_connection(request, |conn| insert_observation_row(conn, request, record))
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
            governance_source, governance_at, archive_reason, memory_write_layer, memory_write_decision,
            memory_write_reason, memory_duplicate_strategy, verified, priority, archived, archived_at,
            created_at, updated_at, scope, session_id, timestamp
        ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30)",
        params![
            entry.id, entry.workspace_id, entry.kind, entry.title, entry.summary, entry.content,
            entry.source, entry.source_run_id, entry.source_type, entry.source_title,
            entry.source_event_type, entry.source_artifact_path, entry.governance_version,
            entry.governance_reason, entry.governance_source, entry.governance_at,
            entry.archive_reason, entry.memory_write_layer, entry.memory_write_decision,
            entry.memory_write_reason, entry.memory_duplicate_strategy, bool_flag(entry.verified),
            entry.priority, bool_flag(entry.archived), entry.archived_at, entry.created_at,
            entry.updated_at, entry.scope, entry.session_id, entry.timestamp
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

pub(crate) fn insert_knowledge_record(conn: &Connection, record: &KnowledgeRecord) -> Result<(), String> {
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

#[allow(dead_code)]
pub(crate) fn memory_object_count(conn: &Connection, workspace_id: &str) -> Result<i64, String> {
    count_by_workspace(conn, "memory_objects", workspace_id)
}

pub(crate) fn knowledge_count(conn: &Connection, workspace_id: &str) -> Result<i64, String> {
    count_by_workspace(conn, "knowledge_base", workspace_id)
}

fn load_memory_entries(conn: &Connection, request: &RunRequest) -> Result<Vec<MemoryEntry>, String> {
    let mut statement = conn
        .prepare(
            "select id, memory_type, title, summary, content, scope, workspace_id, session_id,
             source_run_id, source, source_type, source_title, source_event_type, source_artifact_path,
             governance_version, governance_reason, governance_source, governance_at, archive_reason,
             memory_write_layer, memory_write_decision, memory_write_reason, memory_duplicate_strategy,
             verified, priority, archived, archived_at, created_at, updated_at, timestamp
             from long_term_memory where workspace_id = ?1
             order by priority desc, length(updated_at) desc, updated_at desc",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![request.workspace_ref.workspace_id.clone()], map_memory_entry)
        .map_err(|error| error.to_string())?;
    collect_rows(rows)
}

fn load_knowledge_records(conn: &Connection, request: &RunRequest) -> Result<Vec<KnowledgeRecord>, String> {
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

fn load_current_memory_object_entries(conn: &Connection, request: &RunRequest) -> Result<Vec<MemoryEntry>, String> {
    load_current_memory_object_entries_limited(conn, request, usize::MAX)
}

fn load_current_memory_object_entries_limited(
    conn: &Connection,
    request: &RunRequest,
    limit: usize,
) -> Result<Vec<MemoryEntry>, String> {
    let sql = "select o.workspace_id, o.memory_type, o.title, o.canonical_uri, v.version_id,
               v.summary, v.content, v.source_run_id, v.priority, v.verified, v.created_at,
               coalesce((select group_concat(alias_uri, ' || ') from memory_object_aliases a
               where a.object_id = o.object_id), '')
               from memory_objects o join memory_object_versions v on o.current_version_id = v.version_id
               where o.workspace_id = ?1 order by v.priority desc, length(v.created_at) desc, v.created_at desc
               limit ?2";
    let mut statement = conn.prepare(sql).map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(
            params![request.workspace_ref.workspace_id.clone(), limit as i64],
            map_memory_object_entry,
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
    apply_schema(conn)?;
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
        memory_write_layer: row.get(19)?,
        memory_write_decision: row.get(20)?,
        memory_write_reason: row.get(21)?,
        memory_duplicate_strategy: row.get(22)?,
        verified: row.get::<_, i32>(23)? != 0,
        priority: row.get(24)?,
        archived: row.get::<_, i32>(25)? != 0,
        archived_at: row.get(26)?,
        created_at: row.get(27)?,
        updated_at: row.get(28)?,
        timestamp: row.get(29)?,
    })
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

fn map_memory_object_entry(row: &rusqlite::Row<'_>) -> rusqlite::Result<MemoryEntry> {
    let workspace_id = row.get::<_, String>(0)?;
    let title = row.get::<_, String>(2)?;
    let created_at = row.get::<_, String>(10)?;
    Ok(normalized_memory_entry(&MemoryEntry {
        id: row.get::<_, String>(4)?,
        kind: row.get(1)?,
        title: title.clone(),
        summary: row.get(5)?,
        content: row.get(6)?,
        scope: "workspace".to_string(),
        workspace_id,
        session_id: String::new(),
        source_run_id: row.get(7)?,
        source: row.get(3)?,
        source_type: "memory_object_current".to_string(),
        source_title: title,
        source_event_type: "memory_object_current".to_string(),
        source_artifact_path: row.get(11)?,
        governance_version: String::new(),
        governance_reason: String::new(),
        governance_source: String::new(),
        governance_at: created_at.clone(),
        archive_reason: String::new(),
        memory_write_layer: String::new(),
        memory_write_decision: String::new(),
        memory_write_reason: String::new(),
        memory_duplicate_strategy: String::new(),
        verified: row.get::<_, i32>(9)? != 0,
        priority: row.get(8)?,
        archived: false,
        archived_at: String::new(),
        created_at: created_at.clone(),
        updated_at: created_at.clone(),
        timestamp: created_at,
    }))
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
               memory_write_layer, memory_write_decision, memory_write_reason, memory_duplicate_strategy,
               verified, priority, archived, archived_at, created_at, updated_at, timestamp
               from long_term_memory
               where trim(governance_version) = '' or trim(governance_reason) = ''
               or trim(governance_source) = '' or trim(governance_at) = ''
               or trim(memory_write_layer) = '' or trim(memory_write_decision) = ''
               or trim(memory_write_reason) = '' or trim(memory_duplicate_strategy) = ''
               or (archived != 0 and trim(archive_reason) = '')";
    let mut statement = conn.prepare(sql).map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], map_memory_entry)
        .map_err(|error| error.to_string())?;
    collect_rows(rows)
}

fn load_memory_entries_for_workspace(conn: &Connection, workspace_id: &str) -> Result<Vec<MemoryEntry>, String> {
    let request = workspace_request(workspace_id);
    load_memory_entries(conn, &request)
}

fn load_knowledge_records_for_workspace(conn: &Connection, workspace_id: &str) -> Result<Vec<KnowledgeRecord>, String> {
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
        || current.memory_write_layer != normalized.memory_write_layer
        || current.memory_write_decision != normalized.memory_write_decision
        || current.memory_write_reason != normalized.memory_write_reason
        || current.memory_duplicate_strategy != normalized.memory_duplicate_strategy
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
    let recursive =
        item.source.starts_with("run:") && (item.summary.contains("文件：run:") || item.content.contains("文件：run:"));
    (recursive || is_runtime_generated_knowledge(&item) || !seen.insert(key)).then_some(item.id)
}

fn update_memory_governance(conn: &Connection, entry: &MemoryEntry) -> Result<(), String> {
    conn.execute(
        "update long_term_memory
         set governance_version = ?1, governance_reason = ?2, governance_source = ?3,
             governance_at = ?4, archive_reason = ?5, memory_write_layer = ?6,
             memory_write_decision = ?7, memory_write_reason = ?8, memory_duplicate_strategy = ?9
         where id = ?10",
        params![
            entry.governance_version,
            entry.governance_reason,
            entry.governance_source,
            entry.governance_at,
            entry.archive_reason,
            entry.memory_write_layer,
            entry.memory_write_decision,
            entry.memory_write_reason,
            entry.memory_duplicate_strategy,
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
        conn.execute(&sql, params![id]).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn memory_key(item: &MemoryEntry) -> String {
    format!("{}|{}|{}|{}", item.workspace_id, item.kind, item.title, item.summary)
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

pub(crate) fn load_memory_entries_for_workspace_conn(
    conn: &Connection,
    workspace_id: &str,
) -> Result<Vec<MemoryEntry>, String> {
    load_memory_entries_for_workspace(conn, workspace_id)
}

fn slug_part(value: &str) -> String {
    let output = value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    output.trim_matches('-').to_lowercase()
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
