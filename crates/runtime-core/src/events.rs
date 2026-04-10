use crate::contracts::{
    ConfirmationRequest, RunEvent, RunRequest, RuntimeContextSnapshot, RuntimeRunResponse,
    ToolCallSnapshot, VerificationSnapshot,
};
use crate::memory_schema::MEMORY_GOVERNANCE_VERSION;
use crate::prompt::{
    render_agent_resolve_prompt, render_context_answer_prompt, render_project_answer_prompt,
};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn make_memory_recall_event(
    request: &RunRequest,
    sequence: u32,
    metadata: &BTreeMap<String, String>,
) -> Option<RunEvent> {
    let digest = memory_digest(metadata);
    (!digest.is_empty()).then(|| {
        make_event(
            request,
            sequence,
            "memory_recalled",
            "Plan",
            &memory_recall_title(&digest),
            &digest,
            memory_recall_metadata(metadata, &digest),
        )
    })
}

pub(crate) fn with_runtime_memory_recall_event(
    request: &RunRequest,
    mut response: RuntimeRunResponse,
) -> RuntimeRunResponse {
    if response
        .events
        .iter()
        .any(|event| event.event_type == "memory_recalled")
    {
        return response;
    }
    if let Some((index, metadata)) = memory_recall_anchor(&response.events) {
        if let Some(event) = make_memory_recall_event(request, 0, metadata) {
            response.events.insert(index + 1, event);
            resequence_events(&mut response.events);
        }
    }
    response
}

pub(crate) fn make_confirmation_event(
    request: &RunRequest,
    sequence: u32,
    confirmation: &ConfirmationRequest,
) -> RunEvent {
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "confirmation_id".to_string(),
        confirmation.confirmation_id.clone(),
    );
    metadata.insert("risk_level".to_string(), confirmation.risk_level.clone());
    metadata.insert(
        "action_summary".to_string(),
        confirmation.action_summary.clone(),
    );
    metadata.insert("reason".to_string(), confirmation.reason.clone());
    metadata.insert(
        "impact_scope".to_string(),
        confirmation.impact_scope.clone(),
    );
    metadata.insert(
        "target_paths".to_string(),
        confirmation.target_paths.join("\n"),
    );
    metadata.insert(
        "reversible".to_string(),
        if confirmation.reversible {
            "true"
        } else {
            "false"
        }
        .to_string(),
    );
    metadata.insert("hazards".to_string(), confirmation.hazards.join("\n"));
    metadata.insert(
        "alternatives".to_string(),
        confirmation.alternatives.join("\n"),
    );
    metadata.insert("kind".to_string(), confirmation.kind.clone());
    metadata.insert(
        "task_title".to_string(),
        confirmation.action_summary.clone(),
    );
    metadata.insert("next_step".to_string(), "等待用户确认后再继续".to_string());

    make_event(
        request,
        sequence,
        "confirmation_required",
        "PausedForConfirmation",
        "需要人工确认后才能继续",
        &confirmation.reason,
        metadata,
    )
}

pub(crate) fn make_event(
    request: &RunRequest,
    sequence: u32,
    event_type: &str,
    stage: &str,
    summary: &str,
    detail: &str,
    metadata: BTreeMap<String, String>,
) -> RunEvent {
    let verification_snapshot = verification_snapshot(&metadata);
    RunEvent {
        event_id: format!("{}-{}-{}", request.run_id, sequence, timestamp_now()),
        kind: "run_event".to_string(),
        source: "runtime".to_string(),
        record_type: metadata.get("record_type").cloned().unwrap_or_default(),
        source_type: metadata.get("source_type").cloned().unwrap_or_default(),
        agent_id: "primary".to_string(),
        agent_label: "主智能体".to_string(),
        event_type: event_type.to_string(),
        trace_id: request.trace_id.clone(),
        session_id: request.session_id.clone(),
        run_id: request.run_id.clone(),
        sequence,
        timestamp: timestamp_now(),
        stage: stage.to_string(),
        summary: summary.to_string(),
        detail: detail.to_string(),
        tool_name: metadata.get("tool_name").cloned().unwrap_or_default(),
        tool_display_name: metadata
            .get("tool_display_name")
            .cloned()
            .unwrap_or_default(),
        tool_category: metadata.get("tool_category").cloned().unwrap_or_default(),
        output_kind: metadata.get("output_kind").cloned().unwrap_or_default(),
        result_summary: metadata.get("result_summary").cloned().unwrap_or_default(),
        artifact_path: metadata.get("artifact_path").cloned().unwrap_or_default(),
        risk_level: metadata.get("risk_level").cloned().unwrap_or_default(),
        confirmation_id: metadata.get("confirmation_id").cloned().unwrap_or_default(),
        final_answer: metadata.get("final_answer").cloned().unwrap_or_default(),
        completion_status: metadata
            .get("completion_status")
            .cloned()
            .unwrap_or_default(),
        completion_reason: metadata
            .get("completion_reason")
            .cloned()
            .unwrap_or_default(),
        verification_summary: pick_verification_summary(&metadata, verification_snapshot.as_ref()),
        checkpoint_written: metadata_flag(&metadata, "checkpoint_written"),
        context_snapshot: context_snapshot(&metadata),
        tool_call_snapshot: tool_call_snapshot(&metadata),
        verification_snapshot,
        metadata,
    }
}

fn memory_digest(metadata: &BTreeMap<String, String>) -> String {
    metadata.get("memory_digest").cloned().unwrap_or_default()
}

fn memory_recall_anchor(events: &[RunEvent]) -> Option<(usize, &BTreeMap<String, String>)> {
    events.iter().enumerate().find_map(|(index, event)| {
        (event.event_type == "plan_ready").then_some((index, &event.metadata))
    })
}

fn memory_recall_metadata(
    source: &BTreeMap<String, String>,
    digest: &str,
) -> BTreeMap<String, String> {
    let mut metadata = source.clone();
    metadata.insert("layer".to_string(), "long_term_memory".to_string());
    metadata.insert("record_type".to_string(), "recall_digest".to_string());
    metadata.insert("memory_kind".to_string(), "recall_digest".to_string());
    metadata.insert("governance_status".to_string(), "recalled".to_string());
    metadata.insert("memory_action".to_string(), "recall".to_string());
    metadata.insert(
        "governance_version".to_string(),
        MEMORY_GOVERNANCE_VERSION.to_string(),
    );
    metadata.insert(
        "governance_reason".to_string(),
        memory_recall_reason(digest),
    );
    metadata.insert(
        "governance_source".to_string(),
        "runtime_memory_recall".to_string(),
    );
    metadata.insert("governance_at".to_string(), timestamp_now());
    metadata.insert("source_type".to_string(), "runtime".to_string());
    metadata.insert(
        "source_event_type".to_string(),
        "memory_recalled".to_string(),
    );
    metadata.insert("source_artifact_path".to_string(), String::new());
    metadata.insert("archive_reason".to_string(), String::new());
    metadata.insert("title".to_string(), memory_recall_title(digest));
    metadata.insert("reason".to_string(), memory_recall_reason(digest));
    metadata.insert("result_summary".to_string(), digest.to_string());
    metadata
}

fn memory_recall_title(digest: &str) -> String {
    if digest == "当前没有命中相关长期记忆。" {
        "未命中长期记忆".to_string()
    } else {
        "已完成记忆召回".to_string()
    }
}

fn memory_recall_reason(digest: &str) -> String {
    if digest == "当前没有命中相关长期记忆。" {
        "当前查询未命中可复用长期记忆，已输出空召回结果。".to_string()
    } else {
        "已按当前输入完成长期记忆召回，并将摘要注入上下文。".to_string()
    }
}

fn pick_verification_summary(
    metadata: &BTreeMap<String, String>,
    snapshot: Option<&VerificationSnapshot>,
) -> String {
    metadata
        .get("verification_summary")
        .cloned()
        .or_else(|| snapshot.map(|item| item.summary.clone()))
        .unwrap_or_default()
}

fn context_snapshot(metadata: &BTreeMap<String, String>) -> Option<RuntimeContextSnapshot> {
    let prompts = prompt_snapshot_parts(metadata);
    let snapshot = RuntimeContextSnapshot {
        workspace_root: metadata_value(metadata, "context_workspace_root"),
        mode: metadata_value(metadata, "context_mode"),
        session_summary: metadata_value(metadata, "session_summary"),
        memory_digest: metadata_value(metadata, "memory_digest"),
        knowledge_digest: metadata_value(metadata, "knowledge_digest"),
        tool_preview: metadata_value(metadata, "tool_preview"),
        reasoning_summary: metadata_value(metadata, "reasoning_summary"),
        cache_status: metadata_value(metadata, "cache_status"),
        cache_reason: metadata_value(metadata, "cache_reason"),
        assembly_profile: metadata_value(metadata, "assembly_profile"),
        includes_session: metadata_flag(metadata, "includes_session"),
        includes_memory: metadata_flag(metadata, "includes_memory"),
        includes_knowledge: metadata_flag(metadata, "includes_knowledge"),
        includes_tool_preview: metadata_flag(metadata, "includes_tool_preview"),
        phase_label: metadata_value(metadata, "phase_label"),
        selection_reason: metadata_value(metadata, "selection_reason"),
        prefers_artifact_context: metadata_flag(metadata, "prefers_artifact_context"),
        artifact_hint: metadata_value(metadata, "artifact_hint"),
        prompt_static: prompts.0,
        prompt_project: prompts.1,
        prompt_dynamic: prompts.2,
    };
    has_context_snapshot(&snapshot).then_some(snapshot)
}

fn has_context_snapshot(snapshot: &RuntimeContextSnapshot) -> bool {
    !snapshot.workspace_root.is_empty()
        || !snapshot.mode.is_empty()
        || !snapshot.session_summary.is_empty()
        || !snapshot.memory_digest.is_empty()
        || !snapshot.knowledge_digest.is_empty()
        || !snapshot.tool_preview.is_empty()
        || !snapshot.reasoning_summary.is_empty()
        || !snapshot.cache_status.is_empty()
        || !snapshot.cache_reason.is_empty()
        || !snapshot.assembly_profile.is_empty()
        || !snapshot.phase_label.is_empty()
        || !snapshot.selection_reason.is_empty()
        || !snapshot.artifact_hint.is_empty()
        || !snapshot.prompt_static.is_empty()
        || !snapshot.prompt_project.is_empty()
        || !snapshot.prompt_dynamic.is_empty()
}

fn tool_call_snapshot(metadata: &BTreeMap<String, String>) -> Option<ToolCallSnapshot> {
    let snapshot = ToolCallSnapshot {
        tool_name: metadata.get("tool_name").cloned().unwrap_or_default(),
        display_name: metadata
            .get("tool_display_name")
            .cloned()
            .unwrap_or_default(),
        category: metadata.get("tool_category").cloned().unwrap_or_default(),
        risk_level: metadata.get("risk_level").cloned().unwrap_or_default(),
        input_schema: metadata.get("input_schema").cloned().unwrap_or_default(),
        output_kind: metadata.get("output_kind").cloned().unwrap_or_default(),
        requires_confirmation: metadata
            .get("requires_confirmation")
            .map(|value| value == "true")
            .unwrap_or(false),
    };
    has_tool_call_snapshot(&snapshot).then_some(snapshot)
}

fn has_tool_call_snapshot(snapshot: &ToolCallSnapshot) -> bool {
    !snapshot.tool_name.is_empty()
        || !snapshot.display_name.is_empty()
        || !snapshot.category.is_empty()
        || !snapshot.input_schema.is_empty()
        || !snapshot.output_kind.is_empty()
}

fn verification_snapshot(metadata: &BTreeMap<String, String>) -> Option<VerificationSnapshot> {
    let snapshot = VerificationSnapshot {
        code: metadata
            .get("verification_code")
            .cloned()
            .unwrap_or_default(),
        summary: metadata
            .get("verification_summary")
            .cloned()
            .unwrap_or_default(),
        passed: metadata
            .get("verification_passed")
            .map(|value| value == "true")
            .unwrap_or(false),
        policy: metadata
            .get("verification_policy")
            .cloned()
            .unwrap_or_default(),
        evidence: metadata
            .get("verification_evidence")
            .map(|value| split_lines(value))
            .unwrap_or_default(),
    };
    has_verification_snapshot(&snapshot).then_some(snapshot)
}

fn has_verification_snapshot(snapshot: &VerificationSnapshot) -> bool {
    !snapshot.code.is_empty()
        || !snapshot.summary.is_empty()
        || snapshot.passed
        || !snapshot.policy.is_empty()
        || !snapshot.evidence.is_empty()
}

fn split_lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn prompt_snapshot_parts(metadata: &BTreeMap<String, String>) -> (String, String, String) {
    let profile = metadata
        .get("assembly_profile")
        .map(String::as_str)
        .unwrap_or_default();
    if profile.starts_with("agent_resolve") {
        return split_prompt_sections(&render_agent_resolve_prompt(
            &prompt_user_input(metadata),
            &metadata_value(metadata, "session_summary"),
        ));
    }
    let Some(envelope) = prompt_snapshot_envelope(metadata) else {
        return (String::new(), String::new(), String::new());
    };
    let prompt = if profile.starts_with("context_answer") {
        render_context_answer_prompt(&envelope).full_prompt
    } else {
        render_project_answer_prompt(&envelope).full_prompt
    };
    split_prompt_sections(&prompt)
}

fn prompt_snapshot_envelope(
    metadata: &BTreeMap<String, String>,
) -> Option<crate::context_builder::RuntimeContextEnvelope> {
    let workspace_root = metadata.get("context_workspace_root")?.clone();
    let mode = metadata_value(metadata, "context_mode");
    Some(crate::context_builder::RuntimeContextEnvelope {
        user_input: prompt_user_input(metadata),
        mode,
        workspace_root,
        static_block: prompt_static_block(),
        project_block: prompt_project_block(metadata),
        dynamic_block: prompt_dynamic_block(metadata),
    })
}

fn split_prompt_sections(prompt: &str) -> (String, String, String) {
    let mut parts = prompt.split("\n\n");
    (
        parts.next().unwrap_or_default().to_string(),
        parts.next().unwrap_or_default().to_string(),
        parts.next().unwrap_or_default().to_string(),
    )
}

fn metadata_value(metadata: &BTreeMap<String, String>, key: &str) -> String {
    metadata.get(key).cloned().unwrap_or_default()
}

fn metadata_flag(metadata: &BTreeMap<String, String>, key: &str) -> bool {
    metadata.get(key).is_some_and(|value| value == "true")
}

fn prompt_user_input(metadata: &BTreeMap<String, String>) -> String {
    metadata
        .get("user_input")
        .cloned()
        .or_else(|| metadata.get("task_title").cloned())
        .or_else(|| metadata.get("final_answer").cloned())
        .unwrap_or_default()
}

fn prompt_static_block() -> crate::context_builder::StaticPromptBlock {
    crate::context_builder::StaticPromptBlock {
        role_prompt: "你是本地智能体，负责在当前工作区内完成真实任务。".to_string(),
        mode_prompt: String::new(),
    }
}

fn prompt_project_block(
    metadata: &BTreeMap<String, String>,
) -> crate::context_builder::ProjectPromptBlock {
    crate::context_builder::ProjectPromptBlock {
        workspace_root: metadata_value(metadata, "context_workspace_root"),
        repo_summary: String::new(),
        doc_summary: String::new(),
    }
}

fn prompt_dynamic_block(
    metadata: &BTreeMap<String, String>,
) -> crate::context_builder::DynamicPromptBlock {
    crate::context_builder::DynamicPromptBlock {
        user_input: prompt_user_input(metadata),
        assembly_profile: metadata_value(metadata, "assembly_profile"),
        includes_session: metadata_flag(metadata, "includes_session"),
        includes_memory: metadata_flag(metadata, "includes_memory"),
        includes_knowledge: metadata_flag(metadata, "includes_knowledge"),
        includes_tool_preview: metadata_flag(metadata, "includes_tool_preview"),
        phase_label: metadata_value(metadata, "phase_label"),
        selection_reason: metadata_value(metadata, "selection_reason"),
        prefers_artifact_context: metadata_flag(metadata, "prefers_artifact_context"),
        session_summary: metadata_value(metadata, "session_summary"),
        memory_digest: metadata_value(metadata, "memory_digest"),
        knowledge_digest: metadata_value(metadata, "knowledge_digest"),
        tool_preview: metadata_value(metadata, "tool_preview"),
        artifact_hint: metadata_value(metadata, "artifact_hint"),
        reasoning_summary: metadata_value(metadata, "reasoning_summary"),
        cache_status: metadata_value(metadata, "cache_status"),
        cache_reason: metadata_value(metadata, "cache_reason"),
    }
}

fn resequence_events(events: &mut [RunEvent]) {
    for (index, event) in events.iter_mut().enumerate() {
        event.sequence = index as u32 + 1;
    }
}

pub(crate) fn timestamp_now() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    millis.to_string()
}
