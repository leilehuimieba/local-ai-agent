use crate::checkpoint::RunCheckpoint;

pub(crate) fn resume_recovery_hint(checkpoint: &RunCheckpoint) -> String {
    recovery_hint_candidates(checkpoint).unwrap_or_default()
}

fn recovery_hint_candidates(checkpoint: &RunCheckpoint) -> Option<String> {
    failed_recovery_hint(checkpoint).or_else(|| verification_recovery_hint(checkpoint))
}

fn failed_recovery_hint(checkpoint: &RunCheckpoint) -> Option<String> {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find(|event| event.event_type == "run_failed")
        .and_then(|event| event.metadata.get("failure_recovery_hint").cloned())
}

fn verification_recovery_hint(checkpoint: &RunCheckpoint) -> Option<String> {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find(|event| event.event_type == "verification_completed")
        .and_then(|event| event.metadata.get("verification_summary").cloned())
}
