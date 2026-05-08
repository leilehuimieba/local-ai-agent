use crate::capabilities::ToolExecutionTrace;

use super::VerificationOutcome;
use super::evidence::{guard_decision_ref, guard_downgraded};

pub(super) fn verify_knowledge_answer(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    let has_citation = has_citation(trace);
    let fact_inference_split = has_fact_inference_split(trace);
    let permission_ok = !guard_downgraded(trace);
    if knowledge_answer_insufficient(trace, &evidence, has_citation, fact_inference_split, permission_ok) {
        return knowledge_failure(
            trace,
            policy,
            evidence,
            has_citation,
            fact_inference_split,
            permission_ok,
        );
    }
    knowledge_success(trace, policy, evidence, has_citation, fact_inference_split)
}

fn knowledge_answer_insufficient(
    trace: &ToolExecutionTrace,
    evidence: &[String],
    has_citation: bool,
    fact_inference_split: bool,
    permission_ok: bool,
) -> bool {
    !trace.result.success || !has_citation || evidence.len() < 2 || !fact_inference_split || !permission_ok
}

fn knowledge_failure(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
    has_citation: bool,
    fact_inference_split: bool,
    permission_ok: bool,
) -> VerificationOutcome {
    let mut outcome = base_knowledge_outcome(policy, evidence, has_citation, fact_inference_split);
    outcome.code = "knowledge_answer_insufficient".to_string();
    outcome.permission_boundary_respected = permission_ok;
    outcome.skill_hit_reason = "当前知识回答证据不足，不能按可交付答案收口。".to_string();
    outcome.guard_downgraded = guard_downgraded(trace);
    outcome.guard_decision_ref = guard_decision_ref(trace);
    outcome.summary = "知识回答验证未通过：缺少足够引证、事实/推断边界或风险边界信号。".to_string();
    outcome.next_step = "建议优先 replan 补充检索与引证；若仍不足，进入 handoff。".to_string();
    outcome
}

fn knowledge_success(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
    has_citation: bool,
    fact_inference_split: bool,
) -> VerificationOutcome {
    let mut outcome = base_knowledge_outcome(policy, evidence, has_citation, fact_inference_split);
    outcome.passed = true;
    outcome.code = "verified".to_string();
    outcome.permission_boundary_respected = true;
    outcome.skill_hit_effective = true;
    outcome.skill_hit_reason = "当前知识回答已形成可引证证据，并区分了事实、推断与建议。".to_string();
    outcome.guard_decision_ref = guard_decision_ref(trace);
    outcome.summary = "知识回答验证通过：已具备最小引证、证据数量与边界说明。".to_string();
    outcome.next_step = "当前知识回答已满足收口条件，可直接完成答复。".to_string();
    outcome
}

fn base_knowledge_outcome(
    policy: &str,
    evidence: Vec<String>,
    has_citation: bool,
    fact_inference_split: bool,
) -> VerificationOutcome {
    let evidence_count = evidence.len();
    VerificationOutcome {
        passed: false,
        code: String::new(),
        browser_failure_type: String::new(),
        browser_page_id: String::new(),
        browser_selector: String::new(),
        browser_recovery_attempted: false,
        browser_recovery_exhausted: false,
        policy: policy.to_string(),
        task_type: "knowledge_answer".to_string(),
        evidence,
        evidence_count,
        has_citation,
        fact_inference_split,
        capability_risk_checked: true,
        permission_boundary_respected: false,
        skill_hit_effective: false,
        skill_hit_reason: String::new(),
        guard_downgraded: false,
        guard_decision_ref: String::new(),
        summary: String::new(),
        next_step: String::new(),
    }
}

fn has_citation(trace: &ToolExecutionTrace) -> bool {
    trace.result.summary.contains("知识引证：") && !trace.result.summary.contains("知识引证：未提供")
        || trace.result.final_answer.contains("docs/")
}

fn has_fact_inference_split(trace: &ToolExecutionTrace) -> bool {
    let text = format!("{} {}", trace.result.summary, trace.result.reasoning_summary);
    text.contains("事实") && text.contains("推断") && (text.contains("建议") || text.contains("下一步"))
}
