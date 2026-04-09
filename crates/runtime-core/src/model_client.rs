use crate::contracts::RunRequest;
use crate::model_adapter::{
    ModelAdapter, ModelError, ModelRequest, ModelResponse, OpenAiCompatibleAdapter, provider_config,
};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    temperature: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

pub(crate) fn complete_with_model(
    request: &RunRequest,
    prompt: &str,
) -> Result<ModelResponse, ModelError> {
    let provider = provider_config(&request.provider_ref)?;
    let adapter = OpenAiCompatibleAdapter { provider };
    let body_path = write_body_file(request, prompt)?;
    let model_request = ModelRequest {
        model: &request.model_ref.model_id,
        prompt,
        tools: None,
    };
    let response = adapter.complete(&model_request, &body_path);
    let _ = fs::remove_file(&body_path);
    response
}

fn write_body_file(request: &RunRequest, prompt: &str) -> Result<PathBuf, ModelError> {
    let visible_tools = crate::capabilities::visible_tools("full_access");
    let mapped_tools: Vec<serde_json::Value> = visible_tools
        .iter()
        .map(crate::capabilities::tool_definition_to_json_schema)
        .collect();
    let tools = if mapped_tools.is_empty() {
        None
    } else {
        Some(mapped_tools)
    };
    let body = serde_json::to_string(&ChatRequest {
        model: &request.model_ref.model_id,
        messages: vec![Message {
            role: "user",
            content: prompt,
        }],
        temperature: 0,
        tools,
    })
    .map_err(|error| model_error(&error.to_string()))?;
    let path = std::env::temp_dir().join(format!("local-agent-llm-{}.json", timestamp_now()));
    fs::write(&path, body).map_err(|error| model_error(&error.to_string()))?;
    Ok(path)
}

fn model_error(message: &str) -> ModelError {
    ModelError {
        code: "model_request_build_failed".to_string(),
        message: message.to_string(),
        retryable: false,
    }
}

fn timestamp_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}
