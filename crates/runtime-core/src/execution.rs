use crate::contracts::RunRequest;
use crate::executors::agent_resolve as agent_resolve_executor;
use crate::executors::command as command_executor;
use crate::executors::context as context_executor;
use crate::executors::explain as explain_executor;
use crate::executors::knowledge as knowledge_executor;
use crate::executors::memory as memory_executor;
use crate::executors::project as project_executor;
use crate::executors::siyuan as siyuan_executor;
use crate::executors::workspace as workspace_executor;
use crate::planner::PlannedAction;
use crate::session::SessionMemory;

#[derive(Clone, Debug)]
pub(crate) struct ActionExecution {
    pub action_summary: String,
    pub result_summary: String,
    pub final_answer: String,
    pub detail_preview: String,
    pub raw_output: String,
    pub result_chars: usize,
    pub single_result_budget_chars: usize,
    pub single_result_budget_hit: bool,
    pub success: bool,
    pub memory_write_summary: Option<String>,
    pub reasoning_summary: String,
    pub cache_status: String,
    pub cache_reason: String,
}

impl ActionExecution {
    pub(crate) fn bypass_ok(
        action_summary: String,
        result_summary: String,
        final_answer: String,
        reasoning_summary: String,
        cache_reason: &str,
    ) -> Self {
        Self::bypass(
            action_summary,
            result_summary,
            final_answer,
            true,
            None,
            reasoning_summary,
            cache_reason,
        )
    }

    pub(crate) fn bypass_fail(
        action_summary: String,
        result_summary: String,
        final_answer: String,
        reasoning_summary: String,
        cache_reason: &str,
    ) -> Self {
        Self::bypass(
            action_summary,
            result_summary,
            final_answer,
            false,
            None,
            reasoning_summary,
            cache_reason,
        )
    }

    pub(crate) fn bypass_ok_with_memory_summary(
        action_summary: String,
        memory_write_summary: String,
        final_answer: String,
        reasoning_summary: String,
        cache_reason: &str,
    ) -> Self {
        Self::bypass(
            action_summary,
            memory_write_summary.clone(),
            final_answer,
            true,
            Some(memory_write_summary),
            reasoning_summary,
            cache_reason,
        )
    }

    pub(crate) fn bypass(
        action_summary: String,
        result_summary: String,
        final_answer: String,
        success: bool,
        memory_write_summary: Option<String>,
        reasoning_summary: String,
        cache_reason: &str,
    ) -> Self {
        Self {
            action_summary,
            result_summary,
            final_answer,
            detail_preview: String::new(),
            raw_output: String::new(),
            result_chars: 0,
            single_result_budget_chars: 0,
            single_result_budget_hit: false,
            success,
            memory_write_summary,
            reasoning_summary,
            cache_status: "bypass".to_string(),
            cache_reason: cache_reason.to_string(),
        }
    }

    pub(crate) fn cached_ok(
        action_summary: String,
        result_summary: String,
        final_answer: String,
        reasoning_summary: String,
        cache_status: String,
        cache_reason: String,
    ) -> Self {
        Self::cached(
            action_summary,
            result_summary,
            final_answer,
            true,
            None,
            reasoning_summary,
            cache_status,
            cache_reason,
        )
    }

    pub(crate) fn cached(
        action_summary: String,
        result_summary: String,
        final_answer: String,
        success: bool,
        memory_write_summary: Option<String>,
        reasoning_summary: String,
        cache_status: String,
        cache_reason: String,
    ) -> Self {
        Self {
            action_summary,
            result_summary,
            final_answer,
            detail_preview: String::new(),
            raw_output: String::new(),
            result_chars: 0,
            single_result_budget_chars: 0,
            single_result_budget_hit: false,
            success,
            memory_write_summary,
            reasoning_summary,
            cache_status,
            cache_reason,
        }
    }
}

pub(crate) fn execute_action(
    request: &RunRequest,
    action: &PlannedAction,
    session_context: &SessionMemory,
) -> ActionExecution {
    if let Some(exec) = execute_action_workspace(request, action) {
        return exec;
    }
    if let Some(exec) = execute_action_memory(request, action) {
        return exec;
    }
    if let Some(exec) = execute_action_knowledge(request, action) {
        return exec;
    }
    if let Some(exec) = execute_action_siyuan(request, action) {
        return exec;
    }
    if let Some(exec) = execute_action_answers(request, action, session_context) {
        return exec;
    }
    execute_action_misc(request, action, session_context)
}

fn execute_action_workspace(
    request: &RunRequest,
    action: &PlannedAction,
) -> Option<ActionExecution> {
    match action {
        PlannedAction::ReadFile { path } => {
            Some(workspace_executor::execute_file_read(request, path))
        }
        PlannedAction::WriteFile { path, content } => Some(workspace_executor::execute_file_write(
            request, path, content,
        )),
        PlannedAction::DeletePath { path } => {
            Some(workspace_executor::execute_delete_path(request, path))
        }
        PlannedAction::ListFiles { path } => Some(workspace_executor::execute_list_files(
            request,
            path.as_deref(),
        )),
        _ => None,
    }
}

fn execute_action_memory(request: &RunRequest, action: &PlannedAction) -> Option<ActionExecution> {
    match action {
        PlannedAction::WriteMemory {
            kind,
            summary,
            content,
        } => Some(memory_executor::execute_memory_write(
            request, kind, summary, content,
        )),
        PlannedAction::RecallMemory { query } => {
            Some(memory_executor::execute_memory_recall(request, query))
        }
        _ => None,
    }
}

fn execute_action_knowledge(
    request: &RunRequest,
    action: &PlannedAction,
) -> Option<ActionExecution> {
    match action {
        PlannedAction::SearchKnowledge { query } => {
            Some(knowledge_executor::execute_knowledge_search(request, query))
        }
        _ => None,
    }
}

fn execute_action_siyuan(request: &RunRequest, action: &PlannedAction) -> Option<ActionExecution> {
    match action {
        PlannedAction::SearchSiyuanNotes { query } => {
            Some(siyuan_executor::execute_siyuan_search(request, query))
        }
        PlannedAction::ReadSiyuanNote { path } => {
            Some(siyuan_executor::execute_siyuan_read(request, path))
        }
        PlannedAction::WriteSiyuanKnowledge => Some(siyuan_executor::execute_siyuan_write(request)),
        _ => None,
    }
}

fn execute_action_answers(
    request: &RunRequest,
    action: &PlannedAction,
    session_context: &SessionMemory,
) -> Option<ActionExecution> {
    match action {
        PlannedAction::ProjectAnswer => Some(project_executor::execute_project_answer(request)),
        PlannedAction::ContextAnswer => Some(context_executor::execute_context_answer(
            request,
            session_context,
        )),
        _ => None,
    }
}

fn execute_action_misc(
    request: &RunRequest,
    action: &PlannedAction,
    session_context: &SessionMemory,
) -> ActionExecution {
    match action {
        PlannedAction::RunCommand { command } => {
            command_executor::execute_command(request, command)
        }
        PlannedAction::Explain => explain_executor::execute_explain(request),
        PlannedAction::AgentResolve => {
            agent_resolve_executor::execute_agent_resolve(request, session_context)
        }
        _ => unreachable!("unexpected planned action branch"),
    }
}

// tool_call 反解析已抽到 action_decode.rs

// AgentResolve 的执行逻辑已抽到 executors/agent_resolve.rs
