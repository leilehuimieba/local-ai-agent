use crate::planner::PlannedAction;

pub(super) fn next_read_action(base: Option<&str>, answer: &str) -> Option<PlannedAction> {
    let path = extract_candidate_path(answer, &["AGENTS.md", "README.md", "Cargo.toml"])?;
    Some(PlannedAction::ReadFile {
        path: join_base_path(base, &path),
    })
}

pub(super) fn next_list_action(path: &str) -> Option<PlannedAction> {
    std::path::Path::new(path)
        .parent()
        .map(|item| item.display().to_string())
        .map(|path| PlannedAction::ListFiles { path: Some(path) })
}

fn extract_candidate_path(answer: &str, names: &[&str]) -> Option<String> {
    answer.lines().map(str::trim).find_map(|line| {
        names
            .iter()
            .find(|name| line.contains(**name))
            .map(|_| clean_path(line))
    })
}

fn clean_path(line: &str) -> String {
    line.trim_matches(|ch| ch == '-' || ch == '*' || ch == '`' || ch == ' ')
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_matches(|ch| ch == '"' || ch == '\'')
        .to_string()
}

fn join_base_path(base: Option<&str>, candidate: &str) -> String {
    if candidate.contains(':') || candidate.starts_with('/') || candidate.starts_with('\\') {
        return candidate.to_string();
    }
    match base {
        Some(value) if !value.is_empty() => format!("{value}/{candidate}"),
        _ => candidate.to_string(),
    }
}
