use crate::compaction::compact_session_turns;
use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::paths::{daily_rollup_path, session_file_path};
use crate::risk::RiskOutcome;
use crate::storage::append_jsonl;
use crate::text::summarize_text;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub(crate) struct SessionTurn {
    pub user_input: String,
    pub final_answer: String,
    pub summary: String,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub(crate) struct ShortTermMemory {
    pub current_goal: String,
    pub current_plan: String,
    pub open_issue: String,
    pub recent_observation: String,
    pub recent_tool_result: String,
    pub pending_confirmation: String,
    pub current_phase: String,
    pub last_run_status: String,
    pub handoff_artifact_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub(crate) struct SessionMemory {
    pub session_id: String,
    pub compressed_summary: String,
    #[serde(default)]
    pub short_term: ShortTermMemory,
    pub recent_turns: Vec<SessionTurn>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DailyRollupEntry {
    run_id: String,
    session_id: String,
    workspace_id: String,
    user_input: String,
    summary: String,
    status: String,
    timestamp: String,
}

pub(crate) fn load_session_context(request: &RunRequest) -> SessionMemory {
    let path = session_file_path(request);
    if !path.exists() {
        return empty_session(request);
    }

    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str::<SessionMemory>(&raw).ok())
        .unwrap_or_else(|| empty_session(request))
}

pub(crate) fn persist_session_outputs(
    request: &RunRequest,
    final_answer: &str,
    summary: &str,
    status: &str,
) {
    let mut session = load_session_context(request);
    append_turn(&mut session, request, final_answer, summary);
    update_finish_memory(&mut session, final_answer, summary, status);
    persist_session_file(request, &session);
    persist_daily_rollup(request, summary, status);
}

pub(crate) fn persist_handoff_path(request: &RunRequest, handoff_path: &str) {
    let mut session = load_session_context(request);
    session.short_term.current_plan = summarize_text("已生成长任务交接包");
    session.short_term.recent_tool_result = summarize_text(handoff_path);
    session.short_term.current_phase = "handoff".to_string();
    session.short_term.handoff_artifact_path = handoff_path.to_string();
    persist_session_file(request, &session);
}

pub(crate) fn record_planning_memory(
    request: &RunRequest,
    session: &mut SessionMemory,
    task_title: &str,
    analysis_detail: &str,
    risk_outcome: &RiskOutcome,
) {
    session.session_id = request.session_id.clone();
    session.short_term.current_goal = summarize_text(&request.user_input);
    session.short_term.current_plan = plan_label(task_title, risk_outcome);
    session.short_term.recent_observation = summarize_text(analysis_detail);
    session.short_term.recent_tool_result.clear();
    session.short_term.current_phase = planning_phase(risk_outcome);
    session.short_term.last_run_status = "planning".to_string();
    session.short_term.handoff_artifact_path.clear();
    apply_risk_state(&mut session.short_term, risk_outcome);
    persist_session_file(request, session);
}

pub(crate) fn record_execution_memory(
    request: &RunRequest,
    session: &mut SessionMemory,
    summary: &str,
    final_answer: &str,
    success: bool,
) {
    session.short_term.current_plan = execution_plan_label(success);
    session.short_term.recent_tool_result = summarize_text(summary);
    session.short_term.recent_observation = summarize_text(final_answer);
    session.short_term.open_issue = failure_issue(final_answer, success);
    session.short_term.pending_confirmation.clear();
    session.short_term.current_phase = "observe".to_string();
    session.short_term.last_run_status = execution_status(success);
    session.short_term.handoff_artifact_path.clear();
    persist_session_file(request, session);
}

pub(crate) fn session_prompt_summary(session: &SessionMemory) -> String {
    let mut parts = short_term_parts(&session.short_term);
    if !session.compressed_summary.is_empty() {
        parts.push(format!("会话摘要：{}", session.compressed_summary));
    }
    if parts.is_empty() {
        return "当前会话还没有可复用的压缩摘要。".to_string();
    }
    summarize_text(&parts.join(" || "))
}

fn build_compressed_summary(turns: &[SessionTurn]) -> String {
    compact_session_turns(turns).summary
}

fn append_turn(
    session: &mut SessionMemory,
    request: &RunRequest,
    final_answer: &str,
    summary: &str,
) {
    session.session_id = request.session_id.clone();
    session.recent_turns.push(SessionTurn {
        user_input: request.user_input.clone(),
        final_answer: final_answer.to_string(),
        summary: summary.to_string(),
        timestamp: timestamp_now(),
    });
    while session.recent_turns.len() > 6 {
        session.recent_turns.remove(0);
    }
    session.compressed_summary = build_compressed_summary(&session.recent_turns);
}

fn persist_session_file(request: &RunRequest, session: &SessionMemory) {
    let session_path = session_file_path(request);
    if let Some(parent) = session_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(raw) = serde_json::to_string_pretty(session) {
        let _ = fs::write(&session_path, raw);
    }
}

fn persist_daily_rollup(request: &RunRequest, summary: &str, status: &str) {
    let rollup = DailyRollupEntry {
        run_id: request.run_id.clone(),
        session_id: request.session_id.clone(),
        workspace_id: request.workspace_ref.workspace_id.clone(),
        user_input: summarize_text(&request.user_input),
        summary: summarize_text(summary),
        status: status.to_string(),
        timestamp: timestamp_now(),
    };
    let _ = append_jsonl(daily_rollup_path(request), &rollup);
}

fn empty_session(request: &RunRequest) -> SessionMemory {
    SessionMemory {
        session_id: request.session_id.clone(),
        compressed_summary: String::new(),
        short_term: ShortTermMemory::default(),
        recent_turns: Vec::new(),
    }
}

fn plan_label(task_title: &str, risk_outcome: &RiskOutcome) -> String {
    match risk_outcome {
        RiskOutcome::Blocked(_) => "当前动作被模式策略阻止".to_string(),
        RiskOutcome::RequireConfirmation(_) => "等待人工确认后继续".to_string(),
        RiskOutcome::Proceed => format!("准备执行：{}", summarize_text(task_title)),
    }
}

fn planning_phase(risk_outcome: &RiskOutcome) -> String {
    match risk_outcome {
        RiskOutcome::Blocked(_) => "blocked".to_string(),
        RiskOutcome::RequireConfirmation(_) => "confirmation".to_string(),
        RiskOutcome::Proceed => "plan".to_string(),
    }
}

fn apply_risk_state(short_term: &mut ShortTermMemory, risk_outcome: &RiskOutcome) {
    match risk_outcome {
        RiskOutcome::Blocked(message) => {
            short_term.open_issue = summarize_text(message);
            short_term.pending_confirmation.clear();
        }
        RiskOutcome::RequireConfirmation(confirmation) => {
            short_term.open_issue.clear();
            short_term.pending_confirmation = summarize_text(&confirmation.action_summary);
        }
        RiskOutcome::Proceed => {
            short_term.open_issue.clear();
            short_term.pending_confirmation.clear();
        }
    }
}

fn execution_plan_label(success: bool) -> String {
    if success {
        "等待验证与结果收口".to_string()
    } else {
        "处理执行失败并准备收口".to_string()
    }
}

fn execution_status(success: bool) -> String {
    if success {
        "executed".to_string()
    } else {
        "failed".to_string()
    }
}

fn failure_issue(final_answer: &str, success: bool) -> String {
    if success {
        String::new()
    } else {
        summarize_text(final_answer)
    }
}

fn update_finish_memory(
    session: &mut SessionMemory,
    final_answer: &str,
    summary: &str,
    status: &str,
) {
    session.short_term.current_plan = finish_plan_label(status);
    session.short_term.recent_tool_result = summarize_text(summary);
    session.short_term.recent_observation = summarize_text(final_answer);
    session.short_term.open_issue = failure_issue(final_answer, status == "completed");
    session.short_term.pending_confirmation.clear();
    session.short_term.current_phase = "finish".to_string();
    session.short_term.last_run_status = status.to_string();
}

fn finish_plan_label(status: &str) -> String {
    if status == "completed" {
        "当前任务已完成".to_string()
    } else {
        "当前任务已失败，等待下一步处理".to_string()
    }
}

fn short_term_parts(short_term: &ShortTermMemory) -> Vec<String> {
    let mut parts = Vec::new();
    push_part(&mut parts, "当前目标", &short_term.current_goal);
    push_part(&mut parts, "当前计划", &short_term.current_plan);
    push_part(&mut parts, "当前阶段", &short_term.current_phase);
    push_part(&mut parts, "最近状态", &short_term.last_run_status);
    push_part(&mut parts, "阻塞问题", &short_term.open_issue);
    push_part(&mut parts, "最近观察", &short_term.recent_observation);
    push_part(&mut parts, "最近工具结果", &short_term.recent_tool_result);
    push_part(&mut parts, "待确认事项", &short_term.pending_confirmation);
    push_part(&mut parts, "交接包", &short_term.handoff_artifact_path);
    parts
}

fn push_part(parts: &mut Vec<String>, label: &str, value: &str) {
    if !value.is_empty() {
        parts.push(format!("{label}：{value}"));
    }
}
