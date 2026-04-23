use crate::capabilities::slots::is_mutating_tool;
use crate::capabilities::spec::ToolDefinition;
use crate::planner::{PlannedAction, normalize_mode};

pub(crate) fn resolve_tool(action: &PlannedAction) -> ToolDefinition {
    match action {
        PlannedAction::RunCommand { .. } => run_command_tool(),
        PlannedAction::ReadFile { .. } => read_file_tool(),
        PlannedAction::WriteFile { .. } => write_file_tool(),
        PlannedAction::DeletePath { .. } => delete_path_tool(),
        PlannedAction::ListFiles { .. } => list_files_tool(),
        PlannedAction::WriteMemory { .. } => write_memory_tool(),
        PlannedAction::RecallMemory { .. } => recall_memory_tool(),
        PlannedAction::SearchKnowledge { .. } => search_knowledge_tool(),
        PlannedAction::SearchSiyuanNotes { .. } => search_siyuan_tool(),
        PlannedAction::ReadSiyuanNote { .. } => read_siyuan_tool(),
        PlannedAction::WriteSiyuanKnowledge => write_siyuan_tool(),
        PlannedAction::ProjectAnswer => project_answer_tool(),
        PlannedAction::ContextAnswer => session_context_tool(),
        PlannedAction::Explain => explain_tool(),
        PlannedAction::AgentResolve => agent_resolve_tool(),
    }
}

pub(crate) fn visible_tools(mode: &str) -> Vec<ToolDefinition> {
    let current = normalize_mode(mode);
    tool_catalog()
        .into_iter()
        .filter(|tool| allows_tool(&current, tool))
        .collect()
}

fn allows_tool(mode: &str, tool: &ToolDefinition) -> bool {
    match mode {
        "observe" => !is_mutating_tool(tool),
        "standard" => !is_advanced_write_tool(tool),
        "full_access" => true,
        _ => !is_advanced_write_tool(tool),
    }
}

fn is_advanced_write_tool(tool: &ToolDefinition) -> bool {
    matches!(
        tool.tool_name.as_str(),
        "memory_write" | "write_siyuan_knowledge"
    )
}

fn tool_catalog() -> Vec<ToolDefinition> {
    vec![
        run_command_tool(),
        read_file_tool(),
        write_file_tool(),
        delete_path_tool(),
        list_files_tool(),
        write_memory_tool(),
        recall_memory_tool(),
        search_knowledge_tool(),
        search_siyuan_tool(),
        read_siyuan_tool(),
        write_siyuan_tool(),
        project_answer_tool(),
        session_context_tool(),
        explain_tool(),
    ]
}

fn make_tool(
    tool_name: &str,
    display_name: &str,
    category: &str,
    risk_level: &str,
    input_schema: &str,
    output_kind: &str,
    requires_confirmation: bool,
) -> ToolDefinition {
    ToolDefinition {
        tool_name: tool_name.to_string(),
        display_name: display_name.to_string(),
        category: category.to_string(),
        risk_level: risk_level.to_string(),
        input_schema: input_schema.to_string(),
        output_kind: output_kind.to_string(),
        requires_confirmation,
    }
}

fn run_command_tool() -> ToolDefinition {
    make_tool(
        "run_command",
        "执行命令",
        "system_command",
        "high",
        "command_text",
        "text_preview",
        true,
    )
}

fn read_file_tool() -> ToolDefinition {
    make_tool(
        "workspace_read",
        "读取文件",
        "workspace_read",
        "low",
        "path",
        "text_preview",
        false,
    )
}

fn write_file_tool() -> ToolDefinition {
    make_tool(
        "workspace_write",
        "写入文件",
        "workspace_write",
        "medium",
        "path_and_content",
        "write_result",
        false,
    )
}

fn delete_path_tool() -> ToolDefinition {
    make_tool(
        "workspace_delete",
        "删除路径",
        "workspace_write",
        "irreversible",
        "path",
        "delete_result",
        true,
    )
}

fn list_files_tool() -> ToolDefinition {
    make_tool(
        "workspace_list",
        "浏览目录",
        "workspace_read",
        "low",
        "optional_path",
        "list_preview",
        false,
    )
}

fn write_memory_tool() -> ToolDefinition {
    make_tool(
        "memory_write",
        "写入记忆",
        "memory_write",
        "medium",
        "memory_entry",
        "memory_write_result",
        false,
    )
}

fn recall_memory_tool() -> ToolDefinition {
    make_tool(
        "memory_recall",
        "召回记忆",
        "memory_read",
        "low",
        "query",
        "text_preview",
        false,
    )
}

fn search_knowledge_tool() -> ToolDefinition {
    make_tool(
        "knowledge_search",
        "知识检索",
        "knowledge_read",
        "low",
        "query",
        "text_preview",
        false,
    )
}

fn search_siyuan_tool() -> ToolDefinition {
    make_tool(
        "search_siyuan_notes",
        "思源摘要检索",
        "knowledge_read",
        "low",
        "query",
        "text_preview",
        false,
    )
}

fn read_siyuan_tool() -> ToolDefinition {
    make_tool(
        "read_siyuan_note",
        "读取思源正文",
        "knowledge_read",
        "low",
        "path",
        "text_preview",
        false,
    )
}

fn write_siyuan_tool() -> ToolDefinition {
    make_tool(
        "write_siyuan_knowledge",
        "写入思源知识",
        "knowledge_write",
        "medium",
        "none",
        "write_result",
        true,
    )
}

fn project_answer_tool() -> ToolDefinition {
    make_tool(
        "project_answer",
        "项目说明",
        "assistant_answer",
        "low",
        "none",
        "text_preview",
        false,
    )
}

fn session_context_tool() -> ToolDefinition {
    make_tool(
        "session_context",
        "会话续写",
        "assistant_answer",
        "low",
        "none",
        "text_preview",
        false,
    )
}

fn explain_tool() -> ToolDefinition {
    make_tool(
        "explain",
        "能力说明",
        "assistant_answer",
        "low",
        "none",
        "text_preview",
        false,
    )
}

fn agent_resolve_tool() -> ToolDefinition {
    make_tool(
        "agent_resolve",
        "智能体执行",
        "agent",
        "high",
        "none",
        "agent_trace",
        true,
    )
}
