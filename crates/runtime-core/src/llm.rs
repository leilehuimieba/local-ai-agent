use crate::contracts::RunRequest;
use crate::model_adapter::{ToolCall, ModelError};
use crate::model_client::complete_with_model;

#[derive(Clone, Debug)]
pub(crate) struct LlmResponse {
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
}

pub(crate) fn complete_text(request: &RunRequest, prompt: &str) -> Result<LlmResponse, String> {
    complete_with_model(request, prompt)
        .map(|response| LlmResponse {
            content: response.content,
            tool_calls: response.tool_calls,
        })
        .map_err(llm_error_message)
}

fn llm_error_message(error: ModelError) -> String {
    format!("{}: {}", error.code, error.message)
}
