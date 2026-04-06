use crate::contracts::ProviderRef;
use crate::tools::ToolDefinition;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Debug)]
pub(crate) struct ProviderConfig {
    pub base_url: String,
    pub chat_completions_path: String,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ModelRequest<'a> {
    pub model: &'a str,
    pub prompt: &'a str,
    // 新增：LLM 需要的工具列表，使用 ToolDefinition 描述
    pub tools: Option<Vec<ToolDefinition>>,
}

#[derive(Clone, Debug)]
pub(crate) struct ModelResponse {
    // 当返回普通文本时使用
    pub content: String,
    // 当返回工具调用时使用
    pub tool_calls: Option<Vec<ToolCall>>, 
}

#[derive(Clone, Debug)]
pub(crate) struct ModelError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

pub(crate) trait ModelAdapter {
    fn complete(
        &self,
        request: &ModelRequest<'_>,
        body_path: &PathBuf,
    ) -> Result<ModelResponse, ModelError>;
}

#[derive(Clone, Debug)]
pub(crate) struct OpenAiCompatibleAdapter {
    pub provider: ProviderConfig,
}

impl ModelAdapter for OpenAiCompatibleAdapter {
    fn complete(
        &self,
        request: &ModelRequest<'_>,
        body_path: &PathBuf,
    ) -> Result<ModelResponse, ModelError> {
        let uri = model_uri(&self.provider);
        let output = run_curl_with_retry(&self.provider, body_path, &uri)?;
        parse_model_response(request, &output)
    }
}

pub(crate) fn provider_config(provider: &ProviderRef) -> Result<ProviderConfig, ModelError> {
    if provider.base_url.is_empty() || provider.api_key.is_empty() {
        return Err(model_error(
            "provider_not_configured",
            "provider 未配置",
            false,
        ));
    }
    Ok(ProviderConfig {
        base_url: provider.base_url.clone(),
        chat_completions_path: provider.chat_completions_path.clone(),
        api_key: provider.api_key.clone(),
    })
}

fn model_uri(provider: &ProviderConfig) -> String {
    format!(
        "{}{}",
        provider.base_url.trim_end_matches('/'),
        provider.chat_completions_path
    )
}

fn run_curl(
    provider: &ProviderConfig,
    body_path: &PathBuf,
    uri: &str,
) -> Result<Vec<u8>, ModelError> {
    let output = Command::new("curl.exe")
        .arg("-k") // [新增] 跳过 SSL 证书校验，解决本机代理 (Clash/V2ray 等) 握手失败问题
        .arg("-s")
        .arg("-S")
        .arg("--connect-timeout")
        .arg("5")
        .arg("--max-time")
        .arg("12")
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", provider.api_key))
        .arg("-H")
        .arg("Content-Type: application/json; charset=utf-8")
        .arg("--data-binary")
        .arg(format!("@{}", body_path.display()))
        .arg(uri)
        .output()
        .map_err(|error| model_error("model_transport_failed", &error.to_string(), true))?;
    validate_curl_output(output)
}

fn run_curl_with_retry(
    provider: &ProviderConfig,
    body_path: &PathBuf,
    uri: &str,
) -> Result<Vec<u8>, ModelError> {
    let mut last_error = None;
    for attempt in 0..3 {
        match run_curl(provider, body_path, uri) {
            Ok(output) => return Ok(output),
            Err(error) if should_retry(&error) && attempt < 2 => {
                last_error = Some(error);
                sleep(retry_delay(attempt));
            }
            Err(error) => return Err(finalize_error(error)),
        }
    }
    Err(finalize_error(last_error.unwrap_or_else(|| {
        model_error("model_transport_failed", "模型请求失败", true)
    })))
}

fn validate_curl_output(output: std::process::Output) -> Result<Vec<u8>, ModelError> {
    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(model_error("model_transport_failed", &message, true));
    }
    if output.stdout.is_empty() {
        return Err(model_error("llm_empty_response", "模型返回为空", true));
    }
    Ok(output.stdout)
}

fn should_retry(error: &ModelError) -> bool {
    error.retryable && transient_message(&error.message)
}

fn transient_message(message: &str) -> bool {
    let lower = message.to_lowercase();
    [
        "connection was reset",
        "timeout",
        "timed out",
        "recv failure",
        "connection aborted",
        "empty response",
        "http 5",
    ]
    .iter()
    .any(|token| lower.contains(token))
}

fn retry_delay(attempt: usize) -> Duration {
    Duration::from_millis(((attempt + 1) * 400) as u64)
}

fn finalize_error(error: ModelError) -> ModelError {
    if should_retry(&error) {
        ModelError {
            code: error.code,
            message: format!("provider 瞬时失败，已自动重试仍未恢复：{}", error.message),
            retryable: true,
        }
    } else {
        error
    }
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ToolCall {
    pub id: String,
    pub function: ToolFunction,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ToolFunction {
    pub name: String,
    pub arguments: String, // arguments is often passed as a string field containing JSON
}

#[derive(Deserialize)]
struct ResponseMessage {
    // 当返回普通文本时使用
    content: Option<String>,
    // 当返回工具调用时使用
    tool_calls: Option<Vec<ToolCall>>,
}

fn parse_model_response(
    _request: &ModelRequest<'_>,
    output: &[u8],
) -> Result<ModelResponse, ModelError> {
    let parsed: ChatResponse = serde_json::from_slice(output)
        .map_err(|error| model_error("model_parse_failed", &error.to_string(), true))?;
    let message = parsed
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| model_error("empty_llm_choices", "模型未返回 choices", true))?
        .message;
    Ok(ModelResponse {
        content: message.content.unwrap_or_default().trim().to_string(),
        tool_calls: message.tool_calls,
    })
}

fn model_error(code: &str, message: &str, retryable: bool) -> ModelError {
    ModelError {
        code: code.to_string(),
        message: message.to_string(),
        retryable,
    }
}
