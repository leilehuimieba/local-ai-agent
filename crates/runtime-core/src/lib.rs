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
mod run_finish_events;
mod run_metadata;
mod run_state_builder;
mod session;
mod sqlite_store;
mod storage;
mod storage_migration;
mod task_title;
mod text;
mod tool_registry;
mod tool_trace;
// tools 模块已收敛为 capabilities（能力注册与元信息单一事实源）
mod verify;

use crate::checkpoint::{
    checkpoint_resume_event, with_checkpoint_resume_event, with_runtime_checkpoint,
};
use crate::completion::decide_completion;
use crate::events::{make_confirmation_event, make_event, with_runtime_memory_recall_event};
use crate::handoff::persist_handoff_artifact;
use crate::memory_router::evaluate_finish_memory_writes;
use crate::query_engine::{bootstrap_run, execute_stage};
use crate::repo_context::repo_context_metadata;
use crate::risk::RiskOutcome;
use crate::run_finish_events::{make_memory_event, make_run_failed_event, read_result_mode};
use crate::run_metadata::{
    append_context_metadata, append_tool_spec_metadata, append_verification_metadata,
};
use crate::session::{persist_handoff_path, persist_session_outputs};
use crate::task_title::derive_task_title;
use crate::verify::verify_tool_execution;
use std::collections::BTreeMap;

pub use crate::contracts::{
    CapabilityListResponse, CapabilitySpec, ConfirmationDecision, ConfirmationRequest,
    ConnectorListResponse, ConnectorSlotSpec, ErrorInfo, GitCommitSummary, GitSnapshot, ModelRef,
    RUNTIME_NAME, RUNTIME_VERSION, RepoContextSnapshot, RunEvent, RunRequest, RunResult,
    RuntimeRunResponse, RuntimeSnapshot, WorkspaceDocSummary, WorkspaceRef,
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
    let resume_event = checkpoint_resume_event(request);
    let response = simulate_run(request);
    let response = with_runtime_memory_recall_event(request, response);
    let response = with_checkpoint_resume_event(response, resume_event);
    with_runtime_checkpoint(request, response)
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
                        ("result_mode".to_string(), "system".to_string()),
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
                        ("result_mode".to_string(), "system".to_string()),
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
                    checkpoint_id: None,
                    resumable: None,
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
                    checkpoint_id: None,
                    resumable: Some(true),
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
        "result_mode".to_string(),
        read_result_mode(
            &action_result.result.final_answer,
            &completion.status,
            &verification_report.outcome.code,
        )
        .to_string(),
    );
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
    finish_metadata.insert(
        "result_mode".to_string(),
        read_result_mode(
            &action_result.result.final_answer,
            &completion.status,
            &verification_report.outcome.code,
        )
        .to_string(),
    );
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
            checkpoint_id: None,
            resumable: None,
        },
        confirmation_request: None,
    }
}
