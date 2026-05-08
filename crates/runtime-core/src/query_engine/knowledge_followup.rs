use crate::planner::PlannedAction;

use super::RuntimeRunState;

pub(super) fn search_knowledge_followup_action(state: &RuntimeRunState) -> Option<PlannedAction> {
    let PlannedAction::SearchKnowledge { .. } = state.action else {
        return None;
    };
    let trace = state.tool_trace.as_ref()?;
    if !trace.result.success || !agent_knowledge_question(&state.envelope.request.user_input) {
        return None;
    }
    Some(PlannedAction::ProjectAnswer)
}

pub(super) fn knowledge_answer_verify_failed(state: &RuntimeRunState) -> bool {
    let Some(report) = state.verification_report.as_ref() else {
        return false;
    };
    report.outcome.task_type == "knowledge_answer" && !report.outcome.passed
}

fn agent_knowledge_question(input: &str) -> bool {
    let lower = input.trim().to_lowercase();
    (lower.contains("agent") || lower.contains("智能体"))
        && (lower.contains("什么")
            || lower.contains("memory")
            || lower.contains("context")
            || lower.contains("可靠")
            || lower.contains("评估")
            || lower.contains("多智能体")
            || lower.contains("主循环")
            || lower.contains("loop"))
}
