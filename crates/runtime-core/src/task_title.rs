use crate::planner::PlannedAction;

pub(crate) fn derive_task_title(action: &PlannedAction, user_input: &str) -> String {
    match action {
        PlannedAction::RunCommand { command } => label_with_text("执行命令", command),
        PlannedAction::ReadFile { path } => format!("读取文件: {path}"),
        PlannedAction::WriteFile { path, .. } => format!("写入文件: {path}"),
        PlannedAction::DeletePath { path } => format!("删除路径: {path}"),
        PlannedAction::ListFiles { path } => list_files_title(path.as_deref()),
        PlannedAction::WriteMemory { summary, .. } => label_with_text("写入记忆", summary),
        PlannedAction::RecallMemory { query } => label_with_text("召回记忆", query),
        PlannedAction::SearchKnowledge { query } => label_with_text("检索知识", query),
        PlannedAction::SearchSiyuanNotes { query } => label_with_text("检索思源", query),
        PlannedAction::ReadSiyuanNote { path } => label_with_text("读取思源", path),
        PlannedAction::WriteSiyuanKnowledge => "导出知识到思源".to_string(),
        PlannedAction::ProjectAnswer => label_with_text("项目问答", user_input),
        PlannedAction::ContextAnswer => "延续当前会话".to_string(),
        PlannedAction::Explain => label_with_text("解释可用能力", user_input),
        PlannedAction::AgentResolve => label_with_text("智能体执行", user_input),
    }
}

fn list_files_title(path: Option<&str>) -> String {
    match path {
        Some(value) if !value.is_empty() => format!("浏览目录: {value}"),
        _ => "浏览工作区目录".to_string(),
    }
}

fn label_with_text(label: &str, content: &str) -> String {
    format!("{label}: {}", truncate_text(content, 42))
}

fn truncate_text(input: &str, limit: usize) -> String {
    let mut chars = input.chars();
    let truncated: String = chars.by_ref().take(limit).collect();
    if chars.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
}
