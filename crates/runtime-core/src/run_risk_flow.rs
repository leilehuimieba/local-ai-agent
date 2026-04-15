use crate::contracts::{
    ConfirmationRequest, ErrorInfo, RunEvent, RunRequest, RunResult, RuntimeRunResponse,
};
use crate::events::{make_confirmation_event, make_event};
use crate::query_engine::RuntimeRunState;
use crate::repo_context::repo_context_metadata;
use crate::risk::RiskOutcome;
use crate::run_finish_events::make_run_failed_event;
use crate::run_metadata::append_context_metadata;
use crate::session::persist_session_outputs;
use std::collections::BTreeMap;

pub(crate) fn handle_risk_outcome(
    request: &RunRequest,
    state: &RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
) -> Option<RuntimeRunResponse> {
    match &state.risk_outcome {
        RiskOutcome::Blocked(message) => {
            Some(blocked_response(request, state, events, sequence, message))
        }
        RiskOutcome::RequireConfirmation(confirmation) => Some(confirmation_response(
            request,
            state,
            events,
            sequence,
            confirmation,
        )),
        RiskOutcome::Proceed => None,
    }
}

fn blocked_response(
    request: &RunRequest,
    state: &RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    message: &str,
) -> RuntimeRunResponse {
    let error = blocked_error();
    push_blocked_events(request, state, events, sequence, message, &error);
    persist_session_outputs(request, message, message, "failed");
    RuntimeRunResponse {
        events: events.clone(),
        result: blocked_result(request, message, &error),
        confirmation_request: None,
    }
}

fn blocked_error() -> ErrorInfo {
    ErrorInfo {
        error_code: "blocked_by_mode".to_string(),
        message: "当前动作被模式策略阻止".to_string(),
        summary: "观察模式不会执行修改性动作。".to_string(),
        retryable: true,
        source: "runtime".to_string(),
        stage: "Verify".to_string(),
        metadata: BTreeMap::new(),
    }
}

fn push_blocked_events(
    request: &RunRequest,
    state: &RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    message: &str,
    error: &ErrorInfo,
) {
    push_blocked_verification_event(request, state, events, sequence, message);
    push_blocked_failure_event(request, state, events, sequence, message, error);
    push_blocked_finish_event(request, state, events, sequence, message);
}

fn push_blocked_verification_event(
    request: &RunRequest,
    state: &RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    message: &str,
) {
    events.push(make_event(
        request,
        *sequence,
        "verification_completed",
        "Verify",
        "当前动作被模式策略阻止",
        message,
        blocked_verify_metadata(state, message),
    ));
    *sequence += 1;
}

fn blocked_verify_metadata(state: &RuntimeRunState, message: &str) -> BTreeMap<String, String> {
    let mut metadata = BTreeMap::from([
        ("task_title".to_string(), state.task_title.clone()),
        ("next_step".to_string(), "生成失败结果".to_string()),
        ("result_mode".to_string(), "system".to_string()),
        ("final_answer".to_string(), message.to_string()),
    ]);
    append_permission_metadata(&mut metadata, state);
    metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    metadata
}

fn push_blocked_failure_event(
    request: &RunRequest,
    state: &RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    message: &str,
    error: &ErrorInfo,
) {
    events.push(make_run_failed_event(
        request,
        *sequence,
        "当前动作被模式策略阻止",
        message,
        error,
        None,
        &state.task_title,
        &state.envelope.repo_context,
    ));
    *sequence += 1;
}

fn push_blocked_finish_event(
    request: &RunRequest,
    state: &RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    message: &str,
) {
    events.push(make_event(
        request,
        *sequence,
        "run_finished",
        "Finish",
        "任务未执行",
        message,
        blocked_finish_metadata(state, message),
    ));
    *sequence += 1;
}

fn blocked_finish_metadata(state: &RuntimeRunState, message: &str) -> BTreeMap<String, String> {
    let mut metadata = BTreeMap::from([
        ("task_title".to_string(), state.task_title.clone()),
        ("next_step".to_string(), "任务已结束".to_string()),
        ("final_answer".to_string(), message.to_string()),
        ("result_mode".to_string(), "system".to_string()),
    ]);
    append_permission_metadata(&mut metadata, state);
    metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    metadata
}

fn blocked_result(request: &RunRequest, message: &str, error: &ErrorInfo) -> RunResult {
    RunResult {
        request_id: request.request_id.clone(),
        run_id: request.run_id.clone(),
        session_id: request.session_id.clone(),
        trace_id: request.trace_id.clone(),
        kind: "run_result".to_string(),
        source: "runtime".to_string(),
        status: "failed".to_string(),
        final_answer: message.to_string(),
        summary: message.to_string(),
        error: Some(error.clone()),
        memory_write_summary: None,
        final_stage: "Finish".to_string(),
        checkpoint_id: None,
        resumable: None,
    }
}

fn confirmation_response(
    request: &RunRequest,
    state: &RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    confirmation: &ConfirmationRequest,
) -> RuntimeRunResponse {
    push_confirmation_plan_event(request, state, events, sequence, confirmation);
    events.push(make_confirmation_event(request, *sequence, confirmation));
    RuntimeRunResponse {
        events: events.clone(),
        result: confirmation_result(request, confirmation),
        confirmation_request: Some(confirmation.clone()),
    }
}

fn push_confirmation_plan_event(
    request: &RunRequest,
    state: &RuntimeRunState,
    events: &mut Vec<RunEvent>,
    sequence: &mut u32,
    confirmation: &ConfirmationRequest,
) {
    let mut metadata = BTreeMap::new();
    metadata.insert("task_title".to_string(), state.task_title.clone());
    metadata.insert("next_step".to_string(), "等待用户确认后再继续".to_string());
    append_permission_metadata(&mut metadata, state);
    metadata.insert(
        "confirmation_chain_step".to_string(),
        "required".to_string(),
    );
    metadata.insert(
        "confirmation_decision_source".to_string(),
        "runtime_risk_gate".to_string(),
    );
    append_context_metadata(&mut metadata, &state.envelope.context_envelope);
    metadata.extend(repo_context_metadata(&state.envelope.repo_context));
    events.push(make_event(
        request,
        *sequence,
        "plan_ready",
        "Plan",
        "已识别需要确认的动作",
        &confirmation.action_summary,
        metadata,
    ));
    *sequence += 1;
}

fn confirmation_result(request: &RunRequest, confirmation: &ConfirmationRequest) -> RunResult {
    RunResult {
        request_id: request.request_id.clone(),
        run_id: request.run_id.clone(),
        session_id: request.session_id.clone(),
        trace_id: request.trace_id.clone(),
        kind: "run_result".to_string(),
        source: "runtime".to_string(),
        status: "awaiting_confirmation".to_string(),
        final_answer: String::new(),
        summary: confirmation.reason.clone(),
        error: Some(confirmation_error(confirmation)),
        memory_write_summary: None,
        final_stage: "PausedForConfirmation".to_string(),
        checkpoint_id: None,
        resumable: Some(true),
    }
}

fn confirmation_error(confirmation: &ConfirmationRequest) -> ErrorInfo {
    ErrorInfo {
        error_code: "risk_confirmation_required".to_string(),
        message: confirmation.reason.clone(),
        summary: "高风险动作需要人工确认。".to_string(),
        retryable: true,
        source: "runtime".to_string(),
        stage: "PausedForConfirmation".to_string(),
        metadata: confirmation_error_metadata(confirmation),
    }
}

fn confirmation_error_metadata(confirmation: &ConfirmationRequest) -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "confirmation_id".to_string(),
            confirmation.confirmation_id.clone(),
        ),
        (
            "confirmation_chain_step".to_string(),
            "required".to_string(),
        ),
        (
            "confirmation_decision_source".to_string(),
            "runtime_risk_gate".to_string(),
        ),
        (
            "permission_decision".to_string(),
            "require_confirmation".to_string(),
        ),
        (
            "permission_rule_layer".to_string(),
            permission_rule_layer_from_confirmation(confirmation).to_string(),
        ),
    ])
}

fn append_permission_metadata(metadata: &mut BTreeMap<String, String>, state: &RuntimeRunState) {
    match &state.risk_outcome {
        RiskOutcome::Blocked(_) => {
            metadata.insert("permission_decision".to_string(), "blocked".to_string());
            metadata.insert(
                "permission_flow_step".to_string(),
                "rule_blocked".to_string(),
            );
            metadata.insert(
                "permission_rule_layer".to_string(),
                "mode_guard".to_string(),
            );
            metadata.insert(
                "confirmation_chain_step".to_string(),
                "rule_blocked".to_string(),
            );
        }
        RiskOutcome::RequireConfirmation(confirmation) => {
            metadata.insert(
                "permission_decision".to_string(),
                "require_confirmation".to_string(),
            );
            metadata.insert(
                "permission_flow_step".to_string(),
                "ask_required".to_string(),
            );
            metadata.insert(
                "permission_rule_layer".to_string(),
                permission_rule_layer_from_confirmation(confirmation).to_string(),
            );
        }
        RiskOutcome::Proceed => {
            metadata.insert("permission_decision".to_string(), "proceed".to_string());
            metadata.insert(
                "permission_flow_step".to_string(),
                "rule_passed".to_string(),
            );
            metadata.insert("permission_rule_layer".to_string(), "none".to_string());
        }
    }
}

fn permission_rule_layer_from_confirmation(confirmation: &ConfirmationRequest) -> &'static str {
    match confirmation.kind.as_str() {
        "workspace_access" => "workspace_guard",
        "high_risk_action" => "high_risk_guard",
        _ => "risk_guard",
    }
}
