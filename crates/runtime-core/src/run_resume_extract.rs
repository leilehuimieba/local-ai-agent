use crate::checkpoint::RunCheckpoint;

pub(crate) fn resume_action_hint(checkpoint: &RunCheckpoint) -> String {
    crate::run_resume_action_hint::resume_action_hint(checkpoint)
}

pub(crate) fn resume_execution_boundary(checkpoint: &RunCheckpoint) -> String {
    crate::run_resume_boundary::resume_execution_boundary(checkpoint)
}

pub(crate) fn resume_recovery_hint(checkpoint: &RunCheckpoint) -> String {
    crate::run_resume_hint::resume_recovery_hint(checkpoint)
}

pub(crate) fn resume_verification_summary(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(|event| event.verification_snapshot.as_ref())
        .map(|snapshot| snapshot.summary.clone())
        .filter(|summary| !summary.is_empty())
        .unwrap_or_default()
}

pub(crate) fn resume_artifact_path(checkpoint: &RunCheckpoint) -> String {
    crate::run_resume_artifact::resume_artifact_path(checkpoint)
}
