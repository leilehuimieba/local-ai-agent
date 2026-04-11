use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunEvent;

pub(crate) fn resume_artifact_path(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(artifact_path_from_event)
        .unwrap_or_default()
}

fn artifact_path_from_event(event: &RunEvent) -> Option<String> {
    event
        .metadata
        .get("artifact_path")
        .cloned()
        .filter(|path| !path.is_empty())
        .or_else(|| artifact_from_verification_snapshot(event))
}

fn artifact_from_verification_snapshot(event: &RunEvent) -> Option<String> {
    event
        .verification_snapshot
        .as_ref()
        .and_then(artifact_from_verification_evidence)
}

fn artifact_from_verification_evidence(
    snapshot: &crate::contracts::VerificationSnapshot,
) -> Option<String> {
    snapshot
        .evidence
        .iter()
        .find_map(|line| line.strip_prefix("artifact=").map(str::to_string))
}
