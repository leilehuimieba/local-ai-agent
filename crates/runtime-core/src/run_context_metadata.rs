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
        "memory_has_system_views".to_string(),
        bool_string(context.dynamic_block.memory_has_system_views),
    );
    metadata.insert(
        "memory_has_current_objects".to_string(),
        bool_string(context.dynamic_block.memory_has_current_objects),
    );
    metadata.insert(
        "memory_current_object_count".to_string(),
        context
            .dynamic_block
            .memory_current_object_count
            .to_string(),
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
        context
            .dynamic_block
            .observation_budget_total_tokens
            .to_string(),
    );
    metadata.insert(
        "observation_budget_used_tokens".to_string(),
        context
            .dynamic_block
            .observation_budget_used_tokens
            .to_string(),
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
    metadata.insert(
        "skill_injection_enabled".to_string(),
        bool_string(context.dynamic_block.skill_injection_enabled),
    );
    metadata.insert(
        "max_skill_level".to_string(),
        context.dynamic_block.max_skill_level.clone(),
    );
    metadata.insert(
        "injected_skill_level".to_string(),
        context.dynamic_block.injected_skill_level.clone(),
    );
    metadata.insert(
        "injected_skill_ids".to_string(),
        context.dynamic_block.injected_skill_ids.clone(),
    );
    metadata.insert(
        "evidence_refs".to_string(),
        context.dynamic_block.evidence_refs.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_builder::{
        DynamicPromptBlock, ProjectPromptBlock, RuntimeContextEnvelope, StaticPromptBlock,
    };

    #[test]
    fn append_context_metadata_keeps_memory_layer_flags() {
        let context = RuntimeContextEnvelope {
            user_input: "对象摘要".to_string(),
            mode: "standard".to_string(),
            workspace_root: "D:/repo".to_string(),
            static_block: StaticPromptBlock {
                role_prompt: String::new(),
                mode_prompt: String::new(),
            },
            project_block: ProjectPromptBlock {
                workspace_root: "D:/repo".to_string(),
                repo_summary: String::new(),
                doc_summary: String::new(),
            },
            dynamic_block: DynamicPromptBlock {
                memory_digest: "digest".to_string(),
                memory_has_system_views: true,
                memory_has_current_objects: true,
                memory_current_object_count: 2,
                ..Default::default()
            },
        };
        let mut metadata = BTreeMap::new();
        append_context_metadata(&mut metadata, &context);
        assert_eq!(
            metadata.get("memory_has_system_views"),
            Some(&"true".to_string())
        );
        assert_eq!(
            metadata.get("memory_has_current_objects"),
            Some(&"true".to_string())
        );
        assert_eq!(
            metadata.get("memory_current_object_count"),
            Some(&"2".to_string())
        );
    }
}
