use crate::capabilities::ToolExecutionTrace;

use super::evidence::{guard_decision_ref, guard_downgraded};
use super::{VerificationOutcome, used_recovery};

pub(super) fn verify_browser_interaction(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    let permission_ok = !guard_downgraded(trace);
    let state_visible = browser_state_visible(trace);
    let target_visible = browser_target_visible(trace);
    let risk_visible = browser_risk_trace_visible(trace);
    let evidence_count = evidence.len();
    if browser_interaction_failed(
        trace,
        permission_ok,
        state_visible,
        target_visible,
        risk_visible,
        evidence_count,
    ) {
        return browser_interaction_failed_outcome(trace, policy, evidence, evidence_count, permission_ok);
    }
    browser_interaction_passed_outcome(trace, policy, evidence, evidence_count)
}

fn browser_state_visible(trace: &ToolExecutionTrace) -> bool {
    let text = browser_output_text(trace);
    browser_page_visible(&text) || browser_interaction_effect_visible(&text)
}

fn browser_target_visible(trace: &ToolExecutionTrace) -> bool {
    let text = browser_output_text(trace);
    text.contains("\"page_id\":")
        && (text.contains("\"selector\":") || text.contains("\"url\":") || text.contains("\"final_url\":"))
}

fn browser_risk_trace_visible(trace: &ToolExecutionTrace) -> bool {
    let text = format!("{} {}", trace.result.reasoning_summary, trace.result.summary);
    text.contains("allowlist") || text.contains("audit") || text.contains("审计") || text.contains("confirmation")
}

fn browser_output_text(trace: &ToolExecutionTrace) -> String {
    format!("{} {}", trace.result.detail_preview, trace.result.final_answer)
}

fn browser_interaction_failed(
    trace: &ToolExecutionTrace,
    permission_ok: bool,
    state_visible: bool,
    target_visible: bool,
    risk_visible: bool,
    evidence_count: usize,
) -> bool {
    !trace.result.success || !permission_ok || !state_visible || !target_visible || !risk_visible || evidence_count < 2
}

fn browser_interaction_failed_outcome(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
    evidence_count: usize,
    permission_ok: bool,
) -> VerificationOutcome {
    let browser_failure_type = browser_failure_type(trace, permission_ok);
    let browser_recovery_attempted = used_recovery(trace);
    let browser_recovery_exhausted = browser_recovery_attempted;
    let mut outcome = base_browser_outcome(trace, policy, evidence, evidence_count);
    outcome.code = "browser_interaction_insufficient".to_string();
    outcome.browser_failure_type = browser_failure_type.clone();
    outcome.browser_recovery_attempted = browser_recovery_attempted;
    outcome.browser_recovery_exhausted = browser_recovery_exhausted;
    outcome.permission_boundary_respected = permission_ok;
    outcome.skill_hit_effective = trace.result.success;
    outcome.skill_hit_reason =
        "当前浏览器交互缺少页面状态变化、目标元素或审计边界信号，不能按已验证交互收口。".to_string();
    outcome.guard_downgraded = guard_downgraded(trace);
    outcome.summary = browser_failure_summary(&browser_failure_type, browser_recovery_exhausted);
    outcome.next_step = browser_failure_next_step(browser_recovery_exhausted);
    outcome
}

fn browser_interaction_passed_outcome(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
    evidence_count: usize,
) -> VerificationOutcome {
    let mut outcome = base_browser_outcome(trace, policy, evidence, evidence_count);
    outcome.passed = true;
    outcome.code = "verified".to_string();
    outcome.fact_inference_split = true;
    outcome.permission_boundary_respected = true;
    outcome.skill_hit_effective = true;
    outcome.skill_hit_reason = "当前浏览器交互已具备页面状态变化、目标元素与审计边界信号。".to_string();
    outcome.summary = "浏览器交互验证通过：已具备最小页面变化与边界证据。".to_string();
    outcome.next_step = "当前浏览器交互已满足最小收口条件，可继续后续答复或下一步动作。".to_string();
    outcome
}

fn base_browser_outcome(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
    evidence_count: usize,
) -> VerificationOutcome {
    VerificationOutcome {
        passed: false,
        code: String::new(),
        browser_failure_type: String::new(),
        browser_page_id: browser_page_id(trace),
        browser_selector: browser_selector(trace),
        browser_recovery_attempted: false,
        browser_recovery_exhausted: false,
        policy: policy.to_string(),
        task_type: "browser_interaction".to_string(),
        evidence,
        evidence_count,
        has_citation: false,
        fact_inference_split: false,
        capability_risk_checked: true,
        permission_boundary_respected: false,
        skill_hit_effective: false,
        skill_hit_reason: String::new(),
        guard_downgraded: false,
        guard_decision_ref: guard_decision_ref(trace),
        summary: String::new(),
        next_step: String::new(),
    }
}

fn browser_failure_type(trace: &ToolExecutionTrace, permission_ok: bool) -> String {
    if !permission_ok || !browser_risk_trace_visible(trace) {
        return "permission_or_risk_gap".to_string();
    }
    if !browser_target_visible(trace) {
        return "target_not_found".to_string();
    }
    if used_recovery(trace) {
        return "recovery_exhausted".to_string();
    }
    "state_not_changed".to_string()
}

fn browser_failure_summary(failure_type: &str, recovery_exhausted: bool) -> String {
    let detail = match failure_type {
        "permission_or_risk_gap" => "缺少 allowlist、审计或确认边界信号。",
        "target_not_found" => "未保留 page_id、selector 或目标定位信号。",
        "recovery_exhausted" => "完成一次 read_page 回读后仍未形成可验证状态变化。",
        _ => "动作执行后未观察到足够页面状态变化。",
    };
    if recovery_exhausted {
        return format!("浏览器交互验证未通过：{detail} 已停止自动续跑并等待交接。");
    }
    format!("浏览器交互验证未通过：{detail}")
}

fn browser_failure_next_step(recovery_exhausted: bool) -> String {
    if recovery_exhausted {
        return "已完成单次 read_page 回读仍未通过验证，建议交给人工继续检查页面状态或调整目标元素。".to_string();
    }
    "建议先补充 read_page 或页面回读结果，再决定是否继续交互。".to_string()
}

fn browser_page_id(trace: &ToolExecutionTrace) -> String {
    browser_argument_value(trace, "page_id")
}

fn browser_selector(trace: &ToolExecutionTrace) -> String {
    browser_argument_value(trace, "selector")
}

fn browser_argument_value(trace: &ToolExecutionTrace, field: &str) -> String {
    let marker = format!("\"{field}\":\"");
    let text = browser_output_text(trace);
    text.split(&marker)
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .unwrap_or_default()
        .to_string()
}

fn browser_page_visible(text: &str) -> bool {
    text.contains("\"loaded\":true") || text.contains("\"content\":")
}

fn browser_interaction_effect_visible(text: &str) -> bool {
    text.contains("\"clicked\":true")
        || text.contains("\"typed_char_count\":")
        || text.contains("\"selected_value\":")
        || text.contains("\"submitted\":true")
        || text.contains("\"uploaded_file_count\":")
        || text.contains("\"uploaded_files\":")
}
