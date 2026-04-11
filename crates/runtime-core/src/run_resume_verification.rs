use crate::checkpoint::RunCheckpoint;

pub(crate) fn resume_verification_summary(checkpoint: &RunCheckpoint) -> String {
    verification_summary_candidate(checkpoint).unwrap_or_default()
}

fn verification_summary_candidate(checkpoint: &RunCheckpoint) -> Option<String> {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(verification_summary_from_event)
}

fn verification_summary_from_event(event: &crate::contracts::RunEvent) -> Option<String> {
    event
        .verification_snapshot
        .as_ref()
        .map(|snapshot| snapshot.summary.clone())
        .filter(|summary| !summary.is_empty())
}
