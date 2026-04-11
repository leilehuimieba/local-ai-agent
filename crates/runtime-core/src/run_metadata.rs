use std::collections::BTreeMap;

pub(crate) fn append_context_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    crate::run_context_metadata::append_context_metadata(metadata, context);
}

pub(crate) fn append_tool_spec_metadata(
    metadata: &mut BTreeMap<String, String>,
    tool_call: &crate::tool_registry::ToolCall,
) {
    crate::run_tool_metadata::append_tool_spec_metadata(metadata, tool_call);
}

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
}

fn bool_string(value: bool) -> String {
    if value {
        "true".to_string()
    } else {
        "false".to_string()
    }
}
