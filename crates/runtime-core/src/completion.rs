use crate::verify::VerificationReport;

#[derive(Clone, Debug)]
pub(crate) struct CompletionDecision {
    pub status: String,
    pub reason: String,
}

pub(crate) fn decide_completion(report: &VerificationReport) -> CompletionDecision {
    if report.outcome.code == "verified_with_recovery" {
        CompletionDecision {
            status: "completed".to_string(),
            reason: "当前任务经单次受控恢复后通过验证。".to_string(),
        }
    } else if report.outcome.passed {
        CompletionDecision {
            status: "completed".to_string(),
            reason: "当前任务满足收口条件。".to_string(),
        }
    } else {
        CompletionDecision {
            status: "failed".to_string(),
            reason: "当前任务未通过验证，不能按完成收口。".to_string(),
        }
    }
}
