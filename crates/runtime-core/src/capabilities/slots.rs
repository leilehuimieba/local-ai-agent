use crate::capabilities::spec::ToolDefinition;
use crate::contracts::ConnectorSlotSpec;

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

pub(crate) fn external_connection_slots() -> Vec<ExternalConnectionSlot> {
    vec![
        local_files_slot(),
        local_notes_slot(),
        browser_capture_slot(),
        personal_management_slot(),
    ]
}

pub(crate) fn connector_slot_spec(slot: &ExternalConnectionSlot) -> ConnectorSlotSpec {
    ConnectorSlotSpec {
        slot_id: slot.slot_id.clone(),
        display_name: slot.display_name.clone(),
        priority: slot.priority,
        status: slot.status.clone(),
        scope: slot.scope.clone(),
        current_capabilities: slot.current_tools.clone(),
        supported_actions: supported_actions(&slot.slot_id),
        boundary: slot.boundary.clone(),
        next_step: slot.next_step.clone(),
    }
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

pub(crate) fn is_mutating_tool(tool: &ToolDefinition) -> bool {
    matches!(
        tool.category.as_str(),
        "workspace_write" | "system_command" | "memory_write"
    )
}

fn supported_actions(slot_id: &str) -> Vec<String> {
    if matches!(slot_id, "local_files_project" | "local_notes_knowledge") {
        return vec!["recheck".to_string()];
    }
    Vec::new()
}
