#[cfg(test)]
pub(crate) mod testkit {
    use crate::contracts::{RunEvent, VerificationSnapshot};
    use std::collections::BTreeMap;

    pub(crate) fn sample_event(handoff_path: &str) -> RunEvent {
        sample_failed_event_with_metadata(sample_event_metadata(handoff_path))
    }

    pub(crate) fn sample_verification_event() -> RunEvent {
        let mut event = sample_event("");
        assign_event_marker(&mut event, "event-2", "verification_completed", "Verify");
        event.summary = "verification passed".to_string();
        event.metadata.insert(
            "artifact_path".to_string(),
            "D:/repo/verify/report.txt".to_string(),
        );
        event.verification_snapshot = Some(sample_verification_snapshot());
        event
    }

    pub(crate) fn sample_execution_boundary_event() -> RunEvent {
        sample_boundary_event("event-3", "action_completed", "Execute", "进入验证阶段")
    }

    pub(crate) fn sample_confirmation_boundary_event() -> RunEvent {
        sample_boundary_event(
            "event-4",
            "confirmation_required",
            "PausedForConfirmation",
            "等待用户确认后再继续",
        )
    }

    fn sample_boundary_event(
        event_id: &str,
        event_type: &str,
        stage: &str,
        next_step: &str,
    ) -> RunEvent {
        let mut event = sample_event("");
        assign_event_marker(&mut event, event_id, event_type, stage);
        event
            .metadata
            .insert("next_step".to_string(), next_step.to_string());
        event
    }

    fn assign_event_marker(event: &mut RunEvent, event_id: &str, event_type: &str, stage: &str) {
        event.event_id = event_id.to_string();
        event.event_type = event_type.to_string();
        event.stage = stage.to_string();
    }

    fn sample_event_metadata(handoff_path: &str) -> BTreeMap<String, String> {
        let mut metadata = BTreeMap::new();
        insert_default_event_metadata(&mut metadata);
        if !handoff_path.is_empty() {
            metadata.insert(
                "handoff_artifact_path".to_string(),
                handoff_path.to_string(),
            );
        }
        metadata
    }

    fn insert_default_event_metadata(metadata: &mut BTreeMap<String, String>) {
        metadata.insert("tool_name".to_string(), "run_command".to_string());
        metadata.insert("tool_display_name".to_string(), "执行命令".to_string());
        metadata.insert(
            "task_title".to_string(),
            "执行命令: Write-Error 'stage-b retry acceptance'; ex...".to_string(),
        );
        metadata.insert(
            "failure_recovery_hint".to_string(),
            "建议先检查命令语法、依赖和当前环境，再决定是否重试。".to_string(),
        );
    }

    fn sample_failed_event_with_metadata(metadata: BTreeMap<String, String>) -> RunEvent {
        let mut event = sample_failed_event_base();
        event.metadata = metadata;
        event
    }

    fn sample_failed_event_base() -> RunEvent {
        sample_failed_event_core()
    }

    fn sample_failed_event_core() -> RunEvent {
        RunEvent {
            event_id: "event-1".to_string(),
            kind: "run_event".to_string(),
            source: "runtime".to_string(),
            record_type: String::new(),
            source_type: String::new(),
            agent_id: "primary".to_string(),
            agent_label: "主智能体".to_string(),
            event_type: "run_failed".to_string(),
            trace_id: "trace-1".to_string(),
            session_id: "session-1".to_string(),
            run_id: "run-1".to_string(),
            sequence: 1,
            timestamp: "1".to_string(),
            stage: "Failed".to_string(),
            summary: "failed".to_string(),
            ..sample_failed_event_empty_fields()
        }
    }

    fn sample_failed_event_empty_fields() -> RunEvent {
        let mut event = sample_failed_event_identity_placeholders();
        assign_failed_event_runtime_defaults(&mut event);
        event
    }

    fn assign_failed_event_runtime_defaults(event: &mut RunEvent) {
        event.detail = String::new();
        event.tool_name = String::new();
        event.tool_display_name = String::new();
        event.tool_category = String::new();
        event.output_kind = String::new();
        event.result_summary = String::new();
        event.artifact_path = String::new();
        event.risk_level = String::new();
        event.confirmation_id = String::new();
        event.final_answer = String::new();
        event.completion_status = String::new();
        event.completion_reason = String::new();
        event.verification_summary = String::new();
    }

    fn sample_failed_event_identity_placeholders() -> RunEvent {
        RunEvent {
            event_id: String::new(),
            kind: String::new(),
            source: String::new(),
            record_type: String::new(),
            source_type: String::new(),
            agent_id: String::new(),
            agent_label: String::new(),
            event_type: String::new(),
            trace_id: String::new(),
            session_id: String::new(),
            run_id: String::new(),
            sequence: 0,
            timestamp: String::new(),
            stage: String::new(),
            summary: String::new(),
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

    fn sample_verification_snapshot() -> VerificationSnapshot {
        VerificationSnapshot {
            code: "verified".to_string(),
            summary: "验证通过并产生产物".to_string(),
            passed: true,
            policy: "inspect_command_result".to_string(),
            evidence: vec![
                "summary=ok".to_string(),
                "artifact=D:/repo/verify/report.txt".to_string(),
            ],
        }
    }
}
