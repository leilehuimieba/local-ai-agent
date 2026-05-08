use crate::planner::{PlannedAction, action_step_label};

use super::RuntimeRunState;
use super::browser_followup::{
    browser_interaction_needs_readback, browser_interaction_verify_failed, browser_read_page_action,
};
use super::knowledge_followup::{knowledge_answer_verify_failed, search_knowledge_followup_action};
use super::path_followup::{next_list_action, next_read_action};

pub(crate) fn should_replan(state: &RuntimeRunState) -> bool {
    if verify_failed_for_replan(state) {
        return true;
    }
    if search_knowledge_followup_action(state).is_some() {
        return true;
    }
    matches!(
        state.action,
        PlannedAction::ReadFile { .. } | PlannedAction::ListFiles { .. }
    ) && state.tool_trace.as_ref().is_some_and(|trace| trace.result.success)
}

pub(crate) fn replan_state(state: &RuntimeRunState, iteration_index: u8) -> Option<RuntimeRunState> {
    let next_action = next_action_from_trace(state)?;
    let mut next_state = state.clone();
    next_state.action = next_action.clone();
    next_state.tool_call.action = next_action.clone();
    next_state.tool_call.spec =
        crate::capabilities::resolve_tool_for_request(&next_state.envelope.request, &next_action);
    next_state.task_title = crate::derive_task_title(&next_action, &next_state.envelope.request.user_input);
    next_state.analysis_detail = format!(
        "主循环基于上一步结果触发补充动作：{}。",
        action_step_label(&next_action)
    );
    next_state.envelope.context_envelope = crate::run_state_builder::rebuild_execution_context(
        &next_state.envelope.request,
        &next_state.envelope.session_context,
        &next_state.envelope.repo_context,
        &next_state.envelope.visible_tools,
        &next_action,
    );
    next_state.risk_outcome = crate::risk::assess_risk(&next_state.envelope.request, &next_action);
    next_state.tool_trace = None;
    next_state.verification_report = None;
    apply_iteration_metadata(&mut next_state, iteration_index, &next_action);
    Some(next_state)
}

pub(crate) fn budget_exhausted(state: &RuntimeRunState) -> bool {
    state.plan_envelope.iteration_index >= state.plan_envelope.max_iterations
}

fn next_action_from_trace(state: &RuntimeRunState) -> Option<PlannedAction> {
    if knowledge_answer_verify_failed(state) {
        return Some(PlannedAction::SearchKnowledge {
            query: state.envelope.request.user_input.clone(),
        });
    }
    if browser_interaction_verify_failed(state) {
        return browser_read_page_action(&state.action, state.tool_trace.as_ref()?);
    }
    if let Some(action) = search_knowledge_followup_action(state) {
        return Some(action);
    }
    let trace = state.tool_trace.as_ref()?;
    match &state.action {
        PlannedAction::ListFiles { path } => next_read_action(path.as_deref(), &trace.result.final_answer),
        PlannedAction::ReadFile { path } => next_list_action(path),
        _ => None,
    }
}

fn remaining_steps_for_action(action: &PlannedAction) -> Vec<String> {
    match action {
        PlannedAction::ReadFile { .. } => vec!["整理文件内容并判断是否还需补充目录观察".to_string()],
        PlannedAction::ListFiles { .. } => vec!["从目录结果中选择候选文件继续读取".to_string()],
        PlannedAction::SearchKnowledge { .. } => vec!["补充知识命中并重新形成带引证回答".to_string()],
        PlannedAction::MCPCall {
            server_id, tool_name, ..
        } if server_id == "browser" && tool_name == "read_page" => {
            vec!["回读页面并确认交互结果是否已经可见".to_string()]
        }
        _ => vec!["完成验证并决定是否收口".to_string()],
    }
}

fn verify_failed_for_replan(state: &RuntimeRunState) -> bool {
    knowledge_answer_verify_failed(state) || browser_interaction_needs_readback(state)
}

fn apply_iteration_metadata(state: &mut RuntimeRunState, iteration_index: u8, action: &PlannedAction) {
    state.plan_envelope.iteration_index = iteration_index;
    state.plan_envelope.current_step = action_step_label(action);
    state.plan_envelope.remaining_steps = remaining_steps_for_action(action);
}
