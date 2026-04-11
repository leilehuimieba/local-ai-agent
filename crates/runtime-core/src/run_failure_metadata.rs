use crate::contracts::ErrorInfo;
use std::collections::BTreeMap;

pub(crate) fn append_error_metadata(metadata: &mut BTreeMap<String, String>, error: &ErrorInfo) {
    metadata.insert("error_code".to_string(), error.error_code.clone());
    metadata.insert("error_message".to_string(), error.message.clone());
    metadata.insert("error_source".to_string(), error.source.clone());
    let retryable = if error.retryable { "true" } else { "false" };
    metadata.insert("retryable".to_string(), retryable.to_string());
}

pub(crate) fn append_tool_failure_metadata(
    metadata: &mut BTreeMap<String, String>,
    tool_trace: Option<&crate::capabilities::ToolExecutionTrace>,
) {
    let Some(trace) = tool_trace else {
        return;
    };
    append_tool_identity(metadata, trace);
    append_tool_outcome(metadata, trace);
    append_tool_cache(metadata, trace);
    if let Some(path) = trace.result.artifact_path.clone() {
        metadata.insert("artifact_path".to_string(), path);
    }
}

pub(crate) fn failure_next_step(
    tool_trace: Option<&crate::capabilities::ToolExecutionTrace>,
    error: &ErrorInfo,
) -> String {
    if !error.retryable {
        return "当前失败不建议直接重试，建议先缩小影响范围或改成更安全的动作。".to_string();
    }
    tool_trace
        .map(|trace| tool_failure_hint(trace.tool.tool_name.as_str()))
        .unwrap_or_else(|| "建议先查看错误详情，再补上下文或调整任务后继续。".to_string())
}

fn append_tool_identity(
    metadata: &mut BTreeMap<String, String>,
    trace: &crate::capabilities::ToolExecutionTrace,
) {
    metadata.insert("tool_name".to_string(), trace.tool.tool_name.clone());
    metadata.insert(
        "tool_display_name".to_string(),
        trace.tool.display_name.clone(),
    );
    metadata.insert("tool_category".to_string(), trace.tool.category.clone());
    metadata.insert("output_kind".to_string(), trace.tool.output_kind.clone());
}

fn append_tool_outcome(
    metadata: &mut BTreeMap<String, String>,
    trace: &crate::capabilities::ToolExecutionTrace,
) {
    metadata.insert("result_summary".to_string(), trace.result.summary.clone());
    metadata.insert("risk_level".to_string(), trace.tool.risk_level.clone());
    metadata.insert(
        "reasoning_summary".to_string(),
        trace.result.reasoning_summary.clone(),
    );
    metadata.insert(
        "failure_recovery_hint".to_string(),
        tool_failure_hint(trace.tool.tool_name.as_str()),
    );
}

fn append_tool_cache(
    metadata: &mut BTreeMap<String, String>,
    trace: &crate::capabilities::ToolExecutionTrace,
) {
    metadata.insert(
        "cache_status".to_string(),
        trace.result.cache_status.clone(),
    );
    metadata.insert(
        "cache_reason".to_string(),
        trace.result.cache_reason.clone(),
    );
}

fn tool_failure_hint(tool_name: &str) -> String {
    match tool_name {
        "run_command" => "建议先检查命令语法、依赖和当前环境，再决定是否重试。".to_string(),
        "workspace_write" => "建议先核对目标路径和父目录状态，再决定是否继续写入。".to_string(),
        "workspace_delete" => "建议先读取或列出目标路径，确认范围后再决定是否删除。".to_string(),
        "workspace_read" => "建议先确认目标文件存在且路径位于当前工作区。".to_string(),
        "project_answer" => "建议先检查项目文档命中情况，必要时补充上下文后再追问。".to_string(),
        _ => "建议先查看错误摘要与验证结果，再决定是否重试当前动作。".to_string(),
    }
}
