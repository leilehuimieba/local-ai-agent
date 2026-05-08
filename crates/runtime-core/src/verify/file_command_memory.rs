use crate::capabilities::ToolExecutionTrace;

use super::VerificationOutcome;
use super::evidence::{guard_decision_ref, guard_downgraded};

pub(super) fn verify_file_change(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    let permission_ok = !guard_downgraded(trace);
    let ready = trace.result.success && permission_ok && file_change_path_visible(trace);
    if !ready || !file_change_effect_visible(trace) || evidence.len() < 2 {
        return file_change_failure(trace, policy, evidence, permission_ok);
    }
    file_change_success(trace, policy, evidence)
}

pub(super) fn verify_command_execution(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    let permission_ok = !guard_downgraded(trace);
    let ready = trace.result.success && permission_ok && command_output_visible(trace);
    if !ready || !command_artifact_visible(trace) || evidence.len() < 2 {
        return command_failure(trace, policy, evidence, permission_ok);
    }
    command_success(trace, policy, evidence)
}

pub(super) fn verify_memory_write(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    let permission_ok = !guard_downgraded(trace);
    let ready = trace.result.success && permission_ok && memory_write_summary_visible(trace);
    if !ready || !memory_write_final_visible(trace) || evidence.len() < 2 {
        return memory_failure(trace, policy, evidence, permission_ok);
    }
    memory_success(trace, policy, evidence)
}

fn file_change_failure(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
    permission_ok: bool,
) -> VerificationOutcome {
    let mut outcome = base_outcome(policy, "file_change", evidence);
    outcome.code = "file_change_insufficient".to_string();
    outcome.permission_boundary_respected = permission_ok;
    outcome.skill_hit_reason = "当前文件变更缺少目标路径或变更效果信号，不能按可交付修改收口。".to_string();
    outcome.guard_downgraded = guard_downgraded(trace);
    outcome.guard_decision_ref = guard_decision_ref(trace);
    outcome.summary = "文件变更验证未通过：缺少路径、预览或删除完成等最小效果证据。".to_string();
    outcome.next_step = "建议先补充 dry-run、目标路径确认或变更摘要，再决定是否继续。".to_string();
    outcome
}

fn file_change_success(trace: &ToolExecutionTrace, policy: &str, evidence: Vec<String>) -> VerificationOutcome {
    let mut outcome = base_outcome(policy, "file_change", evidence);
    outcome.passed = true;
    outcome.code = "verified".to_string();
    outcome.fact_inference_split = true;
    outcome.permission_boundary_respected = true;
    outcome.skill_hit_effective = true;
    outcome.skill_hit_reason = "当前文件变更已具备路径、效果摘要与风险边界信号。".to_string();
    outcome.guard_decision_ref = guard_decision_ref(trace);
    outcome.summary = "文件变更验证通过：已具备最小路径与效果证据。".to_string();
    outcome.next_step = "当前文件变更已满足最小收口条件，可进入后续答复或下一步。".to_string();
    outcome
}

fn command_failure(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
    permission_ok: bool,
) -> VerificationOutcome {
    let mut outcome = base_outcome(policy, "command_execution", evidence);
    outcome.code = "command_execution_insufficient".to_string();
    outcome.permission_boundary_respected = permission_ok;
    outcome.skill_hit_effective = trace.result.success;
    outcome.skill_hit_reason = "当前命令执行缺少输出、错误或产物路径信号，不能按可交付执行结果收口。".to_string();
    outcome.guard_downgraded = guard_downgraded(trace);
    outcome.guard_decision_ref = guard_decision_ref(trace);
    outcome.summary = "命令执行验证未通过：缺少输出摘要、错误信号或原始产物引用。".to_string();
    outcome.next_step = "建议先检查 exit code、stderr 摘要与原始输出产物，再决定是否重试。".to_string();
    outcome
}

fn command_success(trace: &ToolExecutionTrace, policy: &str, evidence: Vec<String>) -> VerificationOutcome {
    let mut outcome = base_outcome(policy, "command_execution", evidence);
    outcome.passed = true;
    outcome.code = "verified".to_string();
    outcome.fact_inference_split = true;
    outcome.permission_boundary_respected = true;
    outcome.skill_hit_effective = true;
    outcome.skill_hit_reason = "当前命令执行已具备输出摘要、原始产物引用与风险边界信号。".to_string();
    outcome.guard_decision_ref = guard_decision_ref(trace);
    outcome.summary = "命令执行验证通过：已具备最小输出与产物证据。".to_string();
    outcome.next_step = "当前命令执行已满足最小收口条件，可继续后续判断。".to_string();
    outcome
}

fn memory_failure(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
    permission_ok: bool,
) -> VerificationOutcome {
    let mut outcome = base_outcome(policy, "memory_write", evidence);
    outcome.code = "memory_write_insufficient".to_string();
    outcome.permission_boundary_respected = permission_ok;
    outcome.skill_hit_effective = trace.result.success;
    outcome.skill_hit_reason = "当前记忆写入缺少写入摘要或类型信号，不能按可复用沉淀收口。".to_string();
    outcome.guard_downgraded = guard_downgraded(trace);
    outcome.guard_decision_ref = guard_decision_ref(trace);
    outcome.summary = "记忆写入验证未通过：缺少写入摘要、类型或最终答复信号。".to_string();
    outcome.next_step = "建议先确认 write 成功摘要，再决定是否继续沉淀或回读。".to_string();
    outcome
}

fn memory_success(trace: &ToolExecutionTrace, policy: &str, evidence: Vec<String>) -> VerificationOutcome {
    let mut outcome = base_outcome(policy, "memory_write", evidence);
    outcome.passed = true;
    outcome.code = "verified".to_string();
    outcome.fact_inference_split = true;
    outcome.permission_boundary_respected = true;
    outcome.skill_hit_effective = true;
    outcome.skill_hit_reason = "当前记忆写入已具备写入摘要、类型和结果信号。".to_string();
    outcome.guard_decision_ref = guard_decision_ref(trace);
    outcome.summary = "记忆写入验证通过：已具备最小沉淀证据。".to_string();
    outcome.next_step = "当前记忆写入已满足最小收口条件，可继续后续判断。".to_string();
    outcome
}

fn base_outcome(policy: &str, task_type: &str, evidence: Vec<String>) -> VerificationOutcome {
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
        guard_downgraded: false,
        guard_decision_ref: String::new(),
        summary: String::new(),
        next_step: String::new(),
    }
}

fn file_change_path_visible(trace: &ToolExecutionTrace) -> bool {
    let text = format!("{} {}", trace.result.summary, trace.result.final_answer);
    text.contains("写入文件：")
        || text.contains("删除路径：")
        || text.contains("应用 patch：")
        || text.contains("文件写入完成：")
        || text.contains("删除完成：")
}

fn file_change_effect_visible(trace: &ToolExecutionTrace) -> bool {
    let text = format!("{} {}", trace.result.summary, trace.result.final_answer);
    text.contains("文件写入成功")
        || text.contains("目标路径已删除")
        || text.contains("删除完成：")
        || text.contains("dry-run")
        || text.contains("预览")
        || text.contains("patch")
}

fn command_output_visible(trace: &ToolExecutionTrace) -> bool {
    !trace.result.detail_preview.trim().is_empty()
        || trace.result.summary.contains("命令执行成功")
        || trace.result.summary.contains("命令执行失败")
        || trace.result.summary.contains("MCP 工具")
}

fn command_artifact_visible(trace: &ToolExecutionTrace) -> bool {
    trace.result.raw_output_ref.is_some()
        || trace.result.artifact_path.is_some()
        || trace.result.single_result_budget_hit
        || trace.result.final_answer.contains("工作区：")
}

fn memory_write_summary_visible(trace: &ToolExecutionTrace) -> bool {
    trace
        .result
        .memory_write_summary
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty())
}

fn memory_write_final_visible(trace: &ToolExecutionTrace) -> bool {
    trace.result.final_answer.contains("记忆写入完成")
        && (trace.result.final_answer.contains("类型：") || trace.result.final_answer.contains("摘要："))
}
