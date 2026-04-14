use std::collections::BTreeMap;

pub(crate) fn append_verification_metadata(
    metadata: &mut BTreeMap<String, String>,
    report: &crate::verify::VerificationReport,
) {
    append_verification_core(metadata, report);
    append_result_budget_metadata(metadata, report);
}

fn append_verification_core(
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

fn append_result_budget_metadata(
    metadata: &mut BTreeMap<String, String>,
    report: &crate::verify::VerificationReport,
) {
    metadata.insert("result_chars".to_string(), report.result_chars.to_string());
    metadata.insert(
        "single_result_budget_chars".to_string(),
        report.single_result_budget_chars.to_string(),
    );
    metadata.insert(
        "single_result_budget_hit".to_string(),
        bool_string(report.single_result_budget_hit),
    );
}

#[cfg(test)]
mod tests {
    use super::append_verification_metadata;
    use crate::verify::{VerificationOutcome, VerificationReport};
    use std::collections::BTreeMap;

    #[test]
    fn writes_single_result_budget_fields_into_metadata() {
        let mut metadata = BTreeMap::new();
        append_verification_metadata(&mut metadata, &sample_report());
        assert_eq!(metadata.get("result_chars"), Some(&"48000".to_string()));
        assert_eq!(
            metadata.get("single_result_budget_chars"),
            Some(&"30000".to_string())
        );
        assert_eq!(
            metadata.get("single_result_budget_hit"),
            Some(&"true".to_string())
        );
    }

    fn sample_report() -> VerificationReport {
        VerificationReport {
            outcome: VerificationOutcome {
                passed: true,
                code: "verified".to_string(),
                policy: "inspect_command_result".to_string(),
                evidence: vec!["summary=ok".to_string()],
                summary: "验证通过".to_string(),
                next_step: "继续".to_string(),
            },
            tool_elapsed_ms: 9,
            result_chars: 48_000,
            single_result_budget_chars: 30_000,
            single_result_budget_hit: true,
        }
    }
}
