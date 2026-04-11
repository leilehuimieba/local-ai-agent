use crate::checkpoint::RunCheckpoint;

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
