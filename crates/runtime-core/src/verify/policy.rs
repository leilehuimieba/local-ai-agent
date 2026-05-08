use crate::planner::PlannedAction;
use crate::tool_registry::ToolCall;

pub(super) fn verification_task_type(tool_call: &ToolCall) -> String {
    match &tool_call.action {
        PlannedAction::ProjectAnswer | PlannedAction::ContextAnswer => "knowledge_answer".to_string(),
        PlannedAction::WriteFile { .. } | PlannedAction::ApplyPatch { .. } | PlannedAction::DeletePath { .. } => {
            "file_change".to_string()
        }
        PlannedAction::MCPCall { server_id, .. } if server_id == "browser" => "browser_interaction".to_string(),
        PlannedAction::RunCommand { .. } | PlannedAction::MCPCall { .. } => "command_execution".to_string(),
        PlannedAction::WriteMemory { .. } | PlannedAction::WriteSiyuanKnowledge => "memory_write".to_string(),
        _ => "generic".to_string(),
    }
}

pub(super) fn verification_policy(tool_call: &ToolCall) -> String {
    if is_browser_tool_name(&tool_call.spec.tool_name) {
        return "confirm_browser_state_change".to_string();
    }
    match tool_call.spec.tool_name.as_str() {
        "workspace_write" => "confirm_write_effect".to_string(),
        "workspace_delete" => "confirm_delete_effect".to_string(),
        "run_command" => "inspect_command_result".to_string(),
        "memory_write" => "confirm_memory_persisted".to_string(),
        "project_answer" | "context_answer" => "check_knowledge_answer".to_string(),
        "knowledge_search" | "search_siyuan_notes" | "read_siyuan_note" => "check_result_relevance".to_string(),
        _ => "check_result_summary".to_string(),
    }
}

fn is_browser_tool_name(tool_name: &str) -> bool {
    tool_name.starts_with("mcp__browser__")
}
