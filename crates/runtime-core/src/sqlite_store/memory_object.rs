use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::memory::MemoryEntry;
use crate::memory_object_store::{MemoryObjectRollbackResult, MemoryObjectVersion};
use crate::sqlite_store::{bool_flag, collect_rows, insert_memory_entry, slug_part};
use rusqlite::{Connection, params};
use std::hash::{DefaultHasher, Hash, Hasher};

pub(crate) fn upsert_memory_object_version(
    conn: &Connection,
    entry: &MemoryEntry,
) -> Result<MemoryObjectVersion, String> {
    if let Some(version) = load_memory_object_version_for_entry(conn, entry)? {
        return Ok(version);
    }
    upsert_memory_object_version_with_restore(conn, entry, "")
}

fn load_memory_object_version_for_entry(
    conn: &Connection,
    entry: &MemoryEntry,
) -> Result<Option<MemoryObjectVersion>, String> {
    let object_id = object_id_for_entry(entry);
    let version_id = version_id_for_entry(entry);
    match load_memory_object_version_row(conn, &object_id, &version_id) {
        Ok(version) => Ok(Some(version)),
        Err(error) if error.contains("Query returned no rows") => Ok(None),
        Err(error) => Err(error),
    }
}

fn upsert_memory_object_version_with_restore(
    conn: &Connection,
    entry: &MemoryEntry,
    restored_from_version_id: &str,
) -> Result<MemoryObjectVersion, String> {
    let object_id = object_id_for_entry(entry);
    let version_id = version_id_for_entry(entry);
    let canonical_uri = canonical_uri_for_entry(entry);
    insert_memory_object(conn, &object_id, &canonical_uri, entry)?;
    deactivate_current_versions(conn, &object_id)?;
    insert_memory_object_version(
        conn,
        &object_id,
        &version_id,
        entry,
        restored_from_version_id,
    )?;
    insert_memory_object_alias(conn, &object_id, &canonical_uri, &entry.created_at)?;
    set_memory_object_current(conn, &object_id, &version_id, &canonical_uri, entry)?;
    Ok(build_memory_object_version(
        &object_id,
        &version_id,
        &canonical_uri,
        entry,
        true,
        restored_from_version_id,
    ))
}

fn insert_memory_object(
    conn: &Connection,
    object_id: &str,
    canonical_uri: &str,
    entry: &MemoryEntry,
) -> Result<(), String> {
    conn.execute(
        "insert or ignore into memory_objects (
            object_id, workspace_id, memory_type, title, canonical_uri, current_version_id, created_at, updated_at
        ) values (?1, ?2, ?3, ?4, ?5, '', ?6, ?7)",
        params![
            object_id,
            entry.workspace_id,
            entry.kind,
            entry.title,
            canonical_uri,
            entry.created_at,
            entry.updated_at
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

fn deactivate_current_versions(conn: &Connection, object_id: &str) -> Result<(), String> {
    conn.execute(
        "update memory_object_versions set is_current = 0 where object_id = ?1",
        params![object_id],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

fn insert_memory_object_version(
    conn: &Connection,
    object_id: &str,
    version_id: &str,
    entry: &MemoryEntry,
    restored_from_version_id: &str,
) -> Result<(), String> {
    conn.execute(
        "insert or replace into memory_object_versions (
            version_id, object_id, summary, content, source_run_id, priority, verified,
            is_current, restored_from_version_id, created_at
        ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1, ?8, ?9)",
        params![
            version_id,
            object_id,
            entry.summary,
            entry.content,
            entry.source_run_id,
            entry.priority,
            bool_flag(entry.verified),
            restored_from_version_id,
            entry.created_at
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

fn insert_memory_object_alias(
    conn: &Connection,
    object_id: &str,
    canonical_uri: &str,
    created_at: &str,
) -> Result<(), String> {
    conn.execute(
        "insert or ignore into memory_object_aliases (alias_uri, object_id, created_at) values (?1, ?2, ?3)",
        params![canonical_uri, object_id, created_at],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

fn set_memory_object_current(
    conn: &Connection,
    object_id: &str,
    version_id: &str,
    canonical_uri: &str,
    entry: &MemoryEntry,
) -> Result<(), String> {
    conn.execute(
        "update memory_objects
         set title = ?1, canonical_uri = ?2, current_version_id = ?3, updated_at = ?4
         where object_id = ?5",
        params![
            entry.title,
            canonical_uri,
            version_id,
            entry.updated_at,
            object_id
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

pub(crate) fn load_memory_object_versions(
    conn: &Connection,
    object_id: &str,
) -> Result<Vec<MemoryObjectVersion>, String> {
    let uri = load_memory_object_uri(conn, object_id)?;
    let mut statement = conn
        .prepare(
            "select version_id, summary, content, is_current, restored_from_version_id, created_at
             from memory_object_versions where object_id = ?1
             order by length(created_at) desc, created_at desc",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![object_id], |row| {
            map_memory_object_version_row(row, object_id, &uri)
        })
        .map_err(|error| error.to_string())?;
    collect_rows(rows)
}

fn load_memory_object_uri(conn: &Connection, object_id: &str) -> Result<String, String> {
    conn.query_row(
        "select canonical_uri from memory_objects where object_id = ?1",
        params![object_id],
        |row| row.get(0),
    )
    .map_err(|error| error.to_string())
}

fn map_memory_object_version_row(
    row: &rusqlite::Row<'_>,
    object_id: &str,
    canonical_uri: &str,
) -> rusqlite::Result<MemoryObjectVersion> {
    Ok(MemoryObjectVersion {
        object_id: object_id.to_string(),
        version_id: row.get(0)?,
        canonical_uri: canonical_uri.to_string(),
        summary: row.get(1)?,
        content: row.get(2)?,
        is_current: row.get::<_, i32>(3)? != 0,
        restored_from_version_id: row.get(4)?,
        created_at: row.get(5)?,
    })
}

pub(crate) fn load_memory_object_aliases(conn: &Connection, object_id: &str) -> Result<Vec<String>, String> {
    let mut statement = conn
        .prepare(
            "select alias_uri from memory_object_aliases where object_id = ?1 order by alias_uri asc",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![object_id], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    collect_rows(rows)
}

fn build_memory_object_version(
    object_id: &str,
    version_id: &str,
    canonical_uri: &str,
    entry: &MemoryEntry,
    is_current: bool,
    restored_from_version_id: &str,
) -> MemoryObjectVersion {
    MemoryObjectVersion {
        object_id: object_id.to_string(),
        version_id: version_id.to_string(),
        canonical_uri: canonical_uri.to_string(),
        summary: entry.summary.clone(),
        content: entry.content.clone(),
        is_current,
        restored_from_version_id: restored_from_version_id.to_string(),
        created_at: entry.created_at.clone(),
    }
}

pub(crate) fn rollback_memory_object_conn(
    conn: &Connection,
    request: &RunRequest,
    object_id: &str,
    target_version_id: &str,
) -> Result<MemoryObjectRollbackResult, String> {
    let meta = load_memory_object_meta(conn, object_id)?;
    let target = load_memory_object_version_row(conn, object_id, target_version_id)?;
    let entry = build_rollback_entry(request, &meta, &target);
    insert_memory_entry(conn, &entry)?;
    let restored = upsert_memory_object_version_with_restore(conn, &entry, target_version_id)?;
    Ok(MemoryObjectRollbackResult {
        object_id: object_id.to_string(),
        target_version_id: target_version_id.to_string(),
        restored_version_id: restored.version_id,
        canonical_uri: restored.canonical_uri,
    })
}

fn load_memory_object_meta(conn: &Connection, object_id: &str) -> Result<MemoryObjectMeta, String> {
    conn.query_row(
        "select workspace_id, memory_type, title, canonical_uri from memory_objects where object_id = ?1",
        params![object_id],
        |row| {
            Ok(MemoryObjectMeta {
                workspace_id: row.get(0)?,
                kind: row.get(1)?,
                title: row.get(2)?,
                canonical_uri: row.get(3)?,
            })
        },
    )
    .map_err(|error| error.to_string())
}

fn load_memory_object_version_row(
    conn: &Connection,
    object_id: &str,
    version_id: &str,
) -> Result<MemoryObjectVersion, String> {
    let uri = load_memory_object_uri(conn, object_id)?;
    conn.query_row(
        "select version_id, summary, content, is_current, restored_from_version_id, created_at
         from memory_object_versions where object_id = ?1 and version_id = ?2",
        params![object_id, version_id],
        |row| map_memory_object_version_row(row, object_id, &uri),
    )
    .map_err(|error| error.to_string())
}

fn build_rollback_entry(
    request: &RunRequest,
    meta: &MemoryObjectMeta,
    target: &MemoryObjectVersion,
) -> MemoryEntry {
    let now = timestamp_now();
    let summary = target.summary.clone();
    let rollback_id = format!("memory-rollback-{now}");
    MemoryEntry {
        id: rollback_id,
        kind: meta.kind.clone(),
        title: meta.title.clone(),
        summary,
        content: target.content.clone(),
        scope: "workspace".to_string(),
        workspace_id: meta.workspace_id.clone(),
        session_id: request.session_id.clone(),
        source_run_id: format!("rollback:{}", request.run_id),
        source: format!("memory_object.rollback:{}", meta.canonical_uri),
        source_type: "governed_rollback".to_string(),
        source_title: meta.title.clone(),
        source_event_type: "rollback_applied".to_string(),
        source_artifact_path: String::new(),
        governance_version: "memory-object-rollback-v1".to_string(),
        governance_reason: format!(
            "回滚到 {target_version_id}",
            target_version_id = target.version_id
        ),
        governance_source: "memory_object_store.rollback".to_string(),
        governance_at: now.clone(),
        archive_reason: String::new(),
        verified: true,
        priority: 10,
        archived: false,
        archived_at: String::new(),
        created_at: now.clone(),
        updated_at: now.clone(),
        timestamp: now,
    }
}

struct MemoryObjectMeta {
    workspace_id: String,
    kind: String,
    title: String,
    canonical_uri: String,
}

fn object_id_for_entry(entry: &MemoryEntry) -> String {
    let mut hasher = DefaultHasher::new();
    entry.workspace_id.hash(&mut hasher);
    entry.kind.hash(&mut hasher);
    entry.title.hash(&mut hasher);
    format!("object-{:x}", hasher.finish())
}

fn version_id_for_entry(entry: &MemoryEntry) -> String {
    format!("version-{}", entry.id)
}

fn canonical_uri_for_entry(entry: &MemoryEntry) -> String {
    format!(
        "memory://{}/{}/{}",
        slug_part(&entry.workspace_id),
        slug_part(&entry.kind),
        slug_part(&entry.title)
    )
}
