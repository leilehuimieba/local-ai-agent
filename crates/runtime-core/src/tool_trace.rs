use crate::action_meta::{action_tag, default_error_code};
use crate::artifacts::externalize_text_artifact;
use crate::capabilities::{ToolCallResult, ToolExecutionTrace, resolve_tool};
use crate::contracts::RunRequest;
use crate::execution::execute_action;
use crate::planner::PlannedAction;
use crate::session::SessionMemory;

pub(crate) fn execute_tool(
    request: &RunRequest,
    action: &PlannedAction,
    session_context: &SessionMemory,
) -> ToolExecutionTrace {
    let execution = execute_action(request, action, session_context);
    let artifact_path = materialize_artifact(request, action, &execution);
    ToolExecutionTrace {
        tool: resolve_tool(action),
        action_summary: execution.action_summary.clone(),
        result: ToolCallResult {
            summary: execution.result_summary.clone(),
            final_answer: execution.final_answer,
            artifact_path,
            error_code: if execution.success {
                None
            } else {
                Some(default_error_code(action))
            },
            retryable: !execution.success,
            success: execution.success,
            memory_write_summary: execution.memory_write_summary,
            reasoning_summary: execution.reasoning_summary,
            cache_status: execution.cache_status,
            cache_reason: execution.cache_reason,
        },
    }
}

pub(crate) fn materialize_artifact(
    request: &RunRequest,
    action: &PlannedAction,
    execution: &crate::execution::ActionExecution,
) -> Option<String> {
    let content = artifact_content(execution);
    externalize_text_artifact(request, action_tag(action), &content).map(|item| item.path)
}

// 如需在其它模块复用 ActionExecution -> ToolCallResult，可再补一个转换函数。

fn artifact_content(execution: &crate::execution::ActionExecution) -> String {
    format!(
        "action_summary={}\nresult_summary={}\nfinal_answer={}\nreasoning_summary={}\ncache_status={}\ncache_reason={}",
        execution.action_summary,
        execution.result_summary,
        execution.final_answer,
        execution.reasoning_summary,
        execution.cache_status,
        execution.cache_reason,
    )
}
