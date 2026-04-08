mod action_decode;
mod action_meta;
mod answer_cache;
mod answer_cache_helpers;
mod answer_sanitize;
mod artifacts;
mod capabilities;
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
mod memory;
mod memory_recall;
mod memory_router;
mod memory_schema;
mod model_adapter;
mod model_client;
mod paths;
mod planner;
mod prompt;
mod query_engine;
mod repo_context;
mod risk;
mod session;
mod sqlite_store;
mod storage;
mod storage_migration;
mod text;
mod tool_registry;
mod tool_trace;
// tools 模块已收敛为 capabilities（能力注册与元信息单一事实源）
mod verify;

use crate::completion::decide_completion;
use crate::events::{make_confirmation_event, make_event, with_runtime_memory_recall_event};
use crate::handoff::persist_handoff_artifact;
use crate::memory_router::evaluate_finish_memory_writes;
use crate::planner::PlannedAction;
use crate::query_engine::{bootstrap_run, execute_stage};
use crate::repo_context::repo_context_metadata;
use crate::risk::RiskOutcome;
use crate::session::{persist_handoff_path, persist_session_outputs};
use crate::verify::verify_tool_execution;
use std::collections::BTreeMap;

pub use crate::contracts::{
    CapabilityListResponse, CapabilitySpec, ConfirmationDecision, ConfirmationRequest,
    ConnectorListResponse, ConnectorSlotSpec, ErrorInfo, GitCommitSummary, GitSnapshot, ModelRef,
    RepoContextSnapshot, RunEvent, RunRequest, RunResult, RuntimeRunResponse, RuntimeSnapshot,
    WorkspaceDocSummary, WorkspaceRef, RUNTIME_NAME, RUNTIME_VERSION,
};

pub fn capability_catalog(mode: &str) -> CapabilityListResponse {
    CapabilityListResponse {
        items: tool_registry::runtime_tool_registry().capability_specs(mode),
    }
}

pub fn connector_catalog() -> ConnectorListResponse {
    ConnectorListResponse {
        items: tool_registry::runtime_tool_registry().connector_slot_specs(),
    }
}

pub fn simulate_run_with_runtime_events(request: &RunRequest) -> RuntimeRunResponse {
    with_runtime_memory_recall_event(request, simulate_run(request))
}

pub fn simulate_run(request: &RunRequest) -> RuntimeRunResponse {
    let mut state = bootstrap_run(request);
    let mut events = Vec::new();
    let mut sequence = 1;

    let mut start_metadata = BTreeMap::new();
    start_metadata.insert("task_title".to_string(), state.task_title.clone());
    start_metadata.insert("next_step".to_string(), "分析用户输入".to_string());
    start_metadata.insert(
        "visible_tool_count".to_string(),
        state.envelope.visible_tools.len().to_string(),
    );
    start_metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    events.push(make_event(
        request,
        sequence,
        "run_started",
        "Analyze",
        "开始处理任务",
        "Go 控制面已经把本次任务提交给 Rust 运行时。",
        start_metadata,
    ));
    sequence += 1;

    let mut analysis_metadata = BTreeMap::new();
    analysis_metadata.insert("task_title".to_string(), state.task_title.clone());
    analysis_metadata.insert(
        "next_step".to_string(),
        match state.risk_outcome {
            RiskOutcome::Blocked(_) => "受模式策略限制，准备收口".to_string(),
            RiskOutcome::RequireConfirmation(_) => "需要等待人工确认".to_string(),
            RiskOutcome::Proceed => "准备生成执行计划".to_string(),
        },
    );
    if !state.envelope.session_context.compressed_summary.is_empty() {
        analysis_metadata.insert(
            "session_turn_count".to_string(),
            state
                .envelope
                .session_context
                .recent_turns
                .len()
                .to_string(),
        );
    }
    analysis_metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    events.push(make_event(
        request,
        sequence,
        "analysis_ready",
        "Analyze",
        "已分析用户输入",
        &state.analysis_detail,
        analysis_metadata,
    ));
    sequence += 1;

    match &state.risk_outcome {
        RiskOutcome::Blocked(message) => {
            let error = ErrorInfo {
                error_code: "blocked_by_mode".to_string(),
                message: "当前动作被模式策略阻止".to_string(),
                summary: "观察模式不会执行修改性动作。".to_string(),
                retryable: true,
                source: "runtime".to_string(),
                stage: "Verify".to_string(),
                metadata: BTreeMap::new(),
            };
            events.push(make_event(
                request,
                sequence,
                "verification_completed",
                "Verify",
                "当前动作被模式策略阻止",
                &message,
                {
                    let mut metadata = BTreeMap::from([
                        ("task_title".to_string(), state.task_title.clone()),
                        ("next_step".to_string(), "生成失败结果".to_string()),
                    ]);
                    metadata.extend(repo_context_metadata(&state.envelope.repo_context));
                    metadata
                },
            ));
            sequence += 1;
            events.push(make_run_failed_event(
                request,
                sequence,
                "当前动作被模式策略阻止",
                &message,
                &error,
                None,
                &state.task_title,
                &state.envelope.repo_context,
            ));
            sequence += 1;
            events.push(make_event(
                request,
                sequence,
                "run_finished",
                "Finish",
                "任务未执行",
                &message,
                {
                    let mut metadata = BTreeMap::from([
                        ("task_title".to_string(), state.task_title.clone()),
                        ("next_step".to_string(), "任务已结束".to_string()),
                        ("final_answer".to_string(), message.clone()),
                    ]);
                    metadata.extend(repo_context_metadata(&state.envelope.repo_context));
                    metadata
                },
            ));
            persist_session_outputs(request, &message, &message, "failed");
            return RuntimeRunResponse {
                events,
                result: RunResult {
                    request_id: request.request_id.clone(),
                    run_id: request.run_id.clone(),
                    session_id: request.session_id.clone(),
                    trace_id: request.trace_id.clone(),
                    kind: "run_result".to_string(),
                    source: "runtime".to_string(),
                    status: "failed".to_string(),
                    final_answer: message.clone(),
                    summary: message.to_string(),
                    error: Some(error),
                    memory_write_summary: None,
                    final_stage: "Finish".to_string(),
                },
                confirmation_request: None,
            };
        }
        RiskOutcome::RequireConfirmation(confirmation) => {
            let mut confirmation_plan_metadata = BTreeMap::new();
            confirmation_plan_metadata.insert("task_title".to_string(), state.task_title.clone());
            confirmation_plan_metadata
                .insert("next_step".to_string(), "等待用户确认后再继续".to_string());
            append_context_metadata(
                &mut confirmation_plan_metadata,
                &state.envelope.context_envelope,
            );
            confirmation_plan_metadata.extend(repo_context_metadata(&state.envelope.repo_context));
            events.push(make_event(
                request,
                sequence,
                "plan_ready",
                "Plan",
                "已识别需要确认的动作",
                &confirmation.action_summary,
                confirmation_plan_metadata,
            ));
            sequence += 1;
            events.push(make_confirmation_event(request, sequence, &confirmation));
            return RuntimeRunResponse {
                events,
                result: RunResult {
                    request_id: request.request_id.clone(),
                    run_id: request.run_id.clone(),
                    session_id: request.session_id.clone(),
                    trace_id: request.trace_id.clone(),
                    kind: "run_result".to_string(),
                    source: "runtime".to_string(),
                    status: "awaiting_confirmation".to_string(),
                    final_answer: String::new(),
                    summary: confirmation.reason.clone(),
                    error: Some(ErrorInfo {
                        error_code: "risk_confirmation_required".to_string(),
                        message: confirmation.reason.clone(),
                        summary: "高风险动作需要人工确认。".to_string(),
                        retryable: true,
                        source: "runtime".to_string(),
                        stage: "PausedForConfirmation".to_string(),
                        metadata: BTreeMap::new(),
                    }),
                    memory_write_summary: None,
                    final_stage: "PausedForConfirmation".to_string(),
                },
                confirmation_request: Some(confirmation.clone()),
            };
        }
        RiskOutcome::Proceed => {}
    }

    execute_stage(&mut state);
    let action_result = state.tool_trace.as_ref().expect("tool trace");
    let verification_report = verify_tool_execution(&state.tool_call, action_result);
    let completion = decide_completion(&verification_report);
    state.verification_report = Some(verification_report.clone());
    let handoff_path = persist_handoff_artifact(
        request,
        &state.task_title,
        &state.action,
        action_result,
        &verification_report,
    );
    let mut plan_metadata = BTreeMap::new();
    plan_metadata.insert("task_title".to_string(), state.task_title.clone());
    plan_metadata.insert(
        "next_step".to_string(),
        format!("执行 {}", action_result.tool.display_name),
    );
    plan_metadata.insert(
        "tool_name".to_string(),
        action_result.tool.tool_name.clone(),
    );
    plan_metadata.insert(
        "tool_display_name".to_string(),
        action_result.tool.display_name.clone(),
    );
    plan_metadata.insert(
        "tool_category".to_string(),
        action_result.tool.category.clone(),
    );
    plan_metadata.insert(
        "output_kind".to_string(),
        action_result.tool.output_kind.clone(),
    );
    append_context_metadata(&mut plan_metadata, &state.envelope.context_envelope);
    append_tool_spec_metadata(&mut plan_metadata, &state.tool_call);
    plan_metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    events.push(make_event(
        request,
        sequence,
        "plan_ready",
        "Plan",
        "已生成当前执行计划",
        &action_result.action_summary,
        plan_metadata,
    ));
    sequence += 1;

    let mut request_metadata = BTreeMap::new();
    request_metadata.insert("task_title".to_string(), state.task_title.clone());
    request_metadata.insert(
        "tool_name".to_string(),
        action_result.tool.tool_name.clone(),
    );
    request_metadata.insert(
        "tool_display_name".to_string(),
        action_result.tool.display_name.clone(),
    );
    request_metadata.insert(
        "tool_category".to_string(),
        action_result.tool.category.clone(),
    );
    request_metadata.insert(
        "tool_risk_level".to_string(),
        action_result.tool.risk_level.clone(),
    );
    request_metadata.insert(
        "risk_level".to_string(),
        action_result.tool.risk_level.clone(),
    );
    request_metadata.insert(
        "output_kind".to_string(),
        action_result.tool.output_kind.clone(),
    );
    request_metadata.insert(
        "workspace_id".to_string(),
        request.workspace_ref.workspace_id.clone(),
    );
    request_metadata.insert("mode".to_string(), request.mode.clone());
    request_metadata.insert("next_step".to_string(), "等待工具执行结果".to_string());
    append_context_metadata(&mut request_metadata, &state.envelope.context_envelope);
    append_tool_spec_metadata(&mut request_metadata, &state.tool_call);
    request_metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    events.push(make_event(
        request,
        sequence,
        "action_requested",
        "Execute",
        &format!("准备调用 {}", action_result.tool.display_name),
        &action_result.action_summary,
        request_metadata,
    ));
    sequence += 1;

    let mut completed_metadata = BTreeMap::new();
    completed_metadata.insert("task_title".to_string(), state.task_title.clone());
    completed_metadata.insert(
        "tool_name".to_string(),
        action_result.tool.tool_name.clone(),
    );
    completed_metadata.insert(
        "tool_display_name".to_string(),
        action_result.tool.display_name.clone(),
    );
    completed_metadata.insert(
        "output_kind".to_string(),
        action_result.tool.output_kind.clone(),
    );
    completed_metadata.insert(
        "result_summary".to_string(),
        action_result.result.summary.clone(),
    );
    if let Some(path) = action_result.result.artifact_path.clone() {
        completed_metadata.insert("artifact_path".to_string(), path);
    }
    append_context_metadata(&mut completed_metadata, &state.envelope.context_envelope);
    append_tool_spec_metadata(&mut completed_metadata, &state.tool_call);
    completed_metadata.insert(
        "next_step".to_string(),
        if action_result.result.success {
            "准备整理结果并验证".to_string()
        } else {
            "准备收口失败结果".to_string()
        },
    );
    completed_metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    events.push(make_event(
        request,
        sequence,
        "action_completed",
        "Observe",
        &format!("{} 已完成", action_result.tool.display_name),
        &action_result.result.summary,
        completed_metadata,
    ));
    sequence += 1;

    if let Some(memory_summary) = action_result.result.memory_write_summary.clone() {
        let mut metadata = BTreeMap::new();
        metadata.insert("memory_kind".to_string(), "explicit_memory".to_string());
        metadata.insert(
            "memory_scope".to_string(),
            request.workspace_ref.workspace_id.clone(),
        );
        metadata.insert("task_title".to_string(), state.task_title.clone());
        metadata.insert("next_step".to_string(), "准备完成本次任务".to_string());
        metadata.extend(repo_context_metadata(&state.envelope.repo_context));
        events.push(make_event(
            request,
            sequence,
            "memory_written",
            "Finish",
            "已写入长期记忆",
            &memory_summary,
            metadata,
        ));
        sequence += 1;
    }

    let mut verification_metadata = BTreeMap::new();
    verification_metadata.insert("task_title".to_string(), state.task_title.clone());
    verification_metadata.insert(
        "tool_name".to_string(),
        action_result.tool.tool_name.clone(),
    );
    verification_metadata.insert(
        "tool_display_name".to_string(),
        action_result.tool.display_name.clone(),
    );
    verification_metadata.insert(
        "tool_category".to_string(),
        action_result.tool.category.clone(),
    );
    verification_metadata.insert(
        "output_kind".to_string(),
        action_result.tool.output_kind.clone(),
    );
    if let Some(path) = action_result.result.artifact_path.clone() {
        verification_metadata.insert("artifact_path".to_string(), path);
    }
    verification_metadata.insert("next_step".to_string(), "生成最终答复".to_string());
    append_context_metadata(&mut verification_metadata, &state.envelope.context_envelope);
    append_tool_spec_metadata(&mut verification_metadata, &state.tool_call);
    append_verification_metadata(&mut verification_metadata, &verification_report);
    verification_metadata.insert(
        "final_answer".to_string(),
        action_result.result.final_answer.clone(),
    );
    verification_metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    events.push(make_event(
        request,
        sequence,
        "verification_completed",
        "Verify",
        if action_result.result.success {
            "已形成验证结果"
        } else {
            "已形成失败验证结果"
        },
        &verification_report.outcome.summary,
        verification_metadata,
    ));
    sequence += 1;

    let failure_error = if action_result.result.success {
        None
    } else {
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
    };

    persist_session_outputs(
        request,
        &action_result.result.final_answer,
        &action_result.result.summary,
        if action_result.result.success {
            "completed"
        } else {
            "failed"
        },
    );
    if let Some(path) = handoff_path.as_ref() {
        persist_handoff_path(request, path);
    }
    for outcome in evaluate_finish_memory_writes(request, action_result, &verification_report) {
        events.push(make_memory_event(
            request,
            sequence,
            &state.task_title,
            &state.envelope.repo_context,
            &outcome,
        ));
        sequence += 1;
    }

    if let Some(error) = failure_error.as_ref() {
        events.push(make_run_failed_event(
            request,
            sequence,
            "当前动作执行失败",
            &action_result.result.final_answer,
            error,
            Some(action_result),
            &state.task_title,
            &state.envelope.repo_context,
        ));
        sequence += 1;
    }

    let mut finish_metadata = BTreeMap::new();
    finish_metadata.insert("task_title".to_string(), state.task_title.clone());
    finish_metadata.insert(
        "final_answer".to_string(),
        action_result.result.final_answer.clone(),
    );
    finish_metadata.insert("model_id".to_string(), request.model_ref.model_id.clone());
    finish_metadata.insert(
        "tool_name".to_string(),
        action_result.tool.tool_name.clone(),
    );
    finish_metadata.insert(
        "tool_display_name".to_string(),
        action_result.tool.display_name.clone(),
    );
    finish_metadata.insert(
        "tool_category".to_string(),
        action_result.tool.category.clone(),
    );
    finish_metadata.insert(
        "output_kind".to_string(),
        action_result.tool.output_kind.clone(),
    );
    finish_metadata.insert(
        "result_summary".to_string(),
        action_result.result.summary.clone(),
    );
    if let Some(path) = action_result.result.artifact_path.clone() {
        finish_metadata.insert("artifact_path".to_string(), path);
    }
    if let Some(path) = handoff_path {
        finish_metadata.insert("handoff_artifact_path".to_string(), path);
    }
    finish_metadata.insert("mode".to_string(), request.mode.clone());
    finish_metadata.insert("next_step".to_string(), "任务已结束".to_string());
    finish_metadata.insert("completion_status".to_string(), completion.status.clone());
    finish_metadata.insert("completion_reason".to_string(), completion.reason.clone());
    append_context_metadata(&mut finish_metadata, &state.envelope.context_envelope);
    append_tool_spec_metadata(&mut finish_metadata, &state.tool_call);
    append_verification_metadata(&mut finish_metadata, &verification_report);
    finish_metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    events.push(make_event(
        request,
        sequence,
        "run_finished",
        "Finish",
        if action_result.result.success {
            "任务已完成"
        } else {
            "任务已结束，存在执行失败"
        },
        &action_result.result.final_answer,
        finish_metadata,
    ));

    RuntimeRunResponse {
        events,
        result: RunResult {
            request_id: request.request_id.clone(),
            run_id: request.run_id.clone(),
            session_id: request.session_id.clone(),
            trace_id: request.trace_id.clone(),
            kind: "run_result".to_string(),
            source: "runtime".to_string(),
            status: completion.status.clone(),
            final_answer: action_result.result.final_answer.clone(),
            summary: action_result.result.summary.clone(),
            error: failure_error,
            memory_write_summary: action_result.result.memory_write_summary.clone(),
            final_stage: "Finish".to_string(),
        },
        confirmation_request: None,
    }
}

fn append_context_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    append_context_core_metadata(metadata, context);
    append_context_dynamic_metadata(metadata, context);
    append_context_policy_metadata(metadata, context);
}

fn append_tool_spec_metadata(
    metadata: &mut BTreeMap<String, String>,
    tool_call: &crate::tool_registry::ToolCall,
) {
    metadata.insert(
        "input_schema".to_string(),
        tool_call.spec.input_schema.clone(),
    );
    metadata.insert(
        "requires_confirmation".to_string(),
        if tool_call.spec.requires_confirmation {
            "true".to_string()
        } else {
            "false".to_string()
        },
    );
}

fn append_verification_metadata(
    metadata: &mut BTreeMap<String, String>,
    report: &crate::verify::VerificationReport,
) {
    metadata.insert("verification_code".to_string(), report.outcome.code.clone());
    metadata.insert("verification_passed".to_string(), bool_string(report.outcome.passed));
    metadata.insert("verification_summary".to_string(), report.outcome.summary.clone());
    metadata.insert("verification_next_step".to_string(), report.outcome.next_step.clone());
    metadata.insert("verification_policy".to_string(), report.outcome.policy.clone());
    metadata.insert("verification_evidence".to_string(), report.outcome.evidence.join("\n"));
}

fn bool_string(value: bool) -> String {
    if value {
        "true".to_string()
    } else {
        "false".to_string()
    }
}

fn append_context_core_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert("context_mode".to_string(), context.mode.clone());
    metadata.insert("context_workspace_root".to_string(), context.workspace_root.clone());
}

fn append_context_dynamic_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert("session_summary".to_string(), context.dynamic_block.session_summary.clone());
    metadata.insert("memory_digest".to_string(), context.dynamic_block.memory_digest.clone());
    metadata.insert(
        "knowledge_digest".to_string(),
        context.dynamic_block.knowledge_digest.clone(),
    );
    metadata.insert("tool_preview".to_string(), context.dynamic_block.tool_preview.clone());
    metadata.insert(
        "reasoning_summary".to_string(),
        context.dynamic_block.reasoning_summary.clone(),
    );
    metadata.insert("cache_status".to_string(), context.dynamic_block.cache_status.clone());
    metadata.insert("cache_reason".to_string(), context.dynamic_block.cache_reason.clone());
}

fn append_context_policy_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert(
        "assembly_profile".to_string(),
        context.dynamic_block.assembly_profile.clone(),
    );
    metadata.insert(
        "includes_session".to_string(),
        bool_string(context.dynamic_block.includes_session),
    );
    metadata.insert(
        "includes_memory".to_string(),
        bool_string(context.dynamic_block.includes_memory),
    );
    metadata.insert(
        "includes_knowledge".to_string(),
        bool_string(context.dynamic_block.includes_knowledge),
    );
    metadata.insert(
        "includes_tool_preview".to_string(),
        bool_string(context.dynamic_block.includes_tool_preview),
    );
}

fn make_run_failed_event(
    request: &RunRequest,
    sequence: u32,
    summary: &str,
    detail: &str,
    error: &ErrorInfo,
    tool_trace: Option<&crate::capabilities::ToolExecutionTrace>,
    task_title: &str,
    repo_context: &crate::repo_context::RepoContextLoadResult,
) -> RunEvent {
    let mut metadata = repo_context_metadata(repo_context);
    metadata.insert("task_title".to_string(), task_title.to_string());
    append_error_metadata(&mut metadata, error);
    append_tool_failure_metadata(&mut metadata, tool_trace);
    metadata.insert(
        "next_step".to_string(),
        failure_next_step(tool_trace, error),
    );
    make_event(
        request,
        sequence,
        "run_failed",
        "Failed",
        summary,
        detail,
        metadata,
    )
}

fn append_error_metadata(metadata: &mut BTreeMap<String, String>, error: &ErrorInfo) {
    metadata.insert("error_code".to_string(), error.error_code.clone());
    metadata.insert("error_message".to_string(), error.message.clone());
    metadata.insert("error_source".to_string(), error.source.clone());
    metadata.insert(
        "retryable".to_string(),
        if error.retryable { "true" } else { "false" }.to_string(),
    );
}

fn append_tool_failure_metadata(
    metadata: &mut BTreeMap<String, String>,
    tool_trace: Option<&crate::capabilities::ToolExecutionTrace>,
) {
    let Some(trace) = tool_trace else {
        return;
    };
    metadata.insert("tool_name".to_string(), trace.tool.tool_name.clone());
    metadata.insert(
        "tool_display_name".to_string(),
        trace.tool.display_name.clone(),
    );
    metadata.insert("tool_category".to_string(), trace.tool.category.clone());
    metadata.insert("output_kind".to_string(), trace.tool.output_kind.clone());
    metadata.insert("result_summary".to_string(), trace.result.summary.clone());
    metadata.insert("risk_level".to_string(), trace.tool.risk_level.clone());
    metadata.insert(
        "reasoning_summary".to_string(),
        trace.result.reasoning_summary.clone(),
    );
    metadata.insert(
        "failure_recovery_hint".to_string(),
        tool_failure_hint(trace.tool.tool_name.as_str()),
    );
    metadata.insert(
        "cache_status".to_string(),
        trace.result.cache_status.clone(),
    );
    metadata.insert(
        "cache_reason".to_string(),
        trace.result.cache_reason.clone(),
    );
    if let Some(path) = trace.result.artifact_path.clone() {
        metadata.insert("artifact_path".to_string(), path);
    }
}

fn failure_next_step(
    tool_trace: Option<&crate::capabilities::ToolExecutionTrace>,
    error: &ErrorInfo,
) -> String {
    if !error.retryable {
        return "当前失败不建议直接重试，建议先缩小影响范围或改成更安全的动作。".to_string();
    }
    tool_trace
        .map(|trace| tool_failure_hint(trace.tool.tool_name.as_str()))
        .unwrap_or_else(|| "建议先查看错误详情，再补上下文或调整任务后继续。".to_string())
}

fn tool_failure_hint(tool_name: &str) -> String {
    match tool_name {
        "run_command" => "建议先检查命令语法、依赖和当前环境，再决定是否重试。".to_string(),
        "workspace_write" => "建议先核对目标路径和父目录状态，再决定是否继续写入。".to_string(),
        "workspace_delete" => "建议先读取或列出目标路径，确认范围后再决定是否删除。".to_string(),
        "workspace_read" => "建议先确认目标文件存在且路径位于当前工作区。".to_string(),
        "project_answer" => "建议先检查项目文档命中情况，必要时补充上下文后再追问。".to_string(),
        _ => "建议先查看错误摘要与验证结果，再决定是否重试当前动作。".to_string(),
    }
}

fn make_memory_event(
    request: &RunRequest,
    sequence: u32,
    task_title: &str,
    repo_context: &crate::repo_context::RepoContextLoadResult,
    outcome: &crate::memory_router::MemoryWriteOutcome,
) -> RunEvent {
    let mut metadata = repo_context_metadata(repo_context);
    metadata.insert("task_title".to_string(), task_title.to_string());
    metadata.insert("layer".to_string(), outcome.layer.to_string());
    metadata.insert("record_type".to_string(), outcome.record_type.clone());
    if !outcome.source_type.is_empty() {
        metadata.insert("source_type".to_string(), outcome.source_type.clone());
    }
    append_memory_governance_metadata(&mut metadata, outcome);
    metadata.insert("title".to_string(), outcome.title.clone());
    metadata.insert("reason".to_string(), outcome.reason.clone());
    make_event(
        request,
        sequence,
        outcome.event_type,
        "Finish",
        &outcome.title,
        &outcome.summary,
        metadata,
    )
}

fn append_memory_governance_metadata(
    metadata: &mut BTreeMap<String, String>,
    outcome: &crate::memory_router::MemoryWriteOutcome,
) {
    metadata.insert("memory_kind".to_string(), outcome.record_type.clone());
    metadata.insert("governance_status".to_string(), outcome.audit.governance_status.clone());
    metadata.insert("memory_action".to_string(), outcome.audit.memory_action.clone());
    metadata.insert("governance_version".to_string(), outcome.audit.governance_version.clone());
    metadata.insert("governance_reason".to_string(), outcome.audit.governance_reason.clone());
    metadata.insert("governance_source".to_string(), outcome.audit.governance_source.clone());
    metadata.insert("governance_at".to_string(), outcome.audit.governance_at.clone());
    metadata.insert("source_event_type".to_string(), outcome.audit.source_event_type.clone());
    metadata.insert("source_artifact_path".to_string(), outcome.audit.source_artifact_path.clone());
    metadata.insert("archive_reason".to_string(), outcome.audit.archive_reason.clone());
}

fn derive_task_title(action: &PlannedAction, user_input: &str) -> String {
    match action {
        PlannedAction::RunCommand { command } => {
            format!("执行命令: {}", truncate_text(command, 42))
        }
        PlannedAction::ReadFile { path } => format!("读取文件: {}", path),
        PlannedAction::WriteFile { path, .. } => format!("写入文件: {}", path),
        PlannedAction::DeletePath { path } => format!("删除路径: {}", path),
        PlannedAction::ListFiles { path } => match path.as_deref() {
            Some(value) if !value.is_empty() => format!("浏览目录: {}", value),
            _ => "浏览工作区目录".to_string(),
        },
        PlannedAction::WriteMemory { summary, .. } => {
            format!("写入记忆: {}", truncate_text(summary, 42))
        }
        PlannedAction::RecallMemory { query } => {
            format!("召回记忆: {}", truncate_text(query, 42))
        }
        PlannedAction::SearchKnowledge { query } => {
            format!("检索知识: {}", truncate_text(query, 42))
        }
        PlannedAction::SearchSiyuanNotes { query } => {
            format!("检索思源: {}", truncate_text(query, 42))
        }
        PlannedAction::ReadSiyuanNote { path } => {
            format!("读取思源: {}", truncate_text(path, 42))
        }
        PlannedAction::WriteSiyuanKnowledge => "导出知识到思源".to_string(),
        PlannedAction::ProjectAnswer => {
            format!("项目问答: {}", truncate_text(user_input, 42))
        }
        PlannedAction::ContextAnswer => "延续当前会话".to_string(),
        PlannedAction::Explain => format!("解释可用能力: {}", truncate_text(user_input, 42)),
        PlannedAction::AgentResolve => format!("智能体执行: {}", truncate_text(user_input, 42)),
    }
}

fn truncate_text(input: &str, limit: usize) -> String {
    let mut chars = input.chars();
    let truncated: String = chars.by_ref().take(limit).collect();
    if chars.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
}
