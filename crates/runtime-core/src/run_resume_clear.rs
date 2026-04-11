use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::session::SessionMemory;

pub(crate) fn clear_resume_confirmation_state(
    session: &mut SessionMemory,
    checkpoint: &RunCheckpoint,
    request: &RunRequest,
) {
    if checkpoint.resume_reason == "confirmation_required" {
        clear_confirmation_state(session);
        return;
    }
    if request.resume_strategy == "retry_failure" {
        clear_retry_failure_state(session, checkpoint);
    }
}

fn clear_confirmation_state(session: &mut SessionMemory) {
    session.short_term.pending_confirmation.clear();
    session.short_term.open_issue.clear();
}

fn clear_retry_failure_state(session: &mut SessionMemory, checkpoint: &RunCheckpoint) {
    session.short_term.pending_confirmation.clear();
    session.short_term.open_issue = checkpoint.response.result.summary.clone();
}
