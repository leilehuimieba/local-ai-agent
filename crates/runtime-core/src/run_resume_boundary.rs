use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunEvent;

pub(crate) fn resume_execution_boundary(checkpoint: &RunCheckpoint) -> String {
    execution_boundary_from_events(checkpoint).unwrap_or_default()
}

fn execution_boundary_from_events(checkpoint: &RunCheckpoint) -> Option<String> {
    primary_execution_boundary(checkpoint).or_else(|| confirmation_boundary_from_events(checkpoint))
}

fn primary_execution_boundary(checkpoint: &RunCheckpoint) -> Option<String> {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(execution_boundary_from_event)
}

fn execution_boundary_from_event(event: &RunEvent) -> Option<String> {
    if !is_execution_boundary_event(event) {
        return None;
    }
    let mut parts = vec![
        format!("阶段={}", event.stage),
        format!("事件={}", event.event_type),
    ];
    if let Some(step) = next_step_metadata(event) {
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
    let step = next_step_metadata(event).unwrap_or_else(default_confirmation_step);
    format_boundary_parts(
        event.stage.as_str(),
        event.event_type.as_str(),
        step.as_str(),
    )
}

fn format_boundary_parts(stage: &str, event_type: &str, step: &str) -> String {
    format!("阶段={}，事件={}，下一步={}", stage, event_type, step)
}

fn next_step_metadata(event: &RunEvent) -> Option<String> {
    event
        .metadata
        .get("next_step")
        .cloned()
        .filter(|s| !s.is_empty())
}

fn default_confirmation_step() -> String {
    "等待用户确认后再继续".to_string()
}
