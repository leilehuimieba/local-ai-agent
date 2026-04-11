use crate::capabilities::{ToolDefinition, ToolExecutionTrace};
use crate::checkpoint::load_matching_resume_checkpoint;
use crate::context_builder::RuntimeContextEnvelope;
use crate::contracts::RunRequest;
use crate::planner::PlannedAction;
use crate::repo_context::{RepoContextLoadResult, load_repo_context};
use crate::risk::RiskOutcome;
use crate::run_recover_action::resumed_prepared_state;
use crate::run_resume::apply_resume_checkpoint;
use crate::run_runtime_state::{assemble_runtime_state, refresh_context_after_execution};
use crate::run_state_builder::{bootstrap_context, prepare_run_state, record_bootstrap_memory};
use crate::session::{SessionMemory, load_session_context, record_execution_memory};
use crate::tool_registry::{ToolCall, runtime_tool_registry};
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

#[cfg(test)]
mod tests {
    use crate::query_engine_testkit::testkit::{
        sample_checkpoint, sample_checkpoint_with_tool, sample_repo_context, sample_request,
        sample_session,
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
        let checkpoint =
            sample_checkpoint_with_tool("run_command", r#"{"command":"echo restored"}"#);
        let session = sample_session();
        let repo = sample_repo_context();
        let visible = runtime_tool_registry().visible_tools(&request.mode);
        let prepared =
            resumed_prepared_state(&request, &session, &repo, &visible, Some(&checkpoint));
        assert!(matches!(
            prepared.expect("prepared").action,
            crate::planner::PlannedAction::RunCommand { command }
            if command == "echo restored"
        ));
    }

}
