use crate::checkpoint::RunCheckpoint;

pub(crate) fn resume_plan(checkpoint: &RunCheckpoint) -> String {
    let base = format!(
        "从 checkpoint 恢复：{} -> {}",
        checkpoint.resume_reason, checkpoint.resume_stage
    );
    let action = crate::run_resume_action_hint::resume_action_hint(checkpoint);
    let boundary = crate::run_resume_boundary::resume_execution_boundary(checkpoint);
    let hint = crate::run_resume_hint::resume_recovery_hint(checkpoint);
    let with_action = append_plan_segment(base, "继续动作", action);
    let with_boundary = append_plan_segment(with_action, "恢复边界", boundary);
    append_plan_segment(with_boundary, "恢复提示", hint)
}

pub(crate) fn resume_phase(checkpoint: &RunCheckpoint) -> String {
    if checkpoint.resume_reason == "confirmation_required" {
        "confirmation_resume".to_string()
    } else {
        "recovery".to_string()
    }
}

fn append_plan_segment(base: String, label: &str, value: String) -> String {
    if value.is_empty() {
        base
    } else {
        format!("{base}；{label}：{value}")
    }
}
