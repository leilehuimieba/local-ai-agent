use crate::capabilities::{ToolDefinition, ToolExecutionTrace};
use crate::checkpoint::{RunCheckpoint, load_matching_resume_checkpoint};
use crate::context_builder::RuntimeContextEnvelope;
use crate::contracts::RunRequest;
use crate::planner::PlannedAction;
use crate::repo_context::{RepoContextLoadResult, load_repo_context};
use crate::risk::RiskOutcome;
use crate::run_state_builder::{
    PreparedRunState, bootstrap_context, prepare_run_state, record_bootstrap_memory,
};
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

fn apply_resume_checkpoint(
    session: &mut SessionMemory,
    checkpoint: Option<&RunCheckpoint>,
    request: &RunRequest,
) {
    let Some(checkpoint) = checkpoint else {
        return;
    };
    session.short_term.current_plan = resume_plan(checkpoint);
    session.short_term.current_phase = resume_phase(checkpoint);
    session.short_term.last_run_status = checkpoint.status.clone();
    session.short_term.recent_observation = checkpoint.response.result.final_answer.clone();
    session.short_term.recent_tool_result = checkpoint.response.result.summary.clone();
    if checkpoint.resume_reason == "confirmation_required" {
        session.short_term.pending_confirmation.clear();
        session.short_term.open_issue.clear();
    } else if request.resume_strategy == "retry_failure" {
        session.short_term.pending_confirmation.clear();
        session.short_term.open_issue = checkpoint.response.result.summary.clone();
    }
}

fn resume_plan(checkpoint: &RunCheckpoint) -> String {
    format!(
        "从 checkpoint 恢复：{} -> {}",
        checkpoint.resume_reason, checkpoint.resume_stage
    )
}

fn resume_phase(checkpoint: &RunCheckpoint) -> String {
    if checkpoint.resume_reason == "confirmation_required" {
        "confirmation_resume".to_string()
    } else {
        "recovery".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::apply_resume_checkpoint;
    use crate::checkpoint::RunCheckpoint;
    use crate::contracts::{
        ModelRef, ProviderRef, RunRequest, RunResult, RuntimeRunResponse, WorkspaceRef,
    };
    use crate::session::SessionMemory;
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

    fn sample_checkpoint_response() -> RuntimeRunResponse {
        RuntimeRunResponse {
            events: Vec::new(),
            result: sample_checkpoint_result(),
            confirmation_request: None,
        }
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
}

fn refresh_context_after_execution(
    envelope: &mut RuntimeContextEnvelope,
    trace: &ToolExecutionTrace,
) {
    envelope.dynamic_block.reasoning_summary = trace.result.reasoning_summary.clone();
    envelope.dynamic_block.cache_status = trace.result.cache_status.clone();
    envelope.dynamic_block.cache_reason = trace.result.cache_reason.clone();
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
