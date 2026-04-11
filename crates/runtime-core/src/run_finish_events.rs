use crate::contracts::{ErrorInfo, RunEvent, RunRequest};
use crate::events::make_event;
use crate::repo_context::repo_context_metadata;
use crate::run_failure_metadata::{
    append_error_metadata, append_tool_failure_metadata, failure_next_step,
};
use crate::run_memory_metadata::append_memory_governance_metadata;

pub(crate) fn read_result_mode(
    final_answer: &str,
    completion_status: &str,
    verification_code: &str,
) -> &'static str {
    if completion_status == "failed" {
        return "system";
    }
    if verification_code == "verified_with_recovery" {
        return "recovery";
    }
    if final_answer.trim().is_empty() {
        return "system";
    }
    "answer"
}

pub(crate) fn make_run_failed_event(
    request: &RunRequest,
    sequence: u32,
    summary: &str,
    detail: &str,
    error: &ErrorInfo,
    tool_trace: Option<&crate::capabilities::ToolExecutionTrace>,
    task_title: &str,
    repo_context: &crate::repo_context::RepoContextLoadResult,
) -> RunEvent {
    let mut metadata = repo_context_metadata(repo_context);
    metadata.insert("task_title".to_string(), task_title.to_string());
    metadata.insert("result_mode".to_string(), "system".to_string());
    append_error_metadata(&mut metadata, error);
    append_tool_failure_metadata(&mut metadata, tool_trace);
    metadata.insert(
        "next_step".to_string(),
        failure_next_step(tool_trace, error),
    );
    make_event(
        request,
        sequence,
        "run_failed",
        "Failed",
        summary,
        detail,
        metadata,
    )
}

pub(crate) fn make_memory_event(
    request: &RunRequest,
    sequence: u32,
    task_title: &str,
    repo_context: &crate::repo_context::RepoContextLoadResult,
    outcome: &crate::memory_router::MemoryWriteOutcome,
) -> RunEvent {
    let mut metadata = repo_context_metadata(repo_context);
    metadata.insert("task_title".to_string(), task_title.to_string());
    metadata.insert("layer".to_string(), outcome.layer.to_string());
    metadata.insert("record_type".to_string(), outcome.record_type.clone());
    if !outcome.source_type.is_empty() {
        metadata.insert("source_type".to_string(), outcome.source_type.clone());
    }
    append_memory_governance_metadata(&mut metadata, outcome);
    metadata.insert("title".to_string(), outcome.title.clone());
    metadata.insert("reason".to_string(), outcome.reason.clone());
    make_event(
        request,
        sequence,
        outcome.event_type,
        "Finish",
        &outcome.title,
        &outcome.summary,
        metadata,
    )
}
