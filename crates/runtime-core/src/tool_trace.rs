use crate::action_meta::{action_tag, default_error_code};
use crate::artifacts::{externalize_text_artifact, externalize_text_artifact_always};
use crate::capabilities::{resolve_tool, ToolCallResult, ToolExecutionTrace};
use crate::contracts::RunRequest;
use crate::execution::execute_action;
use crate::planner::PlannedAction;
use crate::session::SessionMemory;
use std::time::Instant;

pub(crate) fn execute_tool(
    request: &RunRequest,
    action: &PlannedAction,
    session_context: &SessionMemory,
) -> ToolExecutionTrace {
    let started_at = Instant::now();
    let execution = execute_action(request, action, session_context);
    let artifact_path = materialize_artifact(request, action, &execution);
    let raw_output_ref = command_raw_output_ref(action, artifact_path.as_deref());
    ToolExecutionTrace {
        tool: resolve_tool(action),
        action_summary: execution.action_summary.clone(),
        result: build_tool_result(started_at, action, execution, artifact_path, raw_output_ref),
    }
}

fn build_tool_result(
    started_at: Instant,
    action: &PlannedAction,
    execution: crate::execution::ActionExecution,
    artifact_path: Option<String>,
    raw_output_ref: Option<String>,
) -> ToolCallResult {
    ToolCallResult {
        summary: execution.result_summary.clone(),
        final_answer: execution.final_answer.clone(),
        artifact_path,
        detail_preview: read_detail_preview(&execution),
        raw_output_ref,
        result_chars: execution.result_chars,
        single_result_budget_chars: execution.single_result_budget_chars,
        single_result_budget_hit: execution.single_result_budget_hit,
        error_code: (!execution.success).then(|| default_error_code(action)),
        elapsed_ms: tool_elapsed_ms(started_at),
        retryable: !execution.success,
        success: execution.success,
        memory_write_summary: execution.memory_write_summary,
        reasoning_summary: execution.reasoning_summary,
        cache_status: execution.cache_status,
        cache_reason: execution.cache_reason,
    }
}

fn tool_elapsed_ms(started_at: Instant) -> u64 {
    let elapsed = started_at.elapsed().as_millis();
    elapsed.min(u128::from(u64::MAX)) as u64
}

pub(crate) fn materialize_artifact(
    request: &RunRequest,
    action: &PlannedAction,
    execution: &crate::execution::ActionExecution,
) -> Option<String> {
    if matches!(action, PlannedAction::RunCommand { .. }) {
        let raw_kind = format!("{}-raw-output", action_tag(action));
        return externalize_text_artifact_always(request, &raw_kind, &execution.raw_output)
            .map(|item| item.path);
    }
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

fn read_detail_preview(execution: &crate::execution::ActionExecution) -> String {
    if execution.detail_preview.is_empty() {
        execution.result_summary.clone()
    } else {
        execution.detail_preview.clone()
    }
}

fn command_raw_output_ref(action: &PlannedAction, artifact_path: Option<&str>) -> Option<String> {
    if matches!(action, PlannedAction::RunCommand { .. }) {
        return artifact_path.map(str::to_string);
    }
    None
}
