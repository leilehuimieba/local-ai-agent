use crate::capabilities::ToolExecutionTrace;
use crate::text::summarize_text;
use crate::tool_registry::ToolCall;

mod browser_interaction;
mod evidence;
mod file_command_memory;
mod knowledge_answer;
mod policy;

use self::browser_interaction::verify_browser_interaction;
use self::evidence::{guard_decision_ref, guard_downgraded, skill_hit_reason, verification_evidence};
use self::file_command_memory::{verify_command_execution, verify_file_change, verify_memory_write};
use self::knowledge_answer::verify_knowledge_answer;
use self::policy::{verification_policy, verification_task_type};

#[derive(Clone, Debug)]
pub(crate) struct VerificationOutcome {
    pub passed: bool,
    pub code: String,
    pub browser_failure_type: String,
    pub browser_page_id: String,
    pub browser_selector: String,
    pub browser_recovery_attempted: bool,
    pub browser_recovery_exhausted: bool,
    pub policy: String,
    pub task_type: String,
    pub evidence: Vec<String>,
    pub evidence_count: usize,
    pub has_citation: bool,
    pub fact_inference_split: bool,
    pub capability_risk_checked: bool,
    pub permission_boundary_respected: bool,
    pub skill_hit_effective: bool,
    pub skill_hit_reason: String,
    pub guard_downgraded: bool,
    pub guard_decision_ref: String,
    pub summary: String,
    pub next_step: String,
}

#[derive(Clone, Debug)]
pub(crate) struct VerificationReport {
    pub outcome: VerificationOutcome,
    pub tool_elapsed_ms: u64,
    pub result_chars: usize,
    pub single_result_budget_chars: usize,
    pub single_result_budget_hit: bool,
}

pub(crate) fn verify_tool_execution(tool_call: &ToolCall, trace: &ToolExecutionTrace) -> VerificationReport {
    let policy = verification_policy(tool_call);
    let evidence = verification_evidence(trace);
    let task_type = verification_task_type(tool_call);
    let outcome = if task_type == "knowledge_answer" {
        verify_knowledge_answer(trace, &policy, evidence)
    } else if task_type == "browser_interaction" {
        verify_browser_interaction(trace, &policy, evidence)
    } else if task_type == "file_change" {
        verify_file_change(trace, &policy, evidence)
    } else if task_type == "command_execution" {
        verify_command_execution(trace, &policy, evidence)
    } else if task_type == "memory_write" {
        verify_memory_write(trace, &policy, evidence)
    } else if !trace.result.success {
        failed_outcome(trace, &policy, &task_type, evidence)
    } else if used_recovery(trace) {
        recovered_outcome(trace, &policy, &task_type, evidence)
    } else {
        passed_outcome(trace, &policy, &task_type, evidence)
    };
    VerificationReport {
        outcome,
        tool_elapsed_ms: trace.result.elapsed_ms,
        result_chars: trace.result.result_chars,
        single_result_budget_chars: trace.result.single_result_budget_chars,
        single_result_budget_hit: trace.result.single_result_budget_hit,
    }
}

fn passed_outcome(
    trace: &ToolExecutionTrace,
    policy: &str,
    task_type: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    let mut outcome = generic_outcome(trace, policy, task_type, evidence);
    outcome.passed = true;
    outcome.code = "verified".to_string();
    outcome.fact_inference_split = true;
    outcome.permission_boundary_respected = !guard_downgraded(trace);
    outcome.skill_hit_effective = trace.result.success;
    outcome.skill_hit_reason = skill_hit_reason(trace, false);
    outcome.summary = success_summary(trace);
    outcome.next_step = success_next_step(trace);
    outcome
}

fn recovered_outcome(
    trace: &ToolExecutionTrace,
    policy: &str,
    task_type: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    let mut outcome = generic_outcome(trace, policy, task_type, evidence);
    outcome.passed = true;
    outcome.code = "verified_with_recovery".to_string();
    outcome.fact_inference_split = true;
    outcome.permission_boundary_respected = !guard_downgraded(trace);
    outcome.skill_hit_effective = trace.result.success;
    outcome.skill_hit_reason = skill_hit_reason(trace, true);
    outcome.summary = recovery_summary(trace);
    outcome.next_step = recovery_next_step(trace);
    outcome
}

fn failed_outcome(
    trace: &ToolExecutionTrace,
    policy: &str,
    task_type: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    let mut outcome = generic_outcome(trace, policy, task_type, evidence);
    outcome.code = "verification_failed".to_string();
    outcome.permission_boundary_respected = !guard_downgraded(trace);
    outcome.skill_hit_reason = skill_hit_reason(trace, false);
    outcome.summary = failure_summary(trace);
    outcome.next_step = failure_next_step(trace);
    outcome
}

fn used_recovery(trace: &ToolExecutionTrace) -> bool {
    trace.result.summary.contains("已执行单次恢复")
}

fn generic_outcome(
    trace: &ToolExecutionTrace,
    policy: &str,
    task_type: &str,
    evidence: Vec<String>,
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
        task_type: task_type.to_string(),
        evidence,
        evidence_count,
        has_citation: false,
        fact_inference_split: false,
        capability_risk_checked: true,
        permission_boundary_respected: false,
        skill_hit_effective: false,
        skill_hit_reason: String::new(),
        guard_downgraded: guard_downgraded(trace),
        guard_decision_ref: guard_decision_ref(trace),
        summary: String::new(),
        next_step: String::new(),
    }
}

fn success_summary(trace: &ToolExecutionTrace) -> String {
    format!(
        "验证通过：{}；执行依据：{}",
        summarize_text(&trace.result.summary),
        summarize_text(&trace.result.reasoning_summary)
    )
}

fn recovery_summary(trace: &ToolExecutionTrace) -> String {
    format!(
        "验证通过（受控恢复）：{}；恢复依据：{}",
        summarize_text(&trace.result.summary),
        summarize_text(&trace.result.reasoning_summary)
    )
}

fn failure_summary(trace: &ToolExecutionTrace) -> String {
    format!(
        "验证失败：{}；失败依据：{}",
        summarize_text(&trace.result.final_answer),
        summarize_text(&trace.result.reasoning_summary)
    )
}

#[cfg(test)]
mod tests;

fn success_next_step(trace: &ToolExecutionTrace) -> String {
    match trace.tool.tool_name.as_str() {
        "workspace_read" | "knowledge_search" | "project_answer" => {
            "可继续追问、复盘结果，或基于当前结论进入下一步执行。".to_string()
        }
        "workspace_write" | "run_command" => "建议先检查产物或输出，再决定是否继续下一步修改。".to_string(),
        _ => "当前动作已验证通过，可继续推进主任务。".to_string(),
    }
}

fn recovery_next_step(trace: &ToolExecutionTrace) -> String {
    if trace.tool.tool_name == "project_answer" || trace.tool.tool_name == "session_context" {
        "当前已走受控恢复路径，建议先确认恢复结果是否足够，再决定是否补充上下文后重试。".to_string()
    } else {
        "当前已通过受控恢复完成收口，建议先检查恢复结果再继续。".to_string()
    }
}

fn failure_next_step(trace: &ToolExecutionTrace) -> String {
    match trace.tool.tool_name.as_str() {
        "workspace_read" => "建议先核对目标路径是否存在且位于工作区内，然后重试读取。".to_string(),
        "workspace_write" => "建议先核对目标路径和父目录状态，再决定是否重试写入。".to_string(),
        "workspace_delete" => "建议先改成读取或列出目标路径，确认影响范围后再继续。".to_string(),
        "run_command" => "建议先检查命令语法、依赖和工作区环境，再决定是否重试执行。".to_string(),
        _ => "建议先查看错误摘要与验证结果，再补上下文或调整动作后继续。".to_string(),
    }
}
