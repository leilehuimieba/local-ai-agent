use crate::planner::PlannedAction;
use crate::session::SessionMemory;

#[derive(Clone, Debug)]
pub(crate) struct ContextAssemblyPolicy {
    pub profile: String,
    pub include_session: bool,
    pub include_memory: bool,
    pub include_knowledge: bool,
    pub include_tool_preview: bool,
    pub phase_label: String,
    pub selection_reason: String,
    pub prefer_artifact_context: bool,
}

pub(crate) fn planning_context_policy(
    user_input: &str,
    session: &SessionMemory,
) -> ContextAssemblyPolicy {
    let mut policy = ContextAssemblyPolicy {
        profile: "planning".to_string(),
        include_session: true,
        include_memory: true,
        include_knowledge: needs_project_knowledge(user_input),
        include_tool_preview: true,
        phase_label: "plan".to_string(),
        selection_reason: "当前处于规划阶段，优先加载目标、短期状态和必要知识。".to_string(),
        prefer_artifact_context: false,
    };
    apply_session_overrides(&mut policy, session);
    policy
}

pub(crate) fn action_context_policy(
    action: &PlannedAction,
    session: &SessionMemory,
) -> ContextAssemblyPolicy {
    let mut policy = match action {
        PlannedAction::ProjectAnswer => project_answer_policy(),
        PlannedAction::ContextAnswer => context_answer_policy(),
        PlannedAction::AgentResolve => agent_resolve_policy(),
        PlannedAction::SearchKnowledge { .. }
        | PlannedAction::SearchSiyuanNotes { .. }
        | PlannedAction::ReadSiyuanNote { .. }
        | PlannedAction::WriteSiyuanKnowledge => knowledge_policy(),
        PlannedAction::WriteMemory { .. } | PlannedAction::RecallMemory { .. } => memory_policy(),
        PlannedAction::Explain => explain_policy(),
        _ => workspace_policy(),
    };
    apply_session_overrides(&mut policy, session);
    policy
}

pub(crate) fn project_answer_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "project_answer".to_string(),
        include_session: false,
        include_memory: false,
        include_knowledge: true,
        include_tool_preview: false,
        phase_label: "answer".to_string(),
        selection_reason: "当前更像项目说明或状态问答，优先使用项目知识而不是会话流水。"
            .to_string(),
        prefer_artifact_context: false,
    }
}

pub(crate) fn context_answer_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "context_answer".to_string(),
        include_session: true,
        include_memory: false,
        include_knowledge: false,
        include_tool_preview: false,
        phase_label: "continue".to_string(),
        selection_reason: "当前更像续推类问题，优先使用短期会话状态继续回答。".to_string(),
        prefer_artifact_context: false,
    }
}

fn agent_resolve_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "agent_resolve".to_string(),
        include_session: true,
        include_memory: true,
        include_knowledge: true,
        include_tool_preview: true,
        phase_label: "execute".to_string(),
        selection_reason: "当前需要较完整的执行上下文，保留会话、记忆、知识和工具预览。"
            .to_string(),
        prefer_artifact_context: false,
    }
}

fn knowledge_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "knowledge".to_string(),
        include_session: false,
        include_memory: false,
        include_knowledge: true,
        include_tool_preview: false,
        phase_label: "knowledge".to_string(),
        selection_reason: "当前是知识检索类动作，优先收紧到知识命中结果。".to_string(),
        prefer_artifact_context: false,
    }
}

fn memory_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "memory".to_string(),
        include_session: true,
        include_memory: true,
        include_knowledge: false,
        include_tool_preview: false,
        phase_label: "memory".to_string(),
        selection_reason: "当前是记忆读写动作，优先使用会话状态和长期记忆摘要。".to_string(),
        prefer_artifact_context: false,
    }
}

fn explain_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "explain".to_string(),
        include_session: false,
        include_memory: false,
        include_knowledge: false,
        include_tool_preview: true,
        phase_label: "explain".to_string(),
        selection_reason: "当前是在解释可用能力，只保留工具预览即可。".to_string(),
        prefer_artifact_context: false,
    }
}

fn workspace_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "workspace".to_string(),
        include_session: true,
        include_memory: false,
        include_knowledge: false,
        include_tool_preview: false,
        phase_label: "execute".to_string(),
        selection_reason: "当前是工作区动作，优先保留短期状态，避免引入无关知识噪声。".to_string(),
        prefer_artifact_context: false,
    }
}

fn apply_session_overrides(policy: &mut ContextAssemblyPolicy, session: &SessionMemory) {
    if has_pending_confirmation(session) {
        apply_confirmation_override(policy);
    }
    if has_handoff(session) {
        apply_handoff_override(policy);
    }
    if needs_recovery(session) {
        apply_recovery_override(policy);
    }
}

fn apply_confirmation_override(policy: &mut ContextAssemblyPolicy) {
    policy.include_session = true;
    policy.phase_label = "confirmation_resume".to_string();
    policy.selection_reason = "当前存在待确认事项，优先带入短期状态以恢复原主线。".to_string();
    mark_profile(policy, "confirm");
}

fn apply_handoff_override(policy: &mut ContextAssemblyPolicy) {
    policy.include_session = true;
    policy.include_memory = true;
    policy.prefer_artifact_context = true;
    policy.phase_label = "handoff_resume".to_string();
    policy.selection_reason =
        "当前存在长任务交接包，优先结合会话状态和交接 artifact 续跑。".to_string();
    mark_profile(policy, "handoff");
}

fn apply_recovery_override(policy: &mut ContextAssemblyPolicy) {
    policy.include_session = true;
    policy.include_memory = true;
    policy.prefer_artifact_context = true;
    policy.phase_label = "recovery".to_string();
    policy.selection_reason =
        "当前存在失败或阻塞信号，优先加载短期状态、记忆和最近交接线索。".to_string();
    mark_profile(policy, "recovery");
}

fn mark_profile(policy: &mut ContextAssemblyPolicy, suffix: &str) {
    if !policy.profile.ends_with(suffix) {
        policy.profile = format!("{}_{}", policy.profile, suffix);
    }
}

fn needs_recovery(session: &SessionMemory) -> bool {
    !session.short_term.open_issue.is_empty() || session.short_term.last_run_status == "failed"
}

fn has_pending_confirmation(session: &SessionMemory) -> bool {
    !session.short_term.pending_confirmation.is_empty()
}

fn has_handoff(session: &SessionMemory) -> bool {
    !session.short_term.handoff_artifact_path.is_empty()
}

fn needs_project_knowledge(user_input: &str) -> bool {
    let lower = user_input.to_lowercase();
    [
        "项目",
        "仓库",
        "架构",
        "文档",
        "知识",
        "思源",
        "阶段",
        "进度",
        "运行时",
    ]
    .iter()
    .any(|token| lower.contains(token))
}
