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
    use crate::checkpoint::RunCheckpoint;
    use crate::contracts::{
        ModelRef, ProviderRef, RepoContextSnapshot, RunEvent, RunRequest, RunResult,
        RuntimeRunResponse, ToolCallSnapshot, WorkspaceRef,
    };
    use crate::repo_context::RepoContextLoadResult;
    use crate::run_recover_action::resumed_prepared_state;
    use crate::run_resume::apply_resume_checkpoint;
    use crate::session::SessionMemory;
    use crate::tool_registry::runtime_tool_registry;
    use std::collections::BTreeMap;

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

    fn sample_request(strategy: &str) -> RunRequest {
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "retry task".to_string(),
            mode: "standard".to_string(),
            model_ref: ModelRef {
                provider_id: "provider".to_string(),
                model_id: "model".to_string(),
                display_name: "Model".to_string(),
            },
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-1".to_string(),
                name: "Workspace".to_string(),
                root_path: "D:/repo".to_string(),
                is_active: true,
            },
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: "cp-1".to_string(),
            resume_strategy: strategy.to_string(),
            confirmation_decision: None,
        }
    }

    fn sample_checkpoint(reason: &str) -> RunCheckpoint {
        RunCheckpoint {
            checkpoint_id: "cp-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            workspace_id: "workspace-1".to_string(),
            status: "failed".to_string(),
            final_stage: "Finish".to_string(),
            resumable: true,
            resume_reason: reason.to_string(),
            resume_stage: "Execute".to_string(),
            event_count: 2,
            request: sample_request(reason),
            response: sample_checkpoint_response(),
            created_at: "1".to_string(),
        }
    }

    fn sample_session() -> SessionMemory {
        let mut session = SessionMemory::default();
        session.short_term.pending_confirmation = "need approve".to_string();
        session
    }

    fn sample_repo_context() -> RepoContextLoadResult {
        RepoContextLoadResult {
            snapshot: RepoContextSnapshot {
                workspace_root: "D:/repo".to_string(),
                repo_root: None,
                git_available: false,
                git_snapshot: None,
                doc_summaries: Vec::new(),
                warnings: Vec::new(),
                collected_at: "1".to_string(),
            },
            degraded: false,
            error_count: 0,
        }
    }

    fn sample_checkpoint_response() -> RuntimeRunResponse {
        RuntimeRunResponse {
            events: Vec::new(),
            result: sample_checkpoint_result(),
            confirmation_request: None,
        }
    }

    fn sample_checkpoint_with_tool(tool_name: &str, arguments_json: &str) -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint("retryable_failure");
        checkpoint
            .response
            .events
            .push(sample_tool_event(tool_name, arguments_json));
        checkpoint
    }

    fn sample_checkpoint_result() -> RunResult {
        RunResult {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            kind: "run_result".to_string(),
            source: "runtime".to_string(),
            status: "failed".to_string(),
            final_answer: "temporary failure".to_string(),
            summary: "temporary failure".to_string(),
            error: None,
            memory_write_summary: None,
            final_stage: "Finish".to_string(),
            checkpoint_id: Some("cp-1".to_string()),
            resumable: Some(true),
        }
    }

    fn sample_tool_event(tool_name: &str, arguments_json: &str) -> RunEvent {
        RunEvent {
            tool_name: tool_name.to_string(),
            tool_call_snapshot: Some(sample_tool_snapshot(tool_name, arguments_json)),
            ..sample_action_requested_event()
        }
    }

    fn sample_tool_snapshot(tool_name: &str, arguments_json: &str) -> ToolCallSnapshot {
        ToolCallSnapshot {
            tool_name: tool_name.to_string(),
            display_name: "执行命令".to_string(),
            category: "system_command".to_string(),
            risk_level: "high".to_string(),
            input_schema: "command_text".to_string(),
            output_kind: "text_preview".to_string(),
            requires_confirmation: true,
            arguments_json: arguments_json.to_string(),
        }
    }

    fn sample_action_requested_event() -> RunEvent {
        RunEvent {
            event_id: "run-1-1".to_string(),
            kind: "run_event".to_string(),
            source: "runtime".to_string(),
            record_type: String::new(),
            source_type: String::new(),
            agent_id: "primary".to_string(),
            agent_label: "主智能体".to_string(),
            event_type: "action_requested".to_string(),
            trace_id: "trace-1".to_string(),
            session_id: "session-1".to_string(),
            run_id: "run-1".to_string(),
            sequence: 1,
            timestamp: "1".to_string(),
            stage: "Execute".to_string(),
            summary: "准备恢复动作".to_string(),
            detail: String::new(),
            tool_name: String::new(),
            tool_display_name: "执行命令".to_string(),
            tool_category: "system_command".to_string(),
            output_kind: "text_preview".to_string(),
            result_summary: String::new(),
            artifact_path: String::new(),
            risk_level: "high".to_string(),
            confirmation_id: String::new(),
            final_answer: String::new(),
            completion_status: String::new(),
            completion_reason: String::new(),
            verification_summary: String::new(),
            checkpoint_written: false,
            context_snapshot: None,
            tool_call_snapshot: None,
            verification_snapshot: None,
            metadata: BTreeMap::new(),
        }
    }
}
