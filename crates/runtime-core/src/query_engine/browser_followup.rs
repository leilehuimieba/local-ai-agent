use crate::capabilities::ToolExecutionTrace;
use crate::planner::PlannedAction;

use super::RuntimeRunState;

pub(super) fn browser_interaction_verify_failed(state: &RuntimeRunState) -> bool {
    let Some(report) = state.verification_report.as_ref() else {
        return false;
    };
    report.outcome.task_type == "browser_interaction" && !report.outcome.passed
}

pub(super) fn browser_interaction_needs_readback(state: &RuntimeRunState) -> bool {
    browser_interaction_verify_failed(state) && !is_browser_read_page(&state.action)
}

pub(super) fn browser_read_page_action(action: &PlannedAction, trace: &ToolExecutionTrace) -> Option<PlannedAction> {
    let page_id = browser_page_id_from_action(action).or_else(|| browser_page_id_from_trace(trace))?;
    Some(PlannedAction::MCPCall {
        server_id: "browser".to_string(),
        tool_name: "read_page".to_string(),
        arguments_json: serde_json::json!({ "page_id": page_id }).to_string(),
    })
}

fn browser_page_id_from_action(action: &PlannedAction) -> Option<String> {
    let PlannedAction::MCPCall { arguments_json, .. } = action else {
        return None;
    };
    json_field(arguments_json, "page_id")
}

fn browser_page_id_from_trace(trace: &ToolExecutionTrace) -> Option<String> {
    json_field(&trace.result.detail_preview, "page_id").or_else(|| json_field(&trace.result.final_answer, "page_id"))
}

fn json_field(raw: &str, key: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(raw)
        .ok()?
        .get(key)?
        .as_str()
        .map(str::to_string)
}

pub(super) fn is_browser_read_page(action: &PlannedAction) -> bool {
    matches!(
        action,
        PlannedAction::MCPCall { server_id, tool_name, .. }
        if server_id == "browser" && tool_name == "read_page"
    )
}
