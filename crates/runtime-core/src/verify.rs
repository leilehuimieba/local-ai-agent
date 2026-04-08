use crate::capabilities::ToolExecutionTrace;
use crate::text::summarize_text;
use crate::tool_registry::ToolCall;

#[derive(Clone, Debug)]
pub(crate) struct VerificationOutcome {
    pub passed: bool,
    pub code: String,
    pub policy: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub next_step: String,
}

#[derive(Clone, Debug)]
pub(crate) struct VerificationReport {
    pub outcome: VerificationOutcome,
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
    VerificationReport { outcome }
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
    if let Some(path) = trace.result.artifact_path.as_ref() {
        evidence.push(format!("artifact={path}"));
    }
    evidence.push(format!("cache_status={}", trace.result.cache_status));
    evidence
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
