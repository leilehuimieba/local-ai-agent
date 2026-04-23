use crate::contracts::{ErrorInfo, RunEvent, RunRequest};
use crate::events::make_event;
use crate::memory_layer::reasoning_layer_summary;
use crate::repo_context::repo_context_metadata;
use crate::run_failure_metadata::{
    append_error_metadata, append_tool_failure_metadata, failure_next_step,
};
use crate::run_memory_metadata::append_memory_governance_metadata;

pub(crate) fn read_result_mode(
    final_answer: &str,
    completion_status: &str,
    verification_code: &str,
) -> &'static str {
    if completion_status == "failed" {
        return "system";
    }
    if verification_code == "verified_with_recovery" {
        return "recovery";
    }
    if final_answer.trim().is_empty() {
        return "system";
    }
    "answer"
}

pub(crate) fn make_run_failed_event(
    request: &RunRequest,
    sequence: u32,
    summary: &str,
    detail: &str,
    error: &ErrorInfo,
    tool_trace: Option<&crate::capabilities::ToolExecutionTrace>,
    task_title: &str,
    repo_context: &crate::repo_context::RepoContextLoadResult,
) -> RunEvent {
    let mut metadata = repo_context_metadata(repo_context);
    metadata.insert("task_title".to_string(), task_title.to_string());
    metadata.insert("result_mode".to_string(), "system".to_string());
    append_error_metadata(&mut metadata, error);
    append_tool_failure_metadata(&mut metadata, tool_trace);
    append_recall_visibility_metadata(&mut metadata, tool_trace);
    let summary = run_failed_summary(summary, tool_trace);
    metadata.insert(
        "next_step".to_string(),
        failure_next_step(tool_trace, error),
    );
    make_event(
        request,
        sequence,
        "run_failed",
        "Failed",
        &summary,
        detail,
        metadata,
    )
}

pub(crate) fn make_memory_event(
    request: &RunRequest,
    sequence: u32,
    task_title: &str,
    repo_context: &crate::repo_context::RepoContextLoadResult,
    outcome: &crate::memory_router::MemoryWriteOutcome,
) -> RunEvent {
    let mut metadata = repo_context_metadata(repo_context);
    metadata.insert("task_title".to_string(), task_title.to_string());
    metadata.insert("layer".to_string(), outcome.layer.to_string());
    metadata.insert("record_type".to_string(), outcome.record_type.clone());
    if !outcome.source_type.is_empty() {
        metadata.insert("source_type".to_string(), outcome.source_type.clone());
    }
    append_memory_governance_metadata(&mut metadata, outcome);
    metadata.insert("title".to_string(), outcome.title.clone());
    metadata.insert("reason".to_string(), outcome.reason.clone());
    make_event(
        request,
        sequence,
        outcome.event_type,
        "Finish",
        &outcome.title,
        &outcome.summary,
        metadata,
    )
}

pub(crate) fn append_recall_visibility_metadata(
    metadata: &mut std::collections::BTreeMap<String, String>,
    tool_trace: Option<&crate::capabilities::ToolExecutionTrace>,
) {
    let Some(trace) = tool_trace else {
        return;
    };
    if trace.tool.tool_name != "memory_recall" {
        return;
    }
    let layer = recall_layer_from_reasoning(&trace.result.reasoning_summary);
    if layer.is_empty() {
        return;
    }
    metadata.insert("recall_layer_summary".to_string(), layer.clone());
    metadata.insert(
        "result_summary".to_string(),
        format!("{}（{}）", trace.result.summary, layer),
    );
}

pub(crate) fn run_finished_summary(
    success: bool,
    tool_trace: &crate::capabilities::ToolExecutionTrace,
) -> String {
    if tool_trace.tool.tool_name != "memory_recall" {
        return if success {
            "任务已完成".to_string()
        } else {
            "任务已结束，存在执行失败".to_string()
        };
    }
    let layer = recall_layer_from_reasoning(&tool_trace.result.reasoning_summary);
    if layer.is_empty() {
        return if success {
            "任务已完成".to_string()
        } else {
            "任务已结束，存在执行失败".to_string()
        };
    }
    if success {
        format!("任务已完成（记忆召回：{}）", layer)
    } else {
        format!("任务已结束，存在执行失败（记忆召回：{}）", layer)
    }
}

fn run_failed_summary(summary: &str, tool_trace: Option<&crate::capabilities::ToolExecutionTrace>) -> String {
    let Some(trace) = tool_trace else {
        return summary.to_string();
    };
    if trace.tool.tool_name != "memory_recall" {
        return summary.to_string();
    }
    let layer = recall_layer_from_reasoning(&trace.result.reasoning_summary);
    if layer.is_empty() {
        summary.to_string()
    } else {
        format!("{summary}（记忆召回：{layer}）")
    }
}

fn recall_layer_from_reasoning(reasoning: &str) -> String {
    reasoning_layer_summary(reasoning)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::{ToolCallResult, ToolDefinition, ToolExecutionTrace};
    use crate::repo_context::RepoContextLoadResult;

    #[test]
    fn run_finished_summary_surfaces_recall_layer() {
        let trace = sample_memory_recall_trace(true);
        let summary = run_finished_summary(true, &trace);
        assert!(summary.contains("记忆召回"));
        assert!(summary.contains("system views + current memory object"));
    }

    #[test]
    fn run_failed_event_surfaces_recall_layer() {
        let request = sample_request();
        let trace = sample_memory_recall_trace(false);
        let error = ErrorInfo {
            error_code: "failed".to_string(),
            message: "failed".to_string(),
            summary: "failed".to_string(),
            retryable: false,
            source: "runtime".to_string(),
            stage: "Finish".to_string(),
            metadata: std::collections::BTreeMap::new(),
        };
        let event = make_run_failed_event(
            &request,
            1,
            "当前动作执行失败",
            "failed",
            &error,
            Some(&trace),
            "task",
            &sample_repo_context(),
        );
        assert!(event.summary.contains("记忆召回"));
        assert_eq!(
            event.metadata.get("recall_layer_summary"),
            Some(&"system views + current memory object，对象 2 条".to_string())
        );
    }

    fn sample_request() -> RunRequest {
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "对象摘要".to_string(),
            mode: "standard".to_string(),
            model_ref: crate::contracts::ModelRef {
                provider_id: "provider".to_string(),
                model_id: "model".to_string(),
                display_name: "Model".to_string(),
            },
            provider_ref: crate::contracts::ProviderRef::default(),
            workspace_ref: crate::contracts::WorkspaceRef {
                workspace_id: "workspace-1".to_string(),
                name: "Workspace".to_string(),
                root_path: "D:/repo".to_string(),
                is_active: true,
            },
            context_hints: std::collections::BTreeMap::new(),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }

    fn sample_repo_context() -> RepoContextLoadResult {
        RepoContextLoadResult {
            snapshot: crate::contracts::RepoContextSnapshot {
                workspace_root: "D:/repo".to_string(),
                repo_root: Some("D:/repo".to_string()),
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

    fn sample_memory_recall_trace(success: bool) -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: ToolDefinition {
                tool_name: "memory_recall".to_string(),
                display_name: "召回记忆".to_string(),
                category: "memory_read".to_string(),
                risk_level: "low".to_string(),
                input_schema: "query".to_string(),
                output_kind: "text_preview".to_string(),
                requires_confirmation: false,
            },
            action_summary: "按需召回记忆：对象摘要".to_string(),
            result: ToolCallResult {
                summary: "已召回 2 条相关记忆。".to_string(),
                final_answer: "已召回相关长期记忆。".to_string(),
                artifact_path: None,
                detail_preview: String::new(),
                raw_output_ref: None,
                result_chars: 10,
                single_result_budget_chars: 30000,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 10,
                retryable: !success,
                success,
                memory_write_summary: None,
                reasoning_summary:
                    "按查询词检索长期记忆，并返回前几条高相关结果；本次召回层为system views + current memory object，对象 2 条。"
                        .to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }
}
