use std::collections::BTreeMap;

pub(crate) fn append_verification_metadata(
    metadata: &mut BTreeMap<String, String>,
    report: &crate::verify::VerificationReport,
) {
    metadata.insert("verification_code".to_string(), report.outcome.code.clone());
    metadata.insert(
        "verification_passed".to_string(),
        bool_string(report.outcome.passed),
    );
    metadata.insert(
        "verification_summary".to_string(),
        report.outcome.summary.clone(),
    );
    metadata.insert(
        "verification_next_step".to_string(),
        report.outcome.next_step.clone(),
    );
    metadata.insert(
        "verification_policy".to_string(),
        report.outcome.policy.clone(),
    );
    metadata.insert(
        "verification_evidence".to_string(),
        report.outcome.evidence.join("\n"),
    );
    metadata.insert(
        "tool_elapsed_ms".to_string(),
        report.tool_elapsed_ms.to_string(),
    );
}

fn bool_string(value: bool) -> String {
    if value {
        "true".to_string()
    } else {
        "false".to_string()
    }
}
