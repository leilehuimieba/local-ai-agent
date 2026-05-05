use crate::capabilities::ToolExecutionTrace;
use crate::planner::PlannedAction;
use crate::text::summarize_text;
use crate::tool_registry::ToolCall;

#[derive(Clone, Debug)]
pub(crate) struct VerificationOutcome {
    pub passed: bool,
    pub code: String,
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
    VerificationOutcome {
        passed: true,
        code: "verified".to_string(),
        policy: policy.to_string(),
        task_type: task_type.to_string(),
        evidence_count: evidence.len(),
        evidence,
        has_citation: false,
        fact_inference_split: true,
        capability_risk_checked: true,
        permission_boundary_respected: !guard_downgraded(trace),
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
    task_type: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    VerificationOutcome {
        passed: true,
        code: "verified_with_recovery".to_string(),
        policy: policy.to_string(),
        task_type: task_type.to_string(),
        evidence_count: evidence.len(),
        evidence,
        has_citation: false,
        fact_inference_split: true,
        capability_risk_checked: true,
        permission_boundary_respected: !guard_downgraded(trace),
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
    task_type: &str,
    evidence: Vec<String>,
) -> VerificationOutcome {
    VerificationOutcome {
        passed: false,
        code: "verification_failed".to_string(),
        policy: policy.to_string(),
        task_type: task_type.to_string(),
        evidence_count: evidence.len(),
        evidence,
        has_citation: false,
        fact_inference_split: false,
        capability_risk_checked: true,
        permission_boundary_respected: !guard_downgraded(trace),
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

fn verification_task_type(tool_call: &ToolCall) -> String {
    match tool_call.action {
        PlannedAction::ProjectAnswer | PlannedAction::ContextAnswer => "knowledge_answer".to_string(),
        PlannedAction::WriteFile { .. } | PlannedAction::ApplyPatch { .. } | PlannedAction::DeletePath { .. } => {
            "file_change".to_string()
        }
        PlannedAction::RunCommand { .. } | PlannedAction::MCPCall { .. } => "command_execution".to_string(),
        PlannedAction::WriteMemory { .. } | PlannedAction::WriteSiyuanKnowledge => "memory_write".to_string(),
        _ => "generic".to_string(),
    }
}

fn verification_policy(tool_call: &ToolCall) -> String {
    match tool_call.spec.tool_name.as_str() {
        "workspace_write" => "confirm_write_effect".to_string(),
        "workspace_delete" => "confirm_delete_effect".to_string(),
        "run_command" => "inspect_command_result".to_string(),
        "memory_write" => "confirm_memory_persisted".to_string(),
        "project_answer" | "context_answer" => "check_knowledge_answer".to_string(),
        "knowledge_search" | "search_siyuan_notes" | "read_siyuan_note" => "check_result_relevance".to_string(),
        _ => "check_result_summary".to_string(),
    }
}

fn verification_evidence(trace: &ToolExecutionTrace) -> Vec<String> {
    let mut evidence = vec![format!("summary={}", summarize_text(&trace.result.summary))];
    evidence.push(format!("reasoning={}", summarize_text(&trace.result.reasoning_summary)));
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
    trace.result.reasoning_summary.contains("guard downgraded") || trace.result.summary.contains("guard downgraded")
}

fn guard_decision_ref(trace: &ToolExecutionTrace) -> String {
    if guard_downgraded(trace) {
        return format!("tool={};decision=review", trace.tool.tool_name);
    }
    format!("tool={};decision=allow", trace.tool.tool_name)
}

fn verify_knowledge_answer(trace: &ToolExecutionTrace, policy: &str, evidence: Vec<String>) -> VerificationOutcome {
    let has_citation = has_citation(trace);
    let fact_inference_split = has_fact_inference_split(trace);
    let permission_ok = !guard_downgraded(trace);
    let evidence_count = evidence.len();
    if !trace.result.success || !has_citation || evidence_count < 2 || !fact_inference_split || !permission_ok {
        return VerificationOutcome {
            passed: false,
            code: "knowledge_answer_insufficient".to_string(),
            policy: policy.to_string(),
            task_type: "knowledge_answer".to_string(),
            evidence_count,
            evidence,
            has_citation,
            fact_inference_split,
            capability_risk_checked: true,
            permission_boundary_respected: permission_ok,
            skill_hit_effective: false,
            skill_hit_reason: "当前知识回答证据不足，不能按可交付答案收口。".to_string(),
            guard_downgraded: guard_downgraded(trace),
            guard_decision_ref: guard_decision_ref(trace),
            summary: "知识回答验证未通过：缺少足够引证、事实/推断边界或风险边界信号。".to_string(),
            next_step: "建议优先 replan 补充检索与引证；若仍不足，进入 handoff。".to_string(),
        };
    }
    VerificationOutcome {
        passed: true,
        code: "verified".to_string(),
        policy: policy.to_string(),
        task_type: "knowledge_answer".to_string(),
        evidence_count,
        evidence,
        has_citation,
        fact_inference_split,
        capability_risk_checked: true,
        permission_boundary_respected: true,
        skill_hit_effective: true,
        skill_hit_reason: "当前知识回答已形成可引证证据，并区分了事实、推断与建议。".to_string(),
        guard_downgraded: false,
        guard_decision_ref: guard_decision_ref(trace),
        summary: "知识回答验证通过：已具备最小引证、证据数量与边界说明。".to_string(),
        next_step: "当前知识回答已满足收口条件，可直接完成答复。".to_string(),
    }
}

fn verify_file_change(trace: &ToolExecutionTrace, policy: &str, evidence: Vec<String>) -> VerificationOutcome {
    let permission_ok = !guard_downgraded(trace);
    let path_visible = file_change_path_visible(trace);
    let effect_visible = file_change_effect_visible(trace);
    let evidence_count = evidence.len();
    if !trace.result.success || !permission_ok || !path_visible || !effect_visible || evidence_count < 2 {
        return VerificationOutcome {
            passed: false,
            code: "file_change_insufficient".to_string(),
            policy: policy.to_string(),
            task_type: "file_change".to_string(),
            evidence_count,
            evidence,
            has_citation: false,
            fact_inference_split: false,
            capability_risk_checked: true,
            permission_boundary_respected: permission_ok,
            skill_hit_effective: false,
            skill_hit_reason: "当前文件变更缺少目标路径或变更效果信号，不能按可交付修改收口。".to_string(),
            guard_downgraded: guard_downgraded(trace),
            guard_decision_ref: guard_decision_ref(trace),
            summary: "文件变更验证未通过：缺少路径、预览或删除完成等最小效果证据。".to_string(),
            next_step: "建议先补充 dry-run、目标路径确认或变更摘要，再决定是否继续。".to_string(),
        };
    }
    VerificationOutcome {
        passed: true,
        code: "verified".to_string(),
        policy: policy.to_string(),
        task_type: "file_change".to_string(),
        evidence_count,
        evidence,
        has_citation: false,
        fact_inference_split: true,
        capability_risk_checked: true,
        permission_boundary_respected: true,
        skill_hit_effective: true,
        skill_hit_reason: "当前文件变更已具备路径、效果摘要与风险边界信号。".to_string(),
        guard_downgraded: false,
        guard_decision_ref: guard_decision_ref(trace),
        summary: "文件变更验证通过：已具备最小路径与效果证据。".to_string(),
        next_step: "当前文件变更已满足最小收口条件，可进入后续答复或下一步。".to_string(),
    }
}

fn verify_command_execution(trace: &ToolExecutionTrace, policy: &str, evidence: Vec<String>) -> VerificationOutcome {
    let permission_ok = !guard_downgraded(trace);
    let output_visible = command_output_visible(trace);
    let artifact_visible = command_artifact_visible(trace);
    let evidence_count = evidence.len();
    if !trace.result.success || !permission_ok || !output_visible || !artifact_visible || evidence_count < 2 {
        return VerificationOutcome {
            passed: false,
            code: "command_execution_insufficient".to_string(),
            policy: policy.to_string(),
            task_type: "command_execution".to_string(),
            evidence_count,
            evidence,
            has_citation: false,
            fact_inference_split: false,
            capability_risk_checked: true,
            permission_boundary_respected: permission_ok,
            skill_hit_effective: trace.result.success,
            skill_hit_reason: "当前命令执行缺少输出、错误或产物路径信号，不能按可交付执行结果收口。".to_string(),
            guard_downgraded: guard_downgraded(trace),
            guard_decision_ref: guard_decision_ref(trace),
            summary: "命令执行验证未通过：缺少输出摘要、错误信号或原始产物引用。".to_string(),
            next_step: "建议先检查 exit code、stderr 摘要与原始输出产物，再决定是否重试。".to_string(),
        };
    }
    VerificationOutcome {
        passed: true,
        code: "verified".to_string(),
        policy: policy.to_string(),
        task_type: "command_execution".to_string(),
        evidence_count,
        evidence,
        has_citation: false,
        fact_inference_split: true,
        capability_risk_checked: true,
        permission_boundary_respected: true,
        skill_hit_effective: true,
        skill_hit_reason: "当前命令执行已具备输出摘要、原始产物引用与风险边界信号。".to_string(),
        guard_downgraded: false,
        guard_decision_ref: guard_decision_ref(trace),
        summary: "命令执行验证通过：已具备最小输出与产物证据。".to_string(),
        next_step: "当前命令执行已满足最小收口条件，可继续后续判断。".to_string(),
    }
}

fn verify_memory_write(trace: &ToolExecutionTrace, policy: &str, evidence: Vec<String>) -> VerificationOutcome {
    let permission_ok = !guard_downgraded(trace);
    let summary_visible = memory_write_summary_visible(trace);
    let final_visible = memory_write_final_visible(trace);
    let evidence_count = evidence.len();
    if !trace.result.success || !permission_ok || !summary_visible || !final_visible || evidence_count < 2 {
        return VerificationOutcome {
            passed: false,
            code: "memory_write_insufficient".to_string(),
            policy: policy.to_string(),
            task_type: "memory_write".to_string(),
            evidence_count,
            evidence,
            has_citation: false,
            fact_inference_split: false,
            capability_risk_checked: true,
            permission_boundary_respected: permission_ok,
            skill_hit_effective: trace.result.success,
            skill_hit_reason: "当前记忆写入缺少写入摘要或类型信号，不能按可复用沉淀收口。".to_string(),
            guard_downgraded: guard_downgraded(trace),
            guard_decision_ref: guard_decision_ref(trace),
            summary: "记忆写入验证未通过：缺少写入摘要、类型或最终答复信号。".to_string(),
            next_step: "建议先确认 write 成功摘要，再决定是否继续沉淀或回读。".to_string(),
        };
    }
    VerificationOutcome {
        passed: true,
        code: "verified".to_string(),
        policy: policy.to_string(),
        task_type: "memory_write".to_string(),
        evidence_count,
        evidence,
        has_citation: false,
        fact_inference_split: true,
        capability_risk_checked: true,
        permission_boundary_respected: true,
        skill_hit_effective: true,
        skill_hit_reason: "当前记忆写入已具备写入摘要、类型和结果信号。".to_string(),
        guard_downgraded: false,
        guard_decision_ref: guard_decision_ref(trace),
        summary: "记忆写入验证通过：已具备最小沉淀证据。".to_string(),
        next_step: "当前记忆写入已满足最小收口条件，可继续后续判断。".to_string(),
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

    #[test]
    fn verification_summary_keeps_object_aware_recall_reasoning() {
        let report = verify_tool_execution(
            &memory_recall_tool_call(),
            &sample_memory_recall_trace("system views + current memory object，对象 2 条"),
        );
        assert!(report.outcome.summary.contains("system views + current memory object"));
        assert!(
            report
                .outcome
                .evidence
                .iter()
                .any(|item| item.contains("system views + current memory object"))
        );
    }

    #[test]
    fn knowledge_answer_requires_citation_and_fact_boundary() {
        let report = verify_tool_execution(&knowledge_tool_call(), &knowledge_trace(false));
        assert!(!report.outcome.passed);
        assert_eq!(report.outcome.task_type, "knowledge_answer");
        assert_eq!(report.outcome.code, "knowledge_answer_insufficient");
        assert!(!report.outcome.has_citation);
        assert!(!report.outcome.fact_inference_split);
    }

    #[test]
    fn knowledge_answer_passes_with_citation_and_fact_boundary() {
        let report = verify_tool_execution(&knowledge_tool_call(), &knowledge_trace(true));
        assert!(report.outcome.passed);
        assert_eq!(report.outcome.task_type, "knowledge_answer");
        assert_eq!(report.outcome.policy, "check_knowledge_answer");
        assert!(report.outcome.has_citation);
        assert!(report.outcome.fact_inference_split);
    }

    #[test]
    fn file_change_requires_path_and_effect_signal() {
        let report = verify_tool_execution(&file_write_tool_call(), &file_change_trace(false));
        assert!(!report.outcome.passed);
        assert_eq!(report.outcome.task_type, "file_change");
        assert_eq!(report.outcome.code, "file_change_insufficient");
    }

    #[test]
    fn file_change_passes_with_path_and_effect_signal() {
        let report = verify_tool_execution(&file_write_tool_call(), &file_change_trace(true));
        assert!(report.outcome.passed);
        assert_eq!(report.outcome.task_type, "file_change");
        assert_eq!(report.outcome.policy, "confirm_write_effect");
    }

    #[test]
    fn command_execution_requires_output_and_artifact_signal() {
        let report = verify_tool_execution(&sample_tool_call(), &command_trace(false));
        assert!(!report.outcome.passed);
        assert_eq!(report.outcome.task_type, "command_execution");
        assert_eq!(report.outcome.code, "command_execution_insufficient");
    }

    #[test]
    fn command_execution_passes_with_output_and_artifact_signal() {
        let report = verify_tool_execution(&sample_tool_call(), &command_trace(true));
        assert!(report.outcome.passed);
        assert_eq!(report.outcome.task_type, "command_execution");
        assert_eq!(report.outcome.policy, "inspect_command_result");
    }

    #[test]
    fn memory_write_requires_summary_and_type_signal() {
        let report = verify_tool_execution(&memory_write_tool_call(), &memory_write_trace(false));
        assert!(!report.outcome.passed);
        assert_eq!(report.outcome.task_type, "memory_write");
        assert_eq!(report.outcome.code, "memory_write_insufficient");
    }

    #[test]
    fn memory_write_passes_with_summary_and_type_signal() {
        let report = verify_tool_execution(&memory_write_tool_call(), &memory_write_trace(true));
        assert!(report.outcome.passed);
        assert_eq!(report.outcome.task_type, "memory_write");
        assert_eq!(report.outcome.policy, "confirm_memory_persisted");
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
                model_schema: None,
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
                final_answer: if success {
                    "命令已执行完成。\n工作区：D:/repo\n命令：echo ok\n输出摘要：ok".to_string()
                } else {
                    "failed".to_string()
                },
                artifact_path: Some("D:/repo/tmp/command.txt".to_string()),
                detail_preview: "ok".to_string(),
                raw_output_ref: Some("D:/repo/tmp/command.txt".to_string()),
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

    fn memory_recall_tool_call() -> ToolCall {
        ToolCall {
            action: PlannedAction::RecallMemory {
                query: "对象摘要".to_string(),
            },
            spec: ToolDefinition {
                tool_name: "memory_recall".to_string(),
                display_name: "召回记忆".to_string(),
                category: "memory_read".to_string(),
                risk_level: "low".to_string(),
                input_schema: "query".to_string(),
                output_kind: "text_preview".to_string(),
                requires_confirmation: false,
                model_schema: None,
            },
        }
    }

    fn knowledge_tool_call() -> ToolCall {
        ToolCall {
            action: PlannedAction::ProjectAnswer,
            spec: ToolDefinition {
                tool_name: "project_answer".to_string(),
                display_name: "项目回答".to_string(),
                category: "assistant_answer".to_string(),
                risk_level: "low".to_string(),
                input_schema: "none".to_string(),
                output_kind: "text".to_string(),
                requires_confirmation: false,
                model_schema: None,
            },
        }
    }

    fn file_write_tool_call() -> ToolCall {
        ToolCall {
            action: PlannedAction::WriteFile {
                path: "docs/out.md".to_string(),
                content: "content".to_string(),
            },
            spec: ToolDefinition {
                tool_name: "workspace_write".to_string(),
                display_name: "写入文件".to_string(),
                category: "workspace_write".to_string(),
                risk_level: "medium".to_string(),
                input_schema: "path_and_content".to_string(),
                output_kind: "text".to_string(),
                requires_confirmation: false,
                model_schema: None,
            },
        }
    }

    fn memory_write_tool_call() -> ToolCall {
        ToolCall {
            action: PlannedAction::WriteMemory {
                kind: "project_rule".to_string(),
                summary: "记忆摘要".to_string(),
                content: "记忆内容".to_string(),
            },
            spec: ToolDefinition {
                tool_name: "memory_write".to_string(),
                display_name: "写入记忆".to_string(),
                category: "memory_write".to_string(),
                risk_level: "medium".to_string(),
                input_schema: "memory_entry".to_string(),
                output_kind: "memory_write_result".to_string(),
                requires_confirmation: false,
                model_schema: None,
            },
        }
    }

    fn sample_memory_recall_trace(layer: &str) -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: memory_recall_tool_call().spec.clone(),
            action_summary: "按需召回记忆：对象摘要".to_string(),
            result: ToolCallResult {
                summary: format!("已召回 2 条相关记忆。（{layer}）"),
                final_answer: format!("已召回相关长期记忆。\n召回层：{layer}"),
                artifact_path: None,
                detail_preview: "preview".to_string(),
                raw_output_ref: None,
                result_chars: 64,
                single_result_budget_chars: 30000,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 10,
                retryable: false,
                success: true,
                memory_write_summary: None,
                reasoning_summary: format!("按查询词检索长期记忆，并返回前几条高相关结果；本次召回层为{layer}。"),
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }

    fn knowledge_trace(complete: bool) -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: knowledge_tool_call().spec.clone(),
            action_summary: "基于项目上下文回答".to_string(),
            result: ToolCallResult {
                summary: if complete {
                    "知识摘要：knowledge pack || 知识引证：docs/README.md,docs/11-hermes-rebuild/current-state.md || 事实：已命中文档 || 推断：当前更适合 verify 先行 || 建议：进入下一步".to_string()
                } else {
                    "知识摘要：knowledge pack || 知识引证：未提供 || 统一总结：看起来可以回答".to_string()
                },
                final_answer: if complete {
                    "结论来自 docs/README.md 与 docs/11-hermes-rebuild/current-state.md".to_string()
                } else {
                    "我认为现在可以直接回答".to_string()
                },
                artifact_path: None,
                detail_preview: "preview".to_string(),
                raw_output_ref: None,
                result_chars: 120,
                single_result_budget_chars: 30000,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 12,
                retryable: false,
                success: true,
                memory_write_summary: None,
                reasoning_summary: if complete {
                    "事实：引用当前文档；推断：当前知识链已稳定；建议：可直接完成答复。".to_string()
                } else {
                    "仅有统一总结，没有继续区分事实与推断。".to_string()
                },
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }

    fn file_change_trace(complete: bool) -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: file_write_tool_call().spec.clone(),
            action_summary: "写入文件：docs/out.md".to_string(),
            result: ToolCallResult {
                summary: if complete {
                    "文件写入成功，共写入 12 个字符。".to_string()
                } else {
                    "写入动作已执行".to_string()
                },
                final_answer: if complete {
                    "文件写入完成：docs/out.md\n内容摘要：summary".to_string()
                } else {
                    "已尝试写入文件".to_string()
                },
                artifact_path: None,
                detail_preview: "preview".to_string(),
                raw_output_ref: None,
                result_chars: 80,
                single_result_budget_chars: 30000,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 10,
                retryable: false,
                success: true,
                memory_write_summary: None,
                reasoning_summary: "先校验工作区路径，再直接写入目标文件并返回摘要。".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }

    fn command_trace(complete: bool) -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: sample_tool_call().spec.clone(),
            action_summary: "执行命令：echo ok".to_string(),
            result: ToolCallResult {
                summary: if complete {
                    "命令执行成功".to_string()
                } else {
                    "执行结束".to_string()
                },
                final_answer: if complete {
                    "命令已执行完成。\n工作区：D:/repo\n命令：echo ok\n输出摘要：ok".to_string()
                } else {
                    "命令已执行".to_string()
                },
                artifact_path: complete.then(|| "D:/repo/tmp/command.txt".to_string()),
                detail_preview: if complete { "ok".to_string() } else { String::new() },
                raw_output_ref: complete.then(|| "D:/repo/tmp/command.txt".to_string()),
                result_chars: 40,
                single_result_budget_chars: 30000,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 10,
                retryable: false,
                success: true,
                memory_write_summary: None,
                reasoning_summary: "直接执行用户给定命令，并基于 stdout 或 stderr 生成摘要。".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }

    fn memory_write_trace(complete: bool) -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: memory_write_tool_call().spec.clone(),
            action_summary: "写入长期记忆：记忆摘要".to_string(),
            result: ToolCallResult {
                summary: if complete {
                    "已写入 `project_rule` 记忆：记忆摘要".to_string()
                } else {
                    "已尝试写入记忆".to_string()
                },
                final_answer: if complete {
                    "记忆写入完成。\n类型：project_rule\n摘要：记忆摘要\n内容摘要：summary".to_string()
                } else {
                    "记忆写入完成。".to_string()
                },
                artifact_path: None,
                detail_preview: "preview".to_string(),
                raw_output_ref: None,
                result_chars: 60,
                single_result_budget_chars: 30000,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 10,
                retryable: false,
                success: true,
                memory_write_summary: complete.then(|| "已写入 `project_rule` 记忆：记忆摘要".to_string()),
                reasoning_summary: "按用户指定内容构造长期记忆记录并写入本地主存储。".to_string(),
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
