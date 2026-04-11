use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunEvent;

pub(crate) fn resume_action_hint(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(action_hint_from_event)
        .unwrap_or_default()
}

fn action_hint_from_event(event: &RunEvent) -> Option<String> {
    let tool = event
        .metadata
        .get("tool_display_name")
        .cloned()
        .unwrap_or_default();
    let task = event
        .metadata
        .get("task_title")
        .cloned()
        .unwrap_or_default();
    let name = event.metadata.get("tool_name").cloned().unwrap_or_default();
    if !tool.is_empty() && !task.is_empty() {
        return Some(format!("{tool} ({task})"));
    }
    if !tool.is_empty() {
        return Some(tool);
    }
    if !task.is_empty() {
        return Some(task);
    }
    if !name.is_empty() {
        return Some(name);
    }
    None
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
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(|event| {
            event
                .metadata
                .get("artifact_path")
                .cloned()
                .filter(|path| !path.is_empty())
                .or_else(|| artifact_from_verification_snapshot(event))
        })
        .unwrap_or_default()
}

fn artifact_from_verification_snapshot(event: &RunEvent) -> Option<String> {
    event.verification_snapshot.as_ref().and_then(|snapshot| {
        snapshot
            .evidence
            .iter()
            .find_map(|line| line.strip_prefix("artifact=").map(str::to_string))
    })
}
