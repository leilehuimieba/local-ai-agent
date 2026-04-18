use crate::capabilities::ToolExecutionTrace;
use crate::text::summarize_text;
use crate::tool_registry::ToolCall;

#[derive(Clone, Debug)]
pub(crate) struct VerificationOutcome {
    pub passed: bool,
    pub code: String,
    pub policy: String,
    pub evidence: Vec<String>,
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

pub(crate) fn verify_tool_execution(
    tool_call: &ToolCall,
    trace: &ToolExecutionTrace,
) -> VerificationReport {
    let policy = verification_policy(tool_call);
    let evidence = verification_evidence(trace);
    let outcome = if !trace.result.success {
        failed_outcome(trace, &policy, evidence)
    } else if used_recovery(trace) {
        recovered_outcome(trace, &policy, evidence)
    } else {
        passed_outcome(trace, &policy, evidence)
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
    evidence: Vec<String>,
) -> VerificationOutcome {
    VerificationOutcome {
        passed: true,
        code: "verified".to_string(),
        policy: policy.to_string(),
        evidence,
        skill_hit_effective: trace.result.success,
        skill_hit_reason: skill_hit_reason(trace, false),
        guard_downgraded: guard_downgraded(trace),
        guard_decision_ref: guard_decision_ref(trace),
        summary: format!(
            "验证通过：{}；执行依据：{}",
            summarize_text(&trace.result.summary),
            summarize_text(&trace.result.reasoning_summary)
        ),
        next_step: success_next_step(trace),
    }
}

fn recovered_outcome(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    VerificationOutcome {
        passed: true,
        code: "verified_with_recovery".to_string(),
        policy: policy.to_string(),
        evidence,
        skill_hit_effective: trace.result.success,
        skill_hit_reason: skill_hit_reason(trace, true),
        guard_downgraded: guard_downgraded(trace),
        guard_decision_ref: guard_decision_ref(trace),
        summary: format!(
            "验证通过（受控恢复）：{}；恢复依据：{}",
            summarize_text(&trace.result.summary),
            summarize_text(&trace.result.reasoning_summary)
        ),
        next_step: recovery_next_step(trace),
    }
}

fn failed_outcome(
    trace: &ToolExecutionTrace,
    policy: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    VerificationOutcome {
        passed: false,
        code: "verification_failed".to_string(),
        policy: policy.to_string(),
        evidence,
        skill_hit_effective: false,
        skill_hit_reason: skill_hit_reason(trace, false),
        guard_downgraded: guard_downgraded(trace),
        guard_decision_ref: guard_decision_ref(trace),
        summary: format!(
            "验证失败：{}；失败依据：{}",
            summarize_text(&trace.result.final_answer),
            summarize_text(&trace.result.reasoning_summary)
        ),
        next_step: failure_next_step(trace),
    }
}

fn used_recovery(trace: &ToolExecutionTrace) -> bool {
    trace.result.summary.contains("已执行单次恢复")
}

fn verification_policy(tool_call: &ToolCall) -> String {
    match tool_call.spec.tool_name.as_str() {
        "workspace_write" => "confirm_write_effect".to_string(),
        "workspace_delete" => "confirm_delete_effect".to_string(),
        "run_command" => "inspect_command_result".to_string(),
        "memory_write" => "confirm_memory_persisted".to_string(),
        "knowledge_search" | "search_siyuan_notes" | "read_siyuan_note" => {
            "check_result_relevance".to_string()
        }
        _ => "check_result_summary".to_string(),
    }
}

fn verification_evidence(trace: &ToolExecutionTrace) -> Vec<String> {
    let mut evidence = vec![format!("summary={}", summarize_text(&trace.result.summary))];
    evidence.push(format!(
        "reasoning={}",
        summarize_text(&trace.result.reasoning_summary)
    ));
    evidence.push(format!("result_chars={}", trace.result.result_chars));
    evidence.push(format!(
        "single_result_budget_chars={}",
        trace.result.single_result_budget_chars
    ));
    evidence.push(format!(
        "single_result_budget_hit={}",
        if trace.result.single_result_budget_hit {
            "true"
        } else {
            "false"
        }
    ));
    if let Some(path) = trace.result.artifact_path.as_ref() {
        evidence.push(format!("artifact={path}"));
    }
    evidence.push(format!("cache_status={}", trace.result.cache_status));
    evidence.push(format!(
        "skill_hit_effective={}",
        if trace.result.success { "true" } else { "false" }
    ));
    evidence.push(format!(
        "guard_downgraded={}",
        if guard_downgraded(trace) { "true" } else { "false" }
    ));
    evidence.push(format!("guard_decision_ref={}", guard_decision_ref(trace)));
    evidence
}

fn skill_hit_reason(trace: &ToolExecutionTrace, recovered: bool) -> String {
    if !trace.result.success {
        return "当前执行未成功，skill 命中未形成有效增益。".to_string();
    }
    if recovered {
        return "当前执行通过受控恢复完成，skill 命中产生部分有效增益。".to_string();
    }
    "当前执行成功，skill 命中对结果形成有效增益。".to_string()
}

fn guard_downgraded(trace: &ToolExecutionTrace) -> bool {
    trace.result
        .reasoning_summary
        .contains("guard downgraded")
        || trace.result.summary.contains("guard downgraded")
}

fn guard_decision_ref(trace: &ToolExecutionTrace) -> String {
    if guard_downgraded(trace) {
        return format!("tool={};decision=review", trace.tool.tool_name);
    }
    format!("tool={};decision=allow", trace.tool.tool_name)
}

#[cfg(test)]
mod tests {
    use super::verify_tool_execution;
    use crate::capabilities::{ToolCallResult, ToolDefinition, ToolExecutionTrace};
    use crate::planner::PlannedAction;
    use crate::tool_registry::ToolCall;

    #[test]
    fn exposes_skill_hit_fields_on_success() {
        let report = verify_tool_execution(&sample_tool_call(), &sample_trace(true, false));
        assert!(report.outcome.skill_hit_effective);
        assert!(!report.outcome.guard_downgraded);
        assert_eq!(report.outcome.guard_decision_ref, "tool=run_command;decision=allow");
    }

    #[test]
    fn exposes_guard_downgrade_fields_when_reasoning_marks_review() {
        let report = verify_tool_execution(&sample_tool_call(), &sample_trace(true, true));
        assert!(report.outcome.skill_hit_effective);
        assert!(report.outcome.guard_downgraded);
        assert_eq!(report.outcome.guard_decision_ref, "tool=run_command;decision=review");
    }

    fn sample_tool_call() -> ToolCall {
        ToolCall {
            action: PlannedAction::RunCommand {
                command: "echo ok".to_string(),
            },
            spec: ToolDefinition {
                tool_name: "run_command".to_string(),
                display_name: "执行命令".to_string(),
                category: "system_command".to_string(),
                risk_level: "high".to_string(),
                input_schema: "command_text".to_string(),
                output_kind: "text_preview".to_string(),
                requires_confirmation: true,
            },
        }
    }

    fn sample_trace(success: bool, downgraded: bool) -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: sample_tool_call().spec.clone(),
            action_summary: "执行 echo ok".to_string(),
            result: ToolCallResult {
                summary: if downgraded {
                    "命令执行成功，guard downgraded".to_string()
                } else {
                    "命令执行成功".to_string()
                },
                final_answer: if success { "ok" } else { "failed" }.to_string(),
                artifact_path: None,
                detail_preview: "preview".to_string(),
                raw_output_ref: None,
                result_chars: 10,
                single_result_budget_chars: 30000,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 10,
                retryable: false,
                success,
                memory_write_summary: None,
                reasoning_summary: if downgraded {
                    "guard downgraded to review".to_string()
                } else {
                    "测试推理".to_string()
                },
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }
}

fn success_next_step(trace: &ToolExecutionTrace) -> String {
    match trace.tool.tool_name.as_str() {
        "workspace_read" | "knowledge_search" | "project_answer" => {
            "可继续追问、复盘结果，或基于当前结论进入下一步执行。".to_string()
        }
        "workspace_write" | "run_command" => {
            "建议先检查产物或输出，再决定是否继续下一步修改。".to_string()
        }
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
