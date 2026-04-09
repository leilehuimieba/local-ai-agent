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
        next_step: report.outcome.next_step.clone(),
        key_artifacts: key_artifacts(trace),
        verification_summary: report.outcome.summary.clone(),
    }
}

fn current_plan(action: &PlannedAction, trace: &crate::capabilities::ToolExecutionTrace) -> String {
    format!(
        "动作={}; 摘要={}",
        action_label(action),
        trace.action_summary
    )
}

fn action_label(action: &PlannedAction) -> &'static str {
    match action {
        PlannedAction::RunCommand { .. } => "run_command",
        PlannedAction::ReadFile { .. } => "workspace_read",
        PlannedAction::WriteFile { .. } => "workspace_write",
        PlannedAction::DeletePath { .. } => "workspace_delete",
        PlannedAction::ListFiles { .. } => "workspace_list",
        PlannedAction::WriteMemory { .. } => "memory_write",
        PlannedAction::RecallMemory { .. } => "memory_recall",
        PlannedAction::SearchKnowledge { .. } => "knowledge_search",
        PlannedAction::SearchSiyuanNotes { .. } => "search_siyuan_notes",
        PlannedAction::ReadSiyuanNote { .. } => "read_siyuan_note",
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

fn open_risks(
    trace: &crate::capabilities::ToolExecutionTrace,
    report: &VerificationReport,
) -> Vec<String> {
    if report.outcome.passed {
        return Vec::new();
    }
    vec![format!(
        "验证未通过: {}; 工具摘要: {}",
        report.outcome.summary, trace.result.summary
    )]
}

fn key_artifacts(trace: &crate::capabilities::ToolExecutionTrace) -> Vec<String> {
    trace
        .result
        .artifact_path
        .clone()
        .into_iter()
        .collect::<Vec<_>>()
}
