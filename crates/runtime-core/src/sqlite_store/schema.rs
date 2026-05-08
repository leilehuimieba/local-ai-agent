use rusqlite::Connection;

pub(super) fn apply_schema(conn: &Connection) -> Result<(), String> {
    for statement in SCHEMA_STATEMENTS {
        conn.execute(statement, []).map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub(super) fn run_memory_migrations(conn: &Connection) -> Result<(), String> {
    for statement in MEMORY_MIGRATIONS {
        apply_memory_migration(conn, statement)?;
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

const SCHEMA_STATEMENTS: [&str; 22] = [
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
        memory_write_layer text not null default '',
        memory_write_decision text not null default '',
        memory_write_reason text not null default '',
        memory_duplicate_strategy text not null default '',
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
    "create table if not exists runtime_observations (
        id integer primary key autoincrement,
        workspace_id text not null,
        session_id text not null,
        run_id text not null,
        trace_id text not null,
        event_type text not null,
        observation_kind text not null,
        stage text not null,
        summary text not null,
        tool_name text not null default '',
        artifact_ref text not null default '',
        created_at text not null
    )",
    "create index if not exists idx_runtime_observations_workspace_created on runtime_observations (workspace_id, created_at)",
    "create index if not exists idx_runtime_observations_workspace_run on runtime_observations (workspace_id, run_id)",
    "create table if not exists observation_pending_queue (
        id integer primary key autoincrement,
        workspace_id text not null,
        event_type text not null,
        observation_kind text not null,
        payload_json text not null,
        status text not null,
        retry_count integer not null default 0,
        last_error text not null default '',
        updated_at text not null
    )",
    "create index if not exists idx_observation_pending_queue_workspace_status on observation_pending_queue (workspace_id, status)",
    "create index if not exists idx_observation_pending_queue_workspace_updated on observation_pending_queue (workspace_id, updated_at)",
    "create table if not exists memory_objects (
        object_id text primary key,
        workspace_id text not null,
        memory_type text not null,
        title text not null,
        canonical_uri text not null,
        current_version_id text not null default '',
        created_at text not null,
        updated_at text not null
    )",
    "create index if not exists idx_memory_objects_workspace_type on memory_objects (workspace_id, memory_type)",
    "create table if not exists memory_object_versions (
        version_id text primary key,
        object_id text not null,
        summary text not null,
        content text not null,
        source_run_id text not null,
        priority integer not null default 0,
        verified integer not null default 0,
        is_current integer not null default 0,
        restored_from_version_id text not null default '',
        created_at text not null
    )",
    "create index if not exists idx_memory_object_versions_object_created on memory_object_versions (object_id, created_at)",
    "create table if not exists memory_object_aliases (
        alias_uri text primary key,
        object_id text not null,
        created_at text not null
    )",
    "create index if not exists idx_memory_object_aliases_object on memory_object_aliases (object_id)",
];

const MEMORY_MIGRATIONS: [&str; 16] = [
    "alter table long_term_memory add column source_title text not null default ''",
    "alter table long_term_memory add column source_event_type text not null default ''",
    "alter table long_term_memory add column source_artifact_path text not null default ''",
    "alter table long_term_memory add column governance_version text not null default ''",
    "alter table long_term_memory add column governance_reason text not null default ''",
    "alter table long_term_memory add column governance_source text not null default ''",
    "alter table long_term_memory add column governance_at text not null default ''",
    "alter table long_term_memory add column archive_reason text not null default ''",
    "alter table long_term_memory add column memory_write_layer text not null default ''",
    "alter table long_term_memory add column memory_write_decision text not null default ''",
    "alter table long_term_memory add column memory_write_reason text not null default ''",
    "alter table long_term_memory add column memory_duplicate_strategy text not null default ''",
    "alter table long_term_memory add column archived_at text not null default ''",
    "alter table runtime_checkpoints add column resume_reason text not null default ''",
    "alter table runtime_checkpoints add column resume_stage text not null default ''",
    "alter table memory_object_versions add column restored_from_version_id text not null default ''",
];
