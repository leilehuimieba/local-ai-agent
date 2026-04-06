use crate::planner::{PlannedAction, normalize_mode};

#[derive(Clone, Debug)]
pub(crate) struct ToolDefinition {
    pub tool_name: String,
    pub display_name: String,
    pub category: String,
    pub risk_level: String,
    pub input_schema: String,
    pub output_kind: String,
    pub requires_confirmation: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct ToolCallResult {
    pub summary: String,
    pub final_answer: String,
    pub artifact_path: Option<String>,
    pub error_code: Option<String>,
    pub retryable: bool,
    pub success: bool,
    pub memory_write_summary: Option<String>,
    pub reasoning_summary: String,
    pub cache_status: String,
    pub cache_reason: String,
}

#[derive(Clone, Debug)]
pub(crate) struct ToolExecutionTrace {
    pub tool: ToolDefinition,
    pub action_summary: String,
    pub result: ToolCallResult,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct ExternalConnectionSlot {
    pub slot_id: String,
    pub display_name: String,
    pub priority: u8,
    pub status: String,
    pub scope: String,
    pub current_tools: Vec<String>,
    pub boundary: String,
    pub next_step: String,
}

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

#[allow(dead_code)]
pub(crate) fn external_connection_slots() -> Vec<ExternalConnectionSlot> {
    vec![
        local_files_slot(),
        local_notes_slot(),
        browser_capture_slot(),
        personal_management_slot(),
    ]
}

fn allows_tool(mode: &str, tool: &ToolDefinition) -> bool {
    match mode {
        "observe" => !is_mutating_tool(tool),
        "standard" => !is_advanced_write_tool(tool),
        "full_access" => true,
        _ => !is_advanced_write_tool(tool),
    }
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

#[allow(dead_code)]
fn local_files_slot() -> ExternalConnectionSlot {
    make_slot(
        "local_files_project",
        "本地文件与项目目录",
        1,
        "active",
        &[
            "workspace_list",
            "workspace_read",
            "workspace_write",
            "workspace_delete",
            "run_command",
        ],
        "继续作为主链路第一优先级，不引入外部 SaaS 依赖。",
        "后续只补项目目录理解与浏览器侧摘录衔接，不额外分叉连接器。",
    )
}

#[allow(dead_code)]
fn local_notes_slot() -> ExternalConnectionSlot {
    make_slot(
        "local_notes_knowledge",
        "本地笔记与知识库",
        2,
        "active",
        &[
            "knowledge_search",
            "search_siyuan_notes",
            "read_siyuan_note",
            "write_siyuan_knowledge",
        ],
        "继续坚持 SQLite 主索引、思源外挂正文库，不让笔记系统承接高频主存储。",
        "下一阶段优先补浏览器摘录入库，再继续完善笔记侧沉淀入口。",
    )
}

#[allow(dead_code)]
fn browser_capture_slot() -> ExternalConnectionSlot {
    make_slot(
        "browser_capture_ingest",
        "浏览器摘录与网页入库",
        3,
        "reserved",
        &[
            "knowledge_search",
            "write_siyuan_knowledge",
            "project_answer",
        ],
        "本阶段只保留知识读写接入口，不提前接重型浏览器插件或云同步。",
        "后续在现有知识读写链路旁补摘录导入入口，优先落到知识层而不是新数据库。",
    )
}

#[allow(dead_code)]
fn personal_management_slot() -> ExternalConnectionSlot {
    make_slot(
        "calendar_reminder_management",
        "日历、提醒与更重的个人管理连接",
        4,
        "reserved",
        &["session_context", "project_answer"],
        "当前阶段只保留规划位，不接日历、提醒、任务中心等重连接器。",
        "只有前 3 类连接稳定后，才考虑把个人管理动作接进执行层。",
    )
}

fn is_mutating_tool(tool: &ToolDefinition) -> bool {
    matches!(
        tool.category.as_str(),
        "workspace_write" | "system_command" | "memory_write"
    )
}

fn is_advanced_write_tool(tool: &ToolDefinition) -> bool {
    matches!(
        tool.tool_name.as_str(),
        "memory_write" | "write_siyuan_knowledge"
    )
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
        "memory_digest",
        false,
    )
}

fn search_knowledge_tool() -> ToolDefinition {
    make_tool(
        "knowledge_search",
        "检索知识",
        "knowledge_read",
        "low",
        "query",
        "knowledge_digest",
        false,
    )
}

fn search_siyuan_tool() -> ToolDefinition {
    make_tool(
        "search_siyuan_notes",
        "检索思源",
        "knowledge_read",
        "low",
        "query",
        "knowledge_digest",
        false,
    )
}

fn read_siyuan_tool() -> ToolDefinition {
    make_tool(
        "read_siyuan_note",
        "读取思源",
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
        "导出思源",
        "knowledge_write",
        "medium",
        "none",
        "write_result",
        false,
    )
}

fn project_answer_tool() -> ToolDefinition {
    make_tool(
        "project_answer",
        "项目问答",
        "knowledge_read",
        "low",
        "none",
        "text_preview",
        false,
    )
}

fn session_context_tool() -> ToolDefinition {
    make_tool(
        "session_context",
        "延续会话",
        "session_read",
        "low",
        "none",
        "session_digest",
        false,
    )
}

fn explain_tool() -> ToolDefinition {
    make_tool(
        "runtime_output",
        "能力说明",
        "runtime_output",
        "low",
        "none",
        "text_preview",
        false,
    )
}

fn agent_resolve_tool() -> ToolDefinition {
    make_tool(
        "agent_resolve",
        "Agent 透传",
        "runtime_output",
        "low",
        "none",
        "text_preview",
        false,
    )
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

#[allow(dead_code)]
fn make_slot(
    slot_id: &str,
    display_name: &str,
    priority: u8,
    status: &str,
    current_tools: &[&str],
    boundary: &str,
    next_step: &str,
) -> ExternalConnectionSlot {
    ExternalConnectionSlot {
        slot_id: slot_id.to_string(),
        display_name: display_name.to_string(),
        priority,
        status: status.to_string(),
        scope: "external_connection".to_string(),
        current_tools: current_tools
            .iter()
            .map(|item| (*item).to_string())
            .collect(),
        boundary: boundary.to_string(),
        next_step: next_step.to_string(),
    }
}

pub(crate) fn tool_definition_to_json_schema(tool: &ToolDefinition) -> serde_json::Value {
    let properties = match tool.input_schema.as_str() {
        "command_text" => serde_json::json!({
            "command": { "type": "string", "description": "The command line string to execute" }
        }),
        "path" => serde_json::json!({
            "path": { "type": "string", "description": "The file or directory path" }
        }),
        "path_and_content" => serde_json::json!({
            "path": { "type": "string", "description": "The file path" },
            "content": { "type": "string", "description": "The content to write" }
        }),
        "optional_path" => serde_json::json!({
            "path": { "type": "string", "description": "The file or directory path (optional)" }
        }),
        "memory_entry" => serde_json::json!({
            "kind": { "type": "string", "description": "The category of memory" },
            "summary": { "type": "string", "description": "A short summary of the memory" },
            "content": { "type": "string", "description": "The full content of the memory" }
        }),
        "query" => serde_json::json!({
            "query": { "type": "string", "description": "The search term or query string" }
        }),
        "none" => serde_json::json!({}),
        _ => serde_json::json!({}),
    };

    let required = match tool.input_schema.as_str() {
        "command_text" => vec!["command"],
        "path" => vec!["path"],
        "path_and_content" => vec!["path", "content"],
        "optional_path" => vec![],
        "memory_entry" => vec!["kind", "summary", "content"],
        "query" => vec!["query"],
        "none" => vec![],
        _ => vec![],
    };

    let mut parameters = serde_json::json!({
        "type": "object",
        "properties": properties
    });

    if !required.is_empty() {
        parameters["required"] = serde_json::json!(required);
    }

    serde_json::json!({
        "type": "function",
        "function": {
            "name": tool.tool_name,
            "description": tool.display_name, // You might want to use a more detailed description if available
            "parameters": parameters,
        }
    })
}
