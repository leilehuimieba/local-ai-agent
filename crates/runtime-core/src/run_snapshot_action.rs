use crate::checkpoint::RunCheckpoint;
use crate::contracts::ToolCallSnapshot;
use crate::planner::PlannedAction;

pub(crate) fn resumed_action_from_checkpoint(checkpoint: &RunCheckpoint) -> Option<PlannedAction> {
    let snapshot = latest_tool_call_snapshot(checkpoint)?;
    decode_snapshot_action(snapshot)
}

fn latest_tool_call_snapshot(checkpoint: &RunCheckpoint) -> Option<&ToolCallSnapshot> {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(|event| event.tool_call_snapshot.as_ref())
}

fn decode_snapshot_action(snapshot: &ToolCallSnapshot) -> Option<PlannedAction> {
    crate::action_decode::tool_call_to_action(&snapshot.tool_name, &snapshot.arguments_json)
}
