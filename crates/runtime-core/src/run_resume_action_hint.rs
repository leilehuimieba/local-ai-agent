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
