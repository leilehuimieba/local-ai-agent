mod action_decode;
mod action_meta;
mod answer_cache;
mod answer_cache_helpers;
mod answer_sanitize;
mod artifacts;
mod capabilities;
mod checkpoint;
mod compaction;
mod completion;
mod context_builder;
mod context_policy;
mod contracts;
mod events;
mod execution;
mod executors;
mod handoff;
mod knowledge;
mod knowledge_store;
mod llm;
mod mcp_bridge;
mod memory;
mod memory_layer;
mod memory_object_store;
mod memory_recall;
mod memory_router;
mod memory_schema;
mod memory_views;
mod model_adapter;
mod model_client;
mod observation;
mod paths;
mod planner;
mod prompt;
mod query_engine;
mod query_engine_testkit;
mod repo_context;
mod risk;
mod run_context_metadata;
mod run_failure_metadata;
mod run_finish_events;
mod run_memory_metadata;
mod run_metadata;
mod run_recover_action;
mod run_resume;
mod run_resume_action_hint;
mod run_resume_artifact;
mod run_resume_boundary;
mod run_resume_clear;
mod run_resume_event_testkit;
mod run_resume_handoff;
mod run_resume_hint;
mod run_resume_observation;
mod run_resume_plan;
mod run_resume_state;
mod run_resume_testkit;
mod run_resume_tests;
mod run_resume_verification;
mod run_risk_flow;
mod run_risk_flow_tests;
mod run_runtime_state;
mod run_snapshot_action;
mod run_state_builder;
mod run_tool_metadata;
mod run_verification_metadata;
mod run_visibility;
mod sensitive_data;
mod session;
mod skill_catalog;
mod sqlite_store;
mod storage;
mod storage_migration;
mod task_title;
mod text;
mod tool_registry;
mod tool_trace;
// tools 模块已收敛为 capabilities（能力注册与元信息单一事实源）
#[cfg(test)]
mod h03_eval_tests;
mod verify;

use crate::checkpoint::{checkpoint_resume_event, with_checkpoint_resume_event, with_runtime_checkpoint};
use crate::completion::decide_completion;
use crate::events::{make_event, with_runtime_memory_recall_event};
use crate::handoff::persist_handoff_artifact;
use crate::memory_router::evaluate_finish_memory_writes;
use crate::query_engine::{bootstrap_run, budget_exhausted, execute_stage, replan_state, should_replan};
use crate::repo_context::repo_context_metadata;
use crate::risk::RiskOutcome;
use crate::run_finish_events::{
    append_recall_visibility_metadata, make_memory_event, make_run_failed_event, read_result_mode, run_finished_summary,
};
use crate::run_metadata::{append_context_metadata, append_tool_spec_metadata, append_verification_metadata};
use crate::run_risk_flow::handle_risk_outcome;
use crate::run_visibility::apply_visibility_metadata;
use crate::session::{persist_handoff_path, persist_session_outputs};
use crate::task_title::derive_task_title;
use crate::verify::verify_tool_execution;
use std::collections::BTreeMap;

pub use crate::observation::{
    LifecycleMappingSnapshot, ObservationAbTestReport, ObservationDedupeReport, ObservationDetailItem,
    ObservationGetReport, ObservationLayeredInjectionReport, ObservationPersistenceReport,
    ObservationPrivacyRedactReport, ObservationPrivateSkipReport, ObservationQueueFlowReport,
    ObservationQueueHealthReport, ObservationRankItem, ObservationRankReport, ObservationRecord,
    ObservationRetryReport, ObservationRollbackReport, ObservationSearchItem, ObservationSearchReport,
    ObservationTimelineItem, ObservationTimelineReport, build_layered_injection, compare_layered_vs_full,
    dedupe_lifecycle_observations, get_observations, lifecycle_mapping_snapshot, lifecycle_target_event_types,
    observation_from_event, observation_kind_for_event_type, observation_privacy_redact_flow,
    observation_private_skip_flow, observation_queue_health, observation_rollback_flow, observation_timeline,
    persist_lifecycle_observations, rank_observations, run_observation_queue_flow, run_observation_retry_flow,
    search_observations,
};

pub use crate::contracts::{
    CapabilityListResponse, CapabilitySpec, ConfirmationDecision, ConfirmationRequest, ConnectorListResponse,
    ConnectorSlotSpec, ErrorInfo, GitCommitSummary, GitSnapshot, ModelRef, ProviderRef, RUNTIME_NAME, RUNTIME_VERSION,
    RepoContextSnapshot, RunEvent, RunRequest, RunResult, RuntimeRunResponse, RuntimeSnapshot, WorkspaceDocSummary,
    WorkspaceRef,
};

pub fn capability_catalog(mode: &str) -> CapabilityListResponse {
    CapabilityListResponse {
        items: tool_registry::runtime_tool_registry().capability_specs(mode),
    }
}

pub fn capability_catalog_for_request(request: &RunRequest) -> CapabilityListResponse {
    CapabilityListResponse {
        items: tool_registry::runtime_tool_registry().request_capability_specs(request),
    }
}

pub fn connector_catalog() -> ConnectorListResponse {
    ConnectorListResponse {
        items: tool_registry::runtime_tool_registry().connector_slot_specs(),
    }
}

pub fn simulate_run_with_runtime_events(request: &RunRequest) -> RuntimeRunResponse {
    let resume_event = checkpoint_resume_event(request);
    let response = simulate_run(request);
    let response = with_runtime_memory_recall_event(request, response);
    let response = with_checkpoint_resume_event(response, resume_event);
    let mut response = with_runtime_checkpoint(request, response);
    apply_visibility_metadata(&mut response.events);
    response
}

pub fn simulate_run(request: &RunRequest) -> RuntimeRunResponse {
    let mut state = bootstrap_run(request);
    let mut events = Vec::new();
    let mut sequence = 1;
    push_run_started(request, &state, &mut events, &mut sequence);
    push_analysis_ready(request, &state, &mut events, &mut sequence);
    if let Some(response) = handle_risk_outcome(request, &state, &mut events, &mut sequence) {
        return response;
    }
    let finished = run_main_loop(request, &mut state, &mut events, &mut sequence);
    finish_response(request, &state, &mut events, &mut sequence, finished)
}

struct FinishedRun {
    completion: crate::completion::CompletionDecision,
    handoff_path: Option<String>,
}

fn push_run_started(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let mut metadata = base_run_metadata(state);
    metadata.insert("next_step".to_string(), "分析用户输入".to_string());
    metadata.insert(
        "visible_tool_count".to_string(),
        state.envelope.visible_tools.len().to_string(),
    );
    events.push(make_event(
        request,
        *sequence,
        "run_started",
        "Analyze",
        "开始处理任务",
        "Go 控制面已经把本次任务提交给 Rust 运行时。",
        metadata,
    ));
    *sequence += 1;
}

fn push_analysis_ready(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let mut metadata = base_run_metadata(state);
    metadata.insert("next_step".to_string(), analysis_next_step(state));
    if !state.envelope.session_context.compressed_summary.is_empty() {
        metadata.insert(
            "session_turn_count".to_string(),
            state.envelope.session_context.recent_turns.len().to_string(),
        );
    }
    events.push(make_event(
        request,
        *sequence,
        "analysis_ready",
        "Analyze",
        "已分析用户输入",
        &state.analysis_detail,
        metadata,
    ));
    *sequence += 1;
}

fn analysis_next_step(state: &crate::query_engine::RuntimeRunState) -> String {
    match state.risk_outcome {
        RiskOutcome::Blocked(_) => "受模式策略限制，准备收口".to_string(),
        RiskOutcome::RequireConfirmation(_) => "需要等待人工确认".to_string(),
        RiskOutcome::Proceed => "准备生成执行计划".to_string(),
    }
}

fn run_main_loop(
    request: &RunRequest,
    state: &mut crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) -> FinishedRun {
    loop {
        run_single_iteration(request, state, events, sequence);
        if let Some(next_state) = continue_or_replan(request, state, events, sequence) {
            *state = next_state;
            continue;
        }
        return finalize_iteration(request, state, events, sequence);
    }
}

fn run_single_iteration(
    request: &RunRequest,
    state: &mut crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    push_iteration_started(request, state, events, sequence);
    execute_stage(state);
    push_plan_ready(request, state, events, sequence);
    push_action_requested(request, state, events, sequence);
    push_action_completed(request, state, events, sequence);
}

fn continue_or_replan(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) -> Option<crate::query_engine::RuntimeRunState> {
    if !should_replan(state) {
        push_iteration_completed(request, state, events, sequence, "当前动作已完成，准备进入验证");
        return None;
    }
    if budget_exhausted(state) {
        push_budget_exhausted(request, state, events, sequence);
        return None;
    }
    let next_iteration = state.plan_envelope.iteration_index + 1;
    let next_state = replan_state(state, next_iteration)?;
    push_iteration_completed(request, state, events, sequence, "当前动作已完成，进入补充动作");
    push_replan_requested(request, &next_state, events, sequence, "上一步结果显示仍需补充观察");
    Some(next_state)
}

fn finalize_iteration(
    request: &RunRequest,
    state: &mut crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) -> FinishedRun {
    let action_result = state.tool_trace.as_ref().expect("tool trace");
    let verification_report = verify_tool_execution(&state.tool_call, action_result);
    let completion = decide_completion(&verification_report);
    let handoff_path = persist_handoff_artifact(
        request,
        &state.task_title,
        &state.action,
        action_result,
        &verification_report,
    );
    state.verification_report = Some(verification_report);
    push_verification_completed(request, state, events, sequence, &completion);
    FinishedRun {
        completion,
        handoff_path,
    }
}

fn finish_response(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    finished: FinishedRun,
) -> RuntimeRunResponse {
    let action_result = state.tool_trace.as_ref().expect("tool trace");
    let verification_report = state.verification_report.as_ref().expect("verification report");
    let failure_error = build_failure_error(action_result);
    persist_session_outputs(
        request,
        &action_result.result.final_answer,
        &action_result.result.summary,
        finish_status(action_result.result.success),
    );
    persist_handoff_if_needed(request, finished.handoff_path.as_deref());
    push_memory_written_if_needed(request, state, events, sequence);
    push_finish_memory_events(request, state, action_result, verification_report, events, sequence);
    push_failed_event_if_needed(request, state, action_result, failure_error.as_ref(), events, sequence);
    push_run_finished(
        request,
        state,
        action_result,
        verification_report,
        &finished.completion,
        finished.handoff_path.as_deref(),
        events,
        sequence,
    );
    RuntimeRunResponse {
        events: events.clone(),
        result: build_run_result(request, action_result, &finished.completion, failure_error),
        confirmation_request: None,
    }
}

fn finish_status(success: bool) -> &'static str {
    if success { "completed" } else { "failed" }
}

fn persist_handoff_if_needed(request: &RunRequest, handoff_path: Option<&str>) {
    if let Some(path) = handoff_path {
        persist_handoff_path(request, path);
    }
}

fn build_failure_error(action_result: &crate::capabilities::ToolExecutionTrace) -> Option<ErrorInfo> {
    if action_result.result.success {
        return None;
    }
    Some(ErrorInfo {
        error_code: action_result
            .result
            .error_code
            .clone()
            .unwrap_or_else(|| "action_execution_failed".to_string()),
        message: action_result.result.final_answer.clone(),
        summary: action_result.result.summary.clone(),
        retryable: action_result.result.retryable,
        source: "runtime".to_string(),
        stage: "Finish".to_string(),
        metadata: BTreeMap::new(),
    })
}

fn build_run_result(
    request: &RunRequest,
    action_result: &crate::capabilities::ToolExecutionTrace,
    completion: &crate::completion::CompletionDecision,
    error: Option<ErrorInfo>,
) -> RunResult {
    RunResult {
        request_id: request.request_id.clone(),
        run_id: request.run_id.clone(),
        session_id: request.session_id.clone(),
        trace_id: request.trace_id.clone(),
        kind: "run_result".to_string(),
        source: "runtime".to_string(),
        status: completion.status.clone(),
        final_answer: action_result.result.final_answer.clone(),
        summary: action_result.result.summary.clone(),
        error,
        memory_write_summary: action_result.result.memory_write_summary.clone(),
        final_stage: "Finish".to_string(),
        checkpoint_id: None,
        resumable: None,
    }
}

fn push_iteration_started(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let mut metadata = base_run_metadata(state);
    metadata.insert("reason".to_string(), "开始执行当前计划迭代".to_string());
    metadata.insert("next_step".to_string(), "执行当前动作".to_string());
    events.push(make_event(
        request,
        *sequence,
        "plan_iteration_started",
        "Plan",
        "开始执行计划迭代",
        &state.plan_envelope.current_step,
        metadata,
    ));
    *sequence += 1;
}

fn push_plan_ready(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let action_result = state.tool_trace.as_ref().expect("tool trace");
    let mut metadata = action_event_metadata(state);
    metadata.insert(
        "next_step".to_string(),
        format!("执行 {}", action_result.tool.display_name),
    );
    events.push(make_event(
        request,
        *sequence,
        "plan_ready",
        "Plan",
        "已生成当前执行计划",
        &action_result.action_summary,
        metadata,
    ));
    *sequence += 1;
}

fn push_action_requested(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let action_result = state.tool_trace.as_ref().expect("tool trace");
    let mut metadata = action_event_metadata(state);
    metadata.insert("workspace_id".to_string(), request.workspace_ref.workspace_id.clone());
    metadata.insert("mode".to_string(), request.mode.clone());
    metadata.insert("tool_risk_level".to_string(), action_result.tool.risk_level.clone());
    metadata.insert("next_step".to_string(), "等待工具执行结果".to_string());
    events.push(make_event(
        request,
        *sequence,
        "action_requested",
        "Execute",
        &format!("准备调用 {}", action_result.tool.display_name),
        &action_result.action_summary,
        metadata,
    ));
    *sequence += 1;
}

fn push_action_completed(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let action_result = state.tool_trace.as_ref().expect("tool trace");
    let mut metadata = action_event_metadata(state);
    metadata.insert("result_summary".to_string(), action_result.result.summary.clone());
    if let Some(path) = action_result.result.artifact_path.clone() {
        metadata.insert("artifact_path".to_string(), path);
    }
    metadata.insert(
        "next_step".to_string(),
        action_completed_next_step(action_result.result.success),
    );
    events.push(make_event(
        request,
        *sequence,
        "action_completed",
        "Observe",
        &format!("{} 已完成", action_result.tool.display_name),
        &action_result.result.summary,
        metadata,
    ));
    *sequence += 1;
}

fn action_completed_next_step(success: bool) -> String {
    if success {
        "准备整理结果并验证".to_string()
    } else {
        "准备收口失败结果".to_string()
    }
}

fn push_iteration_completed(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    reason: &str,
) {
    let mut metadata = base_run_metadata(state);
    metadata.insert("reason".to_string(), reason.to_string());
    metadata.insert("next_step".to_string(), "判断是否需要继续迭代".to_string());
    events.push(make_event(
        request,
        *sequence,
        "plan_iteration_completed",
        "Observe",
        "当前计划迭代已完成",
        reason,
        metadata,
    ));
    *sequence += 1;
}

fn push_replan_requested(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    reason: &str,
) {
    let mut metadata = base_run_metadata(state);
    metadata.insert("reason".to_string(), reason.to_string());
    metadata.insert("next_step".to_string(), "切换到下一步补充动作".to_string());
    events.push(make_event(
        request,
        *sequence,
        "replan_requested",
        "Plan",
        "主循环请求重规划",
        reason,
        metadata,
    ));
    *sequence += 1;
}

fn push_budget_exhausted(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let mut metadata = base_run_metadata(state);
    metadata.insert("reason".to_string(), "达到迭代预算上限，准备生成 handoff".to_string());
    metadata.insert("next_step".to_string(), "停止自动续跑并进入验证".to_string());
    events.push(make_event(
        request,
        *sequence,
        "iteration_budget_exhausted",
        "Observe",
        "主循环达到预算上限",
        "已停止继续自动迭代，并准备生成 handoff 供后续接力。",
        metadata,
    ));
    *sequence += 1;
}

fn push_verification_completed(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    completion: &crate::completion::CompletionDecision,
) {
    let action_result = state.tool_trace.as_ref().expect("tool trace");
    let verification_report = state.verification_report.as_ref().expect("verification report");
    let mut metadata = action_event_metadata(state);
    if let Some(path) = action_result.result.artifact_path.clone() {
        metadata.insert("artifact_path".to_string(), path);
    }
    metadata.insert("next_step".to_string(), "生成最终答复".to_string());
    metadata.insert("final_answer".to_string(), action_result.result.final_answer.clone());
    metadata.insert(
        "result_mode".to_string(),
        read_result_mode(
            &action_result.result.final_answer,
            &completion.status,
            &verification_report.outcome.code,
        )
        .to_string(),
    );
    append_verification_metadata(&mut metadata, verification_report);
    events.push(make_event(
        request,
        *sequence,
        "verification_completed",
        "Verify",
        verification_summary_title(action_result.result.success),
        &verification_report.outcome.summary,
        metadata,
    ));
    *sequence += 1;
}

fn verification_summary_title(success: bool) -> &'static str {
    if success {
        "已形成验证结果"
    } else {
        "已形成失败验证结果"
    }
}

fn base_run_metadata(state: &crate::query_engine::RuntimeRunState) -> BTreeMap<String, String> {
    let mut metadata = BTreeMap::new();
    metadata.insert("task_title".to_string(), state.task_title.clone());
    metadata.insert(
        "iteration_index".to_string(),
        state.plan_envelope.iteration_index.to_string(),
    );
    metadata.insert(
        "max_iterations".to_string(),
        state.plan_envelope.max_iterations.to_string(),
    );
    metadata.insert("plan_goal".to_string(), state.plan_envelope.goal.clone());
    metadata.insert(
        "plan_current_step".to_string(),
        state.plan_envelope.current_step.clone(),
    );
    metadata.insert(
        "plan_stop_condition".to_string(),
        state.plan_envelope.stop_condition.clone(),
    );
    metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    metadata
}

fn action_event_metadata(state: &crate::query_engine::RuntimeRunState) -> BTreeMap<String, String> {
    let action_result = state.tool_trace.as_ref().expect("tool trace");
    let mut metadata = base_run_metadata(state);
    metadata.insert("tool_name".to_string(), action_result.tool.tool_name.clone());
    metadata.insert("tool_display_name".to_string(), action_result.tool.display_name.clone());
    metadata.insert("tool_category".to_string(), action_result.tool.category.clone());
    metadata.insert("output_kind".to_string(), action_result.tool.output_kind.clone());
    metadata.insert("risk_level".to_string(), action_result.tool.risk_level.clone());
    append_context_metadata(&mut metadata, &state.envelope.context_envelope);
    append_tool_spec_metadata(&mut metadata, &state.tool_call);
    metadata
}

fn push_memory_written_if_needed(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let action_result = state.tool_trace.as_ref().expect("tool trace");
    let Some(memory_summary) = action_result.result.memory_write_summary.clone() else {
        return;
    };
    let mut metadata = BTreeMap::new();
    metadata.insert("memory_kind".to_string(), "explicit_memory".to_string());
    metadata.insert("memory_scope".to_string(), request.workspace_ref.workspace_id.clone());
    metadata.insert("task_title".to_string(), state.task_title.clone());
    metadata.insert("next_step".to_string(), "准备完成本次任务".to_string());
    metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    events.push(make_event(
        request,
        *sequence,
        "memory_written",
        "Finish",
        "已写入长期记忆",
        &memory_summary,
        metadata,
    ));
    *sequence += 1;
}

fn push_finish_memory_events(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    action_result: &crate::capabilities::ToolExecutionTrace,
    verification_report: &crate::verify::VerificationReport,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    for outcome in evaluate_finish_memory_writes(request, action_result, verification_report) {
        events.push(make_memory_event(
            request,
            *sequence,
            &state.task_title,
            &state.envelope.repo_context,
            &outcome,
        ));
        *sequence += 1;
    }
}

fn push_failed_event_if_needed(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    action_result: &crate::capabilities::ToolExecutionTrace,
    error: Option<&ErrorInfo>,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let Some(error) = error else {
        return;
    };
    events.push(make_run_failed_event(
        request,
        *sequence,
        "当前动作执行失败",
        &action_result.result.final_answer,
        error,
        Some(action_result),
        &state.task_title,
        &state.envelope.repo_context,
    ));
    *sequence += 1;
}

fn push_run_finished(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    action_result: &crate::capabilities::ToolExecutionTrace,
    verification_report: &crate::verify::VerificationReport,
    completion: &crate::completion::CompletionDecision,
    handoff_path: Option<&str>,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) {
    let mut metadata = finish_metadata(request, state, action_result, verification_report, completion);
    if let Some(path) = handoff_path {
        metadata.insert("handoff_artifact_path".to_string(), path.to_string());
    }
    events.push(make_event(
        request,
        *sequence,
        "run_finished",
        "Finish",
        &run_finished_summary(action_result.result.success, action_result),
        &action_result.result.final_answer,
        metadata,
    ));
}

fn finish_metadata(
    request: &RunRequest,
    state: &crate::query_engine::RuntimeRunState,
    action_result: &crate::capabilities::ToolExecutionTrace,
    verification_report: &crate::verify::VerificationReport,
    completion: &crate::completion::CompletionDecision,
) -> BTreeMap<String, String> {
    let mut metadata = action_event_metadata(state);
    metadata.insert("final_answer".to_string(), action_result.result.final_answer.clone());
    metadata.insert("model_id".to_string(), request.model_ref.model_id.clone());
    metadata.insert("result_summary".to_string(), action_result.result.summary.clone());
    metadata.insert("mode".to_string(), request.mode.clone());
    metadata.insert("next_step".to_string(), "任务已结束".to_string());
    metadata.insert("completion_status".to_string(), completion.status.clone());
    metadata.insert("completion_reason".to_string(), completion.reason.clone());
    metadata.insert(
        "result_mode".to_string(),
        read_result_mode(
            &action_result.result.final_answer,
            &completion.status,
            &verification_report.outcome.code,
        )
        .to_string(),
    );
    if let Some(path) = action_result.result.artifact_path.clone() {
        metadata.insert("artifact_path".to_string(), path);
    }
    append_recall_visibility_metadata(&mut metadata, Some(action_result));
    append_verification_metadata(&mut metadata, verification_report);
    metadata
}
