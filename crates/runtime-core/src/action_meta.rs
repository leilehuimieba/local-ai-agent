use crate::planner::PlannedAction;

pub(crate) fn default_error_code(action: &PlannedAction) -> String {
    match action {
        PlannedAction::RunCommand { .. } => "command_failed",
        PlannedAction::ReadFile { .. } => "file_read_failed",
        PlannedAction::WriteFile { .. } => "file_write_failed",
        PlannedAction::DeletePath { .. } => "path_delete_failed",
        PlannedAction::ListFiles { .. } => "list_dir_failed",
        PlannedAction::WriteMemory { .. } => "memory_write_failed",
        PlannedAction::RecallMemory { .. } => "memory_recall_failed",
        PlannedAction::SearchKnowledge { .. } => "knowledge_search_failed",
        PlannedAction::SearchSiyuanNotes { .. } => "siyuan_search_failed",
        PlannedAction::ReadSiyuanNote { .. } => "siyuan_read_failed",
        PlannedAction::WriteSiyuanKnowledge => "siyuan_write_failed",
        PlannedAction::ProjectAnswer => "project_answer_failed",
        PlannedAction::ContextAnswer => "context_answer_failed",
        PlannedAction::Explain => "runtime_output_failed",
        PlannedAction::AgentResolve => "agent_resolve_failed",
    }
    .to_string()
}

pub(crate) fn action_tag(action: &PlannedAction) -> &'static str {
    if let PlannedAction::WriteSiyuanKnowledge = action {
        return "siyuan-write";
    }
    if let PlannedAction::ProjectAnswer = action {
        return "project";
    }
    if let PlannedAction::ContextAnswer = action {
        return "context";
    }
    if let PlannedAction::Explain = action {
        return "explain";
    }
    if let PlannedAction::AgentResolve = action {
        return "agent-resolve";
    }
    action_tag_common(action)
}

fn action_tag_common(action: &PlannedAction) -> &'static str {
    match action {
        PlannedAction::RunCommand { .. } => "command",
        PlannedAction::ReadFile { .. } => "read",
        PlannedAction::WriteFile { .. } => "write",
        PlannedAction::DeletePath { .. } => "delete",
        PlannedAction::ListFiles { .. } => "list",
        PlannedAction::WriteMemory { .. } => "memory-write",
        PlannedAction::RecallMemory { .. } => "memory-recall",
        PlannedAction::SearchKnowledge { .. } => "knowledge",
        PlannedAction::SearchSiyuanNotes { .. } => "siyuan-search",
        PlannedAction::ReadSiyuanNote { .. } => "siyuan-read",
        PlannedAction::WriteSiyuanKnowledge => unreachable!("handled before match"),
        PlannedAction::ProjectAnswer => unreachable!("handled before match"),
        PlannedAction::ContextAnswer => unreachable!("handled before match"),
        PlannedAction::Explain => unreachable!("handled before match"),
        PlannedAction::AgentResolve => unreachable!("handled before match"),
    }
}
