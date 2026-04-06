use crate::context_builder::{RuntimeContextEnvelope, build_runtime_context};
use crate::contracts::RunRequest;
use crate::execution::execute_tool;
use crate::planner::{PlannedAction, analysis_summary};
use crate::repo_context::{RepoContextLoadResult, load_repo_context};
use crate::risk::{RiskOutcome, assess_risk};
use crate::session::{
    SessionMemory, load_session_context, record_execution_memory, record_planning_memory,
};
use crate::tool_registry::{ToolCall, runtime_tool_registry};
use crate::tools::{ToolDefinition, ToolExecutionTrace};
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
    record_planning_memory(
        request,
        &mut session_context,
        &prepared.task_title,
        &prepared.analysis_detail,
        &prepared.risk_outcome,
    );
    let context_envelope = build_runtime_context(
        request,
        &session_context,
        &repo_context,
        &visible_tools,
        "cold",
        "当前为主链路首轮规划，还未进入回答缓存探测。",
    );
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
    let context_envelope = build_runtime_context(
        request,
        session_context,
        repo_context,
        visible_tools,
        "cold",
        "当前为工具规划阶段，还未进入回答缓存探测。",
    );
    let tool_call = runtime_tool_registry().plan_tool_call(&context_envelope);
    let action = tool_call.action.clone();
    PreparedRunState {
        task_title: crate::derive_task_title(&action, &request.user_input),
        analysis_detail: analysis_summary(&action, session_context, &repo_context.snapshot),
        risk_outcome: assess_risk(request, &action),
        action,
        tool_call,
    }
}

fn assemble_runtime_state(
    request: &RunRequest,
    session_context: SessionMemory,
    repo_context: RepoContextLoadResult,
    visible_tools: Vec<ToolDefinition>,
    context_envelope: RuntimeContextEnvelope,
    prepared: PreparedRunState,
) -> RuntimeRunState {
    RuntimeRunState {
        envelope: RuntimeEnvelope {
            request: request.clone(),
            session_context,
            repo_context,
            context_envelope,
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
