use crate::checkpoint::RunCheckpoint;

pub(crate) fn resume_recovery_hint(checkpoint: &RunCheckpoint) -> String {
    recovery_hint_candidates(checkpoint).unwrap_or_default()
}

fn recovery_hint_candidates(checkpoint: &RunCheckpoint) -> Option<String> {
    failed_recovery_hint(checkpoint).or_else(|| verification_recovery_hint(checkpoint))
}

fn failed_recovery_hint(checkpoint: &RunCheckpoint) -> Option<String> {
    recovery_hint_from_event_metadata(checkpoint, "run_failed", "failure_recovery_hint")
}

fn verification_recovery_hint(checkpoint: &RunCheckpoint) -> Option<String> {
    recovery_hint_from_event_metadata(checkpoint, "verification_completed", "verification_summary")
}

fn recovery_hint_from_event_metadata(
    checkpoint: &RunCheckpoint,
    event_type: &str,
    metadata_key: &str,
) -> Option<String> {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find(|event| event.event_type == event_type)
        .and_then(|event| event.metadata.get(metadata_key).cloned())
}
