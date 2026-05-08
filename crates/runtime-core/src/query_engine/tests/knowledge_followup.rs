use super::*;

#[test]
fn search_knowledge_replans_to_project_answer_for_agent_questions() {
    let state = sample_knowledge_state("agent 是什么");
    let next = replan_state(&state, 2).expect("next state");
    assert!(matches!(next.action, PlannedAction::ProjectAnswer));
    assert_eq!(
        next.envelope.context_envelope.dynamic_block.prompt_profile,
        "project_answer"
    );
}

#[test]
fn search_knowledge_stays_put_for_non_agent_questions() {
    let state = sample_knowledge_state("这个项目里哪个云租户的生产 SLA 是多少");
    assert!(replan_state(&state, 2).is_none());
}
