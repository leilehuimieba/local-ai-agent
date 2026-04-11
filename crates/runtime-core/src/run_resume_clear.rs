use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::session::SessionMemory;

pub(crate) fn clear_resume_confirmation_state(
    session: &mut SessionMemory,
    checkpoint: &RunCheckpoint,
    request: &RunRequest,
) {
    if is_confirmation_resume(checkpoint) {
        clear_confirmation_state(session);
        return;
    }
    if is_retry_failure_resume(request) {
        clear_retry_failure_state(session, checkpoint);
    }
}

fn clear_confirmation_state(session: &mut SessionMemory) {
    session.short_term.pending_confirmation.clear();
    session.short_term.open_issue.clear();
}

fn clear_retry_failure_state(session: &mut SessionMemory, checkpoint: &RunCheckpoint) {
    session.short_term.pending_confirmation.clear();
    session.short_term.open_issue = retry_failure_summary(checkpoint);
}

fn is_confirmation_resume(checkpoint: &RunCheckpoint) -> bool {
    checkpoint.resume_reason == "confirmation_required"
}

fn is_retry_failure_resume(request: &RunRequest) -> bool {
    request.resume_strategy == "retry_failure"
}

fn retry_failure_summary(checkpoint: &RunCheckpoint) -> String {
    checkpoint.response.result.summary.clone()
}
