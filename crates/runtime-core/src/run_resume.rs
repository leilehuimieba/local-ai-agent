use crate::checkpoint::RunCheckpoint;
use crate::contracts::RunRequest;
use crate::session::SessionMemory;

pub(crate) fn apply_resume_checkpoint(
    session: &mut SessionMemory,
    checkpoint: Option<&RunCheckpoint>,
    request: &RunRequest,
) {
    let Some(checkpoint) = checkpoint else {
        return;
    };
    apply_resume_short_term_state(session, checkpoint);
    crate::run_resume_clear::clear_resume_confirmation_state(session, checkpoint, request);
}

fn apply_resume_short_term_state(session: &mut SessionMemory, checkpoint: &RunCheckpoint) {
    session.short_term.current_plan = crate::run_resume_plan::resume_plan(checkpoint);
    session.short_term.current_phase = crate::run_resume_plan::resume_phase(checkpoint);
    session.short_term.last_run_status = checkpoint.status.clone();
    session.short_term.recent_observation = resume_recent_observation(checkpoint);
    session.short_term.recent_tool_result = resume_recent_tool_result(checkpoint);
    session.short_term.handoff_artifact_path = resume_handoff_artifact_path(checkpoint);
}

fn resume_handoff_artifact_path(checkpoint: &RunCheckpoint) -> String {
    crate::run_resume_handoff::resume_handoff_artifact_path(checkpoint)
}

fn resume_recent_tool_result(checkpoint: &RunCheckpoint) -> String {
    crate::run_resume_observation::resume_recent_tool_result(checkpoint)
}

fn resume_recent_observation(checkpoint: &RunCheckpoint) -> String {
    crate::run_resume_observation::resume_recent_observation(checkpoint)
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
