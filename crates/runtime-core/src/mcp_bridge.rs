use crate::capabilities::ToolDefinition;
use crate::contracts::RunRequest;
use serde::Deserialize;
use serde_json::Value;

const MCP_PREFIX: &str = "mcp__";

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct MCPToolSpec {
    pub server_id: String,
    pub name: String,
    pub function_name: String,
    pub description: String,
    pub risk_level: String,
    pub requires_confirmation: bool,
    pub audit_enabled: bool,
    pub input_schema: Option<Value>,
}

#[cfg(test)]
pub(crate) fn mcp_tool_schemas(request: &RunRequest) -> Vec<Value> {
    mcp_tool_definitions(request)
        .iter()
        .map(crate::capabilities::tool_definition_to_json_schema)
        .collect()
}

pub(crate) fn mcp_tool_definitions(request: &RunRequest) -> Vec<ToolDefinition> {
    mcp_tool_specs(request).iter().map(mcp_tool_definition).collect()
}

pub(crate) fn mcp_tool_specs(request: &RunRequest) -> Vec<MCPToolSpec> {
    let Some(raw) = request.context_hints.get("mcp_tool_specs_json") else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<MCPToolSpec>>(raw)
        .unwrap_or_default()
        .into_iter()
        .filter(executable_spec)
        .collect()
}

pub(crate) fn mcp_action_parts(function_name: &str) -> Option<(String, String)> {
    let rest = function_name.strip_prefix(MCP_PREFIX)?;
    let (server_id, name) = rest.split_once("__")?;
    valid_name_part(server_id)
        .then_some(())
        .filter(|_| valid_name_part(name))?;
    Some((server_id.to_string(), name.to_string()))
}

pub(crate) fn mcp_function_name(server_id: &str, name: &str) -> String {
    format!("{MCP_PREFIX}{server_id}__{name}")
}

fn executable_spec(spec: &MCPToolSpec) -> bool {
    if spec.requires_confirmation {
        return false;
    }
    spec.function_name == mcp_function_name(&spec.server_id, &spec.name)
        && mcp_action_parts(&spec.function_name).is_some()
}

fn mcp_tool_definition(spec: &MCPToolSpec) -> ToolDefinition {
    ToolDefinition {
        tool_name: spec.function_name.clone(),
        display_name: format!("MCP: {}/{}", spec.server_id, spec.name),
        category: "mcp".to_string(),
        risk_level: spec.risk_level.clone(),
        input_schema: mcp_input_schema(spec),
        output_kind: "json_preview".to_string(),
        requires_confirmation: spec.requires_confirmation,
        model_schema: Some(serde_json::json!({
            "type": "function",
            "function": {
                "name": spec.function_name,
                "description": mcp_description(spec),
                "parameters": mcp_parameters(spec),
            }
        })),
    }
}

fn mcp_description(spec: &MCPToolSpec) -> String {
    let audit = if spec.audit_enabled { "audit=on" } else { "audit=off" };
    format!(
        "MCP {}/{} [risk={}, {}] {}",
        spec.server_id, spec.name, spec.risk_level, audit, spec.description
    )
}

fn mcp_parameters(spec: &MCPToolSpec) -> Value {
    let schema = spec.input_schema.clone().unwrap_or_else(default_arg_schema);
    serde_json::json!({
        "type": "object",
        "properties": {
            "arguments": schema,
        },
        "required": ["arguments"]
    })
}

fn mcp_input_schema(spec: &MCPToolSpec) -> String {
    spec.input_schema.clone().unwrap_or_else(default_arg_schema).to_string()
}

fn default_arg_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "description": "JSON arguments passed to the MCP tool"
    })
}

fn valid_name_part(value: &str) -> bool {
    !value.is_empty() && !value.contains("__") && value.chars().all(valid_name_char)
}

fn valid_name_char(ch: char) -> bool {
    ch == '_' || ch == '-' || ch.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_engine_testkit::testkit::sample_request;

    #[test]
    fn mcp_schemas_use_executable_specs_only() {
        let mut request = sample_request("mcp");
        request
            .context_hints
            .insert("mcp_tool_specs_json".to_string(), specs_json());
        let schemas = mcp_tool_schemas(&request);
        assert_eq!(schemas.len(), 1);
        assert_eq!(schemas[0]["function"]["name"], "mcp__docs__search");
    }

    #[test]
    fn mcp_action_parts_rejects_ambiguous_names() {
        assert!(mcp_action_parts("mcp__docs__search").is_some());
        assert!(mcp_action_parts("mcp__bad__id__search").is_none());
    }

    fn specs_json() -> String {
        serde_json::json!([
            {
                "server_id": "docs",
                "name": "search",
                "function_name": "mcp__docs__search",
                "description": "Search docs",
                "risk_level": "low",
                "requires_confirmation": false,
                "audit_enabled": true,
                "input_schema": {"type": "object"}
            },
            {
                "server_id": "docs",
                "name": "write",
                "function_name": "mcp__docs__write",
                "description": "Write docs",
                "risk_level": "high",
                "requires_confirmation": true,
                "audit_enabled": true
            }
        ])
        .to_string()
    }
}
