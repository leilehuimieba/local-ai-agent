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
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(execution_boundary_from_event)
        .or_else(|| confirmation_boundary_from_events(checkpoint))
        .unwrap_or_default()
}

fn execution_boundary_from_event(event: &RunEvent) -> Option<String> {
    if !is_execution_boundary_event(event) {
        return None;
    }
    let mut parts = vec![
        format!("阶段={}", event.stage),
        format!("事件={}", event.event_type),
    ];
    if let Some(step) = event
        .metadata
        .get("next_step")
        .filter(|step| !step.is_empty())
    {
        parts.push(format!("下一步={step}"));
    }
    Some(parts.join("，"))
}

fn is_execution_boundary_event(event: &RunEvent) -> bool {
    matches!(
        event.event_type.as_str(),
        "action_requested" | "action_completed" | "verification_completed" | "run_failed"
    )
}

fn confirmation_boundary_from_events(checkpoint: &RunCheckpoint) -> Option<String> {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find(|event| event.event_type == "confirmation_required")
        .map(format_confirmation_boundary)
}

fn format_confirmation_boundary(event: &RunEvent) -> String {
    let step = event
        .metadata
        .get("next_step")
        .cloned()
        .unwrap_or_else(|| "等待用户确认后再继续".to_string());
    format!(
        "阶段={}，事件={}，下一步={}",
        event.stage, event.event_type, step
    )
}

pub(crate) fn resume_recovery_hint(checkpoint: &RunCheckpoint) -> String {
    failed_recovery_hint(checkpoint).unwrap_or_else(|| verification_hint(checkpoint))
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

fn verification_hint(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find(|event| event.event_type == "verification_completed")
        .and_then(|event| event.metadata.get("verification_summary").cloned())
        .unwrap_or_default()
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
