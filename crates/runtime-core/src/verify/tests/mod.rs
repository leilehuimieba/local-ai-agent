pub(super) use super::super::verify_tool_execution;
use crate::capabilities::{ToolCallResult, ToolDefinition, ToolExecutionTrace};
use crate::planner::PlannedAction;
use crate::tool_registry::ToolCall;

mod browser;
mod core;
mod file_command_memory;
mod knowledge;

pub(super) fn sample_tool_call() -> ToolCall {
    ToolCall {
        action: PlannedAction::RunCommand {
            command: "echo ok".to_string(),
        },
        spec: tool(
            "run_command",
            "执行命令",
            "system_command",
            "high",
            "command_text",
            "text_preview",
            true,
        ),
    }
}

pub(super) fn sample_trace(success: bool, downgraded: bool) -> ToolExecutionTrace {
    let summary = if downgraded {
        "命令执行成功，guard downgraded"
    } else {
        "命令执行成功"
    };
    let final_answer = if success {
        "命令已执行完成。\n工作区：D:/repo\n命令：echo ok\n输出摘要：ok"
    } else {
        "failed"
    };
    let reasoning = if downgraded {
        "guard downgraded to review"
    } else {
        "测试推理"
    };
    trace(
        &sample_tool_call(),
        "执行 echo ok",
        result(
            summary,
            final_answer,
            "ok",
            Some("D:/repo/tmp/command.txt"),
            Some("D:/repo/tmp/command.txt"),
            10,
            30000,
            false,
            success,
            None,
            reasoning,
        ),
    )
}

pub(super) fn memory_recall_tool_call() -> ToolCall {
    ToolCall {
        action: PlannedAction::RecallMemory {
            query: "对象摘要".to_string(),
        },
        spec: tool(
            "memory_recall",
            "召回记忆",
            "memory_read",
            "low",
            "query",
            "text_preview",
            false,
        ),
    }
}

pub(super) fn sample_memory_recall_trace(layer: &str) -> ToolExecutionTrace {
    let summary = format!("已召回 2 条相关记忆。（{layer}）");
    let final_answer = format!("已召回相关长期记忆。\n召回层：{layer}");
    let reasoning = format!("按查询词检索长期记忆，并返回前几条高相关结果；本次召回层为{layer}。");
    trace(
        &memory_recall_tool_call(),
        "按需召回记忆：对象摘要",
        result(
            &summary,
            &final_answer,
            "preview",
            None,
            None,
            64,
            30000,
            false,
            true,
            None,
            &reasoning,
        ),
    )
}

pub(super) fn knowledge_tool_call() -> ToolCall {
    ToolCall {
        action: PlannedAction::ProjectAnswer,
        spec: tool(
            "project_answer",
            "项目回答",
            "assistant_answer",
            "low",
            "none",
            "text",
            false,
        ),
    }
}

pub(super) fn knowledge_trace(complete: bool) -> ToolExecutionTrace {
    let summary = if complete {
        "知识摘要：knowledge pack || 知识引证：docs/README.md,docs/11-hermes-rebuild/current-state.md || 事实：已命中文档 || 推断：当前更适合 verify 先行 || 建议：进入下一步"
    } else {
        "知识摘要：knowledge pack || 知识引证：未提供 || 统一总结：看起来可以回答"
    };
    let final_answer = if complete {
        "结论来自 docs/README.md 与 docs/11-hermes-rebuild/current-state.md"
    } else {
        "我认为现在可以直接回答"
    };
    let reasoning = if complete {
        "事实：引用当前文档；推断：当前知识链已稳定；建议：可直接完成答复。"
    } else {
        "仅有统一总结，没有继续区分事实与推断。"
    };
    trace(
        &knowledge_tool_call(),
        "基于项目上下文回答",
        result(
            summary,
            final_answer,
            "preview",
            None,
            None,
            120,
            30000,
            false,
            true,
            None,
            reasoning,
        ),
    )
}

pub(super) fn file_write_tool_call() -> ToolCall {
    ToolCall {
        action: PlannedAction::WriteFile {
            path: "docs/out.md".to_string(),
            content: "content".to_string(),
        },
        spec: tool(
            "workspace_write",
            "写入文件",
            "workspace_write",
            "medium",
            "path_and_content",
            "text",
            false,
        ),
    }
}

pub(super) fn file_change_trace(complete: bool) -> ToolExecutionTrace {
    let summary = if complete {
        "文件写入成功，共写入 12 个字符。"
    } else {
        "写入动作已执行"
    };
    let final_answer = if complete {
        "文件写入完成：docs/out.md\n内容摘要：summary"
    } else {
        "已尝试写入文件"
    };
    trace(
        &file_write_tool_call(),
        "写入文件：docs/out.md",
        result(
            summary,
            final_answer,
            "preview",
            None,
            None,
            80,
            30000,
            false,
            true,
            None,
            "先校验工作区路径，再直接写入目标文件并返回摘要。",
        ),
    )
}

pub(super) fn memory_write_tool_call() -> ToolCall {
    ToolCall {
        action: PlannedAction::WriteMemory {
            kind: "project_rule".to_string(),
            summary: "记忆摘要".to_string(),
            content: "记忆内容".to_string(),
        },
        spec: tool(
            "memory_write",
            "写入记忆",
            "memory_write",
            "medium",
            "memory_entry",
            "memory_write_result",
            false,
        ),
    }
}

pub(super) fn memory_write_trace(complete: bool) -> ToolExecutionTrace {
    let summary = if complete {
        "已写入 `project_rule` 记忆：记忆摘要"
    } else {
        "已尝试写入记忆"
    };
    let final_answer = if complete {
        "记忆写入完成。\n类型：project_rule\n摘要：记忆摘要\n内容摘要：summary"
    } else {
        "记忆写入完成。"
    };
    let memory_summary = complete.then_some("已写入 `project_rule` 记忆：记忆摘要");
    trace(
        &memory_write_tool_call(),
        "写入长期记忆：记忆摘要",
        result(
            summary,
            final_answer,
            "preview",
            None,
            None,
            60,
            30000,
            false,
            true,
            memory_summary,
            "按用户指定内容构造长期记忆记录并写入本地主存储。",
        ),
    )
}

pub(super) fn command_trace(complete: bool) -> ToolExecutionTrace {
    let summary = if complete { "命令执行成功" } else { "执行结束" };
    let final_answer = if complete {
        "命令已执行完成。\n工作区：D:/repo\n命令：echo ok\n输出摘要：ok"
    } else {
        "命令已执行"
    };
    let detail = if complete { "ok" } else { "" };
    let artifact = complete.then_some("D:/repo/tmp/command.txt");
    trace(
        &sample_tool_call(),
        "执行命令：echo ok",
        result(
            summary,
            final_answer,
            detail,
            artifact,
            artifact,
            40,
            30000,
            false,
            true,
            None,
            "直接执行用户给定命令，并基于 stdout 或 stderr 生成摘要。",
        ),
    )
}

pub(super) fn browser_click_tool_call() -> ToolCall {
    browser_tool_call("click", r##"{"page_id":"page_01","selector":"#advance"}"##, "medium")
}

pub(super) fn browser_select_tool_call() -> ToolCall {
    browser_tool_call(
        "select",
        r##"{"page_id":"page_01","selector":"#tier","value":"advanced"}"##,
        "medium",
    )
}

pub(super) fn browser_submit_tool_call() -> ToolCall {
    browser_tool_call("submit", r##"{"page_id":"page_01","selector":"#submit"}"##, "high")
}

pub(super) fn browser_upload_tool_call() -> ToolCall {
    browser_tool_call(
        "upload",
        r##"{"page_id":"page_01","selector":"#upload","file_paths":["D:/repo/tmp/upload.txt"]}"##,
        "high",
    )
}

pub(super) fn browser_trace(complete: bool) -> ToolExecutionTrace {
    let payload = if complete {
        r##"{"ok":true,"page_id":"page_01","url":"http://local/page","title":"AF Browser Interaction","selector":"#advance","clicked":true,"elapsed_ms":35}"##
    } else {
        r#"{"ok":true,"elapsed_ms":35}"#
    };
    let reasoning = if complete {
        "Runtime 通过 Gateway MCP endpoint 执行工具，复用 allowlist 与审计。"
    } else {
        "Runtime 调用外部工具并返回结果。"
    };
    browser_tool_trace(&browser_click_tool_call(), payload, reasoning)
}

pub(super) fn browser_target_missing_trace() -> ToolExecutionTrace {
    browser_tool_trace(
        &browser_click_tool_call(),
        r##"{"ok":true,"page_id":"page_01","elapsed_ms":35}"##,
        "Runtime 通过 Gateway MCP endpoint 执行工具，复用 allowlist 与审计。",
    )
}

pub(super) fn browser_state_unchanged_trace() -> ToolExecutionTrace {
    browser_tool_trace(
        &browser_click_tool_call(),
        r##"{"ok":true,"page_id":"page_01","selector":"#advance","elapsed_ms":35}"##,
        "Runtime 通过 Gateway MCP endpoint 执行工具，复用 allowlist 与审计。",
    )
}

pub(super) fn browser_recovery_exhausted_trace() -> ToolExecutionTrace {
    let mut trace = browser_state_unchanged_trace();
    trace.result.summary = "已执行单次恢复 read_page，但当前浏览器交互仍未验证通过。".to_string();
    trace
}

pub(super) fn browser_select_trace(complete: bool) -> ToolExecutionTrace {
    let payload = if complete {
        r##"{"ok":true,"page_id":"page_01","url":"http://local/page","title":"AM Browser Risky Interaction","selector":"#tier","selected_value":"advanced","elapsed_ms":35}"##
    } else {
        r#"{"ok":true,"elapsed_ms":35}"#
    };
    browser_tool_trace(&browser_select_tool_call(), payload, browser_reasoning(complete))
}

pub(super) fn browser_submit_trace(complete: bool) -> ToolExecutionTrace {
    let payload = if complete {
        r##"{"ok":true,"page_id":"page_01","url":"http://local/page","title":"AM Browser Risky Interaction","selector":"#submit","submitted":true,"elapsed_ms":35}"##
    } else {
        r#"{"ok":true,"elapsed_ms":35}"#
    };
    browser_tool_trace(&browser_submit_tool_call(), payload, browser_reasoning(complete))
}

pub(super) fn browser_upload_trace(complete: bool) -> ToolExecutionTrace {
    let payload = if complete {
        r##"{"ok":true,"page_id":"page_01","url":"http://local/page","title":"AM Browser Risky Interaction","selector":"#upload","uploaded_file_count":1,"uploaded_files":["upload.txt"],"elapsed_ms":35}"##
    } else {
        r#"{"ok":true,"elapsed_ms":35}"#
    };
    browser_tool_trace(&browser_upload_tool_call(), payload, browser_reasoning(complete))
}

fn tool(
    name: &str,
    display: &str,
    category: &str,
    risk: &str,
    input: &str,
    output: &str,
    confirm: bool,
) -> ToolDefinition {
    ToolDefinition {
        tool_name: name.to_string(),
        display_name: display.to_string(),
        category: category.to_string(),
        risk_level: risk.to_string(),
        input_schema: input.to_string(),
        output_kind: output.to_string(),
        requires_confirmation: confirm,
        model_schema: None,
    }
}

fn trace(tool_call: &ToolCall, action_summary: &str, result: ToolCallResult) -> ToolExecutionTrace {
    ToolExecutionTrace {
        tool: tool_call.spec.clone(),
        action_summary: action_summary.to_string(),
        result,
    }
}

fn result(
    summary: &str,
    final_answer: &str,
    detail_preview: &str,
    artifact_path: Option<&str>,
    raw_output_ref: Option<&str>,
    result_chars: usize,
    single_result_budget_chars: usize,
    single_result_budget_hit: bool,
    success: bool,
    memory_write_summary: Option<&str>,
    reasoning_summary: &str,
) -> ToolCallResult {
    ToolCallResult {
        summary: summary.to_string(),
        final_answer: final_answer.to_string(),
        artifact_path: artifact_path.map(str::to_string),
        detail_preview: detail_preview.to_string(),
        raw_output_ref: raw_output_ref.map(str::to_string),
        result_chars,
        single_result_budget_chars,
        single_result_budget_hit,
        error_code: None,
        elapsed_ms: 10,
        retryable: false,
        success,
        memory_write_summary: memory_write_summary.map(str::to_string),
        reasoning_summary: reasoning_summary.to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: String::new(),
    }
}

fn browser_tool_call(name: &str, arguments_json: &str, risk_level: &str) -> ToolCall {
    ToolCall {
        action: PlannedAction::MCPCall {
            server_id: "browser".to_string(),
            tool_name: name.to_string(),
            arguments_json: arguments_json.to_string(),
        },
        spec: tool(
            &format!("mcp__browser__{name}"),
            &format!("MCP: browser/{name}"),
            "mcp",
            risk_level,
            "json",
            "json_preview",
            true,
        ),
    }
}

fn browser_tool_trace(tool_call: &ToolCall, payload: &str, reasoning: &str) -> ToolExecutionTrace {
    let name = browser_tool_name(tool_call);
    let summary = format!("MCP 工具 `browser/{name}` 已返回结果。");
    let artifact = format!("D:/repo/tmp/browser-{name}.json");
    let action = format!("调用 MCP 工具：browser/{name}");
    trace(
        tool_call,
        &action,
        result(
            &summary,
            payload,
            payload,
            Some(&artifact),
            Some(&artifact),
            payload.len(),
            1200,
            false,
            true,
            None,
            reasoning,
        ),
    )
}

fn browser_tool_name(tool_call: &ToolCall) -> &str {
    let PlannedAction::MCPCall { tool_name, .. } = &tool_call.action else {
        return "click";
    };
    tool_name.as_str()
}

fn browser_reasoning(complete: bool) -> &'static str {
    if complete {
        "Runtime 通过 Gateway MCP endpoint 执行工具，复用 allowlist 与审计。"
    } else {
        "Runtime 调用外部工具并返回结果。"
    }
}
