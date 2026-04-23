use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::llm::complete_text;
use crate::paths::resolve_workspace_path;
use crate::planner::PlannedAction;
use crate::prompt::render_agent_resolve_prompt;
use crate::session::SessionMemory;
use crate::session::session_prompt_summary;
use std::fs;

const CACHE_REASON: &str = "Agent 调用依赖实时模型输出，不使用回答缓存。";
const MAX_AGENT_TURNS: usize = 3;

pub(crate) fn execute_agent_resolve(
    request: &RunRequest,
    session_context: &SessionMemory,
) -> ActionExecution {
    execute_agent_loop(request, session_context)
}

fn execute_agent_loop(request: &RunRequest, session_context: &SessionMemory) -> ActionExecution {
    let mut traces = Vec::new();
    let mut skipped = Vec::new();
    let mut prompt = build_agent_resolve_prompt(request, session_context);
    for turn in 0..MAX_AGENT_TURNS {
        let response = match complete_text(request, &prompt) {
            Ok(response) => response,
            Err(error) => return fail_agent_resolve(&error),
        };
        if let Some(calls) = response.tool_calls.filter(|calls| !calls.is_empty()) {
            let (round_traces, round_skipped) =
                execute_tool_calls(request, session_context, &calls);
            traces.extend(round_traces.clone());
            skipped.extend(round_skipped.clone());
            if let Some(result) =
                maybe_recover_required_write(request, session_context, &traces, &skipped)
            {
                return result;
            }
            if turn + 1 == MAX_AGENT_TURNS {
                if can_finalize_from_traces(&request.user_input, &traces) {
                    return finalize_from_traces(request, &traces, &skipped);
                }
                return incomplete_agent_resolve(
                    &traces,
                    &skipped,
                    "达到最大执行轮次，任务仍未明确完成。",
                );
            }
            prompt = build_followup_prompt(request, session_context, &traces, &skipped);
            continue;
        }
        return finalize_agent_response(request, &traces, &skipped, response.content);
    }
    incomplete_agent_resolve(&traces, &skipped, "Agent 循环意外提前结束。")
}

fn finalize_agent_response(
    request: &RunRequest,
    traces: &[crate::capabilities::ToolExecutionTrace],
    skipped: &[String],
    content: String,
) -> ActionExecution {
    if let Some(reason) = incomplete_reason(&request.user_input, traces, &content) {
        return incomplete_agent_resolve(traces, skipped, &reason);
    }
    if traces.is_empty() {
        return natural_language_response(content);
    }
    let success = traces.iter().all(|trace| trace.result.success) && skipped.is_empty();
    let header = tool_call_header(
        traces.len(),
        traces.iter().filter(|trace| !trace.result.success).count(),
        skipped.len(),
    );
    let result_summary = format!("{} 最终已形成中文结果。", header);
    let reasoning = format!(
        "模型通过多轮 tool_calls 推进任务，并在工具执行后形成最终答复。{}",
        header
    );
    ActionExecution::bypass(
        "大模型多轮工具执行并完成任务".to_string(),
        result_summary,
        content.trim().to_string(),
        success,
        None,
        reasoning,
        CACHE_REASON,
    )
}

fn natural_language_response(content: String) -> ActionExecution {
    ActionExecution::bypass_ok(
        "大模型自然语言回答".to_string(),
        "模型未下发工具调用，返回文本回答。".to_string(),
        content,
        "模型未选择工具，直接返回生成文本。".to_string(),
        CACHE_REASON,
    )
}

fn execute_tool_calls(
    request: &RunRequest,
    session_context: &SessionMemory,
    calls: &[crate::model_adapter::ToolCall],
) -> (Vec<crate::capabilities::ToolExecutionTrace>, Vec<String>) {
    let mut traces = Vec::new();
    let mut skipped = Vec::new();
    for tc in calls {
        match execute_single_tool_call(request, session_context, tc) {
            Some(trace) => traces.push(trace),
            None => skipped.push(format!("{} (id={})", tc.function.name, tc.id)),
        }
    }
    (traces, skipped)
}

fn execute_single_tool_call(
    request: &RunRequest,
    session_context: &SessionMemory,
    tc: &crate::model_adapter::ToolCall,
) -> Option<crate::capabilities::ToolExecutionTrace> {
    let action =
        crate::action_decode::tool_call_to_action(&tc.function.name, &tc.function.arguments)?;
    Some(crate::tool_trace::execute_tool(
        request,
        &action,
        session_context,
    ))
}

fn render_tool_call_traces(
    traces: &[crate::capabilities::ToolExecutionTrace],
    skipped: &[String],
) -> (String, String, bool) {
    if traces.is_empty() && skipped.is_empty() {
        return (
            "模型未返回可执行的工具调用。".to_string(),
            "模型未返回可执行的工具调用。".to_string(),
            false,
        );
    }
    let failed = traces.iter().filter(|t| !t.result.success).count();
    let skipped_count = skipped.len();
    let success = failed == 0 && skipped_count == 0;
    let header = tool_call_header(traces.len(), failed, skipped_count);
    let detail = tool_call_detail(traces);
    let skipped_block = render_skipped_tool_calls(skipped);
    let final_answer = tool_call_final_answer(&header, &detail, &skipped_block);
    (header, final_answer, success)
}

fn render_single_trace(index: usize, trace: &crate::capabilities::ToolExecutionTrace) -> String {
    let status = if trace.result.success {
        "成功"
    } else {
        "失败"
    };
    let mut parts = Vec::new();
    parts.push(format!(
        "{}. [{}] {} ({})",
        index, status, trace.tool.display_name, trace.tool.tool_name
    ));
    parts.push(format!("动作：{}", trace.action_summary));
    parts.push(format!("摘要：{}", trace.result.summary));
    if let Some(path) = trace.result.artifact_path.as_ref() {
        parts.push(format!("产物：{}", path));
    }
    if let Some(code) = trace.result.error_code.as_ref() {
        parts.push(format!("错误码：{}", code));
    }
    parts.join("\n")
}

fn tool_call_header(total: usize, failed: usize, skipped: usize) -> String {
    format!(
        "工具调用执行完成：{} 个，总失败 {} 个，未识别 {} 个。",
        total, failed, skipped
    )
}

fn tool_call_detail(traces: &[crate::capabilities::ToolExecutionTrace]) -> String {
    traces
        .iter()
        .enumerate()
        .map(|(idx, t)| render_single_trace(idx + 1, t))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn tool_call_final_answer(header: &str, detail: &str, skipped: &str) -> String {
    if skipped.is_empty() {
        format!("{}\n\n{}", header, detail)
    } else if detail.is_empty() {
        format!("{}\n\n{}", header, skipped)
    } else {
        format!("{}\n\n{}\n\n{}", header, detail, skipped)
    }
}

fn render_skipped_tool_calls(skipped: &[String]) -> String {
    if skipped.is_empty() {
        return String::new();
    }
    let names = skipped
        .iter()
        .take(20)
        .map(|name| format!("- {}", name))
        .collect::<Vec<_>>()
        .join("\n");
    format!("未识别的工具调用（已跳过）：\n{}", names)
}

fn fail_agent_resolve(error: &str) -> ActionExecution {
    ActionExecution::bypass_fail(
        "透传大模型 Agent Resolve".to_string(),
        format!("模型调用失败：{}", error),
        format!("大模型调用失败：{}", error),
        "AgentResolve 调用模型失败，直接按错误收口。".to_string(),
        CACHE_REASON,
    )
}

fn build_agent_resolve_prompt(request: &RunRequest, session_context: &SessionMemory) -> String {
    render_agent_resolve_prompt(
        &request.user_input,
        &session_prompt_summary(session_context),
    )
}

fn build_followup_prompt(
    request: &RunRequest,
    session_context: &SessionMemory,
    traces: &[crate::capabilities::ToolExecutionTrace],
    skipped: &[String],
) -> String {
    format!(
        "{}\n\n上一轮工具结果：\n{}\n\n继续要求：{}",
        build_agent_resolve_prompt(request, session_context),
        render_round_results(traces, skipped),
        next_step_instruction(&request.user_input, traces)
    )
}

fn render_round_results(
    traces: &[crate::capabilities::ToolExecutionTrace],
    skipped: &[String],
) -> String {
    let mut parts = traces
        .iter()
        .enumerate()
        .map(|(idx, trace)| render_single_trace(idx + 1, trace))
        .collect::<Vec<_>>();
    if !skipped.is_empty() {
        parts.push(render_skipped_tool_calls(skipped));
    }
    parts.join("\n\n")
}

fn incomplete_reason(
    user_input: &str,
    traces: &[crate::capabilities::ToolExecutionTrace],
    content: &str,
) -> Option<String> {
    if can_finalize_from_traces(user_input, traces) {
        return None;
    }
    if requires_write(user_input) && !has_write_trace(traces) {
        return Some(
            "用户请求要求写回结果，但本次执行还没有成功调用 workspace_write。".to_string(),
        );
    }
    if content.trim().is_empty() && !traces.is_empty() {
        return Some("运行时已执行工具，但模型没有给出明确完成信号。".to_string());
    }
    None
}

fn incomplete_agent_resolve(
    traces: &[crate::capabilities::ToolExecutionTrace],
    skipped: &[String],
    reason: &str,
) -> ActionExecution {
    let (header, detail, _) = render_tool_call_traces(traces, skipped);
    ActionExecution::bypass_fail(
        "大模型工具执行未完成任务".to_string(),
        reason.to_string(),
        format!("{}\n\n{}\n\n{}", reason, header, detail),
        "模型虽然执行了部分工具，但还没有把原始任务真正做完。".to_string(),
        CACHE_REASON,
    )
}

fn requires_write(user_input: &str) -> bool {
    ["写到", "写入", "写回", "保存到", "输出到", "创建文件"]
        .iter()
        .any(|token| user_input.contains(token))
}

fn has_write_trace(traces: &[crate::capabilities::ToolExecutionTrace]) -> bool {
    traces
        .iter()
        .any(|trace| trace.tool.tool_name == "workspace_write" && trace.result.success)
}

fn has_read_trace(traces: &[crate::capabilities::ToolExecutionTrace]) -> bool {
    traces
        .iter()
        .any(|trace| trace.tool.tool_name == "workspace_read" && trace.result.success)
}

fn next_step_instruction(
    user_input: &str,
    traces: &[crate::capabilities::ToolExecutionTrace],
) -> String {
    if requires_write(user_input) && !has_write_trace(traces) {
        return format!(
            "原始目标仍然是“{}”。你已经拿到生成摘要所需的信息，不要再次读取同一份文件，也不要继续列目录；下一步必须调用 workspace_write，把符合原始结构的中文摘要写到 {}。写入成功后，再输出最终中文结果。",
            user_input,
            extract_write_target(user_input).unwrap_or_else(|| "用户指定的目标文件".to_string())
        );
    }
    format!(
        "原始目标仍然是“{}”。如果还没真正完成，请继续调用必要工具；只有全部完成后才输出最终中文结果。",
        user_input
    )
}

fn extract_write_target(user_input: &str) -> Option<String> {
    ["写到", "写入", "保存到", "输出到"]
        .iter()
        .find_map(|marker| user_input.split_once(marker))
        .map(|(_, tail)| {
            tail.lines()
                .next()
                .unwrap_or_default()
                .trim()
                .trim_end_matches('。')
                .to_string()
        })
        .filter(|value| !value.is_empty())
}

fn maybe_recover_required_write(
    request: &RunRequest,
    session_context: &SessionMemory,
    traces: &[crate::capabilities::ToolExecutionTrace],
    skipped: &[String],
) -> Option<ActionExecution> {
    if !requires_write(&request.user_input) || has_write_trace(traces) || !has_read_trace(traces) {
        return None;
    }
    let target_path = extract_write_target(&request.user_input)?;
    let content = generate_required_write_content(request)?;
    let trace = crate::tool_trace::execute_tool(
        request,
        &PlannedAction::WriteFile {
            path: target_path.clone(),
            content,
        },
        session_context,
    );
    Some(recovered_write_response(
        traces,
        skipped,
        &trace,
        &target_path,
    ))
}

fn generate_required_write_content(request: &RunRequest) -> Option<String> {
    let source_path = extract_source_path(&request.user_input)?;
    let source_content = load_source_content(request, &source_path).ok()?;
    let prompt = build_summary_write_prompt(&request.user_input, &source_content);
    let response = complete_text(request, &prompt).ok()?;
    let content = response.content.trim().to_string();
    (!content.is_empty()).then_some(content)
}

fn extract_source_path(user_input: &str) -> Option<String> {
    ["学习资料：", "学习资料:", "读取文件：", "读取文件:"]
        .iter()
        .find_map(|marker| user_input.split_once(marker))
        .map(|(_, tail)| {
            tail.lines()
                .next()
                .unwrap_or_default()
                .trim()
                .trim_end_matches('。')
                .to_string()
        })
        .filter(|value| !value.is_empty())
}

fn load_source_content(request: &RunRequest, source_path: &str) -> Result<String, ()> {
    let path =
        resolve_workspace_path(&request.workspace_ref.root_path, source_path).map_err(|_| ())?;
    fs::read_to_string(path).map_err(|_| ())
}

fn build_summary_write_prompt(user_input: &str, source_content: &str) -> String {
    format!(
        "请基于下面这份学习资料，直接生成要写入目标文件的最终中文正文。只输出正文，不要解释，不要再调用工具，不要加引号。\n\n原始用户请求：{}\n\n学习资料全文：\n{}",
        user_input, source_content
    )
}

fn recovered_write_response(
    traces: &[crate::capabilities::ToolExecutionTrace],
    skipped: &[String],
    write_trace: &crate::capabilities::ToolExecutionTrace,
    target_path: &str,
) -> ActionExecution {
    let mut final_traces = traces.to_vec();
    final_traces.push(write_trace.clone());
    let failed = final_traces
        .iter()
        .filter(|trace| !trace.result.success)
        .count();
    let success = failed == 0 && skipped.is_empty();
    let header = tool_call_header(final_traces.len(), failed, skipped.len());
    let final_answer = if success {
        format!("已根据学习资料生成摘要，并写入 {}。", target_path)
    } else {
        tool_call_final_answer(
            &header,
            &tool_call_detail(&final_traces),
            &render_skipped_tool_calls(skipped),
        )
    };
    ActionExecution::bypass(
        "运行时补全摘要写回".to_string(),
        format!("{} 已补完成摘要写回。", header),
        final_answer,
        success,
        None,
        "AgentResolve 已读取资料但未继续写回，运行时按原始目标补全摘要生成与写入。".to_string(),
        CACHE_REASON,
    )
}

fn can_finalize_from_traces(
    user_input: &str,
    traces: &[crate::capabilities::ToolExecutionTrace],
) -> bool {
    requires_write(user_input) && has_write_trace(traces)
}

fn finalize_from_traces(
    request: &RunRequest,
    traces: &[crate::capabilities::ToolExecutionTrace],
    skipped: &[String],
) -> ActionExecution {
    let failed = traces.iter().filter(|trace| !trace.result.success).count();
    let success = failed == 0 && skipped.is_empty();
    let header = tool_call_header(traces.len(), failed, skipped.len());
    let target_path = extract_write_target(&request.user_input).unwrap_or_default();
    ActionExecution::bypass(
        "运行时按真实写回结果完成收口".to_string(),
        format!("{} 已确认目标文件写回成功。", header),
        format!("已根据学习资料生成摘要，并写入 {}。", target_path),
        success,
        None,
        "运行时已观测到目标写入成功，因此按真实完成状态收口。".to_string(),
        CACHE_REASON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tool() -> crate::capabilities::ToolDefinition {
        crate::capabilities::ToolDefinition {
            tool_name: "workspace_read".to_string(),
            display_name: "读取文件".to_string(),
            category: "workspace_read".to_string(),
            risk_level: "low".to_string(),
            input_schema: "path".to_string(),
            output_kind: "text_preview".to_string(),
            requires_confirmation: false,
        }
    }

    fn make_trace(
        success: bool,
        artifact: bool,
        error_code: Option<&str>,
    ) -> crate::capabilities::ToolExecutionTrace {
        crate::capabilities::ToolExecutionTrace {
            tool: make_tool(),
            action_summary: "动作摘要".to_string(),
            result: crate::capabilities::ToolCallResult {
                summary: "结果摘要".to_string(),
                final_answer: "结果正文".to_string(),
                artifact_path: artifact.then(|| "tmp/out.txt".to_string()),
                detail_preview: "结果摘要".to_string(),
                raw_output_ref: None,
                result_chars: 4,
                single_result_budget_chars: 30_000,
                single_result_budget_hit: false,
                error_code: error_code.map(|v| v.to_string()),
                elapsed_ms: 0,
                retryable: !success,
                success,
                memory_write_summary: None,
                reasoning_summary: "reasoning".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: "test".to_string(),
            },
        }
    }

    #[test]
    fn renders_empty_tool_calls_as_failure() {
        let (header, final_answer, success) = render_tool_call_traces(&[], &[]);
        assert!(!success);
        assert_eq!(header, "模型未返回可执行的工具调用。");
        assert_eq!(final_answer, "模型未返回可执行的工具调用。");
    }

    #[test]
    fn renders_success_trace_without_skipped() {
        let trace = make_trace(true, true, None);
        let (header, final_answer, success) = render_tool_call_traces(&[trace], &[]);
        assert!(success);
        assert!(header.contains("工具调用执行完成：1 个"));
        assert!(final_answer.contains("1. [成功] 读取文件 (workspace_read)"));
        assert!(final_answer.contains("产物：tmp/out.txt"));
    }

    #[test]
    fn renders_failure_and_skipped_as_not_success() {
        let traces = vec![
            make_trace(true, false, None),
            make_trace(false, false, Some("failed")),
        ];
        let skipped = vec!["unknown_tool (id=call_1)".to_string()];
        let (header, final_answer, success) = render_tool_call_traces(&traces, &skipped);
        assert!(!success);
        assert!(header.contains("总失败 1 个"));
        assert!(final_answer.contains("未识别的工具调用（已跳过）："));
        assert!(final_answer.contains("- unknown_tool (id=call_1)"));
        assert!(final_answer.contains("错误码：failed"));
    }
}
