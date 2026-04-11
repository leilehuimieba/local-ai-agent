use crate::checkpoint::RunCheckpoint;
use crate::session::SessionMemory;

pub(crate) fn apply_resume_short_term_state(
    session: &mut SessionMemory,
    checkpoint: &RunCheckpoint,
) {
    session.short_term.current_plan = crate::run_resume_plan::resume_plan(checkpoint);
    session.short_term.current_phase = crate::run_resume_plan::resume_phase(checkpoint);
    session.short_term.last_run_status = checkpoint.status.clone();
    session.short_term.recent_observation =
        crate::run_resume_observation::resume_recent_observation(checkpoint);
    session.short_term.recent_tool_result =
        crate::run_resume_observation::resume_recent_tool_result(checkpoint);
    session.short_term.handoff_artifact_path =
        crate::run_resume_handoff::resume_handoff_artifact_path(checkpoint);
}
