use crate::contracts::{
    ConfirmationRequest, RunEvent, RunRequest, RuntimeContextSnapshot, ToolCallSnapshot,
    VerificationSnapshot,
};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

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
        context_snapshot: context_snapshot(&metadata),
        tool_call_snapshot: tool_call_snapshot(&metadata),
        verification_snapshot,
        metadata,
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
    let snapshot = RuntimeContextSnapshot {
        workspace_root: metadata
            .get("context_workspace_root")
            .cloned()
            .unwrap_or_default(),
        mode: metadata.get("context_mode").cloned().unwrap_or_default(),
        session_summary: metadata.get("session_summary").cloned().unwrap_or_default(),
        memory_digest: metadata.get("memory_digest").cloned().unwrap_or_default(),
        knowledge_digest: metadata
            .get("knowledge_digest")
            .cloned()
            .unwrap_or_default(),
        tool_preview: metadata.get("tool_preview").cloned().unwrap_or_default(),
        reasoning_summary: metadata
            .get("reasoning_summary")
            .cloned()
            .unwrap_or_default(),
        cache_status: metadata.get("cache_status").cloned().unwrap_or_default(),
        cache_reason: metadata.get("cache_reason").cloned().unwrap_or_default(),
        prompt_static: String::new(),
        prompt_project: String::new(),
        prompt_dynamic: String::new(),
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
    };
    has_verification_snapshot(&snapshot).then_some(snapshot)
}

fn has_verification_snapshot(snapshot: &VerificationSnapshot) -> bool {
    !snapshot.code.is_empty() || !snapshot.summary.is_empty() || snapshot.passed
}

pub(crate) fn timestamp_now() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    millis.to_string()
}
