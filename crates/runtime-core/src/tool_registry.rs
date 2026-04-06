use crate::context_builder::RuntimeContextEnvelope;
use crate::planner::{PlannedAction, plan_action_with_context};
use crate::tools::{
    ExternalConnectionSlot, ToolDefinition, external_connection_slots, resolve_tool, visible_tools,
};

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

impl ToolRegistry {
    pub(crate) fn visible_tools(&self, mode: &str) -> Vec<ToolDefinition> {
        visible_tools(mode)
    }

    #[allow(dead_code)]
    pub(crate) fn external_connection_slots(&self) -> Vec<ExternalConnectionSlot> {
        external_connection_slots()
    }

    pub(crate) fn plan_tool_call(&self, envelope: &RuntimeContextEnvelope) -> ToolCall {
        let action = plan_action_with_context(envelope);
        ToolCall {
            spec: resolve_tool(&action),
            action,
        }
    }
}
