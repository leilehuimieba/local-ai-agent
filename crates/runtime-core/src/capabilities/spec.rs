use crate::contracts::CapabilitySpec;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ToolDefinition {
    pub tool_name: String,
    pub display_name: String,
    pub category: String,
    pub risk_level: String,
    pub input_schema: String,
    pub output_kind: String,
    pub requires_confirmation: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct ToolCallResult {
    pub summary: String,
    pub final_answer: String,
    pub artifact_path: Option<String>,
    pub error_code: Option<String>,
    pub retryable: bool,
    pub success: bool,
    pub memory_write_summary: Option<String>,
    pub reasoning_summary: String,
    pub cache_status: String,
    pub cache_reason: String,
}

#[derive(Clone, Debug)]
pub(crate) struct ToolExecutionTrace {
    pub tool: ToolDefinition,
    pub action_summary: String,
    pub result: ToolCallResult,
}

pub(crate) fn capability_spec(tool: &ToolDefinition) -> CapabilitySpec {
    let connector_slot = connector_slot(tool);
    CapabilitySpec {
        capability_id: tool.tool_name.clone(),
        display_name: tool.display_name.clone(),
        domain: tool.category.clone(),
        risk_level: tool.risk_level.clone(),
        input_schema: tool.input_schema.clone(),
        output_kind: tool.output_kind.clone(),
        side_effect_level: side_effect_level(tool).to_string(),
        supports_modes: supports_modes(tool),
        verification_policy: verification_policy(tool).to_string(),
        source_kind: source_kind(&connector_slot).to_string(),
        connector_slot,
        requires_confirmation: tool.requires_confirmation,
    }
}

pub(crate) fn tool_definition_to_json_schema(tool: &ToolDefinition) -> Value {
    let properties = tool_schema_properties(&tool.input_schema);
    let required = tool_schema_required(&tool.input_schema);
    let mut parameters = serde_json::json!({ "type": "object", "properties": properties });
    if !required.is_empty() {
        parameters["required"] = serde_json::json!(required);
    }
    serde_json::json!({
        "type": "function",
        "function": {
            "name": tool.tool_name,
            "description": tool.display_name,
            "parameters": parameters,
        }
    })
}

fn side_effect_level(tool: &ToolDefinition) -> &'static str {
    match tool.category.as_str() {
        "workspace_read" | "knowledge_read" | "memory_read" | "assistant_answer" => "read_only",
        "workspace_write" => "workspace_mutation",
        "memory_write" => "memory_mutation",
        "knowledge_write" => "connector_mutation",
        "system_command" => "system_side_effect",
        "agent" => "agent_side_effect",
        _ => "local_side_effect",
    }
}

fn supports_modes(tool: &ToolDefinition) -> Vec<String> {
    let modes = match tool.tool_name.as_str() {
        "memory_write" | "write_siyuan_knowledge" => ["full_access"].as_slice(),
        "workspace_write" | "workspace_delete" | "run_command" => {
            ["standard", "full_access"].as_slice()
        }
        _ => ["observe", "standard", "full_access"].as_slice(),
    };
    modes.iter().map(|item| (*item).to_string()).collect()
}

fn verification_policy(tool: &ToolDefinition) -> &'static str {
    match tool.tool_name.as_str() {
        "workspace_write" => "confirm_write_effect",
        "workspace_delete" => "confirm_delete_effect",
        "run_command" => "inspect_command_result",
        "memory_write" => "confirm_memory_persisted",
        "knowledge_search" | "search_siyuan_notes" | "read_siyuan_note" => "check_result_relevance",
        _ => "check_result_summary",
    }
}

fn connector_slot(tool: &ToolDefinition) -> String {
    match tool.tool_name.as_str() {
        "workspace_list" | "workspace_read" | "workspace_write" | "workspace_delete"
        | "run_command" => "local_files_project".to_string(),
        "knowledge_search"
        | "search_siyuan_notes"
        | "read_siyuan_note"
        | "write_siyuan_knowledge" => "local_notes_knowledge".to_string(),
        _ => String::new(),
    }
}

fn source_kind(connector_slot: &str) -> &'static str {
    if connector_slot.is_empty() {
        "runtime_native"
    } else {
        "connector_backed"
    }
}

fn tool_schema_properties(input_schema: &str) -> Value {
    match input_schema {
        "command_text" => serde_json::json!({
            "command": { "type": "string", "description": "The command line string to execute" }
        }),
        "path" => serde_json::json!({
            "path": { "type": "string", "description": "The file or directory path" }
        }),
        "path_and_content" => serde_json::json!({
            "path": { "type": "string", "description": "The file path" },
            "content": { "type": "string", "description": "The content to write" }
        }),
        "optional_path" => serde_json::json!({
            "path": { "type": "string", "description": "The file or directory path (optional)" }
        }),
        "memory_entry" => serde_json::json!({
            "kind": { "type": "string", "description": "The category of memory" },
            "summary": { "type": "string", "description": "A short summary of the memory" },
            "content": { "type": "string", "description": "The full content of the memory" }
        }),
        "query" => serde_json::json!({
            "query": { "type": "string", "description": "The search term or query string" }
        }),
        "none" => serde_json::json!({}),
        _ => serde_json::json!({}),
    }
}

fn tool_schema_required(input_schema: &str) -> Vec<&'static str> {
    match input_schema {
        "command_text" => vec!["command"],
        "path" => vec!["path"],
        "path_and_content" => vec!["path", "content"],
        "memory_entry" => vec!["kind", "summary", "content"],
        "query" => vec!["query"],
        "optional_path" | "none" => vec![],
        _ => vec![],
    }
}
