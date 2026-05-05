use crate::planner::PlannedAction;
use crate::session::SessionMemory;

#[derive(Clone, Debug)]
pub(crate) struct ContextAssemblyPolicy {
    pub profile: String,
    pub prompt_profile: String,
    pub include_session: bool,
    pub include_memory: bool,
    pub include_knowledge: bool,
    pub include_tool_preview: bool,
    pub skill_injection_enabled: bool,
    pub max_skill_level: String,
    pub phase_label: String,
    pub selection_reason: String,
    pub prefer_artifact_context: bool,
}

pub(crate) fn planning_context_policy(user_input: &str, session: &SessionMemory) -> ContextAssemblyPolicy {
    apply_session_overrides(base_ask_policy(user_input), session)
}

pub(crate) fn action_context_policy(action: &PlannedAction, session: &SessionMemory) -> ContextAssemblyPolicy {
    apply_session_overrides(base_action_policy(action), session)
}

pub(crate) fn project_answer_policy() -> ContextAssemblyPolicy {
    base_project_answer_policy()
}

pub(crate) fn context_answer_policy() -> ContextAssemblyPolicy {
    base_context_answer_policy()
}

fn base_action_policy(action: &PlannedAction) -> ContextAssemblyPolicy {
    match action {
        PlannedAction::ProjectAnswer => base_project_answer_policy(),
        PlannedAction::ContextAnswer => base_context_answer_policy(),
        PlannedAction::Explain => explain_policy(),
        PlannedAction::SearchKnowledge { .. }
        | PlannedAction::SearchSiyuanNotes { .. }
        | PlannedAction::ReadSiyuanNote { .. }
        | PlannedAction::WriteSiyuanKnowledge
        | PlannedAction::WriteMemory { .. }
        | PlannedAction::RecallMemory { .. } => learn_policy(),
        _ => act_policy(),
    }
}

fn base_ask_policy(user_input: &str) -> ContextAssemblyPolicy {
    if needs_project_knowledge(user_input) {
        ContextAssemblyPolicy {
            profile: "ask_profile".to_string(),
            prompt_profile: "project_answer".to_string(),
            include_session: true,
            include_memory: false,
            include_knowledge: true,
            include_tool_preview: false,
            skill_injection_enabled: false,
            max_skill_level: "disabled".to_string(),
            phase_label: "ask".to_string(),
            selection_reason: "当前更像解释、比较或项目问答，优先加载会话摘要和项目知识最小包。".to_string(),
            prefer_artifact_context: false,
        }
    } else {
        base_context_answer_policy()
    }
}

fn base_project_answer_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "ask_profile".to_string(),
        prompt_profile: "project_answer".to_string(),
        include_session: true,
        include_memory: false,
        include_knowledge: true,
        include_tool_preview: false,
        skill_injection_enabled: false,
        max_skill_level: "disabled".to_string(),
        phase_label: "ask".to_string(),
        selection_reason: "当前是项目说明或状态问答，优先使用会话摘要和项目知识片段。".to_string(),
        prefer_artifact_context: false,
    }
}

fn base_context_answer_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "ask_profile".to_string(),
        prompt_profile: "context_answer".to_string(),
        include_session: true,
        include_memory: false,
        include_knowledge: false,
        include_tool_preview: false,
        skill_injection_enabled: false,
        max_skill_level: "disabled".to_string(),
        phase_label: "ask".to_string(),
        selection_reason: "当前是续推、解释或轻问答，优先使用短期会话摘要直接回答。".to_string(),
        prefer_artifact_context: false,
    }
}

fn act_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "act_profile".to_string(),
        prompt_profile: "agent_resolve".to_string(),
        include_session: true,
        include_memory: false,
        include_knowledge: false,
        include_tool_preview: true,
        skill_injection_enabled: true,
        max_skill_level: "level1:index-summary".to_string(),
        phase_label: "act".to_string(),
        selection_reason: "当前需要执行或推进动作，优先加载目标文件线索、工具预览和短期状态。".to_string(),
        prefer_artifact_context: false,
    }
}

fn learn_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "learn_profile".to_string(),
        prompt_profile: "agent_resolve".to_string(),
        include_session: false,
        include_memory: true,
        include_knowledge: true,
        include_tool_preview: false,
        skill_injection_enabled: false,
        max_skill_level: "disabled".to_string(),
        phase_label: "learn".to_string(),
        selection_reason: "当前是知识或记忆相关动作，优先收紧到可复用知识与记忆摘要。".to_string(),
        prefer_artifact_context: false,
    }
}

fn explain_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "ask_profile".to_string(),
        prompt_profile: "context_answer".to_string(),
        include_session: false,
        include_memory: false,
        include_knowledge: false,
        include_tool_preview: true,
        skill_injection_enabled: false,
        max_skill_level: "disabled".to_string(),
        phase_label: "ask".to_string(),
        selection_reason: "当前是在解释能力边界，只保留工具预览即可。".to_string(),
        prefer_artifact_context: false,
    }
}

fn apply_session_overrides(mut policy: ContextAssemblyPolicy, session: &SessionMemory) -> ContextAssemblyPolicy {
    if has_pending_confirmation(session) {
        apply_confirmation_override(&mut policy);
    }
    if has_handoff(session) {
        apply_handoff_override(&mut policy);
    }
    if needs_recovery(session) {
        apply_recovery_override(&mut policy);
    }
    policy
}

fn apply_confirmation_override(policy: &mut ContextAssemblyPolicy) {
    policy.profile = "repair_profile".to_string();
    policy.prompt_profile = "agent_resolve".to_string();
    policy.include_session = true;
    policy.include_memory = false;
    policy.include_knowledge = false;
    policy.include_tool_preview = false;
    policy.skill_injection_enabled = false;
    policy.max_skill_level = "disabled".to_string();
    policy.phase_label = "repair".to_string();
    policy.selection_reason = "当前存在待确认事项，优先带入短期状态恢复原主线，不注入无关知识。".to_string();
    policy.prefer_artifact_context = false;
}

fn apply_handoff_override(policy: &mut ContextAssemblyPolicy) {
    policy.profile = "repair_profile".to_string();
    policy.prompt_profile = "agent_resolve".to_string();
    policy.include_session = true;
    policy.include_memory = true;
    policy.include_knowledge = false;
    policy.include_tool_preview = true;
    policy.skill_injection_enabled = true;
    policy.max_skill_level = "level1:index-summary".to_string();
    policy.phase_label = "repair".to_string();
    policy.selection_reason = "当前存在长任务交接包，优先结合会话状态、记忆摘要和交接 artifact 续跑。".to_string();
    policy.prefer_artifact_context = true;
}

fn apply_recovery_override(policy: &mut ContextAssemblyPolicy) {
    policy.profile = "repair_profile".to_string();
    policy.prompt_profile = "agent_resolve".to_string();
    policy.include_session = true;
    policy.include_memory = true;
    policy.include_knowledge = false;
    policy.include_tool_preview = true;
    policy.skill_injection_enabled = true;
    policy.max_skill_level = "level1:index-summary".to_string();
    policy.phase_label = "repair".to_string();
    policy.selection_reason = "当前存在失败或阻塞信号，优先加载短期状态、失败线索和最近交接信息。".to_string();
    policy.prefer_artifact_context = true;
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
    ["项目", "仓库", "架构", "文档", "知识", "思源", "阶段", "进度", "运行时"]
        .iter()
        .any(|token| user_input.contains(token))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{SessionMemory, ShortTermMemory};

    #[test]
    fn project_answer_uses_ask_profile() {
        let policy = project_answer_policy();
        assert_eq!(policy.profile, "ask_profile");
        assert_eq!(policy.prompt_profile, "project_answer");
        assert!(!policy.skill_injection_enabled);
    }

    #[test]
    fn agent_resolve_uses_act_profile() {
        let policy = action_context_policy(&PlannedAction::AgentResolve, &empty_session());
        assert_eq!(policy.profile, "act_profile");
        assert_eq!(policy.prompt_profile, "agent_resolve");
        assert!(policy.include_tool_preview);
    }

    #[test]
    fn knowledge_action_uses_learn_profile() {
        let policy = action_context_policy(
            &PlannedAction::SearchKnowledge {
                query: "runtime".to_string(),
            },
            &empty_session(),
        );
        assert_eq!(policy.profile, "learn_profile");
        assert!(policy.include_memory);
        assert!(policy.include_knowledge);
    }

    #[test]
    fn recovery_override_forces_repair_profile() {
        let session = SessionMemory {
            session_id: "session-1".to_string(),
            short_term: ShortTermMemory {
                open_issue: "temporary failure".to_string(),
                last_run_status: "failed".to_string(),
                ..Default::default()
            },
            recent_turns: Vec::new(),
            compressed_summary: String::new(),
        };
        let policy = action_context_policy(&PlannedAction::AgentResolve, &session);
        assert_eq!(policy.profile, "repair_profile");
        assert_eq!(policy.phase_label, "repair");
        assert!(!policy.include_knowledge);
    }

    fn empty_session() -> SessionMemory {
        SessionMemory {
            session_id: "session-1".to_string(),
            short_term: ShortTermMemory::default(),
            recent_turns: Vec::new(),
            compressed_summary: String::new(),
        }
    }
}
