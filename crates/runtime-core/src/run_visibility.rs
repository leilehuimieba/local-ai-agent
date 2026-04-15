use crate::contracts::RunEvent;

pub(crate) fn apply_visibility_metadata(events: &mut [RunEvent]) {
    for event in events {
        apply_event_visibility(event);
    }
}

fn apply_event_visibility(event: &mut RunEvent) {
    let timestamp = event.timestamp.clone();
    let trace_id = event.trace_id.clone();
    upsert_metadata(event, "activity_state", &activity_state(event));
    upsert_metadata(event, "heartbeat_at", &timestamp);
    upsert_metadata(event, "stall_seconds", "0");
    upsert_metadata(event, "waiting_reason", &waiting_reason(event));
    upsert_metadata(event, "failure_route", &failure_route(event));
    upsert_metadata(event, "updated_at", &timestamp);
    upsert_metadata(event, "next_action_hint", &next_action_hint(event));
    upsert_metadata(event, "trace_id", &trace_id);
    upsert_metadata(event, "task_title", &task_title(event));
    upsert_metadata(event, "active_tool", &active_tool(event));
    upsert_metadata(event, "evidence_ref", &evidence_ref(event));
}

fn upsert_metadata(event: &mut RunEvent, key: &str, value: &str) {
    if value.is_empty() {
        return;
    }
    event.metadata.insert(key.to_string(), value.to_string());
}

fn activity_state(event: &RunEvent) -> String {
    match event.event_type.as_str() {
        "run_finished" => "completed".to_string(),
        "run_failed" => "blocked".to_string(),
        "confirmation_required" => "waiting".to_string(),
        "checkpoint_resumed" => "retrying".to_string(),
        _ => "running".to_string(),
    }
}

fn waiting_reason(event: &RunEvent) -> String {
    if let Some(reason) = event.metadata.get("waiting_reason") {
        if !reason.is_empty() {
            return reason.clone();
        }
    }
    match event.event_type.as_str() {
        "confirmation_required" => "confirmation".to_string(),
        "checkpoint_resumed" => "retry_window".to_string(),
        "run_failed" => "failed".to_string(),
        _ => String::new(),
    }
}

fn failure_route(event: &RunEvent) -> String {
    if let Some(route) = event.metadata.get("failure_route") {
        if !route.is_empty() {
            return route.clone();
        }
    }
    match event.event_type.as_str() {
        "checkpoint_resumed" => "retry".to_string(),
        "confirmation_required" | "run_failed" => "manual".to_string(),
        _ => "none".to_string(),
    }
}

fn next_action_hint(event: &RunEvent) -> String {
    if let Some(hint) = event.metadata.get("next_action_hint") {
        if !hint.is_empty() {
            return hint.clone();
        }
    }
    if let Some(next_step) = event.metadata.get("next_step") {
        if !next_step.is_empty() {
            return next_step.clone();
        }
    }
    match event.event_type.as_str() {
        "run_finished" => "任务已结束".to_string(),
        "run_failed" => "请按失败建议处理后重试".to_string(),
        "confirmation_required" => "等待用户确认".to_string(),
        _ => "继续当前流程".to_string(),
    }
}

fn task_title(event: &RunEvent) -> String {
    if let Some(title) = event.metadata.get("task_title") {
        if !title.is_empty() {
            return title.clone();
        }
    }
    event.summary.clone()
}

fn active_tool(event: &RunEvent) -> String {
    if !event.tool_name.is_empty() {
        return event.tool_name.clone();
    }
    event.metadata.get("tool_name").cloned().unwrap_or_default()
}

fn evidence_ref(event: &RunEvent) -> String {
    if let Some(reference) = event.metadata.get("evidence_ref") {
        if !reference.is_empty() {
            return reference.clone();
        }
    }
    let artifact = pick_first(
        &event.artifact_path,
        event.metadata.get("artifact_path").map(String::as_str),
    );
    let raw = pick_first(
        &event.artifact_path,
        event.metadata.get("raw_output_ref").map(String::as_str),
    );
    let mut refs = vec![format!("event_id={}", event.event_id)];
    if !artifact.is_empty() {
        refs.push(format!("artifact_path={artifact}"));
    }
    if !raw.is_empty() {
        refs.push(format!("raw_output_ref={raw}"));
    }
    refs.join(";")
}

fn pick_first(primary: &str, fallback: Option<&str>) -> String {
    if !primary.is_empty() {
        return primary.to_string();
    }
    fallback.unwrap_or_default().to_string()
}
