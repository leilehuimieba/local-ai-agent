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
    crate::run_resume_state::apply_resume_short_term_state(session, checkpoint);
    crate::run_resume_clear::clear_resume_confirmation_state(session, checkpoint, request);
}
