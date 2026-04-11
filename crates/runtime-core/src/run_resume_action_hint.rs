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
    compose_action_hint(tool, task, name)
}

fn compose_action_hint(tool: String, task: String, name: String) -> Option<String> {
    if !tool.is_empty() && !task.is_empty() {
        return Some(format!("{tool} ({task})"));
    }
    first_non_empty([tool, task, name])
}

fn first_non_empty(candidates: [String; 3]) -> Option<String> {
    candidates.into_iter().find(|value| !value.is_empty())
}
