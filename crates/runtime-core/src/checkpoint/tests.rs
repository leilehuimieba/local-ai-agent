use super::{
    checkpoint_resume_event, load_runtime_checkpoint, with_checkpoint_resume_event,
    with_runtime_checkpoint,
};
use crate::contracts::{
    ErrorInfo, ModelRef, ProviderRef, RunRequest, RunResult, RuntimeRunResponse, WorkspaceRef,
};
use std::collections::BTreeMap;
use std::fs;

#[test]
fn writes_checkpoint_and_inserts_event() {
    let root = test_root("writes_checkpoint_and_inserts_event");
    let request = sample_request(&root);
    let response = with_runtime_checkpoint(&request, sample_response(&request));
    let checkpoint_id = response.result.checkpoint_id.clone().unwrap_or_default();
    let loaded = load_runtime_checkpoint(&request, &checkpoint_id).unwrap_or(None);
    assert!(!checkpoint_id.is_empty());
    assert!(
        response
            .events
            .iter()
            .any(|item| item.event_type == "checkpoint_written")
    );
    assert_eq!(
        response.events.last().map(|item| item.event_type.as_str()),
        Some("run_finished")
    );
    assert!(loaded.is_some());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn resumes_from_matching_confirmation_checkpoint() {
    let root = test_root("resumes_from_matching_confirmation_checkpoint");
    let request = sample_request(&root);
    let response = with_runtime_checkpoint(&request, awaiting_response(&request));
    let mut resumed = request.clone();
    resumed.resume_from_checkpoint_id = response.result.checkpoint_id.unwrap_or_default();
    resumed.resume_strategy = "after_confirmation".to_string();
    resumed.confirmation_decision = Some(approve_decision(&request.run_id));
    let event = checkpoint_resume_event(&resumed);
    let response = with_checkpoint_resume_event(sample_response(&resumed), event);
    let resumed_event = response
        .events
        .iter()
        .find(|item| item.event_type == "checkpoint_resumed")
        .expect("checkpoint_resumed");
    assert_eq!(
        resumed_event
            .metadata
            .get("checkpoint_resume_boundary")
            .map(String::as_str),
        Some(
            "stage=PausedForConfirmation;event=confirmation_required;next_step=等待用户确认后再继续"
        )
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn marks_retryable_failure_checkpoint_as_resumable() {
    let root = test_root("marks_retryable_failure_checkpoint_as_resumable");
    let request = sample_request(&root);
    let response = with_runtime_checkpoint(&request, retryable_failure_response(&request));
    let checkpoint_id = response.result.checkpoint_id.clone().unwrap_or_default();
    let loaded = load_runtime_checkpoint(&request, &checkpoint_id).unwrap_or(None);
    assert!(loaded.as_ref().is_some_and(|item| item.resumable));
    assert!(
        loaded
            .as_ref()
            .is_some_and(|item| item.resume_reason == "retryable_failure")
    );
    assert!(
        loaded
            .as_ref()
            .is_some_and(|item| item.resume_stage == "Execute")
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn carries_verification_fields_on_retry_resume_event() {
    let root = test_root("carries_verification_fields_on_retry_resume_event");
    let request = sample_request(&root);
    let response = with_runtime_checkpoint(&request, retryable_failure_verified_response(&request));
    let mut resumed = request.clone();
    resumed.resume_from_checkpoint_id = response.result.checkpoint_id.unwrap_or_default();
    resumed.resume_strategy = "retry_failure".to_string();
    let event = checkpoint_resume_event(&resumed).expect("resume event");
    assert_eq!(event.event_type, "checkpoint_resumed");
    assert_eq!(
        event.metadata.get("checkpoint_resume_verification_code"),
        Some(&"verification_failed".to_string())
    );
    assert_eq!(
        event.metadata.get("checkpoint_resume_verification_summary"),
        Some(&"验证失败：命令执行失败".to_string())
    );
    assert_eq!(
        event.metadata.get("checkpoint_resume_artifact_path"),
        Some(&"D:/repo/command.txt".to_string())
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn emits_retry_resume_metadata_for_acceptance_filters() {
    let root = test_root("emits_retry_resume_metadata_for_acceptance_filters");
    let request = sample_request(&root);
    let response = with_runtime_checkpoint(&request, retryable_failure_response(&request));
    let checkpoint_id = response.result.checkpoint_id.clone().unwrap_or_default();
    let mut resumed = request.clone();
    resumed.resume_from_checkpoint_id = checkpoint_id.clone();
    resumed.resume_strategy = "retry_failure".to_string();
    let event = checkpoint_resume_event(&resumed).expect("resume event");
    let response = with_checkpoint_resume_event(sample_response(&resumed), Some(event));
    let candidates: Vec<_> = response
        .events
        .iter()
        .filter(|item| item.event_type == "checkpoint_resumed")
        .filter(|item| {
            item.metadata.get("checkpoint_resume_reason") == Some(&"retryable_failure".to_string())
        })
        .filter(|item| item.metadata.get("checkpoint_stage") == Some(&"Execute".to_string()))
        .filter(|item| item.metadata.get("checkpoint_id") == Some(&checkpoint_id))
        .collect();
    assert_eq!(candidates.len(), 1);
    assert_eq!(
        candidates[0].metadata.get("checkpoint_resume_boundary"),
        Some(&"stage=Finish;event=run_failed".to_string())
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn emits_confirmation_resume_metadata_for_acceptance_filters() {
    let root = test_root("emits_confirmation_resume_metadata_for_acceptance_filters");
    let request = sample_request(&root);
    let response = with_runtime_checkpoint(&request, awaiting_response(&request));
    let checkpoint_id = response.result.checkpoint_id.clone().unwrap_or_default();
    let mut resumed = request.clone();
    resumed.resume_from_checkpoint_id = checkpoint_id.clone();
    resumed.resume_strategy = "after_confirmation".to_string();
    resumed.confirmation_decision = Some(approve_decision(&request.run_id));
    let event = checkpoint_resume_event(&resumed).expect("resume event");
    let response = with_checkpoint_resume_event(sample_response(&resumed), Some(event));
    let candidates: Vec<_> = response
        .events
        .iter()
        .filter(|item| item.event_type == "checkpoint_resumed")
        .filter(|item| {
            item.metadata.get("checkpoint_resume_reason")
                == Some(&"confirmation_required".to_string())
        })
        .filter(|item| {
            item.metadata.get("checkpoint_stage") == Some(&"PausedForConfirmation".to_string())
        })
        .filter(|item| item.metadata.get("checkpoint_id") == Some(&checkpoint_id))
        .collect();
    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].metadata.get("checkpoint_resume_boundary"), Some(&"stage=PausedForConfirmation;event=confirmation_required;next_step=等待用户确认后再继续".to_string()));
    assert_eq!(
        candidates[0].metadata.get("confirmation_id"),
        Some(&format!("confirm-risk-{}", request.run_id))
    );
    assert_eq!(
        candidates[0].metadata.get("confirmation_decision"),
        Some(&"approve".to_string())
    );
    assert_eq!(
        candidates[0].metadata.get("confirmation_resume_strategy"),
        Some(&"after_confirmation".to_string())
    );
    assert_eq!(
        candidates[0].metadata.get("confirmation_chain_step"),
        Some(&"resumed".to_string())
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn persists_minimal_resume_fields_for_confirmation_scope() {
    let root = test_root("persists_minimal_resume_fields_for_confirmation_scope");
    let request = sample_request(&root);
    let response = with_runtime_checkpoint(&request, awaiting_response(&request));
    let checkpoint_id = response.result.checkpoint_id.clone().unwrap_or_default();
    let loaded = load_runtime_checkpoint(&request, &checkpoint_id).unwrap_or(None);
    let checkpoint = loaded.expect("checkpoint");
    assert_eq!(checkpoint.checkpoint_id, checkpoint_id);
    assert_eq!(checkpoint.run_id, request.run_id);
    assert_eq!(checkpoint.session_id, request.session_id);
    assert_eq!(checkpoint.trace_id, request.trace_id);
    assert_eq!(checkpoint.workspace_id, request.workspace_ref.workspace_id);
    assert_eq!(checkpoint.status, "awaiting_confirmation");
    assert_eq!(checkpoint.final_stage, "PausedForConfirmation");
    assert!(checkpoint.resumable);
    assert_eq!(checkpoint.resume_reason, "confirmation_required");
    assert_eq!(checkpoint.resume_stage, "PausedForConfirmation");
    assert_eq!(checkpoint.request.run_id, request.run_id);
    assert_eq!(checkpoint.response.result.run_id, request.run_id);
    let _ = fs::remove_dir_all(root);
}

#[test]
fn skips_resume_when_scope_keys_do_not_match() {
    let root = test_root("skips_resume_when_scope_keys_do_not_match");
    let request = sample_request(&root);
    let response = with_runtime_checkpoint(&request, retryable_failure_response(&request));
    let checkpoint_id = response.result.checkpoint_id.clone().unwrap_or_default();
    let mut mismatched = request.clone();
    mismatched.resume_from_checkpoint_id = checkpoint_id;
    mismatched.resume_strategy = "retry_failure".to_string();
    mismatched.run_id = "run-2".to_string();
    let event = checkpoint_resume_event(&mismatched).expect("resume event");
    assert_eq!(event.event_type, "checkpoint_resume_skipped");
    assert!(!event.metadata.contains_key("checkpoint_resume_boundary"));
    let _ = fs::remove_dir_all(root);
}

fn test_root(case_name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("runtime-core-{case_name}"));
    let _ = fs::remove_dir_all(&root);
    let _ = fs::create_dir_all(root.join("workspace"));
    root
}

fn sample_request(root: &std::path::Path) -> RunRequest {
    RunRequest {
        request_id: "request-1".to_string(),
        run_id: "run-1".to_string(),
        session_id: "session-1".to_string(),
        trace_id: "trace-1".to_string(),
        user_input: "test".to_string(),
        mode: "standard".to_string(),
        model_ref: ModelRef {
            provider_id: "provider".to_string(),
            model_id: "model".to_string(),
            display_name: "Model".to_string(),
        },
        provider_ref: ProviderRef {
            provider_id: "provider".to_string(),
            display_name: "Provider".to_string(),
            base_url: "https://example.com".to_string(),
            chat_completions_path: "/chat".to_string(),
            models_path: "/models".to_string(),
            api_key: "secret".to_string(),
        },
        workspace_ref: WorkspaceRef {
            workspace_id: "workspace-1".to_string(),
            name: "Workspace".to_string(),
            root_path: root.join("workspace").display().to_string(),
            is_active: true,
        },
        context_hints: BTreeMap::from([("repo_root".to_string(), root.display().to_string())]),
        resume_from_checkpoint_id: String::new(),
        resume_strategy: String::new(),
        confirmation_decision: None,
    }
}

fn sample_response(request: &RunRequest) -> RuntimeRunResponse {
    RuntimeRunResponse {
        events: vec![
            sample_event(request, 1, "run_started", "Analyze"),
            sample_event(request, 2, "run_finished", "Finish"),
        ],
        result: RunResult {
            request_id: request.request_id.clone(),
            run_id: request.run_id.clone(),
            session_id: request.session_id.clone(),
            trace_id: request.trace_id.clone(),
            kind: "run_result".to_string(),
            source: "runtime".to_string(),
            status: "completed".to_string(),
            final_answer: "ok".to_string(),
            summary: "ok".to_string(),
            error: None,
            memory_write_summary: None,
            final_stage: "Finish".to_string(),
            checkpoint_id: None,
            resumable: None,
        },
        confirmation_request: None,
    }
}

fn awaiting_response(request: &RunRequest) -> RuntimeRunResponse {
    let mut response = sample_response(request);
    response.result.status = "awaiting_confirmation".to_string();
    response.result.final_stage = "PausedForConfirmation".to_string();
    response.events[1].event_type = "confirmation_required".to_string();
    response.events[1].stage = "PausedForConfirmation".to_string();
    response
}

fn approve_decision(run_id: &str) -> crate::contracts::ConfirmationDecision {
    crate::contracts::ConfirmationDecision {
        confirmation_id: format!("confirm-risk-{run_id}"),
        run_id: run_id.to_string(),
        decision: "approve".to_string(),
        note: String::new(),
        remember: false,
    }
}

fn retryable_failure_response(request: &RunRequest) -> RuntimeRunResponse {
    RuntimeRunResponse {
        events: vec![
            sample_event(request, 1, "run_started", "Analyze"),
            sample_event(request, 2, "run_failed", "Finish"),
            sample_event(request, 3, "run_finished", "Finish"),
        ],
        result: RunResult {
            request_id: request.request_id.clone(),
            run_id: request.run_id.clone(),
            session_id: request.session_id.clone(),
            trace_id: request.trace_id.clone(),
            kind: "run_result".to_string(),
            source: "runtime".to_string(),
            status: "failed".to_string(),
            final_answer: "temporary failure".to_string(),
            summary: "temporary failure".to_string(),
            error: Some(ErrorInfo {
                error_code: "action_execution_failed".to_string(),
                message: "temporary failure".to_string(),
                summary: "temporary failure".to_string(),
                retryable: true,
                source: "runtime".to_string(),
                stage: "Finish".to_string(),
                metadata: BTreeMap::new(),
            }),
            memory_write_summary: None,
            final_stage: "Finish".to_string(),
            checkpoint_id: None,
            resumable: None,
        },
        confirmation_request: None,
    }
}

fn retryable_failure_verified_response(request: &RunRequest) -> RuntimeRunResponse {
    let mut response = retryable_failure_response(request);
    response.events[1].metadata.insert(
        "verification_code".to_string(),
        "verification_failed".to_string(),
    );
    response.events[1].metadata.insert(
        "verification_summary".to_string(),
        "验证失败：命令执行失败".to_string(),
    );
    response.events[1].metadata.insert(
        "artifact_path".to_string(),
        "D:/repo/command.txt".to_string(),
    );
    response
}

fn sample_event(
    request: &RunRequest,
    sequence: u32,
    event_type: &str,
    stage: &str,
) -> crate::contracts::RunEvent {
    crate::contracts::RunEvent {
        event_id: format!("{}-{sequence}", request.run_id),
        kind: "run_event".to_string(),
        source: "runtime".to_string(),
        record_type: String::new(),
        source_type: String::new(),
        agent_id: "primary".to_string(),
        agent_label: "主智能体".to_string(),
        event_type: event_type.to_string(),
        trace_id: request.trace_id.clone(),
        session_id: request.session_id.clone(),
        run_id: request.run_id.clone(),
        sequence,
        timestamp: "1".to_string(),
        stage: stage.to_string(),
        summary: event_type.to_string(),
        detail: String::new(),
        tool_name: String::new(),
        tool_display_name: String::new(),
        tool_category: String::new(),
        output_kind: String::new(),
        result_summary: String::new(),
        artifact_path: String::new(),
        risk_level: String::new(),
        confirmation_id: String::new(),
        final_answer: String::new(),
        completion_status: String::new(),
        completion_reason: String::new(),
        verification_summary: String::new(),
        checkpoint_written: false,
        context_snapshot: None,
        tool_call_snapshot: None,
        verification_snapshot: None,
        metadata: BTreeMap::new(),
    }
}
