use crate::checkpoint::RunCheckpoint;

pub(crate) fn resume_handoff_artifact_path(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(handoff_path_from_event)
        .unwrap_or_default()
}

fn handoff_path_from_event(event: &crate::contracts::RunEvent) -> Option<String> {
    event.metadata.get("handoff_artifact_path").cloned()
}
