use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::session::SessionMemory;

pub(crate) fn apply_resume_checkpoint(
    session: &mut SessionMemory,
    checkpoint: Option<&RunCheckpoint>,
    request: &RunRequest,
) {
    let Some(checkpoint) = checkpoint else {
        return;
    };
    session.short_term.current_plan = resume_plan(checkpoint);
    session.short_term.current_phase = resume_phase(checkpoint);
    session.short_term.last_run_status = checkpoint.status.clone();
    session.short_term.recent_observation = checkpoint.response.result.final_answer.clone();
    session.short_term.recent_tool_result = checkpoint.response.result.summary.clone();
    clear_resume_confirmation_state(session, checkpoint, request);
}

fn clear_resume_confirmation_state(
    session: &mut SessionMemory,
    checkpoint: &RunCheckpoint,
    request: &RunRequest,
) {
    if checkpoint.resume_reason == "confirmation_required" {
        session.short_term.pending_confirmation.clear();
        session.short_term.open_issue.clear();
        return;
    }
    if request.resume_strategy == "retry_failure" {
        session.short_term.pending_confirmation.clear();
        session.short_term.open_issue = checkpoint.response.result.summary.clone();
    }
}

fn resume_plan(checkpoint: &RunCheckpoint) -> String {
    format!(
        "从 checkpoint 恢复：{} -> {}",
        checkpoint.resume_reason, checkpoint.resume_stage
    )
}

fn resume_phase(checkpoint: &RunCheckpoint) -> String {
    if checkpoint.resume_reason == "confirmation_required" {
        "confirmation_resume".to_string()
    } else {
        "recovery".to_string()
    }
}
