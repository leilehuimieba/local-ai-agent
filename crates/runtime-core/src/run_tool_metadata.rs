use std::collections::BTreeMap;

pub(crate) fn append_tool_spec_metadata(
    metadata: &mut BTreeMap<String, String>,
    tool_call: &crate::tool_registry::ToolCall,
) {
    metadata.insert(
        "input_schema".to_string(),
        tool_call.spec.input_schema.clone(),
    );
    metadata.insert(
        "requires_confirmation".to_string(),
        bool_string(tool_call.spec.requires_confirmation),
    );
    metadata.insert(
        "tool_arguments_json".to_string(),
        crate::tool_registry::tool_call_arguments_json(tool_call),
    );
}

fn bool_string(value: bool) -> String {
    if value {
        "true".to_string()
    } else {
        "false".to_string()
    }
}
