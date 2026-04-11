use crate::checkpoint::RunCheckpoint;

pub(crate) fn resume_handoff_artifact_path(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(|event| event.metadata.get("handoff_artifact_path").cloned())
        .unwrap_or_default()
}
