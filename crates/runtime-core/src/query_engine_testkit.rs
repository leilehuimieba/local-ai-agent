#[cfg(test)]
pub(crate) mod testkit {
    use crate::checkpoint::RunCheckpoint;
    use crate::contracts::{
        ModelRef, ProviderRef, RepoContextSnapshot, RunEvent, RunRequest, RunResult,
        RuntimeRunResponse, ToolCallSnapshot, WorkspaceRef,
    };
    use crate::repo_context::RepoContextLoadResult;
    use crate::session::SessionMemory;
    use std::collections::BTreeMap;

    pub(crate) fn sample_request(strategy: &str) -> RunRequest {
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "retry task".to_string(),
            mode: "standard".to_string(),
            model_ref: sample_model_ref(),
            provider_ref: ProviderRef::default(),
            workspace_ref: sample_workspace_ref(),
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: "cp-1".to_string(),
            resume_strategy: strategy.to_string(),
            confirmation_decision: None,
        }
    }

    pub(crate) fn sample_checkpoint(reason: &str) -> RunCheckpoint {
        RunCheckpoint {
            checkpoint_id: "cp-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            workspace_id: "workspace-1".to_string(),
            status: "failed".to_string(),
            final_stage: "Finish".to_string(),
            resumable: true,
            resume_reason: reason.to_string(),
            resume_stage: "Execute".to_string(),
            event_count: 2,
            request: sample_request(reason),
            response: sample_checkpoint_response(),
            created_at: "1".to_string(),
        }
    }

    pub(crate) fn sample_session() -> SessionMemory {
        let mut session = SessionMemory::default();
        session.short_term.pending_confirmation = "need approve".to_string();
        session
    }

    pub(crate) fn sample_repo_context() -> RepoContextLoadResult {
        RepoContextLoadResult {
            snapshot: RepoContextSnapshot {
                workspace_root: "D:/repo".to_string(),
                repo_root: None,
                git_available: false,
                git_snapshot: None,
                doc_summaries: Vec::new(),
                warnings: Vec::new(),
                collected_at: "1".to_string(),
            },
            degraded: false,
            error_count: 0,
        }
    }

    pub(crate) fn sample_checkpoint_with_tool(
        tool_name: &str,
        arguments_json: &str,
    ) -> RunCheckpoint {
        let mut checkpoint = sample_checkpoint("retryable_failure");
        checkpoint
            .response
            .events
            .push(sample_tool_event(tool_name, arguments_json));
        checkpoint
    }

    fn sample_checkpoint_response() -> RuntimeRunResponse {
        RuntimeRunResponse {
            events: Vec::new(),
            result: sample_checkpoint_result(),
            confirmation_request: None,
        }
    }

    fn sample_checkpoint_result() -> RunResult {
        RunResult {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            kind: "run_result".to_string(),
            source: "runtime".to_string(),
            status: "failed".to_string(),
            final_answer: "temporary failure".to_string(),
            summary: "temporary failure".to_string(),
            error: None,
            memory_write_summary: None,
            final_stage: "Finish".to_string(),
            checkpoint_id: Some("cp-1".to_string()),
            resumable: Some(true),
        }
    }

    fn sample_tool_event(tool_name: &str, arguments_json: &str) -> RunEvent {
        RunEvent {
            tool_name: tool_name.to_string(),
            tool_call_snapshot: Some(sample_tool_snapshot(tool_name, arguments_json)),
            ..sample_action_requested_event()
        }
    }

    fn sample_tool_snapshot(tool_name: &str, arguments_json: &str) -> ToolCallSnapshot {
        ToolCallSnapshot {
            tool_name: tool_name.to_string(),
            display_name: "执行命令".to_string(),
            category: "system_command".to_string(),
            risk_level: "high".to_string(),
            input_schema: "command_text".to_string(),
            output_kind: "text_preview".to_string(),
            requires_confirmation: true,
            arguments_json: arguments_json.to_string(),
        }
    }

    fn sample_action_requested_event() -> RunEvent {
        RunEvent {
            event_id: "run-1-1".to_string(),
            kind: "run_event".to_string(),
            source: "runtime".to_string(),
            record_type: String::new(),
            source_type: String::new(),
            agent_id: "primary".to_string(),
            agent_label: "主智能体".to_string(),
            event_type: "action_requested".to_string(),
            trace_id: "trace-1".to_string(),
            session_id: "session-1".to_string(),
            run_id: "run-1".to_string(),
            sequence: 1,
            timestamp: "1".to_string(),
            stage: "Execute".to_string(),
            summary: "准备恢复动作".to_string(),
            detail: String::new(),
            tool_name: String::new(),
            tool_display_name: "执行命令".to_string(),
            tool_category: "system_command".to_string(),
            output_kind: "text_preview".to_string(),
            result_summary: String::new(),
            artifact_path: String::new(),
            risk_level: "high".to_string(),
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

    fn sample_model_ref() -> ModelRef {
        ModelRef {
            provider_id: "provider".to_string(),
            model_id: "model".to_string(),
            display_name: "Model".to_string(),
        }
    }

    fn sample_workspace_ref() -> WorkspaceRef {
        WorkspaceRef {
            workspace_id: "workspace-1".to_string(),
            name: "Workspace".to_string(),
            root_path: "D:/repo".to_string(),
            is_active: true,
        }
    }
}
