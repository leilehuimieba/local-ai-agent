use std::collections::BTreeMap;

pub(crate) fn append_context_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    append_context_core_metadata(metadata, context);
    append_context_dynamic_metadata(metadata, context);
    append_context_policy_metadata(metadata, context);
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
    append_context_digest_metadata(metadata, context);
    append_observation_metadata(metadata, context);
    append_runtime_feedback_metadata(metadata, context);
}

fn append_context_digest_metadata(
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
}

fn append_observation_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    append_observation_identity_metadata(metadata, context);
    append_observation_budget_metadata(metadata, context);
    append_observation_token_budget_metadata(metadata, context);
}

fn append_observation_identity_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert(
        "observation_injection".to_string(),
        context.dynamic_block.observation_injection.clone(),
    );
    metadata.insert(
        "observation_references".to_string(),
        context.dynamic_block.observation_references.clone(),
    );
}

fn append_observation_budget_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert(
        "observation_budget_total".to_string(),
        context.dynamic_block.observation_budget_total.to_string(),
    );
    metadata.insert(
        "observation_budget_used".to_string(),
        context.dynamic_block.observation_budget_used.to_string(),
    );
    metadata.insert(
        "observation_budget_hit".to_string(),
        bool_string(context.dynamic_block.observation_budget_hit),
    );
}

fn append_observation_token_budget_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
    metadata.insert(
        "observation_budget_total_tokens".to_string(),
        context.dynamic_block.observation_budget_total_tokens.to_string(),
    );
    metadata.insert(
        "observation_budget_used_tokens".to_string(),
        context.dynamic_block.observation_budget_used_tokens.to_string(),
    );
    metadata.insert(
        "observation_budget_hit_tokens".to_string(),
        bool_string(context.dynamic_block.observation_budget_hit_tokens),
    );
}

fn append_runtime_feedback_metadata(
    metadata: &mut BTreeMap<String, String>,
    context: &crate::context_builder::RuntimeContextEnvelope,
) {
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
