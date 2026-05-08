use super::*;

#[test]
fn clears_pending_confirmation_when_resuming_after_approval() {
    let request = sample_request("after_confirmation");
    let checkpoint = sample_checkpoint("confirmation_required");
    let mut session = sample_session();
    apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
    assert!(session.short_term.pending_confirmation.is_empty());
    assert_eq!(session.short_term.current_phase, "confirmation_resume");
}

#[test]
fn keeps_failure_context_when_resuming_retryable_failure() {
    let request = sample_request("retry_failure");
    let checkpoint = sample_checkpoint("retryable_failure");
    let mut session = sample_session();
    apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
    assert_eq!(session.short_term.current_phase, "recovery");
    assert_eq!(session.short_term.open_issue, "temporary failure");
}

#[test]
fn restores_action_from_checkpoint_tool_snapshot() {
    let request = sample_request("retry_failure");
    let checkpoint = sample_checkpoint_with_tool("run_command", r#"{"command":"echo restored"}"#);
    let session = sample_session();
    let repo = sample_repo_context();
    let visible = runtime_tool_registry().request_visible_tools(&request);
    let prepared = resumed_prepared_state(&request, &session, &repo, &visible, Some(&checkpoint));
    assert!(matches!(
        prepared.expect("prepared").action,
        crate::planner::PlannedAction::RunCommand { command }
        if command == "echo restored"
    ));
}
