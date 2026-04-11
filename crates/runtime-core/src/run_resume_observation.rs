use crate::checkpoint::RunCheckpoint;

pub(crate) fn resume_recent_tool_result(checkpoint: &RunCheckpoint) -> String {
    let verification = crate::run_resume_verification::resume_verification_summary(checkpoint);
    if verification.is_empty() {
        checkpoint.response.result.summary.clone()
    } else {
        format!("验证快照：{verification}")
    }
}

pub(crate) fn resume_recent_observation(checkpoint: &RunCheckpoint) -> String {
    let answer = checkpoint.response.result.final_answer.clone();
    let artifact = crate::run_resume_artifact::resume_artifact_path(checkpoint);
    join_observation_with_artifact(answer, artifact)
}

fn join_observation_with_artifact(answer: String, artifact: String) -> String {
    if artifact.is_empty() {
        return answer;
    }
    if answer.is_empty() {
        return format!("恢复到产物：{artifact}");
    }
    format!("{answer}；产物：{artifact}")
}
