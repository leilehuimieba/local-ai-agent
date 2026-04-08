use crate::planner::PlannedAction;

pub(crate) fn tool_call_to_action(name: &str, arguments: &str) -> Option<PlannedAction> {
    let args: serde_json::Value = serde_json::from_str(arguments).unwrap_or_default();
    match name {
        "run_command" => tool_call_command(&args),
        "workspace_read" => tool_call_read(&args),
        "workspace_write" => tool_call_write(&args),
        "workspace_delete" => tool_call_delete(&args),
        "workspace_list" => tool_call_list(&args),
        "memory_write" => tool_call_memory_write(&args),
        "memory_recall" => tool_call_memory_recall(&args),
        "knowledge_search" => tool_call_knowledge_search(&args),
        "search_siyuan_notes" => tool_call_siyuan_search(&args),
        "read_siyuan_note" => tool_call_siyuan_read(&args),
        "write_siyuan_knowledge" => Some(PlannedAction::WriteSiyuanKnowledge),
        _ => None,
    }
}

fn tool_call_command(args: &serde_json::Value) -> Option<PlannedAction> {
    let command = args["command"].as_str()?.to_string();
    Some(PlannedAction::RunCommand { command })
}

fn tool_call_read(args: &serde_json::Value) -> Option<PlannedAction> {
    let path = args["path"].as_str()?.to_string();
    Some(PlannedAction::ReadFile { path })
}

fn tool_call_write(args: &serde_json::Value) -> Option<PlannedAction> {
    let path = args["path"].as_str()?.to_string();
    let content = args["content"].as_str().unwrap_or("").to_string();
    Some(PlannedAction::WriteFile { path, content })
}

fn tool_call_delete(args: &serde_json::Value) -> Option<PlannedAction> {
    let path = args["path"].as_str()?.to_string();
    Some(PlannedAction::DeletePath { path })
}

fn tool_call_list(args: &serde_json::Value) -> Option<PlannedAction> {
    let path = args["path"].as_str().map(|s| s.to_string());
    Some(PlannedAction::ListFiles { path })
}

fn tool_call_memory_write(args: &serde_json::Value) -> Option<PlannedAction> {
    let kind = args["kind"]
        .as_str()
        .unwrap_or("project_knowledge")
        .to_string();
    let summary = args["summary"].as_str().unwrap_or("").to_string();
    let content = args["content"].as_str().unwrap_or("").to_string();
    Some(PlannedAction::WriteMemory {
        kind,
        summary,
        content,
    })
}

fn tool_call_memory_recall(args: &serde_json::Value) -> Option<PlannedAction> {
    let query = args["query"].as_str()?.to_string();
    Some(PlannedAction::RecallMemory { query })
}

fn tool_call_knowledge_search(args: &serde_json::Value) -> Option<PlannedAction> {
    let query = args["query"].as_str()?.to_string();
    Some(PlannedAction::SearchKnowledge { query })
}

fn tool_call_siyuan_search(args: &serde_json::Value) -> Option<PlannedAction> {
    let query = args["query"].as_str()?.to_string();
    Some(PlannedAction::SearchSiyuanNotes { query })
}

fn tool_call_siyuan_read(args: &serde_json::Value) -> Option<PlannedAction> {
    let path = args["path"].as_str()?.to_string();
    Some(PlannedAction::ReadSiyuanNote { path })
}
