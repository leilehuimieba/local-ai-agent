use crate::checkpoint::RunCheckpoint;

pub(crate) fn resume_plan(checkpoint: &RunCheckpoint) -> String {
    let base = format!(
        "从 checkpoint 恢复：{} -> {}",
        checkpoint.resume_reason, checkpoint.resume_stage
    );
    let action = crate::run_resume_action_hint::resume_action_hint(checkpoint);
    let boundary = crate::run_resume_boundary::resume_execution_boundary(checkpoint);
    let hint = crate::run_resume_hint::resume_recovery_hint(checkpoint);
    let with_action = with_action(base, action);
    let with_boundary = with_boundary(with_action, boundary);
    with_hint(with_boundary, hint)
}

pub(crate) fn resume_phase(checkpoint: &RunCheckpoint) -> String {
    if checkpoint.resume_reason == "confirmation_required" {
        "confirmation_resume".to_string()
    } else {
        "recovery".to_string()
    }
}

fn with_action(base: String, action: String) -> String {
    if action.is_empty() {
        base
    } else {
        format!("{base}；继续动作：{action}")
    }
}

fn with_boundary(base: String, boundary: String) -> String {
    if boundary.is_empty() {
        base
    } else {
        format!("{base}；恢复边界：{boundary}")
    }
}

fn with_hint(base: String, hint: String) -> String {
    if hint.is_empty() {
        base
    } else {
        format!("{base}；恢复提示：{hint}")
    }
}
