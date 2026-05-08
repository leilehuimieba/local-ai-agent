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
    append_browser_failure_metadata(metadata, trace);
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

fn append_tool_identity(metadata: &mut BTreeMap<String, String>, trace: &crate::capabilities::ToolExecutionTrace) {
    metadata.insert("tool_name".to_string(), trace.tool.tool_name.clone());
    metadata.insert("tool_display_name".to_string(), trace.tool.display_name.clone());
    metadata.insert("tool_category".to_string(), trace.tool.category.clone());
    metadata.insert("output_kind".to_string(), trace.tool.output_kind.clone());
}

fn append_tool_outcome(metadata: &mut BTreeMap<String, String>, trace: &crate::capabilities::ToolExecutionTrace) {
    metadata.insert("result_summary".to_string(), trace.result.summary.clone());
    if trace.tool.tool_name == "run_command" {
        metadata.insert("detail_preview".to_string(), trace.result.detail_preview.clone());
        if let Some(value) = trace.result.raw_output_ref.clone() {
            metadata.insert("raw_output_ref".to_string(), value);
        }
    }
    metadata.insert("tool_elapsed_ms".to_string(), trace.result.elapsed_ms.to_string());
    metadata.insert("risk_level".to_string(), trace.tool.risk_level.clone());
    metadata.insert("reasoning_summary".to_string(), trace.result.reasoning_summary.clone());
    append_tool_result_budget(metadata, trace);
    metadata.insert(
        "failure_recovery_hint".to_string(),
        tool_failure_hint(trace.tool.tool_name.as_str()),
    );
}

fn append_tool_cache(metadata: &mut BTreeMap<String, String>, trace: &crate::capabilities::ToolExecutionTrace) {
    metadata.insert("cache_status".to_string(), trace.result.cache_status.clone());
    metadata.insert("cache_reason".to_string(), trace.result.cache_reason.clone());
}

fn append_browser_failure_metadata(
    metadata: &mut BTreeMap<String, String>,
    trace: &crate::capabilities::ToolExecutionTrace,
) {
    if !trace.tool.tool_name.starts_with("mcp__browser__") {
        return;
    }
    let output = format!("{} {}", trace.result.detail_preview, trace.result.final_answer);
    metadata.insert("browser_page_id".to_string(), browser_json_value(&output, "page_id"));
    metadata.insert("browser_selector".to_string(), browser_json_value(&output, "selector"));
}

fn tool_failure_hint(tool_name: &str) -> String {
    match tool_name {
        "mcp__browser__click"
        | "mcp__browser__type"
        | "mcp__browser__select"
        | "mcp__browser__submit"
        | "mcp__browser__upload"
        | "mcp__browser__read_page" => {
            "建议先读取当前页面状态，确认 page_id、selector 与页面变化信号后再决定是否继续。".to_string()
        }
        "run_command" => "建议先检查命令语法、依赖和当前环境，再决定是否重试。".to_string(),
        "workspace_write" => "建议先核对目标路径和父目录状态，再决定是否继续写入。".to_string(),
        "workspace_delete" => "建议先读取或列出目标路径，确认范围后再决定是否删除。".to_string(),
        "workspace_read" => "建议先确认目标文件存在且路径位于当前工作区。".to_string(),
        "project_answer" => "建议先检查项目文档命中情况，必要时补充上下文后再追问。".to_string(),
        _ => "建议先查看错误摘要与验证结果，再决定是否重试当前动作。".to_string(),
    }
}

fn append_tool_result_budget(metadata: &mut BTreeMap<String, String>, trace: &crate::capabilities::ToolExecutionTrace) {
    metadata.insert("result_chars".to_string(), trace.result.result_chars.to_string());
    metadata.insert(
        "single_result_budget_chars".to_string(),
        trace.result.single_result_budget_chars.to_string(),
    );
    metadata.insert(
        "single_result_budget_hit".to_string(),
        bool_string(trace.result.single_result_budget_hit),
    );
}

fn bool_string(value: bool) -> String {
    if value { "true".to_string() } else { "false".to_string() }
}

fn browser_json_value(text: &str, field: &str) -> String {
    let marker = format!("\"{field}\":\"");
    text.split(&marker)
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::{ToolCallResult, ToolDefinition, ToolExecutionTrace};

    #[test]
    fn appends_browser_failure_page_and_selector() {
        let mut metadata = BTreeMap::new();
        append_tool_failure_metadata(&mut metadata, Some(&browser_trace()));
        assert_eq!(metadata.get("browser_page_id"), Some(&"page_01".to_string()));
        assert_eq!(metadata.get("browser_selector"), Some(&"#submit".to_string()));
        assert!(
            metadata
                .get("failure_recovery_hint")
                .is_some_and(|value| value.contains("page_id、selector"))
        );
    }

    fn browser_trace() -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: ToolDefinition {
                tool_name: "mcp__browser__submit".to_string(),
                display_name: "MCP: browser/submit".to_string(),
                category: "mcp".to_string(),
                risk_level: "high".to_string(),
                input_schema: "json".to_string(),
                output_kind: "json_preview".to_string(),
                requires_confirmation: true,
                model_schema: None,
            },
            action_summary: "调用 MCP 工具：browser/submit".to_string(),
            result: ToolCallResult {
                summary: "浏览器提交失败".to_string(),
                final_answer: r##"{"ok":false,"page_id":"page_01","selector":"#submit"}"##.to_string(),
                artifact_path: None,
                detail_preview: r##"{"ok":false,"page_id":"page_01","selector":"#submit"}"##.to_string(),
                raw_output_ref: None,
                result_chars: 64,
                single_result_budget_chars: 1200,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 35,
                retryable: false,
                success: false,
                memory_write_summary: None,
                reasoning_summary: "Runtime 通过 Gateway MCP endpoint 执行工具。".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }
}
