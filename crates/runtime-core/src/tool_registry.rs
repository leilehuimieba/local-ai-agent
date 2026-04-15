use crate::capabilities::{
    capability_spec, connector_slot_spec, external_connection_slots, resolve_tool, visible_tools,
    ExternalConnectionSlot, ToolDefinition,
};
use crate::context_builder::RuntimeContextEnvelope;
use crate::contracts::{CapabilitySpec, ConnectorSlotSpec};
use crate::planner::{plan_action_with_context, PlannedAction};
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

fn action_arguments_json(action: &PlannedAction) -> String {
    match action {
        PlannedAction::RunCommand { command } => json!({ "command": command }).to_string(),
        PlannedAction::ReadFile { path } => json!({ "path": path }).to_string(),
        PlannedAction::WriteFile { path, content } => {
            json!({ "path": path, "content": content }).to_string()
        }
        PlannedAction::DeletePath { path } => json!({ "path": path }).to_string(),
        PlannedAction::ListFiles { path } => json!({ "path": path }).to_string(),
        PlannedAction::WriteMemory {
            kind,
            summary,
            content,
        } => json!({
            "kind": kind,
            "summary": summary,
            "content": content
        })
        .to_string(),
        PlannedAction::RecallMemory { query } => json!({ "query": query }).to_string(),
        PlannedAction::SearchKnowledge { query } => json!({ "query": query }).to_string(),
        PlannedAction::SearchSiyuanNotes { query } => json!({ "query": query }).to_string(),
        PlannedAction::ReadSiyuanNote { path } => json!({ "path": path }).to_string(),
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

    pub(crate) fn capability_specs(&self, mode: &str) -> Vec<CapabilitySpec> {
        self.visible_tools(mode)
            .into_iter()
            .map(|tool| capability_spec(&tool))
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
