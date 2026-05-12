use crate::capabilities::{
    ExternalConnectionSlot, ToolDefinition, capability_spec, connector_slot_spec, external_connection_slots,
    resolve_tool, tool_definition_to_json_schema, visible_tools,
};
use crate::context_builder::RuntimeContextEnvelope;
use crate::contracts::{CapabilitySpec, ConnectorSlotSpec, RunRequest};
use crate::planner::{PlannedAction, SearchOutputMode, plan_action_with_context};
use serde_json::Value;
use serde_json::json;

#[derive(Clone, Debug)]
pub(crate) struct ToolCall {
    pub action: PlannedAction,
    pub spec: ToolSpec,
}

pub(crate) type ToolSpec = ToolDefinition;

#[derive(Clone, Debug, Default)]
pub(crate) struct ToolRegistry;

pub(crate) fn runtime_tool_registry() -> ToolRegistry {
    ToolRegistry
}

pub(crate) fn tool_call_arguments_json(tool_call: &ToolCall) -> String {
    action_arguments_json(&tool_call.action)
}

pub(crate) fn action_arguments_json(action: &PlannedAction) -> String {
    action_arguments_json_inner(action)
}

fn action_arguments_json_inner(action: &PlannedAction) -> String {
    match action {
        PlannedAction::RunCommand {
            command,
            timeout_secs,
        } => {
            let mut obj = serde_json::Map::new();
            obj.insert("command".to_string(), json!(command));
            if let Some(t) = timeout_secs {
                obj.insert("timeout_secs".to_string(), json!(t));
            }
            Value::Object(obj).to_string()
        }
        PlannedAction::ReadFile { path, offset, limit } => {
            let mut obj = serde_json::Map::new();
            obj.insert("path".to_string(), json!(path));
            if let Some(o) = offset {
                obj.insert("offset".to_string(), json!(o));
            }
            if let Some(l) = limit {
                obj.insert("limit".to_string(), json!(l));
            }
            Value::Object(obj).to_string()
        }
        PlannedAction::WriteFile {
            path,
            content,
            write_mode,
        } => {
            let mut obj = serde_json::Map::new();
            obj.insert("path".to_string(), json!(path));
            obj.insert("content".to_string(), json!(content));
            if let Some(mode) = write_mode {
                obj.insert("write_mode".to_string(), json!(mode));
            }
            Value::Object(obj).to_string()
        }
        PlannedAction::ApplyPatch { diff, dry_run } => json!({ "diff": diff, "dry_run": dry_run }).to_string(),
        PlannedAction::DeletePath { path } => json!({ "path": path }).to_string(),
        PlannedAction::ListFiles {
            path,
            recursive,
            file_glob,
        } => {
            let mut obj = serde_json::Map::new();
            if let Some(p) = path {
                obj.insert("path".to_string(), json!(p));
            }
            if *recursive {
                obj.insert("recursive".to_string(), json!(true));
            }
            if let Some(g) = file_glob {
                obj.insert("file_glob".to_string(), json!(g));
            }
            Value::Object(obj).to_string()
        }
        PlannedAction::WriteMemory { kind, summary, content } => json!({
            "kind": kind,
            "summary": summary,
            "content": content
        })
        .to_string(),
        PlannedAction::RecallMemory { query } => json!({ "query": query }).to_string(),
        PlannedAction::SearchFiles {
            query,
            path,
            context_lines,
            file_glob,
            output_mode,
        } => {
            let mut obj = serde_json::Map::new();
            obj.insert("query".to_string(), json!(query));
            if let Some(p) = path {
                obj.insert("path".to_string(), json!(p));
            }
            if *context_lines > 0 {
                obj.insert("context_lines".to_string(), json!(context_lines));
            }
            if let Some(g) = file_glob {
                obj.insert("file_glob".to_string(), json!(g));
            }
            obj.insert(
                "output_mode".to_string(),
                json!(match output_mode {
                    SearchOutputMode::Content => "content",
                    SearchOutputMode::FilesWithMatches => "files_with_matches",
                    SearchOutputMode::Count => "count",
                }),
            );
            Value::Object(obj).to_string()
        }
        PlannedAction::SearchKnowledge { query } => json!({ "query": query }).to_string(),
        PlannedAction::SearchSiyuanNotes { query } => json!({ "query": query }).to_string(),
        PlannedAction::ReadSiyuanNote { path } => json!({ "path": path }).to_string(),
        PlannedAction::MCPCall { arguments_json, .. } => arguments_json.clone(),
        PlannedAction::WriteSiyuanKnowledge
        | PlannedAction::ProjectAnswer
        | PlannedAction::ContextAnswer
        | PlannedAction::Explain
        | PlannedAction::AgentResolve => "{}".to_string(),
    }
}

impl ToolRegistry {
    pub(crate) fn visible_tools(&self, mode: &str) -> Vec<ToolDefinition> {
        visible_tools(mode)
    }

    pub(crate) fn request_visible_tools(&self, request: &RunRequest) -> Vec<ToolDefinition> {
        merge_tools(
            self.visible_tools(&request.mode),
            crate::mcp_bridge::mcp_tool_definitions(request),
        )
    }

    pub(crate) fn capability_specs(&self, mode: &str) -> Vec<CapabilitySpec> {
        self.visible_tools(mode)
            .into_iter()
            .map(|tool| capability_spec(&tool))
            .collect()
    }

    pub(crate) fn request_capability_specs(&self, request: &RunRequest) -> Vec<CapabilitySpec> {
        self.request_visible_tools(request)
            .into_iter()
            .map(|tool| capability_spec(&tool))
            .collect()
    }

    pub(crate) fn request_tool_schemas(&self, request: &RunRequest) -> Vec<Value> {
        self.request_visible_tools(request)
            .iter()
            .map(tool_definition_to_json_schema)
            .collect()
    }

    #[allow(dead_code)]
    pub(crate) fn external_connection_slots(&self) -> Vec<ExternalConnectionSlot> {
        external_connection_slots()
    }

    pub(crate) fn connector_slot_specs(&self) -> Vec<ConnectorSlotSpec> {
        self.external_connection_slots()
            .iter()
            .map(connector_slot_spec)
            .collect()
    }

    pub(crate) fn plan_tool_call(&self, envelope: &RuntimeContextEnvelope) -> ToolCall {
        let action = plan_action_with_context(envelope);
        ToolCall {
            spec: resolve_tool(&action),
            action,
        }
    }
}

fn merge_tools(mut native: Vec<ToolDefinition>, dynamic: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    for tool in dynamic {
        if native.iter().any(|item| item.tool_name == tool.tool_name) {
            continue;
        }
        native.push(tool);
    }
    native
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_engine_testkit::testkit::sample_request;

    #[test]
    fn request_visible_tools_merges_native_and_mcp_tools() {
        let mut request = sample_request("mcp_visible");
        request
            .context_hints
            .insert("mcp_tool_specs_json".to_string(), sample_mcp_specs());
        let tools = runtime_tool_registry().request_visible_tools(&request);
        assert!(tools.iter().any(|tool| tool.tool_name == "workspace_read"));
        assert!(tools.iter().any(|tool| tool.tool_name == "mcp__docs__search"));
    }

    #[test]
    fn request_tool_schemas_keep_mcp_arguments_wrapper() {
        let mut request = sample_request("mcp_schema");
        request
            .context_hints
            .insert("mcp_tool_specs_json".to_string(), sample_mcp_specs());
        let schemas = runtime_tool_registry().request_tool_schemas(&request);
        let mcp = schemas
            .iter()
            .find(|item| item["function"]["name"] == "mcp__docs__search")
            .unwrap();
        assert_eq!(mcp["function"]["parameters"]["required"][0], "arguments");
    }

    #[test]
    fn request_capability_specs_include_mcp_connector_slot() {
        let mut request = sample_request("mcp_capabilities");
        request
            .context_hints
            .insert("mcp_tool_specs_json".to_string(), sample_mcp_specs());
        let items = runtime_tool_registry().request_capability_specs(&request);
        let mcp = items
            .iter()
            .find(|item| item.capability_id == "mcp__docs__search")
            .unwrap();
        assert_eq!(mcp.connector_slot, "mcp_gateway");
    }

    fn sample_mcp_specs() -> String {
        serde_json::json!([{
            "server_id": "docs",
            "name": "search",
            "function_name": "mcp__docs__search",
            "description": "Search docs",
            "risk_level": "low",
            "requires_confirmation": false,
            "audit_enabled": true,
            "input_schema": { "type": "object", "properties": { "q": { "type": "string" } } }
        }])
        .to_string()
    }
}
