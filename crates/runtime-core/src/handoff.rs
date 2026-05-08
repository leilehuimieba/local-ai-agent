use crate::artifacts::externalize_json_artifact;
use crate::contracts::RunRequest;
use crate::planner::PlannedAction;
use crate::verify::VerificationReport;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
struct HandoffArtifact {
    task_title: String,
    current_plan: String,
    completed_steps: Vec<String>,
    open_risks: Vec<String>,
    browser_failure_type: String,
    browser_page_id: String,
    browser_selector: String,
    browser_recovery_exhausted: bool,
    last_readback_summary: String,
    next_step: String,
    key_artifacts: Vec<String>,
    verification_summary: String,
}

pub(crate) fn persist_handoff_artifact(
    request: &RunRequest,
    task_title: &str,
    action: &PlannedAction,
    trace: &crate::capabilities::ToolExecutionTrace,
    report: &VerificationReport,
) -> Option<String> {
    let payload = handoff_artifact(task_title, action, trace, report);
    externalize_json_artifact(request, "handoff", &payload).map(|item| item.path)
}

fn handoff_artifact(
    task_title: &str,
    action: &PlannedAction,
    trace: &crate::capabilities::ToolExecutionTrace,
    report: &VerificationReport,
) -> HandoffArtifact {
    HandoffArtifact {
        task_title: task_title.to_string(),
        current_plan: current_plan(action, trace),
        completed_steps: completed_steps(trace),
        open_risks: open_risks(trace, report),
        browser_failure_type: report.outcome.browser_failure_type.clone(),
        browser_page_id: report.outcome.browser_page_id.clone(),
        browser_selector: report.outcome.browser_selector.clone(),
        browser_recovery_exhausted: report.outcome.browser_recovery_exhausted,
        last_readback_summary: last_readback_summary(trace, report),
        next_step: report.outcome.next_step.clone(),
        key_artifacts: key_artifacts(trace),
        verification_summary: report.outcome.summary.clone(),
    }
}

fn current_plan(action: &PlannedAction, trace: &crate::capabilities::ToolExecutionTrace) -> String {
    format!("动作={}; 摘要={}", action_label(action), trace.action_summary)
}

fn action_label(action: &PlannedAction) -> &'static str {
    match action {
        PlannedAction::RunCommand { .. } => "run_command",
        PlannedAction::ReadFile { .. } => "workspace_read",
        PlannedAction::WriteFile { .. } => "workspace_write",
        PlannedAction::ApplyPatch { .. } => "workspace_apply_patch",
        PlannedAction::DeletePath { .. } => "workspace_delete",
        PlannedAction::ListFiles { .. } => "workspace_list",
        PlannedAction::WriteMemory { .. } => "memory_write",
        PlannedAction::RecallMemory { .. } => "memory_recall",
        PlannedAction::SearchKnowledge { .. } => "knowledge_search",
        PlannedAction::SearchSiyuanNotes { .. } => "search_siyuan_notes",
        PlannedAction::ReadSiyuanNote { .. } => "read_siyuan_note",
        PlannedAction::MCPCall { .. } => "mcp_call",
        PlannedAction::WriteSiyuanKnowledge => "write_siyuan_knowledge",
        PlannedAction::ProjectAnswer => "project_answer",
        PlannedAction::ContextAnswer => "context_answer",
        PlannedAction::Explain => "explain",
        PlannedAction::AgentResolve => "agent_resolve",
    }
}

fn completed_steps(trace: &crate::capabilities::ToolExecutionTrace) -> Vec<String> {
    vec![
        "Analyze".to_string(),
        "Plan".to_string(),
        format!("Execute: {}", trace.tool.display_name),
        "Observe".to_string(),
    ]
}

fn open_risks(trace: &crate::capabilities::ToolExecutionTrace, report: &VerificationReport) -> Vec<String> {
    if report.outcome.passed {
        return Vec::new();
    }
    vec![format!(
        "验证未通过: {}; 工具摘要: {}",
        report.outcome.summary, trace.result.summary
    )]
}

fn key_artifacts(trace: &crate::capabilities::ToolExecutionTrace) -> Vec<String> {
    trace.result.artifact_path.clone().into_iter().collect::<Vec<_>>()
}

fn last_readback_summary(trace: &crate::capabilities::ToolExecutionTrace, report: &VerificationReport) -> String {
    if report.outcome.browser_recovery_attempted {
        return trace.result.summary.clone();
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::{ToolCallResult, ToolDefinition, ToolExecutionTrace};

    #[test]
    fn handoff_artifact_keeps_browser_failure_fields() {
        let artifact = handoff_artifact("浏览器提交", &browser_action(), &browser_trace(), &browser_report());
        assert_eq!(artifact.browser_failure_type, "recovery_exhausted");
        assert_eq!(artifact.browser_page_id, "page_01");
        assert_eq!(artifact.browser_selector, "#submit");
        assert!(artifact.browser_recovery_exhausted);
        assert!(artifact.last_readback_summary.contains("已执行单次恢复"));
    }

    fn browser_action() -> PlannedAction {
        PlannedAction::MCPCall {
            server_id: "browser".to_string(),
            tool_name: "submit".to_string(),
            arguments_json: r##"{"page_id":"page_01","selector":"#submit"}"##.to_string(),
        }
    }

    fn browser_trace() -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: ToolDefinition {
                tool_name: "mcp__browser__submit".to_string(),
                display_name: "MCP: browser/submit".to_string(),
                category: "mcp".to_string(),
                risk_level: "high".to_string(),
                input_schema: "json".to_string(),
                output_kind: "json_preview".to_string(),
                requires_confirmation: true,
                model_schema: None,
            },
            action_summary: "调用 MCP 工具：browser/submit".to_string(),
            result: ToolCallResult {
                summary: "已执行单次恢复 read_page，但当前浏览器交互仍未验证通过。".to_string(),
                final_answer: r##"{"ok":true,"page_id":"page_01","selector":"#submit"}"##.to_string(),
                artifact_path: Some("D:/repo/tmp/browser-submit.json".to_string()),
                detail_preview: r##"{"ok":true,"page_id":"page_01","selector":"#submit"}"##.to_string(),
                raw_output_ref: None,
                result_chars: 64,
                single_result_budget_chars: 1200,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 35,
                retryable: false,
                success: true,
                memory_write_summary: None,
                reasoning_summary: "Runtime 通过 Gateway MCP endpoint 执行工具，复用 allowlist 与审计。".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }

    fn browser_report() -> VerificationReport {
        VerificationReport {
            outcome: crate::verify::VerificationOutcome {
                passed: false,
                code: "browser_interaction_insufficient".to_string(),
                browser_failure_type: "recovery_exhausted".to_string(),
                browser_page_id: "page_01".to_string(),
                browser_selector: "#submit".to_string(),
                browser_recovery_attempted: true,
                browser_recovery_exhausted: true,
                policy: "confirm_browser_state_change".to_string(),
                task_type: "browser_interaction".to_string(),
                evidence: vec!["summary=browser".to_string()],
                evidence_count: 1,
                has_citation: false,
                fact_inference_split: false,
                capability_risk_checked: true,
                permission_boundary_respected: true,
                skill_hit_effective: true,
                skill_hit_reason: "insufficient".to_string(),
                guard_downgraded: false,
                guard_decision_ref: "tool=mcp__browser__submit;decision=allow".to_string(),
                summary: "浏览器交互验证未通过".to_string(),
                next_step: "已完成单次 read_page 回读仍未通过验证，建议交给人工继续检查页面状态或调整目标元素。"
                    .to_string(),
            },
            tool_elapsed_ms: 35,
            result_chars: 64,
            single_result_budget_chars: 1200,
            single_result_budget_hit: false,
        }
    }
}
