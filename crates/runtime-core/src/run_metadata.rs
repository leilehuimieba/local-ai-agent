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
    crate::run_verification_metadata::append_verification_metadata(metadata, report);
}
