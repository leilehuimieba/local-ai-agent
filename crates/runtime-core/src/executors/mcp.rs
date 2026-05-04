use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use serde_json::Value;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

const CACHE_REASON: &str = "MCP 工具调用依赖外部服务实时结果，不使用回答缓存。";

pub(crate) fn execute_mcp_call(
    request: &RunRequest,
    server_id: &str,
    tool_name: &str,
    arguments_json: &str,
) -> ActionExecution {
    match call_gateway(request, server_id, tool_name, arguments_json) {
        Ok(body) => mcp_success(server_id, tool_name, body),
        Err(error) => mcp_failure(server_id, tool_name, error),
    }
}

fn call_gateway(
    request: &RunRequest,
    server_id: &str,
    tool_name: &str,
    arguments_json: &str,
) -> Result<String, String> {
    let bridge = mcp_bridge_config(request)?;
    let payload = mcp_payload(request, server_id, tool_name, arguments_json);
    let body_path = write_payload_file(&payload)?;
    let output = run_curl(&bridge, &body_path);
    let _ = fs::remove_file(&body_path);
    output.and_then(parse_curl_output)
}

fn mcp_bridge_config(request: &RunRequest) -> Result<MCPBridgeConfig, String> {
    let url = request.context_hints.get("mcp_gateway_url").cloned();
    let token = request.context_hints.get("mcp_gateway_token").cloned();
    match (url, token) {
        (Some(url), Some(token)) if !url.is_empty() && !token.is_empty() => Ok(MCPBridgeConfig { url, token }),
        _ => Err("mcp_bridge_not_configured: Runtime 缺少 Gateway MCP 调用配置".to_string()),
    }
}

fn mcp_payload(request: &RunRequest, server_id: &str, tool_name: &str, arguments_json: &str) -> Value {
    serde_json::json!({
        "server_id": server_id,
        "name": tool_name,
        "arguments": mcp_arguments(arguments_json),
        "session_id": request.session_id,
        "run_id": request.run_id,
        "trace_id": request.trace_id,
    })
}

fn mcp_arguments(arguments_json: &str) -> Value {
    let parsed = serde_json::from_str::<Value>(arguments_json).unwrap_or(Value::Null);
    if parsed.is_object() {
        parsed
    } else {
        serde_json::json!({})
    }
}

fn write_payload_file(payload: &Value) -> Result<std::path::PathBuf, String> {
    let data = serde_json::to_vec(payload).map_err(|error| error.to_string())?;
    let path = std::env::temp_dir().join(format!("local-agent-mcp-{}.json", now_ms()));
    fs::write(&path, data).map_err(|error| error.to_string())?;
    Ok(path)
}

fn run_curl(bridge: &MCPBridgeConfig, body_path: &std::path::Path) -> Result<String, String> {
    let mut command = Command::new(curl_bin());
    command
        .arg("-sS")
        .arg("--max-time")
        .arg("30")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg(format!("X-Local-Agent-Token: {}", bridge.token))
        .arg("--data-binary")
        .arg(format!("@{}", body_path.display()))
        .arg(&bridge.url);
    command_output(command)
}

fn command_output(mut command: Command) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);
    let output = command.output().map_err(|error| error.to_string())?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).to_string());
    }
    Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
}

fn parse_curl_output(output: String) -> Result<String, String> {
    let trimmed = output.trim_end();
    let Some((body, code)) = trimmed.rsplit_once('\n') else {
        return Err("mcp_call_failed: Gateway 响应缺少 HTTP 状态码".to_string());
    };
    match code.parse::<u16>().unwrap_or(0) {
        200..=299 => Ok(body.trim().to_string()),
        status => Err(format!("{}: {}", mcp_error_code(body), http_error(status, body))),
    }
}

fn mcp_error_code(body: &str) -> &'static str {
    if body.contains("not allowlisted") {
        return "mcp_tool_not_allowlisted";
    }
    if body.contains("requires confirmation") {
        return "mcp_confirmation_required";
    }
    "mcp_call_failed"
}

fn http_error(status: u16, body: &str) -> String {
    format!("Gateway HTTP {status}: {}", body.trim())
}

fn mcp_success(server_id: &str, tool_name: &str, body: String) -> ActionExecution {
    let preview = truncate(&body, 1200);
    mcp_execution(server_id, tool_name, body, preview, true)
}

fn mcp_failure(server_id: &str, tool_name: &str, error: String) -> ActionExecution {
    let final_answer = format!("MCP 工具 `{server_id}/{tool_name}` 调用失败：{error}");
    mcp_execution(server_id, tool_name, error, final_answer, false)
}

fn mcp_execution(
    server_id: &str,
    tool_name: &str,
    raw_output: String,
    final_answer: String,
    success: bool,
) -> ActionExecution {
    let result_chars = raw_output.chars().count();
    ActionExecution {
        action_summary: format!("调用 MCP 工具：{server_id}/{tool_name}"),
        result_summary: mcp_result_summary(server_id, tool_name, success),
        final_answer,
        detail_preview: truncate(&raw_output, 1200),
        raw_output,
        result_chars,
        single_result_budget_chars: 1200,
        single_result_budget_hit: result_chars > 1200,
        success,
        memory_write_summary: None,
        reasoning_summary: "Runtime 通过 Gateway MCP endpoint 执行工具，复用 allowlist 与审计。".to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: CACHE_REASON.to_string(),
    }
}

fn mcp_result_summary(server_id: &str, tool_name: &str, success: bool) -> String {
    if success {
        return format!("MCP 工具 `{server_id}/{tool_name}` 已返回结果。");
    }
    format!("MCP 工具 `{server_id}/{tool_name}` 调用失败。")
}

fn truncate(value: &str, limit: usize) -> String {
    let mut chars = value.chars();
    let out: String = chars.by_ref().take(limit).collect();
    if chars.next().is_some() {
        format!("{out}...")
    } else {
        out
    }
}

fn curl_bin() -> &'static str {
    if cfg!(target_os = "windows") {
        "curl.exe"
    } else {
        "curl"
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

struct MCPBridgeConfig {
    url: String,
    token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_engine_testkit::testkit::sample_request;

    #[test]
    fn missing_bridge_config_fails_without_calling_curl() {
        let request = sample_request("mcp_missing_bridge");
        let result = execute_mcp_call(&request, "docs", "search", r#"{"q":"rust"}"#);
        assert!(!result.success);
        assert!(result.final_answer.contains("mcp_bridge_not_configured"));
    }

    #[test]
    fn mcp_payload_preserves_run_identity() {
        let request = sample_request("mcp_payload");
        let payload = mcp_payload(&request, "docs", "search", r#"{"q":"rust"}"#);
        assert_eq!(payload["server_id"], "docs");
        assert_eq!(payload["arguments"]["q"], "rust");
        assert_eq!(payload["run_id"], request.run_id);
    }
}
