use super::{
    browser_click_tool_call, browser_recovery_exhausted_trace, browser_select_tool_call, browser_select_trace,
    browser_state_unchanged_trace, browser_submit_tool_call, browser_submit_trace, browser_target_missing_trace,
    browser_trace, browser_upload_tool_call, browser_upload_trace, verify_tool_execution,
};

#[test]
fn browser_interaction_requires_state_target_and_risk_signal() {
    let report = verify_tool_execution(&browser_click_tool_call(), &browser_trace(false));
    assert!(!report.outcome.passed);
    assert_eq!(report.outcome.task_type, "browser_interaction");
    assert_eq!(report.outcome.code, "browser_interaction_insufficient");
    assert_eq!(report.outcome.browser_failure_type, "permission_or_risk_gap");
}

#[test]
fn browser_interaction_passes_with_state_target_and_risk_signal() {
    let report = verify_tool_execution(&browser_click_tool_call(), &browser_trace(true));
    assert!(report.outcome.passed);
    assert_eq!(report.outcome.task_type, "browser_interaction");
    assert_eq!(report.outcome.policy, "confirm_browser_state_change");
}

#[test]
fn browser_select_passes_with_selection_signal() {
    let report = verify_tool_execution(&browser_select_tool_call(), &browser_select_trace(true));
    assert!(report.outcome.passed);
    assert_eq!(report.outcome.task_type, "browser_interaction");
}

#[test]
fn browser_submit_passes_with_submit_signal() {
    let report = verify_tool_execution(&browser_submit_tool_call(), &browser_submit_trace(true));
    assert!(report.outcome.passed);
    assert_eq!(report.outcome.task_type, "browser_interaction");
}

#[test]
fn browser_upload_passes_with_upload_signal() {
    let report = verify_tool_execution(&browser_upload_tool_call(), &browser_upload_trace(true));
    assert!(report.outcome.passed);
    assert_eq!(report.outcome.task_type, "browser_interaction");
}

#[test]
fn browser_failure_classifies_target_not_found() {
    let report = verify_tool_execution(&browser_click_tool_call(), &browser_target_missing_trace());
    assert_eq!(report.outcome.browser_failure_type, "target_not_found");
    assert_eq!(report.outcome.browser_page_id, "page_01");
    assert!(report.outcome.browser_selector.is_empty());
}

#[test]
fn browser_failure_classifies_state_not_changed() {
    let report = verify_tool_execution(&browser_click_tool_call(), &browser_state_unchanged_trace());
    assert_eq!(report.outcome.browser_failure_type, "state_not_changed");
    assert_eq!(report.outcome.browser_selector, "#advance");
}

#[test]
fn browser_failure_classifies_recovery_exhausted() {
    let report = verify_tool_execution(&browser_click_tool_call(), &browser_recovery_exhausted_trace());
    assert_eq!(report.outcome.browser_failure_type, "recovery_exhausted");
    assert!(report.outcome.browser_recovery_attempted);
    assert!(report.outcome.browser_recovery_exhausted);
}
