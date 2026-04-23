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
    session.short_term.current_plan = planning_plan(request, task_title, risk_outcome, session);
    session.short_term.recent_observation = summarize_text(analysis_detail);
    session.short_term.recent_tool_result = planning_tool_result(request, session);
    session.short_term.current_phase = planning_phase(request, risk_outcome, session);
    session.short_term.last_run_status = planning_status(request, session);
    session.short_term.handoff_artifact_path = planning_handoff_path(request, session);
    apply_planning_risk_state(request, &mut session.short_term, risk_outcome);
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
    parts.join(" || ")
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

fn planning_phase(
    request: &RunRequest,
    risk_outcome: &RiskOutcome,
    session: &SessionMemory,
) -> String {
    if should_preserve_resume_state(request) {
        return session.short_term.current_phase.clone();
    }
    match risk_outcome {
        RiskOutcome::Blocked(_) => "blocked".to_string(),
        RiskOutcome::RequireConfirmation(_) => "confirmation".to_string(),
        RiskOutcome::Proceed => "plan".to_string(),
    }
}

fn planning_plan(
    request: &RunRequest,
    task_title: &str,
    risk_outcome: &RiskOutcome,
    session: &SessionMemory,
) -> String {
    if should_preserve_resume_state(request) {
        return session.short_term.current_plan.clone();
    }
    plan_label(task_title, risk_outcome)
}

fn planning_tool_result(request: &RunRequest, session: &SessionMemory) -> String {
    if should_preserve_resume_state(request) {
        return session.short_term.recent_tool_result.clone();
    }
    String::new()
}

fn planning_status(request: &RunRequest, session: &SessionMemory) -> String {
    if should_preserve_resume_state(request) {
        return session.short_term.last_run_status.clone();
    }
    "planning".to_string()
}

fn planning_handoff_path(request: &RunRequest, session: &SessionMemory) -> String {
    if should_preserve_resume_state(request) {
        return session.short_term.handoff_artifact_path.clone();
    }
    String::new()
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

fn apply_planning_risk_state(
    request: &RunRequest,
    short_term: &mut ShortTermMemory,
    risk_outcome: &RiskOutcome,
) {
    if should_keep_recovery_issue(request, short_term, risk_outcome) {
        short_term.pending_confirmation.clear();
        return;
    }
    apply_risk_state(short_term, risk_outcome);
}

fn should_keep_recovery_issue(
    request: &RunRequest,
    short_term: &ShortTermMemory,
    risk_outcome: &RiskOutcome,
) -> bool {
    should_preserve_resume_state(request)
        && short_term.current_phase == "recovery"
        && matches!(risk_outcome, RiskOutcome::Proceed)
}

fn should_preserve_resume_state(request: &RunRequest) -> bool {
    !request.resume_from_checkpoint_id.trim().is_empty()
        && !request.resume_strategy.trim().is_empty()
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

#[cfg(test)]
mod tests {
    use super::{SessionMemory, record_planning_memory, session_prompt_summary};
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use crate::risk::RiskOutcome;
    use std::collections::BTreeMap;

    #[test]
    fn preserves_confirmation_resume_state_during_planning_writeback() {
        let request = sample_request("after_confirmation");
        let mut session = sample_session("confirmation_resume", "awaiting_confirmation");
        session.short_term.current_plan =
            "从 checkpoint 恢复：confirmation_required -> PausedForConfirmation".to_string();
        session.short_term.recent_tool_result = "等待确认后继续".to_string();
        record_planning_memory(
            &request,
            &mut session,
            "执行命令",
            "分析结果",
            &RiskOutcome::Proceed,
        );
        assert_eq!(session.short_term.current_phase, "confirmation_resume");
        assert_eq!(session.short_term.last_run_status, "awaiting_confirmation");
        assert_eq!(
            session.short_term.current_plan,
            "从 checkpoint 恢复：confirmation_required -> PausedForConfirmation"
        );
        assert_eq!(session.short_term.recent_tool_result, "等待确认后继续");
    }

    #[test]
    fn preserves_recovery_issue_during_retry_planning_writeback() {
        let request = sample_request("retry_failure");
        let mut session = sample_session("recovery", "failed");
        session.short_term.current_plan =
            "从 checkpoint 恢复：retryable_failure -> Execute".to_string();
        session.short_term.open_issue = "temporary failure".to_string();
        session.short_term.recent_tool_result = "temporary failure".to_string();
        record_planning_memory(
            &request,
            &mut session,
            "执行命令",
            "分析结果",
            &RiskOutcome::Proceed,
        );
        assert_eq!(session.short_term.current_phase, "recovery");
        assert_eq!(session.short_term.last_run_status, "failed");
        assert_eq!(session.short_term.open_issue, "temporary failure");
        assert_eq!(
            session.short_term.current_plan,
            "从 checkpoint 恢复：retryable_failure -> Execute"
        );
    }

    #[test]
    fn preserves_handoff_path_during_retry_planning_writeback() {
        let request = sample_request("retry_failure");
        let mut session = sample_session("recovery", "failed");
        session.short_term.handoff_artifact_path = "D:/repo/handoff.json".to_string();
        record_planning_memory(
            &request,
            &mut session,
            "执行命令",
            "分析结果",
            &RiskOutcome::Proceed,
        );
        assert_eq!(
            session.short_term.handoff_artifact_path,
            "D:/repo/handoff.json"
        );
    }

    #[test]
    fn keeps_compaction_boundary_hint_visible_in_session_prompt_summary() {
        let mut session = SessionMemory::default();
        session.compressed_summary = format!(
            "{}边界提示：已省略更早 2 轮（聚合预算 900 字符）。",
            "前文".repeat(180)
        );
        let summary = session_prompt_summary(&session);
        assert!(summary.contains("边界提示"));
        assert!(summary.contains("聚合预算 900 字符"));
    }

    fn sample_request(strategy: &str) -> RunRequest {
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "cmd: echo test".to_string(),
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

    fn sample_session(phase: &str, status: &str) -> SessionMemory {
        let mut session = SessionMemory::default();
        session.short_term.current_phase = phase.to_string();
        session.short_term.last_run_status = status.to_string();
        session
    }
}
