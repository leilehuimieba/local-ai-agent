use crate::capabilities::{ToolDefinition, ToolExecutionTrace};
use crate::checkpoint::load_matching_resume_checkpoint;
use crate::context_builder::RuntimeContextEnvelope;
use crate::contracts::RunRequest;
use crate::planner::{PlanEnvelope, PlannedAction, action_step_label};
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
use crate::verify::VerificationReport;
use std::path::PathBuf;

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
}

pub(crate) fn should_replan(state: &RuntimeRunState) -> bool {
    if knowledge_answer_verify_failed(state) {
        return true;
    }
    matches!(
        state.action,
        PlannedAction::ReadFile { .. } | PlannedAction::ListFiles { .. }
    ) && state.tool_trace.as_ref().is_some_and(|trace| trace.result.success)
}

pub(crate) fn replan_state(state: &RuntimeRunState, iteration_index: u8) -> Option<RuntimeRunState> {
    let next_action = next_action_from_trace(state)?;
    let mut next_state = state.clone();
    next_state.action = next_action.clone();
    next_state.tool_call.action = next_action.clone();
    next_state.tool_call.spec =
        crate::capabilities::resolve_tool_for_request(&next_state.envelope.request, &next_action);
    next_state.task_title = crate::derive_task_title(&next_action, &next_state.envelope.request.user_input);
    next_state.analysis_detail = format!(
        "主循环基于上一步结果触发补充动作：{}。",
        action_step_label(&next_action)
    );
    next_state.risk_outcome = crate::risk::assess_risk(&next_state.envelope.request, &next_action);
    next_state.tool_trace = None;
    next_state.verification_report = None;
    next_state.plan_envelope.iteration_index = iteration_index;
    next_state.plan_envelope.current_step = action_step_label(&next_action);
    next_state.plan_envelope.remaining_steps = remaining_steps_for_action(&next_action);
    Some(next_state)
}

pub(crate) fn budget_exhausted(state: &RuntimeRunState) -> bool {
    state.plan_envelope.iteration_index >= state.plan_envelope.max_iterations
}

fn next_action_from_trace(state: &RuntimeRunState) -> Option<PlannedAction> {
    if knowledge_answer_verify_failed(state) {
        return Some(PlannedAction::SearchKnowledge {
            query: state.envelope.request.user_input.clone(),
        });
    }
    let trace = state.tool_trace.as_ref()?;
    match &state.action {
        PlannedAction::ListFiles { path } => next_read_action(path.as_deref(), &trace.result.final_answer),
        PlannedAction::ReadFile { path } => next_list_action(path),
        _ => None,
    }
}

fn next_read_action(base: Option<&str>, answer: &str) -> Option<PlannedAction> {
    let path = extract_candidate_path(answer, &["AGENTS.md", "README.md", "Cargo.toml"])?;
    Some(PlannedAction::ReadFile {
        path: join_base_path(base, &path),
    })
}

fn next_list_action(path: &str) -> Option<PlannedAction> {
    std::path::Path::new(path)
        .parent()
        .map(|item| item.display().to_string())
        .map(|path| PlannedAction::ListFiles { path: Some(path) })
}

fn extract_candidate_path(answer: &str, names: &[&str]) -> Option<String> {
    answer.lines().map(str::trim).find_map(|line| {
        names
            .iter()
            .find(|name| line.contains(**name))
            .map(|_| clean_path(line))
    })
}

fn clean_path(line: &str) -> String {
    line.trim_matches(|ch| ch == '-' || ch == '*' || ch == '`' || ch == ' ')
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_matches(|ch| ch == '"' || ch == '\'')
        .to_string()
}

fn join_base_path(base: Option<&str>, candidate: &str) -> String {
    if candidate.contains(':') || candidate.starts_with('/') || candidate.starts_with('\\') {
        return candidate.to_string();
    }
    match base {
        Some(value) if !value.is_empty() => format!("{value}/{candidate}"),
        _ => candidate.to_string(),
    }
}

fn remaining_steps_for_action(action: &PlannedAction) -> Vec<String> {
    match action {
        PlannedAction::ReadFile { .. } => vec!["整理文件内容并判断是否还需补充目录观察".to_string()],
        PlannedAction::ListFiles { .. } => vec!["从目录结果中选择候选文件继续读取".to_string()],
        PlannedAction::SearchKnowledge { .. } => vec!["补充知识命中并重新形成带引证回答".to_string()],
        _ => vec!["完成验证并决定是否收口".to_string()],
    }
}

fn knowledge_answer_verify_failed(state: &RuntimeRunState) -> bool {
    let Some(report) = state.verification_report.as_ref() else {
        return false;
    };
    report.outcome.task_type == "knowledge_answer" && !report.outcome.passed
}

#[cfg(test)]
mod tests {
    use crate::query_engine_testkit::testkit::{
        sample_checkpoint, sample_checkpoint_with_tool, sample_repo_context, sample_request, sample_session,
    };
    use crate::run_recover_action::resumed_prepared_state;
    use crate::run_resume::apply_resume_checkpoint;
    use crate::tool_registry::runtime_tool_registry;

    #[test]
    fn clears_pending_confirmation_when_resuming_after_approval() {
        let request = sample_request("after_confirmation");
        let checkpoint = sample_checkpoint("confirmation_required");
        let mut session = sample_session();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(session.short_term.pending_confirmation.is_empty());
        assert_eq!(session.short_term.current_phase, "confirmation_resume");
    }

    #[test]
    fn keeps_failure_context_when_resuming_retryable_failure() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint("retryable_failure");
        let mut session = sample_session();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert_eq!(session.short_term.current_phase, "recovery");
        assert_eq!(session.short_term.open_issue, "temporary failure");
    }

    #[test]
    fn restores_action_from_checkpoint_tool_snapshot() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint_with_tool("run_command", r#"{"command":"echo restored"}"#);
        let session = sample_session();
        let repo = sample_repo_context();
        let visible = runtime_tool_registry().request_visible_tools(&request);
        let prepared = resumed_prepared_state(&request, &session, &repo, &visible, Some(&checkpoint));
        assert!(matches!(
            prepared.expect("prepared").action,
            crate::planner::PlannedAction::RunCommand { command }
            if command == "echo restored"
        ));
    }
}
