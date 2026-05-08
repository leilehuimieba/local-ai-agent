use super::{knowledge_tool_call, knowledge_trace, verify_tool_execution};

#[test]
fn knowledge_answer_requires_citation_and_fact_boundary() {
    let report = verify_tool_execution(&knowledge_tool_call(), &knowledge_trace(false));
    assert!(!report.outcome.passed);
    assert_eq!(report.outcome.task_type, "knowledge_answer");
    assert_eq!(report.outcome.code, "knowledge_answer_insufficient");
    assert!(!report.outcome.has_citation);
    assert!(!report.outcome.fact_inference_split);
}

#[test]
fn knowledge_answer_passes_with_citation_and_fact_boundary() {
    let report = verify_tool_execution(&knowledge_tool_call(), &knowledge_trace(true));
    assert!(report.outcome.passed);
    assert_eq!(report.outcome.task_type, "knowledge_answer");
    assert_eq!(report.outcome.policy, "check_knowledge_answer");
    assert!(report.outcome.has_citation);
    assert!(report.outcome.fact_inference_split);
}
