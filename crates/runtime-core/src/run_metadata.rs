use std::collections::BTreeMap;

pub(crate) fn append_context_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    append_context_core_metadata(metadata, context);
    append_context_dynamic_metadata(metadata, context);
    append_context_policy_metadata(metadata, context);
}

pub(crate) fn append_tool_spec_metadata(
    metadata: &mut BTreeMap<String, String>,
    tool_call: &crate::tool_registry::ToolCall,
) {
    metadata.insert(
        "input_schema".to_string(),
        tool_call.spec.input_schema.clone(),
    );
    metadata.insert(
        "requires_confirmation".to_string(),
        bool_string(tool_call.spec.requires_confirmation),
    );
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

fn append_context_core_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert("context_mode".to_string(), context.mode.clone());
    metadata.insert(
        "context_workspace_root".to_string(),
        context.workspace_root.clone(),
    );
}

fn append_context_dynamic_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert("user_input".to_string(), context.user_input.clone());
    metadata.insert(
        "session_summary".to_string(),
        context.dynamic_block.session_summary.clone(),
    );
    metadata.insert(
        "memory_digest".to_string(),
        context.dynamic_block.memory_digest.clone(),
    );
    metadata.insert(
        "knowledge_digest".to_string(),
        context.dynamic_block.knowledge_digest.clone(),
    );
    metadata.insert(
        "tool_preview".to_string(),
        context.dynamic_block.tool_preview.clone(),
    );
    metadata.insert(
        "artifact_hint".to_string(),
        context.dynamic_block.artifact_hint.clone(),
    );
    metadata.insert(
        "reasoning_summary".to_string(),
        context.dynamic_block.reasoning_summary.clone(),
    );
    metadata.insert(
        "cache_status".to_string(),
        context.dynamic_block.cache_status.clone(),
    );
    metadata.insert(
        "cache_reason".to_string(),
        context.dynamic_block.cache_reason.clone(),
    );
}

fn append_context_policy_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert(
        "assembly_profile".to_string(),
        context.dynamic_block.assembly_profile.clone(),
    );
    append_context_includes(metadata, context);
    append_context_selection(metadata, context);
}

fn append_context_includes(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert(
        "includes_session".to_string(),
        bool_string(context.dynamic_block.includes_session),
    );
    metadata.insert(
        "includes_memory".to_string(),
        bool_string(context.dynamic_block.includes_memory),
    );
    metadata.insert(
        "includes_knowledge".to_string(),
        bool_string(context.dynamic_block.includes_knowledge),
    );
    metadata.insert(
        "includes_tool_preview".to_string(),
        bool_string(context.dynamic_block.includes_tool_preview),
    );
}

fn append_context_selection(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert(
        "phase_label".to_string(),
        context.dynamic_block.phase_label.clone(),
    );
    metadata.insert(
        "selection_reason".to_string(),
        context.dynamic_block.selection_reason.clone(),
    );
    metadata.insert(
        "prefers_artifact_context".to_string(),
        bool_string(context.dynamic_block.prefers_artifact_context),
    );
}

fn bool_string(value: bool) -> String {
    if value {
        "true".to_string()
    } else {
        "false".to_string()
    }
}
