#[cfg(test)]
pub(crate) mod testkit {
    use crate::contracts::{RunEvent, VerificationSnapshot};
    use std::collections::BTreeMap;

    pub(crate) fn sample_event(handoff_path: &str) -> RunEvent {
        sample_failed_event_with_metadata(sample_event_metadata(handoff_path))
    }

    pub(crate) fn sample_verification_event() -> RunEvent {
        let mut event = sample_event("");
        event.event_id = "event-2".to_string();
        event.event_type = "verification_completed".to_string();
        event.stage = "Verify".to_string();
        event.summary = "verification passed".to_string();
        event.metadata.insert(
            "artifact_path".to_string(),
            "D:/repo/verify/report.txt".to_string(),
        );
        event.verification_snapshot = Some(sample_verification_snapshot());
        event
    }

    pub(crate) fn sample_execution_boundary_event() -> RunEvent {
        let mut event = sample_event("");
        event.event_id = "event-3".to_string();
        event.event_type = "action_completed".to_string();
        event.stage = "Execute".to_string();
        event
            .metadata
            .insert("next_step".to_string(), "进入验证阶段".to_string());
        event
    }

    pub(crate) fn sample_confirmation_boundary_event() -> RunEvent {
        let mut event = sample_event("");
        event.event_id = "event-4".to_string();
        event.event_type = "confirmation_required".to_string();
        event.stage = "PausedForConfirmation".to_string();
        event
            .metadata
            .insert("next_step".to_string(), "等待用户确认后再继续".to_string());
        event
    }

    fn sample_event_metadata(handoff_path: &str) -> BTreeMap<String, String> {
        let mut metadata = BTreeMap::new();
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
        if !handoff_path.is_empty() {
            metadata.insert(
                "handoff_artifact_path".to_string(),
                handoff_path.to_string(),
            );
        }
        metadata
    }

    fn sample_failed_event_with_metadata(metadata: BTreeMap<String, String>) -> RunEvent {
        let mut event = sample_failed_event_base();
        event.metadata = metadata;
        event
    }

    fn sample_failed_event_base() -> RunEvent {
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
