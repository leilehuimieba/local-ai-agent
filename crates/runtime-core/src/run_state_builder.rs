use crate::capabilities::ToolDefinition;
use crate::context_builder::{build_runtime_context, RuntimeContextEnvelope};
use crate::context_policy::{action_context_policy, planning_context_policy};
use crate::contracts::RunRequest;
use crate::planner::{analysis_summary, PlannedAction};
use crate::repo_context::RepoContextLoadResult;
use crate::risk::{assess_risk, RiskOutcome};
use crate::session::{record_planning_memory, SessionMemory};
use crate::tool_registry::{runtime_tool_registry, ToolCall};

#[derive(Clone, Debug)]
pub(crate) struct PreparedRunState {
    pub(crate) action: PlannedAction,
    pub(crate) tool_call: ToolCall,
    pub(crate) context_envelope: RuntimeContextEnvelope,
    pub(crate) task_title: String,
    pub(crate) analysis_detail: String,
    pub(crate) risk_outcome: RiskOutcome,
}

pub(crate) fn prepare_run_state(
    request: &RunRequest,
    session_context: &SessionMemory,
    repo_context: &RepoContextLoadResult,
    visible_tools: &[ToolDefinition],
) -> PreparedRunState {
    let context_envelope = planning_context(request, session_context, repo_context, visible_tools);
    let tool_call = runtime_tool_registry().plan_tool_call(&context_envelope);
    let action = tool_call.action.clone();
    let execute_context = execution_context(
        request,
        session_context,
        repo_context,
        visible_tools,
        &action,
    );
    PreparedRunState {
        context_envelope: execute_context,
        task_title: crate::derive_task_title(&action, &request.user_input),
        analysis_detail: analysis_summary(&action, session_context, &repo_context.snapshot),
        risk_outcome: assess_risk(request, &action),
        action,
        tool_call,
    }
}

pub(crate) fn record_bootstrap_memory(
    request: &RunRequest,
    session_context: &mut SessionMemory,
    prepared: &PreparedRunState,
) {
    record_planning_memory(
        request,
        session_context,
        &prepared.task_title,
        &prepared.analysis_detail,
        &prepared.risk_outcome,
    );
}

pub(crate) fn bootstrap_context(
    request: &RunRequest,
    session_context: &SessionMemory,
    repo_context: &RepoContextLoadResult,
    visible_tools: &[ToolDefinition],
) -> RuntimeContextEnvelope {
    build_runtime_context(
        request,
        session_context,
        repo_context,
        visible_tools,
        &planning_context_policy(&request.user_input, session_context),
        "cold",
        "当前为主链路首轮规划，还未进入回答缓存探测。",
    )
}

fn planning_context(
    request: &RunRequest,
    session_context: &SessionMemory,
    repo_context: &RepoContextLoadResult,
    visible_tools: &[ToolDefinition],
) -> RuntimeContextEnvelope {
    build_runtime_context(
        request,
        session_context,
        repo_context,
        visible_tools,
        &planning_context_policy(&request.user_input, session_context),
        "cold",
        "当前为工具规划阶段，还未进入回答缓存探测。",
    )
}

fn execution_context(
    request: &RunRequest,
    session_context: &SessionMemory,
    repo_context: &RepoContextLoadResult,
    visible_tools: &[ToolDefinition],
    action: &PlannedAction,
) -> RuntimeContextEnvelope {
    build_runtime_context(
        request,
        session_context,
        repo_context,
        visible_tools,
        &action_context_policy(action, session_context),
        "cold",
        "当前为执行阶段上下文，已按动作类型收紧上下文装配。",
    )
}
