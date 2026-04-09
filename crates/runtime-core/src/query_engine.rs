use crate::capabilities::{ToolDefinition, ToolExecutionTrace};
use crate::context_builder::{build_runtime_context, RuntimeContextEnvelope};
use crate::context_policy::{action_context_policy, planning_context_policy};
use crate::contracts::RunRequest;
use crate::planner::{analysis_summary, PlannedAction};
use crate::repo_context::{load_repo_context, RepoContextLoadResult};
use crate::risk::{assess_risk, RiskOutcome};
use crate::session::{
    load_session_context, record_execution_memory, record_planning_memory, SessionMemory,
};
use crate::tool_registry::{runtime_tool_registry, ToolCall};
use crate::tool_trace::execute_tool;
use crate::verify::VerificationReport;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) struct RuntimeEnvelope {
    pub request: RunRequest,
    pub session_context: SessionMemory,
    pub repo_context: RepoContextLoadResult,
    pub context_envelope: RuntimeContextEnvelope,
    pub visible_tools: Vec<ToolDefinition>,
}

#[derive(Clone, Debug)]
pub(crate) struct RuntimeRunState {
    pub envelope: RuntimeEnvelope,
    pub action: PlannedAction,
    pub tool_call: ToolCall,
    pub task_title: String,
    pub analysis_detail: String,
    pub risk_outcome: RiskOutcome,
    pub tool_trace: Option<ToolExecutionTrace>,
    pub verification_report: Option<VerificationReport>,
}

pub(crate) fn bootstrap_run(request: &RunRequest) -> RuntimeRunState {
    let workspace_root = PathBuf::from(&request.workspace_ref.root_path);
    let repo_context = load_repo_context(&workspace_root);
    let visible_tools = runtime_tool_registry().visible_tools(&request.mode);
    let mut session_context = load_session_context(request);
    let prepared = prepare_run_state(request, &session_context, &repo_context, &visible_tools);
    record_bootstrap_memory(request, &mut session_context, &prepared);
    let context_envelope =
        bootstrap_context(request, &session_context, &repo_context, &visible_tools);
    assemble_runtime_state(
        request,
        session_context,
        repo_context,
        visible_tools,
        context_envelope,
        prepared,
    )
}

pub(crate) fn execute_stage(state: &mut RuntimeRunState) {
    state.tool_trace = Some(execute_tool(
        &state.envelope.request,
        &state.action,
        &state.envelope.session_context,
    ));
    if let Some(trace) = state.tool_trace.as_ref() {
        refresh_context_after_execution(&mut state.envelope.context_envelope, trace);
        record_execution_memory(
            &state.envelope.request,
            &mut state.envelope.session_context,
            &trace.result.summary,
            &trace.result.final_answer,
            trace.result.success,
        );
    }
}

fn refresh_context_after_execution(
    envelope: &mut RuntimeContextEnvelope,
    trace: &ToolExecutionTrace,
) {
    envelope.dynamic_block.reasoning_summary = trace.result.reasoning_summary.clone();
    envelope.dynamic_block.cache_status = trace.result.cache_status.clone();
    envelope.dynamic_block.cache_reason = trace.result.cache_reason.clone();
}

#[derive(Clone, Debug)]
struct PreparedRunState {
    action: PlannedAction,
    tool_call: ToolCall,
    context_envelope: RuntimeContextEnvelope,
    task_title: String,
    analysis_detail: String,
    risk_outcome: RiskOutcome,
}

fn prepare_run_state(
    request: &RunRequest,
    session_context: &SessionMemory,
    repo_context: &RepoContextLoadResult,
    visible_tools: &[ToolDefinition],
) -> PreparedRunState {
    let context_envelope =
        planning_context(request, session_context, repo_context, visible_tools);
    let tool_call = runtime_tool_registry().plan_tool_call(&context_envelope);
    let action = tool_call.action.clone();
    let execute_context =
        execution_context(request, session_context, repo_context, visible_tools, &action);
    PreparedRunState {
        context_envelope: execute_context,
        task_title: crate::derive_task_title(&action, &request.user_input),
        analysis_detail: analysis_summary(&action, session_context, &repo_context.snapshot),
        risk_outcome: assess_risk(request, &action),
        action,
        tool_call,
    }
}

fn record_bootstrap_memory(
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

fn bootstrap_context(
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

fn assemble_runtime_state(
    request: &RunRequest,
    session_context: SessionMemory,
    repo_context: RepoContextLoadResult,
    visible_tools: Vec<ToolDefinition>,
    _context_envelope: RuntimeContextEnvelope,
    prepared: PreparedRunState,
) -> RuntimeRunState {
    RuntimeRunState {
        envelope: RuntimeEnvelope {
            request: request.clone(),
            session_context,
            repo_context,
            context_envelope: prepared.context_envelope,
            visible_tools,
        },
        action: prepared.action,
        tool_call: prepared.tool_call,
        task_title: prepared.task_title,
        analysis_detail: prepared.analysis_detail,
        risk_outcome: prepared.risk_outcome,
        tool_trace: None,
        verification_report: None,
    }
}
