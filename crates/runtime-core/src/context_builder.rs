use crate::contracts::RunRequest;
use crate::knowledge::search_knowledge;
use crate::memory_recall::recall_memory_digest;
use crate::repo_context::{RepoContextLoadResult, repo_context_summary};
use crate::session::{SessionMemory, session_prompt_summary};
use crate::text::summarize_text;
use crate::tools::ToolDefinition;

#[derive(Clone, Debug)]
pub(crate) struct StaticPromptBlock {
    pub role_prompt: String,
    pub mode_prompt: String,
}

#[derive(Clone, Debug)]
pub(crate) struct ProjectPromptBlock {
    pub workspace_root: String,
    pub repo_summary: String,
    pub doc_summary: String,
}

#[derive(Clone, Debug)]
pub(crate) struct DynamicPromptBlock {
    pub user_input: String,
    pub session_summary: String,
    pub memory_digest: String,
    pub knowledge_digest: String,
    pub tool_preview: String,
    pub reasoning_summary: String,
    pub cache_status: String,
    pub cache_reason: String,
}

#[derive(Clone, Debug)]
pub(crate) struct RuntimeContextEnvelope {
    pub user_input: String,
    pub mode: String,
    pub workspace_root: String,
    pub static_block: StaticPromptBlock,
    pub project_block: ProjectPromptBlock,
    pub dynamic_block: DynamicPromptBlock,
}

pub(crate) fn build_runtime_context(
    request: &RunRequest,
    session_context: &SessionMemory,
    repo_context: &RepoContextLoadResult,
    visible_tools: &[ToolDefinition],
    cache_status: &str,
    cache_reason: &str,
) -> RuntimeContextEnvelope {
    RuntimeContextEnvelope {
        user_input: request.user_input.clone(),
        mode: request.mode.clone(),
        workspace_root: request.workspace_ref.root_path.clone(),
        static_block: static_prompt_block(request),
        project_block: project_prompt_block(repo_context),
        dynamic_block: dynamic_prompt_block(
            request,
            session_context,
            visible_tools,
            cache_status,
            cache_reason,
        ),
    }
}

fn static_prompt_block(request: &RunRequest) -> StaticPromptBlock {
    StaticPromptBlock {
        role_prompt: "你是本地智能体，负责在当前工作区内完成真实任务。".to_string(),
        mode_prompt: mode_prompt_text(&request.mode),
    }
}

fn mode_prompt_text(mode: &str) -> String {
    match mode {
        "observe" => "当前模式为 `observe`。只允许观察、读取、检索和解释，不允许执行任何修改性动作。".to_string(),
        "full_access" => "当前模式为 `full_access`。允许执行全部已注册能力，但高危删除和危险命令仍必须经过风险确认。".to_string(),
        _ => "当前模式为 `standard`。允许常见开发读写与任务推进，但高级写入能力和高危动作仍受边界控制。".to_string(),
    }
}

fn project_prompt_block(repo_context: &RepoContextLoadResult) -> ProjectPromptBlock {
    ProjectPromptBlock {
        workspace_root: repo_context.snapshot.workspace_root.clone(),
        repo_summary: repo_context_summary(&repo_context.snapshot),
        doc_summary: repo_doc_summary(repo_context),
    }
}

fn repo_doc_summary(repo_context: &RepoContextLoadResult) -> String {
    let docs = &repo_context.snapshot.doc_summaries;
    if docs.is_empty() {
        return "当前没有命中高价值说明文件。".to_string();
    }
    let joined = docs
        .iter()
        .map(|item| format!("{}: {}", item.path, item.summary))
        .collect::<Vec<_>>()
        .join(" || ");
    summarize_text(&joined)
}

fn dynamic_prompt_block(
    request: &RunRequest,
    session_context: &SessionMemory,
    visible_tools: &[ToolDefinition],
    cache_status: &str,
    cache_reason: &str,
) -> DynamicPromptBlock {
    let session_summary = session_summary(session_context);
    let memory_digest = recall_memory_digest(request, &request.user_input, 3).summary;
    let knowledge_digest = knowledge_digest(request);
    DynamicPromptBlock {
        user_input: request.user_input.clone(),
        session_summary: session_summary.clone(),
        memory_digest: memory_digest.clone(),
        knowledge_digest: knowledge_digest.clone(),
        tool_preview: tool_preview(visible_tools),
        reasoning_summary: reasoning_summary(&session_summary, &memory_digest, &knowledge_digest),
        cache_status: cache_status.to_string(),
        cache_reason: cache_reason.to_string(),
    }
}

fn session_summary(session_context: &SessionMemory) -> String {
    session_prompt_summary(session_context)
}

fn knowledge_digest(request: &RunRequest) -> String {
    let hits = knowledge_hits(request);
    if hits.is_empty() {
        return "当前没有命中相关本地知识片段。".to_string();
    }
    summarize_text(
        &hits
            .into_iter()
            .map(|hit| format!("{}: {}", hit.path, hit.snippet))
            .collect::<Vec<_>>()
            .join(" || "),
    )
}

fn knowledge_hits(request: &RunRequest) -> Vec<crate::knowledge::KnowledgeHit> {
    let direct_hits = search_knowledge(request, &request.user_input, 4);
    if !direct_hits.is_empty() {
        return direct_hits;
    }
    search_knowledge(request, "项目 智能体 本地 主干 架构 运行时", 4)
}

fn tool_preview(visible_tools: &[ToolDefinition]) -> String {
    summarize_text(
        &visible_tools
            .iter()
            .map(|tool| format!("{}({})", tool.display_name, tool.tool_name))
            .collect::<Vec<_>>()
            .join("、"),
    )
}

fn reasoning_summary(session_summary: &str, memory_digest: &str, knowledge_digest: &str) -> String {
    summarize_text(&format!(
        "先结合会话摘要判断当前意图，再参考长期记忆与本地知识命中结果组织回答。会话：{} || 记忆：{} || 知识：{}",
        session_summary, memory_digest, knowledge_digest
    ))
}
