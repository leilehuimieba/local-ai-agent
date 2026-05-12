use crate::planner::{PlannedAction, SearchOutputMode};

pub(crate) fn tool_call_to_action(name: &str, arguments: &str) -> Option<PlannedAction> {
    let args: serde_json::Value = serde_json::from_str(arguments).unwrap_or_default();
    match name {
        "run_command" => tool_call_command(&args),
        "workspace_read" => tool_call_read(&args),
        "workspace_write" => tool_call_write(&args),
        "workspace_apply_patch" => tool_call_apply_patch(&args),
        "workspace_delete" => tool_call_delete(&args),
        "workspace_list" => tool_call_list(&args),
        "workspace_search" => tool_call_search_files(&args),
        "memory_write" => tool_call_memory_write(&args),
        "memory_recall" => tool_call_memory_recall(&args),
        "knowledge_search" => tool_call_knowledge_search(&args),
        "search_siyuan_notes" => tool_call_siyuan_search(&args),
        "read_siyuan_note" => tool_call_siyuan_read(&args),
        "write_siyuan_knowledge" => Some(PlannedAction::WriteSiyuanKnowledge),
        _ => tool_call_mcp(name, &args),
    }
}

fn tool_call_command(args: &serde_json::Value) -> Option<PlannedAction> {
    let command = args["command"].as_str()?.to_string();
    let timeout_secs = args["timeout_secs"].as_u64().map(|v| v as u32);
    Some(PlannedAction::RunCommand {
        command,
        timeout_secs,
    })
}

fn tool_call_read(args: &serde_json::Value) -> Option<PlannedAction> {
    let path = args["path"].as_str()?.to_string();
    let offset = args["offset"].as_u64().map(|v| v as usize);
    let limit = args["limit"].as_u64().map(|v| v as usize);
    Some(PlannedAction::ReadFile { path, offset, limit })
}

fn tool_call_write(args: &serde_json::Value) -> Option<PlannedAction> {
    let path = args["path"].as_str()?.to_string();
    let content = args["content"].as_str().unwrap_or("").to_string();
    let write_mode = args["write_mode"].as_str().map(|s| s.to_string());
    Some(PlannedAction::WriteFile {
        path,
        content,
        write_mode,
    })
}

fn tool_call_apply_patch(args: &serde_json::Value) -> Option<PlannedAction> {
    let diff = args["diff"].as_str()?.to_string();
    let dry_run = args["dry_run"].as_bool().unwrap_or(false);
    Some(PlannedAction::ApplyPatch { diff, dry_run })
}

fn tool_call_delete(args: &serde_json::Value) -> Option<PlannedAction> {
    let path = args["path"].as_str()?.to_string();
    Some(PlannedAction::DeletePath { path })
}

fn tool_call_list(args: &serde_json::Value) -> Option<PlannedAction> {
    let path = args["path"].as_str().map(|s| s.to_string());
    let recursive = args["recursive"].as_bool().unwrap_or(false);
    let file_glob = args["file_glob"].as_str().map(|s| s.to_string());
    Some(PlannedAction::ListFiles {
        path,
        recursive,
        file_glob,
    })
}

fn tool_call_search_files(args: &serde_json::Value) -> Option<PlannedAction> {
    let query = args["query"].as_str()?.to_string();
    let path = args["path"].as_str().map(|s| s.to_string()).filter(|s| !s.is_empty());
    let context_lines = args["context_lines"].as_u64().unwrap_or(0) as u32;
    let file_glob = args["file_glob"].as_str().map(|s| s.to_string()).filter(|s| !s.is_empty());
    let output_mode = match args["output_mode"].as_str().unwrap_or("content") {
        "files_with_matches" => SearchOutputMode::FilesWithMatches,
        "count" => SearchOutputMode::Count,
        _ => SearchOutputMode::Content,
    };
    Some(PlannedAction::SearchFiles {
        query,
        path,
        context_lines,
        file_glob,
        output_mode,
    })
}

fn tool_call_memory_write(args: &serde_json::Value) -> Option<PlannedAction> {
    let kind = args["kind"].as_str().unwrap_or("project_knowledge").to_string();
    let summary = args["summary"].as_str().unwrap_or("").to_string();
    let content = args["content"].as_str().unwrap_or("").to_string();
    Some(PlannedAction::WriteMemory { kind, summary, content })
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

fn tool_call_mcp(name: &str, args: &serde_json::Value) -> Option<PlannedAction> {
    let (server_id, tool_name) = crate::mcp_bridge::mcp_action_parts(name)?;
    Some(PlannedAction::MCPCall {
        server_id,
        tool_name,
        arguments_json: mcp_arguments_json(args),
    })
}

fn mcp_arguments_json(args: &serde_json::Value) -> String {
    args.get("arguments")
        .cloned()
        .unwrap_or_else(|| args.clone())
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::tool_call_to_action;
    use crate::planner::PlannedAction;

    #[test]
    fn decodes_mcp_tool_call_arguments_wrapper() {
        let action = tool_call_to_action("mcp__docs__search", r#"{"arguments":{"q":"rust"}}"#);
        assert!(matches!(
            action,
            Some(PlannedAction::MCPCall { server_id, tool_name, arguments_json })
            if server_id == "docs" && tool_name == "search" && arguments_json == r#"{"q":"rust"}"#
        ));
    }

    #[test]
    fn decodes_apply_patch_tool_call() {
        let action = tool_call_to_action("workspace_apply_patch", r#"{"diff":"--- a/a\n+++ b/a","dry_run":true}"#);
        assert!(matches!(
            action,
            Some(PlannedAction::ApplyPatch { diff, dry_run }) if dry_run && diff.contains("+++ b/a")
        ));
    }
}
