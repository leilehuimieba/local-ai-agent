use crate::capabilities::{
    ExternalConnectionSlot, ToolDefinition, capability_spec, connector_slot_spec,
    external_connection_slots, resolve_tool, visible_tools,
};
use crate::context_builder::RuntimeContextEnvelope;
use crate::contracts::{CapabilitySpec, ConnectorSlotSpec};
use crate::planner::{PlannedAction, plan_action_with_context};

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
