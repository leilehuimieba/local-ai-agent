use crate::capabilities::{ToolDefinition, ToolExecutionTrace};
use crate::checkpoint::load_matching_resume_checkpoint;
use crate::context_builder::RuntimeContextEnvelope;
use crate::contracts::RunRequest;
use crate::planner::{PlanEnvelope, PlannedAction};
use crate::repo_context::{RepoContextLoadResult, load_repo_context};
use crate::risk::RiskOutcome;
use crate::run_recover_action::resumed_prepared_state;
use crate::run_resume::apply_resume_checkpoint;
use crate::run_runtime_state::{assemble_runtime_state, refresh_context_after_execution};
use crate::run_state_builder::{bootstrap_context, prepare_run_state, record_bootstrap_memory};
use crate::session::{SessionMemory, load_session_context, record_execution_memory};
use crate::skill_catalog::{SkillCatalog, load_skill_catalog, skill_catalog_brief};
use crate::tool_registry::{ToolCall, runtime_tool_registry};
use crate::tool_trace::execute_tool;
use crate::verify::{VerificationReport, verify_tool_execution};
use std::path::PathBuf;

mod browser_followup;
mod knowledge_followup;
mod path_followup;
mod replan;

pub(crate) use self::replan::{budget_exhausted, replan_state, should_replan};

#[derive(Clone, Debug)]
pub(crate) struct RuntimeEnvelope {
    pub request: RunRequest,
    pub session_context: SessionMemory,
    pub repo_context: RepoContextLoadResult,
    pub skill_catalog: SkillCatalog,
    pub context_envelope: RuntimeContextEnvelope,
    pub visible_tools: Vec<ToolDefinition>,
}

#[derive(Clone, Debug)]
pub(crate) struct RuntimeRunState {
    pub envelope: RuntimeEnvelope,
    pub plan_envelope: PlanEnvelope,
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
    let skill_catalog = load_skill_catalog(request);
    let visible_tools = runtime_tool_registry().request_visible_tools(request);
    let resume_checkpoint = load_matching_resume_checkpoint(request);
    let mut session_context = load_session_context(request);
    apply_resume_checkpoint(&mut session_context, resume_checkpoint.as_ref(), request);
    let prepared = resumed_prepared_state(
        request,
        &session_context,
        &repo_context,
        &visible_tools,
        resume_checkpoint.as_ref(),
    )
    .unwrap_or_else(|| prepare_run_state(request, &session_context, &repo_context, &visible_tools));
    record_bootstrap_memory(request, &mut session_context, &prepared);
    let context_envelope = bootstrap_context(request, &session_context, &repo_context, &visible_tools);
    assemble_runtime_state(
        request,
        session_context,
        repo_context,
        skill_catalog,
        visible_tools,
        context_envelope,
        prepared,
    )
}

pub(crate) fn execute_stage(state: &mut RuntimeRunState) {
    let _skill_catalog = skill_catalog_brief(&state.envelope.skill_catalog);
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
    update_verification_report(state);
}

fn update_verification_report(state: &mut RuntimeRunState) {
    state.verification_report = state
        .tool_trace
        .as_ref()
        .map(|trace| verify_tool_execution(&state.tool_call, trace));
}

#[cfg(test)]
mod tests;
