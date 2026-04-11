use crate::capabilities::ToolDefinition;
use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::planner::PlannedAction;
use crate::repo_context::RepoContextLoadResult;
use crate::run_state_builder::PreparedRunState;
use crate::session::SessionMemory;
use crate::tool_registry::ToolCall;

pub(crate) fn resumed_prepared_state(
    request: &RunRequest,
    session_context: &SessionMemory,
    repo_context: &RepoContextLoadResult,
    visible_tools: &[ToolDefinition],
    checkpoint: Option<&RunCheckpoint>,
) -> Option<PreparedRunState> {
    let prepared = crate::run_state_builder::prepare_run_state(
        request,
        session_context,
        repo_context,
        visible_tools,
    );
    let action = resumed_action(checkpoint?)?;
    Some(prepared_with_action(
        request,
        session_context,
        repo_context,
        prepared,
        action,
    ))
}

fn prepared_with_action(
    request: &RunRequest,
    session_context: &SessionMemory,
    repo_context: &RepoContextLoadResult,
    prepared: PreparedRunState,
    action: PlannedAction,
) -> PreparedRunState {
    PreparedRunState {
        task_title: crate::derive_task_title(&action, &request.user_input),
        analysis_detail: crate::planner::analysis_summary(
            &action,
            session_context,
            &repo_context.snapshot,
        ),
        risk_outcome: crate::risk::assess_risk(request, &action),
        tool_call: ToolCall {
            spec: crate::capabilities::resolve_tool(&action),
            action: action.clone(),
        },
        action,
        ..prepared
    }
}

fn resumed_action(checkpoint: &RunCheckpoint) -> Option<PlannedAction> {
    let snapshot = checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(|event| event.tool_call_snapshot.as_ref())?;
    crate::action_decode::tool_call_to_action(&snapshot.tool_name, &snapshot.arguments_json)
}
