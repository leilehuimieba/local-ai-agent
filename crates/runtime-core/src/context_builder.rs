use crate::capabilities::ToolDefinition;
use crate::context_policy::ContextAssemblyPolicy;
use crate::contracts::RunRequest;
use crate::knowledge::{KnowledgePack, build_knowledge_pack};
use crate::memory_layer::digest_route_summary;
use crate::memory_recall::{MemoryDigest, recall_memory_digest_with_policy};
use crate::observation::{
    ObservationLayeredInjectionReport, build_layered_injection, resolve_observation_budget_chars,
};
use crate::paths::repo_root;
use crate::repo_context::{RepoContextLoadResult, repo_context_summary};
use crate::session::{SessionMemory, session_prompt_summary};
use crate::text::{extract_snippet, summarize_text};
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Clone, Debug, Default)]
pub(crate) struct DynamicPromptBlock {
    pub user_input: String,
    pub assembly_profile: String,
    pub prompt_profile: String,
    pub includes_session: bool,
    pub includes_memory: bool,
    pub includes_knowledge: bool,
    pub includes_tool_preview: bool,
    pub skill_injection_enabled: bool,
    pub max_skill_level: String,
    pub injected_skill_level: String,
    pub injected_skill_ids: String,
    pub evidence_refs: String,
    pub phase_label: String,
    pub selection_reason: String,
    pub injection_summary: String,
    pub prefers_artifact_context: bool,
    pub session_summary: String,
    pub memory_digest: String,
    pub memory_has_system_views: bool,
    pub memory_has_current_objects: bool,
    pub memory_current_object_count: usize,
    pub memory_route: String,
    pub memory_selected_layers: String,
    pub memory_match_reason: String,
    pub memory_reuse_confidence: String,
    pub memory_skipped_layers: String,
    pub knowledge_digest: String,
    pub knowledge_pack_question_type: String,
    pub knowledge_pack_citations: String,
    pub knowledge_pack_match_reason: String,
    pub tool_preview: String,
    pub artifact_hint: String,
    pub observation_injection: String,
    pub observation_references: String,
    pub observation_budget_total: usize,
    pub observation_budget_used: usize,
    pub observation_budget_hit: bool,
    pub observation_budget_total_tokens: usize,
    pub observation_budget_used_tokens: usize,
    pub observation_budget_hit_tokens: bool,
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
    policy: &ContextAssemblyPolicy,
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
            policy,
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
        "full_access" => {
            "当前模式为 `full_access`。允许执行全部已注册能力，但高危删除和危险命令仍必须经过风险确认。".to_string()
        }
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
    policy: &ContextAssemblyPolicy,
    cache_status: &str,
    cache_reason: &str,
) -> DynamicPromptBlock {
    let session_summary = selected_session_summary(session_context, policy);
    let memory = selected_memory_selection(request, policy);
    let knowledge = selected_knowledge_selection(request, policy);
    let tool_preview = selected_tool_preview(request, visible_tools, policy);
    let artifact_hint = selected_artifact_hint(session_context, policy);
    let observation = observation_injection(request, policy);
    build_dynamic_block(
        request,
        policy,
        cache_status,
        cache_reason,
        PromptParts {
            session_summary,
            memory_digest: memory.digest,
            memory_has_system_views: memory.has_system_views,
            memory_has_current_objects: memory.has_current_objects,
            memory_current_object_count: memory.current_object_count,
            memory_route: memory.route,
            memory_selected_layers: memory.selected_layers,
            memory_match_reason: memory.match_reason,
            memory_reuse_confidence: memory.reuse_confidence,
            memory_skipped_layers: memory.skipped_layers,
            knowledge_digest: knowledge.digest,
            knowledge_pack_question_type: knowledge.question_type,
            knowledge_pack_citations: knowledge.citations,
            knowledge_pack_match_reason: knowledge.match_reason,
            tool_preview,
            artifact_hint,
            observation,
        },
    )
}

struct PromptParts {
    session_summary: String,
    memory_digest: String,
    memory_has_system_views: bool,
    memory_has_current_objects: bool,
    memory_current_object_count: usize,
    memory_route: String,
    memory_selected_layers: String,
    memory_match_reason: String,
    memory_reuse_confidence: String,
    memory_skipped_layers: String,
    knowledge_digest: String,
    knowledge_pack_question_type: String,
    knowledge_pack_citations: String,
    knowledge_pack_match_reason: String,
    tool_preview: String,
    artifact_hint: String,
    observation: ObservationLayeredInjectionReport,
}

struct MemoryPromptSelection {
    digest: String,
    has_system_views: bool,
    has_current_objects: bool,
    current_object_count: usize,
    route: String,
    selected_layers: String,
    match_reason: String,
    reuse_confidence: String,
    skipped_layers: String,
}

struct KnowledgePromptSelection {
    digest: String,
    question_type: String,
    citations: String,
    match_reason: String,
}

fn build_dynamic_block(
    request: &RunRequest,
    policy: &ContextAssemblyPolicy,
    cache_status: &str,
    cache_reason: &str,
    parts: PromptParts,
) -> DynamicPromptBlock {
    let mut block = DynamicPromptBlock::default();
    fill_identity_fields(&mut block, request, policy);
    fill_digest_fields(&mut block, &parts);
    fill_observation_fields(&mut block, &parts.observation);
    fill_runtime_fields(&mut block, cache_status, cache_reason, &parts);
    block
}

fn fill_identity_fields(block: &mut DynamicPromptBlock, request: &RunRequest, policy: &ContextAssemblyPolicy) {
    block.user_input = request.user_input.clone();
    block.assembly_profile = policy.profile.clone();
    block.prompt_profile = policy.prompt_profile.clone();
    block.includes_session = policy.include_session;
    block.includes_memory = policy.include_memory;
    block.includes_knowledge = policy.include_knowledge;
    block.includes_tool_preview = policy.include_tool_preview;
    block.skill_injection_enabled = policy.skill_injection_enabled;
    block.max_skill_level = policy.max_skill_level.clone();
    block.injected_skill_level = effective_skill_level(policy);
    block.injected_skill_ids = injected_skill_ids(request, policy);
    block.evidence_refs = evidence_refs(request, policy);
    block.phase_label = policy.phase_label.clone();
    block.selection_reason = policy.selection_reason.clone();
    block.injection_summary = injection_summary(policy);
    block.prefers_artifact_context = policy.prefer_artifact_context;
}

fn fill_digest_fields(block: &mut DynamicPromptBlock, parts: &PromptParts) {
    block.session_summary = parts.session_summary.clone();
    block.memory_digest = parts.memory_digest.clone();
    block.memory_has_system_views = parts.memory_has_system_views;
    block.memory_has_current_objects = parts.memory_has_current_objects;
    block.memory_current_object_count = parts.memory_current_object_count;
    block.memory_route = parts.memory_route.clone();
    block.memory_selected_layers = parts.memory_selected_layers.clone();
    block.memory_match_reason = parts.memory_match_reason.clone();
    block.memory_reuse_confidence = parts.memory_reuse_confidence.clone();
    block.memory_skipped_layers = parts.memory_skipped_layers.clone();
    block.knowledge_digest = parts.knowledge_digest.clone();
    block.knowledge_pack_question_type = parts.knowledge_pack_question_type.clone();
    block.knowledge_pack_citations = parts.knowledge_pack_citations.clone();
    block.knowledge_pack_match_reason = parts.knowledge_pack_match_reason.clone();
    block.tool_preview = parts.tool_preview.clone();
    block.artifact_hint = parts.artifact_hint.clone();
}

fn fill_observation_fields(block: &mut DynamicPromptBlock, observation: &ObservationLayeredInjectionReport) {
    block.observation_injection = observation.injected_text.clone();
    block.observation_references = observation.references.join(",");
    block.observation_budget_total = observation.budget_total_chars;
    block.observation_budget_used = observation.used_chars;
    block.observation_budget_hit = observation.budget_hit;
    block.observation_budget_total_tokens = observation.budget_total_tokens;
    block.observation_budget_used_tokens = observation.used_tokens;
    block.observation_budget_hit_tokens = observation.budget_hit_tokens;
}

fn fill_runtime_fields(block: &mut DynamicPromptBlock, cache_status: &str, cache_reason: &str, parts: &PromptParts) {
    block.reasoning_summary = reasoning_summary(
        &parts.session_summary,
        &parts.memory_digest,
        &parts.knowledge_digest,
        &parts.observation.injected_text,
        &block.selection_reason,
        &parts.artifact_hint,
    );
    block.cache_status = cache_status.to_string();
    block.cache_reason = cache_reason.to_string();
}

fn injection_summary(policy: &ContextAssemblyPolicy) -> String {
    let mut parts = Vec::new();
    push_injection_part(&mut parts, policy.include_session, "session digest");
    push_injection_part(&mut parts, policy.include_memory, "memory digest");
    push_injection_part(&mut parts, policy.include_knowledge, "knowledge digest");
    push_injection_part(&mut parts, policy.include_tool_preview, "tool preview");
    push_injection_part(&mut parts, policy.prefer_artifact_context, "artifact hint");
    if parts.is_empty() {
        "当前 profile 未注入额外上下文块。".to_string()
    } else {
        format!("当前 profile 注入：{}", parts.join(" + "))
    }
}

fn push_injection_part(parts: &mut Vec<&'static str>, enabled: bool, label: &'static str) {
    if enabled {
        parts.push(label);
    }
}

fn session_summary(session_context: &SessionMemory) -> String {
    session_prompt_summary(session_context)
}

fn selected_session_summary(session_context: &SessionMemory, policy: &ContextAssemblyPolicy) -> String {
    if policy.include_session {
        session_summary(session_context)
    } else {
        "当前阶段未注入会话摘要。".to_string()
    }
}

fn selected_memory_selection(request: &RunRequest, policy: &ContextAssemblyPolicy) -> MemoryPromptSelection {
    if !policy.include_memory {
        return MemoryPromptSelection {
            digest: "当前阶段未注入长期记忆摘要。".to_string(),
            has_system_views: false,
            has_current_objects: false,
            current_object_count: 0,
            route: "memory_disabled".to_string(),
            selected_layers: String::new(),
            match_reason: "当前 profile 未启用长期记忆注入。".to_string(),
            reuse_confidence: "low".to_string(),
            skipped_layers: String::new(),
        };
    }
    let digest = recall_memory_digest_with_policy(request, &request.user_input, 3, Some(policy));
    MemoryPromptSelection {
        digest: format_memory_digest(&digest),
        has_system_views: digest.has_system_views,
        has_current_objects: digest.has_current_objects,
        current_object_count: digest.current_object_count,
        route: digest.memory_route.clone(),
        selected_layers: digest.selected_layers.join(","),
        match_reason: digest.match_reason.clone(),
        reuse_confidence: digest.reuse_confidence.clone(),
        skipped_layers: digest.skipped_layers.join(","),
    }
}

fn format_memory_digest(digest: &MemoryDigest) -> String {
    let focus = memory_digest_focus(digest);
    if focus.is_empty() {
        digest.summary.clone()
    } else {
        format!("{focus} || {}", digest.summary)
    }
}

fn memory_digest_focus(digest: &MemoryDigest) -> String {
    let route = digest_route_summary(digest);
    if route.is_empty() {
        return String::new();
    }
    format!("记忆入口已按最小路由装配：{}", route)
}

fn selected_knowledge_selection(request: &RunRequest, policy: &ContextAssemblyPolicy) -> KnowledgePromptSelection {
    if !policy.include_knowledge {
        return KnowledgePromptSelection {
            digest: "当前阶段未注入知识摘要。".to_string(),
            question_type: String::new(),
            citations: String::new(),
            match_reason: String::new(),
        };
    }
    project_status_knowledge_selection(request, policy).unwrap_or_else(|| knowledge_selection(request, policy))
}

fn knowledge_selection(request: &RunRequest, policy: &ContextAssemblyPolicy) -> KnowledgePromptSelection {
    let pack = build_knowledge_pack(request, &request.user_input, 4);
    KnowledgePromptSelection {
        digest: format_knowledge_digest(&pack, policy),
        question_type: pack.question_type,
        citations: pack.citations.join(","),
        match_reason: pack.match_reason,
    }
}

fn format_knowledge_digest(pack: &KnowledgePack, policy: &ContextAssemblyPolicy) -> String {
    if pack.top_hits.is_empty() {
        return "当前没有命中相关本地知识片段。".to_string();
    }
    if prefers_knowledge_pack(policy) {
        return format_knowledge_pack_digest(pack);
    }
    summarize_text(
        &pack
            .top_hits
            .iter()
            .map(|hit| format!("{}: {}", hit.path, hit.snippet))
            .collect::<Vec<_>>()
            .join(" || "),
    )
}

fn prefers_knowledge_pack(policy: &ContextAssemblyPolicy) -> bool {
    policy.profile == "ask_profile" || policy.profile == "learn_profile"
}

fn format_knowledge_pack_digest(pack: &KnowledgePack) -> String {
    let top_hits = pack
        .top_hits
        .iter()
        .map(|hit| format!("{}（{}）", hit.path, hit.use_for))
        .collect::<Vec<_>>()
        .join(" || ");
    let supporting = pack
        .supporting_hits
        .iter()
        .map(|hit| hit.path.clone())
        .collect::<Vec<_>>()
        .join("、");
    summarize_text(&format!(
        "knowledge pack：问题类型={}；命中理由={}；主命中={}；辅助命中={}；回答提示={}；引证={}",
        pack.question_type,
        pack.match_reason,
        top_hits,
        blank_pack_text(&supporting),
        blank_pack_text(&pack.answer_hints.join(" | ")),
        pack.citations.join("、")
    ))
}

fn blank_pack_text(value: &str) -> &str {
    if value.trim().is_empty() { "未提供" } else { value }
}

fn project_status_knowledge_selection(
    request: &RunRequest,
    policy: &ContextAssemblyPolicy,
) -> Option<KnowledgePromptSelection> {
    if policy.prompt_profile != "project_answer" || !is_project_status_query(&request.user_input) {
        return None;
    }
    let entries = preferred_project_status_paths(request)
        .into_iter()
        .filter_map(|path| status_digest_entry(&path, &request.user_input))
        .collect::<Vec<_>>();
    let citations = preferred_project_status_paths(request)
        .into_iter()
        .filter(|path| path.exists())
        .take(3)
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(",");
    (!entries.is_empty()).then(|| KnowledgePromptSelection {
        digest: summarize_text(&entries.join(" || ")),
        question_type: "project_status".to_string(),
        citations,
        match_reason: "当前问题更像项目状态问答，优先使用 current-state 与阶段文档。".to_string(),
    })
}

fn is_project_status_query(user_input: &str) -> bool {
    [
        "停在什么状态",
        "为什么不能继续",
        "默认推进",
        "为什么停",
        "暂停点",
        "重启",
        "当前阶段",
        "还差什么",
        "下一步做什么",
    ]
    .iter()
    .any(|token| user_input.contains(token))
}

fn preferred_project_status_paths(request: &RunRequest) -> Vec<PathBuf> {
    let docs_root = repo_root(request).join("docs");
    let hermes_root = docs_root.join("11-hermes-rebuild");
    vec![
        hermes_root.join("current-state.md"),
        hermes_root
            .join("changes")
            .join("H-gate-h-signoff-20260416")
            .join("status.md"),
        hermes_root
            .join("changes")
            .join("H-gate-h-signoff-20260416")
            .join("review.md"),
        hermes_root.join("changes").join("INDEX.md"),
        docs_root.join("README.md"),
        hermes_root.join("Hermes重构总路线图_完整计划.md"),
        hermes_root.join("stage-plans").join("阶段计划总表.md"),
    ]
}

fn status_digest_entry(path: &Path, query: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    Some(format!("{}: {}", path.display(), extract_snippet(&content, query)))
}

fn selected_tool_preview(
    request: &RunRequest,
    visible_tools: &[ToolDefinition],
    policy: &ContextAssemblyPolicy,
) -> String {
    if policy.include_tool_preview {
        combined_tool_preview(tool_preview(visible_tools), request)
    } else {
        "当前阶段未注入工具预览。".to_string()
    }
}

fn combined_tool_preview(native_preview: String, request: &RunRequest) -> String {
    let mcp_preview = request
        .context_hints
        .get("mcp_tool_preview")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty());
    match mcp_preview {
        Some(preview) => summarize_text(&format!("{native_preview} || {preview}")),
        None => native_preview,
    }
}

fn tool_preview(visible_tools: &[ToolDefinition]) -> String {
    summarize_text(
        &visible_tools
            .iter()
            .filter(|tool| tool.category != "mcp")
            .map(|tool| format!("{}({})", tool.display_name, tool.tool_name))
            .collect::<Vec<_>>()
            .join("、"),
    )
}

fn selected_artifact_hint(session_context: &SessionMemory, policy: &ContextAssemblyPolicy) -> String {
    if policy.prefer_artifact_context && !session_context.short_term.handoff_artifact_path.is_empty() {
        return format!("优先参考交接包：{}", session_context.short_term.handoff_artifact_path);
    }
    "当前阶段未注入交接包提示。".to_string()
}

fn effective_skill_level(policy: &ContextAssemblyPolicy) -> String {
    if policy.skill_injection_enabled {
        policy.max_skill_level.clone()
    } else {
        "disabled".to_string()
    }
}

fn injected_skill_ids(request: &RunRequest, policy: &ContextAssemblyPolicy) -> String {
    if !policy.skill_injection_enabled {
        return "none".to_string();
    }
    request
        .context_hints
        .get("skill_ids")
        .cloned()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "none".to_string())
}

fn evidence_refs(request: &RunRequest, policy: &ContextAssemblyPolicy) -> String {
    if !policy.include_memory && !policy.include_knowledge {
        return "none".to_string();
    }
    request
        .context_hints
        .get("evidence_refs")
        .cloned()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "observation".to_string())
}

fn observation_injection(request: &RunRequest, policy: &ContextAssemblyPolicy) -> ObservationLayeredInjectionReport {
    if policy.include_memory || policy.include_knowledge {
        let budget = resolve_observation_budget_chars(request, 1200);
        return build_layered_injection(request, &request.user_input, budget);
    }
    let budget = resolve_observation_budget_chars(request, 300);
    build_layered_injection(request, "", budget)
}

fn reasoning_summary(
    session_summary: &str,
    memory_digest: &str,
    knowledge_digest: &str,
    observation_injection: &str,
    selection_reason: &str,
    artifact_hint: &str,
) -> String {
    summarize_text(&format!(
        "当前上下文调度原因：{}。先结合会话摘要判断当前意图，再参考长期记忆、本地知识与 observation 分层注入组织回答。会话：{} || 记忆：{} || 知识：{} || Observation：{} || Artifact：{}",
        selection_reason, session_summary, memory_digest, knowledge_digest, observation_injection, artifact_hint
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use crate::memory::MemoryEntry;
    use crate::sqlite_store::write_memory_entry_sqlite;
    use std::collections::BTreeMap;
    use std::path::Path;

    #[test]
    fn fills_skill_injection_fields_from_policy_and_hints() {
        let request = sample_request();
        let policy = ContextAssemblyPolicy {
            profile: "act_profile".to_string(),
            prompt_profile: "agent_resolve".to_string(),
            include_session: true,
            include_memory: true,
            include_knowledge: true,
            include_tool_preview: false,
            skill_injection_enabled: true,
            max_skill_level: "level1:index-summary".to_string(),
            phase_label: "execute".to_string(),
            selection_reason: "test".to_string(),
            prefer_artifact_context: false,
        };
        let mut block = DynamicPromptBlock::default();
        fill_identity_fields(&mut block, &request, &policy);
        assert!(block.skill_injection_enabled);
        assert_eq!(block.max_skill_level, "level1:index-summary");
        assert_eq!(block.injected_skill_level, "level1:index-summary");
        assert_eq!(block.injected_skill_ids, "skill.alpha,skill.beta");
        assert_eq!(block.evidence_refs, "verify:sample");
    }

    #[test]
    fn disables_skill_fields_when_policy_disables_injection() {
        let request = sample_request();
        let policy = ContextAssemblyPolicy {
            profile: "ask_profile".to_string(),
            prompt_profile: "project_answer".to_string(),
            include_session: false,
            include_memory: false,
            include_knowledge: false,
            include_tool_preview: false,
            skill_injection_enabled: false,
            max_skill_level: "disabled".to_string(),
            phase_label: "answer".to_string(),
            selection_reason: "test".to_string(),
            prefer_artifact_context: false,
        };
        let mut block = DynamicPromptBlock::default();
        fill_identity_fields(&mut block, &request, &policy);
        assert!(!block.skill_injection_enabled);
        assert_eq!(block.prompt_profile, "project_answer");
        assert_eq!(block.injected_skill_level, "disabled");
        assert_eq!(block.injected_skill_ids, "none");
    }

    #[test]
    fn tool_preview_includes_mcp_hints_when_policy_allows_tools() {
        let mut request = sample_request();
        request.context_hints.insert(
            "mcp_tool_preview".to_string(),
            "MCP工具可自动执行或按策略拒绝：web/search - 搜索网页".to_string(),
        );
        let policy = ContextAssemblyPolicy {
            profile: "act_profile".to_string(),
            prompt_profile: "agent_resolve".to_string(),
            include_session: true,
            include_memory: true,
            include_knowledge: true,
            include_tool_preview: true,
            skill_injection_enabled: true,
            max_skill_level: "level1:index-summary".to_string(),
            phase_label: "execute".to_string(),
            selection_reason: "test".to_string(),
            prefer_artifact_context: false,
        };
        let preview = selected_tool_preview(&request, &[sample_tool()], &policy);
        assert!(preview.contains("读取文件(workspace_read)"));
        assert!(preview.contains("web/search - 搜索网页"));
    }

    #[test]
    fn fill_identity_fields_surfaces_injection_summary() {
        let request = sample_request();
        let policy = ContextAssemblyPolicy {
            profile: "repair_profile".to_string(),
            prompt_profile: "agent_resolve".to_string(),
            include_session: true,
            include_memory: true,
            include_knowledge: false,
            include_tool_preview: true,
            skill_injection_enabled: true,
            max_skill_level: "level1:index-summary".to_string(),
            phase_label: "repair".to_string(),
            selection_reason: "test".to_string(),
            prefer_artifact_context: true,
        };
        let mut block = DynamicPromptBlock::default();
        fill_identity_fields(&mut block, &request, &policy);
        assert_eq!(block.assembly_profile, "repair_profile");
        assert_eq!(block.prompt_profile, "agent_resolve");
        assert!(block.injection_summary.contains("session digest"));
        assert!(block.injection_summary.contains("memory digest"));
        assert!(block.injection_summary.contains("tool preview"));
        assert!(block.injection_summary.contains("artifact hint"));
    }

    #[test]
    fn project_answer_status_digest_prefers_current_hermes_docs() {
        let request = status_request();
        let policy = ContextAssemblyPolicy {
            profile: "ask_profile".to_string(),
            prompt_profile: "project_answer".to_string(),
            include_session: false,
            include_memory: false,
            include_knowledge: true,
            include_tool_preview: false,
            skill_injection_enabled: false,
            max_skill_level: "disabled".to_string(),
            phase_label: "answer".to_string(),
            selection_reason: "test".to_string(),
            prefer_artifact_context: false,
        };
        let knowledge = selected_knowledge_selection(&request, &policy);
        assert!(knowledge.digest.contains("current-state"));
        assert!(knowledge.digest.contains("current-state.md"));
        assert!(!knowledge.digest.contains("docs\\07-test\\evidence"));
        assert_eq!(knowledge.question_type, "project_status");
        assert!(knowledge.match_reason.contains("项目状态问答"));
    }

    #[test]
    fn ask_profile_prefers_knowledge_pack_digest() {
        let request = request_with_input("agent 是什么，为什么需要 memory 和 context engineering");
        let policy = ContextAssemblyPolicy {
            profile: "ask_profile".to_string(),
            prompt_profile: "project_answer".to_string(),
            include_session: false,
            include_memory: false,
            include_knowledge: true,
            include_tool_preview: false,
            skill_injection_enabled: false,
            max_skill_level: "disabled".to_string(),
            phase_label: "answer".to_string(),
            selection_reason: "test".to_string(),
            prefer_artifact_context: false,
        };
        let knowledge = selected_knowledge_selection(&request, &policy);
        assert!(knowledge.digest.contains("knowledge pack"));
        assert!(!knowledge.question_type.is_empty());
    }

    #[test]
    fn learn_profile_prefers_knowledge_pack_digest() {
        let request = request_with_input("请学习并整理 agent workflow 的可复用做法");
        let policy = ContextAssemblyPolicy {
            profile: "learn_profile".to_string(),
            prompt_profile: "agent_resolve".to_string(),
            include_session: false,
            include_memory: false,
            include_knowledge: true,
            include_tool_preview: false,
            skill_injection_enabled: false,
            max_skill_level: "disabled".to_string(),
            phase_label: "learn".to_string(),
            selection_reason: "test".to_string(),
            prefer_artifact_context: false,
        };
        let knowledge = selected_knowledge_selection(&request, &policy);
        assert!(knowledge.digest.contains("knowledge pack"));
        assert!(!knowledge.question_type.is_empty());
    }

    #[test]
    fn selected_memory_digest_surfaces_route_metadata() {
        let request = memory_request("对象摘要");
        write_memory_entry_sqlite(&request, &sample_memory_entry("对象摘要")).unwrap();
        let policy = memory_policy();
        let memory = selected_memory_selection(&request, &policy);
        assert!(memory.digest.contains("记忆入口已按最小路由装配"));
        assert!(memory.digest.contains("learn_route"));
        assert!(memory.digest.contains("system views + history entries"));
        assert!(memory.has_current_objects);
        assert_eq!(memory.current_object_count, 1);
        assert_eq!(memory.route, "learn_route");
        assert_eq!(memory.selected_layers, "system views,history entries");
        assert!(memory.match_reason.contains("学习沉淀"));
        assert_eq!(memory.reuse_confidence, "high");
        assert_eq!(memory.skipped_layers, "current memory object");
    }

    fn sample_request() -> RunRequest {
        request_with_input("test")
    }

    fn sample_tool() -> ToolDefinition {
        ToolDefinition {
            tool_name: "workspace_read".to_string(),
            display_name: "读取文件".to_string(),
            category: "workspace_read".to_string(),
            risk_level: "low".to_string(),
            input_schema: "path".to_string(),
            output_kind: "text".to_string(),
            requires_confirmation: false,
            model_schema: None,
        }
    }

    fn status_request() -> RunRequest {
        request_with_input(
            "我现在接手这个项目，请直接告诉我：当前停在什么状态、为什么不能继续默认推进、以及以后满足什么条件才值得重启。",
        )
    }

    fn request_with_input(user_input: &str) -> RunRequest {
        let mut context_hints = BTreeMap::new();
        context_hints.insert("skill_ids".to_string(), "skill.alpha,skill.beta".to_string());
        context_hints.insert("evidence_refs".to_string(), "verify:sample".to_string());
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .unwrap()
            .display()
            .to_string();
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: user_input.to_string(),
            mode: "standard".to_string(),
            model_ref: ModelRef {
                provider_id: "provider".to_string(),
                model_id: "model".to_string(),
                display_name: "Model".to_string(),
            },
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-1".to_string(),
                name: "Workspace".to_string(),
                root_path: repo_root,
                is_active: true,
            },
            context_hints,
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }

    fn memory_request(user_input: &str) -> RunRequest {
        let root = std::env::temp_dir().join(format!("context-builder-memory-{}", crate::events::timestamp_now()));
        std::fs::create_dir_all(&root).unwrap();
        RunRequest {
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-memory".to_string(),
                name: "Workspace".to_string(),
                root_path: root.display().to_string(),
                is_active: true,
            },
            ..request_with_input(user_input)
        }
    }

    fn memory_policy() -> ContextAssemblyPolicy {
        ContextAssemblyPolicy {
            profile: "learn_profile".to_string(),
            prompt_profile: "agent_resolve".to_string(),
            include_session: false,
            include_memory: true,
            include_knowledge: false,
            include_tool_preview: false,
            skill_injection_enabled: false,
            max_skill_level: "disabled".to_string(),
            phase_label: "answer".to_string(),
            selection_reason: "test".to_string(),
            prefer_artifact_context: false,
        }
    }

    fn sample_memory_entry(summary: &str) -> MemoryEntry {
        MemoryEntry {
            id: "memory-object".to_string(),
            kind: "project_rule".to_string(),
            title: "rule-object".to_string(),
            summary: summary.to_string(),
            content: format!("content-{summary}"),
            scope: "workspace".to_string(),
            workspace_id: "workspace-memory".to_string(),
            session_id: "session-1".to_string(),
            source_run_id: "run-1".to_string(),
            source: "run:run-1".to_string(),
            source_type: "runtime".to_string(),
            source_title: "rule-object".to_string(),
            source_event_type: "run_finished".to_string(),
            source_artifact_path: String::new(),
            governance_version: "v1".to_string(),
            governance_reason: "测试".to_string(),
            governance_source: "test".to_string(),
            governance_at: "1".to_string(),
            archive_reason: String::new(),
            verified: true,
            priority: 12,
            archived: false,
            archived_at: String::new(),
            created_at: "1001".to_string(),
            updated_at: "1001".to_string(),
            timestamp: "1001".to_string(),
        }
    }
}
