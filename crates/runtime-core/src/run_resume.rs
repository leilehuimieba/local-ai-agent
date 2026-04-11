use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::run_resume_extract::{
    resume_action_hint, resume_artifact_path, resume_execution_boundary, resume_recovery_hint,
    resume_verification_summary,
};
use crate::session::SessionMemory;

pub(crate) fn apply_resume_checkpoint(
    session: &mut SessionMemory,
    checkpoint: Option<&RunCheckpoint>,
    request: &RunRequest,
) {
    let Some(checkpoint) = checkpoint else {
        return;
    };
    session.short_term.current_plan = resume_plan(checkpoint);
    session.short_term.current_phase = resume_phase(checkpoint);
    session.short_term.last_run_status = checkpoint.status.clone();
    session.short_term.recent_observation = resume_recent_observation(checkpoint);
    session.short_term.recent_tool_result = resume_recent_tool_result(checkpoint);
    session.short_term.handoff_artifact_path = resume_handoff_artifact_path(checkpoint);
    clear_resume_confirmation_state(session, checkpoint, request);
}

fn clear_resume_confirmation_state(
    session: &mut SessionMemory,
    checkpoint: &RunCheckpoint,
    request: &RunRequest,
) {
    if checkpoint.resume_reason == "confirmation_required" {
        session.short_term.pending_confirmation.clear();
        session.short_term.open_issue.clear();
        return;
    }
    if request.resume_strategy == "retry_failure" {
        session.short_term.pending_confirmation.clear();
        session.short_term.open_issue = checkpoint.response.result.summary.clone();
    }
}

fn resume_plan(checkpoint: &RunCheckpoint) -> String {
    let base = format!(
        "从 checkpoint 恢复：{} -> {}",
        checkpoint.resume_reason, checkpoint.resume_stage
    );
    let action = resume_action_hint(checkpoint);
    let boundary = resume_execution_boundary(checkpoint);
    let hint = resume_recovery_hint(checkpoint);
    let with_action = if action.is_empty() {
        base
    } else {
        format!("{base}；继续动作：{action}")
    };
    let with_boundary = if boundary.is_empty() {
        with_action
    } else {
        format!("{with_action}；恢复边界：{boundary}")
    };
    if hint.is_empty() {
        with_boundary
    } else {
        format!("{with_boundary}；恢复提示：{hint}")
    }
}

fn resume_phase(checkpoint: &RunCheckpoint) -> String {
    if checkpoint.resume_reason == "confirmation_required" {
        "confirmation_resume".to_string()
    } else {
        "recovery".to_string()
    }
}

fn resume_handoff_artifact_path(checkpoint: &RunCheckpoint) -> String {
    checkpoint
        .response
        .events
        .iter()
        .rev()
        .find_map(|event| event.metadata.get("handoff_artifact_path").cloned())
        .unwrap_or_default()
}

fn resume_recent_tool_result(checkpoint: &RunCheckpoint) -> String {
    let verification = resume_verification_summary(checkpoint);
    if verification.is_empty() {
        checkpoint.response.result.summary.clone()
    } else {
        format!("验证快照：{verification}")
    }
}

fn resume_recent_observation(checkpoint: &RunCheckpoint) -> String {
    let answer = checkpoint.response.result.final_answer.clone();
    let artifact = resume_artifact_path(checkpoint);
    if artifact.is_empty() {
        return answer;
    }
    if answer.is_empty() {
        return format!("恢复到产物：{artifact}");
    }
    format!("{answer}；产物：{artifact}")
}

#[cfg(test)]
mod tests {
    use super::apply_resume_checkpoint;
    use crate::run_resume_testkit::testkit::{
        sample_checkpoint, sample_checkpoint_with_confirmation_boundary,
        sample_checkpoint_with_execution_boundary, sample_checkpoint_with_verification_snapshot,
        sample_request,
    };
    use crate::session::SessionMemory;

    #[test]
    fn restores_handoff_artifact_path_for_retry_recovery() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert_eq!(session.short_term.current_phase, "recovery");
        assert_eq!(
            session.short_term.handoff_artifact_path,
            "D:/repo/handoff.json"
        );
    }

    #[test]
    fn leaves_handoff_artifact_path_empty_when_checkpoint_has_none() {
        let request = sample_request("after_confirmation");
        let checkpoint = sample_checkpoint("confirmation_required", "");
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(session.short_term.handoff_artifact_path.is_empty());
    }

    #[test]
    fn appends_last_action_hint_into_resume_plan() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(
            session
                .short_term
                .current_plan
                .contains("继续动作：执行命令")
        );
        assert!(
            session
                .short_term
                .current_plan
                .contains("执行命令: Write-Error")
        );
    }

    #[test]
    fn appends_recovery_hint_into_resume_plan() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint("retryable_failure", "D:/repo/handoff.json");
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(
            session
                .short_term
                .current_plan
                .contains("恢复提示：建议先检查命令语法")
        );
    }

    #[test]
    fn appends_execution_boundary_into_resume_plan() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint_with_execution_boundary();
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(
            session
                .short_term
                .current_plan
                .contains("恢复边界：阶段=Execute，事件=action_completed，下一步=进入验证阶段")
        );
    }

    #[test]
    fn restores_verification_snapshot_into_short_term_memory() {
        let request = sample_request("retry_failure");
        let checkpoint = sample_checkpoint_with_verification_snapshot();
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(
            session
                .short_term
                .recent_tool_result
                .contains("验证快照：验证通过并产生产物")
        );
        assert!(
            session
                .short_term
                .recent_observation
                .contains("D:/repo/verify/report.txt")
        );
    }

    #[test]
    fn appends_confirmation_boundary_when_no_execution_event() {
        let request = sample_request("after_confirmation");
        let checkpoint = sample_checkpoint_with_confirmation_boundary();
        let mut session = SessionMemory::default();
        apply_resume_checkpoint(&mut session, Some(&checkpoint), &request);
        assert!(
            session
                .short_term
                .current_plan
                .contains("恢复边界：阶段=PausedForConfirmation，事件=confirmation_required")
        );
    }

}
