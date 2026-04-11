use crate::capabilities::{ToolDefinition, ToolExecutionTrace};
use crate::context_builder::RuntimeContextEnvelope;
use crate::contracts::RunRequest;
use crate::query_engine::{RuntimeEnvelope, RuntimeRunState};
use crate::repo_context::RepoContextLoadResult;
use crate::run_state_builder::PreparedRunState;
use crate::session::SessionMemory;

pub(crate) fn refresh_context_after_execution(
    envelope: &mut RuntimeContextEnvelope,
    trace: &ToolExecutionTrace,
) {
    apply_reasoning_summary(envelope, trace);
    apply_cache_status(envelope, trace);
    apply_cache_reason(envelope, trace);
}

pub(crate) fn assemble_runtime_state(
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

fn apply_reasoning_summary(envelope: &mut RuntimeContextEnvelope, trace: &ToolExecutionTrace) {
    envelope.dynamic_block.reasoning_summary = trace.result.reasoning_summary.clone();
}

fn apply_cache_status(envelope: &mut RuntimeContextEnvelope, trace: &ToolExecutionTrace) {
    envelope.dynamic_block.cache_status = trace.result.cache_status.clone();
}

fn apply_cache_reason(envelope: &mut RuntimeContextEnvelope, trace: &ToolExecutionTrace) {
    envelope.dynamic_block.cache_reason = trace.result.cache_reason.clone();
}
