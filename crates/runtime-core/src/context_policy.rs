use crate::planner::PlannedAction;

#[derive(Clone, Debug)]
pub(crate) struct ContextAssemblyPolicy {
    pub profile: &'static str,
    pub include_session: bool,
    pub include_memory: bool,
    pub include_knowledge: bool,
    pub include_tool_preview: bool,
}

pub(crate) fn planning_context_policy(user_input: &str) -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "planning",
        include_session: true,
        include_memory: true,
        include_knowledge: needs_project_knowledge(user_input),
        include_tool_preview: true,
    }
}

pub(crate) fn action_context_policy(action: &PlannedAction) -> ContextAssemblyPolicy {
    match action {
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
    }
}

pub(crate) fn project_answer_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "project_answer",
        include_session: false,
        include_memory: false,
        include_knowledge: true,
        include_tool_preview: false,
    }
}

pub(crate) fn context_answer_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "context_answer",
        include_session: true,
        include_memory: false,
        include_knowledge: false,
        include_tool_preview: false,
    }
}

fn agent_resolve_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "agent_resolve",
        include_session: true,
        include_memory: true,
        include_knowledge: true,
        include_tool_preview: true,
    }
}

fn knowledge_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "knowledge",
        include_session: false,
        include_memory: false,
        include_knowledge: true,
        include_tool_preview: false,
    }
}

fn memory_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "memory",
        include_session: true,
        include_memory: true,
        include_knowledge: false,
        include_tool_preview: false,
    }
}

fn explain_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "explain",
        include_session: false,
        include_memory: false,
        include_knowledge: false,
        include_tool_preview: true,
    }
}

fn workspace_policy() -> ContextAssemblyPolicy {
    ContextAssemblyPolicy {
        profile: "workspace",
        include_session: true,
        include_memory: false,
        include_knowledge: false,
        include_tool_preview: false,
    }
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
