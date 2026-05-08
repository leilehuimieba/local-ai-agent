use super::{
    memory_recall_tool_call, sample_memory_recall_trace, sample_tool_call, sample_trace, verify_tool_execution,
};

#[test]
fn exposes_skill_hit_fields_on_success() {
    let report = verify_tool_execution(&sample_tool_call(), &sample_trace(true, false));
    assert!(report.outcome.skill_hit_effective);
    assert!(!report.outcome.guard_downgraded);
    assert_eq!(report.outcome.guard_decision_ref, "tool=run_command;decision=allow");
}

#[test]
fn exposes_guard_downgrade_fields_when_reasoning_marks_review() {
    let report = verify_tool_execution(&sample_tool_call(), &sample_trace(true, true));
    assert!(report.outcome.skill_hit_effective);
    assert!(report.outcome.guard_downgraded);
    assert_eq!(report.outcome.guard_decision_ref, "tool=run_command;decision=review");
}

#[test]
fn verification_summary_keeps_object_aware_recall_reasoning() {
    let report = verify_tool_execution(
        &memory_recall_tool_call(),
        &sample_memory_recall_trace("system views + current memory object，对象 2 条"),
    );
    assert!(report.outcome.summary.contains("system views + current memory object"));
    assert!(
        report
            .outcome
            .evidence
            .iter()
            .any(|item| item.contains("system views + current memory object"))
    );
}
