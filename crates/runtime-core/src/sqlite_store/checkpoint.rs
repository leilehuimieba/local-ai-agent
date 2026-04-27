use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::observation::ObservationRecord;
use crate::sqlite_store::bool_flag;
use rusqlite::{Connection, params};
use serde_json::{from_str, to_string};

pub(crate) fn insert_runtime_checkpoint(conn: &Connection, checkpoint: &RunCheckpoint) -> Result<(), String> {
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

pub(crate) fn insert_observation_row(
    conn: &Connection,
    request: &RunRequest,
    record: &ObservationRecord,
) -> Result<(), String> {
    conn.execute(
        "insert into runtime_observations (
            workspace_id, session_id, run_id, trace_id, event_type, observation_kind, stage,
            summary, tool_name, artifact_ref, created_at
        ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            request.workspace_ref.workspace_id.clone(),
            record.session_id.clone(),
            record.run_id.clone(),
            record.trace_id.clone(),
            record.event_type.clone(),
            record.observation_kind.clone(),
            record.stage.clone(),
            record.summary.clone(),
            record.tool_name.clone(),
            record.artifact_ref.clone(),
            crate::events::timestamp_now()
        ],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn select_runtime_checkpoint(
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
pub(crate) fn decode_runtime_checkpoint(row: &rusqlite::Row<'_>) -> Result<RunCheckpoint, String> {
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
