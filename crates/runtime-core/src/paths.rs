use crate::contracts::RunRequest;
use std::path::{Component, Path, PathBuf};

pub(crate) fn resolve_workspace_path(root: &str, raw_path: &str) -> Result<PathBuf, String> {
    let workspace_root = normalize_path(Path::new(root));
    let candidate = if Path::new(raw_path).is_absolute() {
        normalize_path(Path::new(raw_path))
    } else {
        normalize_path(&workspace_root.join(raw_path))
    };

    if !candidate.starts_with(&workspace_root) {
        return Err("目标路径超出当前工作区，当前模式只允许在已选工作区内操作。".to_string());
    }

    Ok(candidate)
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(Path::new(std::path::MAIN_SEPARATOR_STR)),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    normalized
}

pub(crate) fn repo_root(request: &RunRequest) -> PathBuf {
    request
        .context_hints
        .get("repo_root")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(&request.workspace_ref.root_path))
}

fn data_root(request: &RunRequest) -> PathBuf {
    repo_root(request).join("data")
}

pub(crate) fn storage_dir(request: &RunRequest) -> PathBuf {
    data_root(request).join("storage")
}

pub(crate) fn session_file_path(request: &RunRequest) -> PathBuf {
    data_root(request)
        .join("sessions")
        .join(format!("{}.json", safe_name(&request.session_id)))
}

pub(crate) fn memory_file_path(request: &RunRequest) -> PathBuf {
    data_root(request).join("memory").join("entries.jsonl")
}

pub(crate) fn memory_tombstone_file_path(request: &RunRequest) -> PathBuf {
    data_root(request)
        .join("memory")
        .join("deletions")
        .join(format!(
            "{}.jsonl",
            safe_name(&request.workspace_ref.workspace_id)
        ))
}

pub(crate) fn long_term_memory_file_path(request: &RunRequest) -> PathBuf {
    data_root(request).join("long_term_memory").join(format!(
        "{}.jsonl",
        safe_name(&request.workspace_ref.workspace_id)
    ))
}

pub(crate) fn knowledge_base_file_path(request: &RunRequest) -> PathBuf {
    data_root(request).join("knowledge_base").join(format!(
        "{}.jsonl",
        safe_name(&request.workspace_ref.workspace_id)
    ))
}

pub(crate) fn sqlite_db_path(request: &RunRequest) -> PathBuf {
    storage_dir(request).join("main.db")
}

pub(crate) fn working_memory_dir(request: &RunRequest) -> PathBuf {
    data_root(request).join("working_memory")
}

pub(crate) fn daily_rollup_path(request: &RunRequest) -> PathBuf {
    data_root(request).join("daily").join("daily-rollup.jsonl")
}

pub(crate) fn answer_cache_file_path(request: &RunRequest) -> PathBuf {
    data_root(request).join("cache").join(format!(
        "{}.jsonl",
        safe_name(&request.workspace_ref.workspace_id)
    ))
}

pub(crate) fn artifact_dir(request: &RunRequest) -> PathBuf {
    data_root(request)
        .join("artifacts")
        .join(safe_name(&request.session_id))
        .join(safe_name(&request.run_id))
}

pub(crate) fn artifact_index_path(request: &RunRequest) -> PathBuf {
    data_root(request).join("artifacts").join("index.jsonl")
}

pub(crate) fn external_memory_audit_path(request: &RunRequest) -> PathBuf {
    data_root(request)
        .join("logs")
        .join("external-memory-cortex.jsonl")
}

pub(crate) fn siyuan_root_dir(request: &RunRequest) -> Option<PathBuf> {
    request.context_hints.get("siyuan_root").map(PathBuf::from)
}

pub(crate) fn siyuan_export_dir(request: &RunRequest) -> Option<PathBuf> {
    request
        .context_hints
        .get("siyuan_export_dir")
        .map(PathBuf::from)
}

pub(crate) fn siyuan_auto_write_enabled(request: &RunRequest) -> bool {
    request
        .context_hints
        .get("siyuan_auto_write_enabled")
        .is_some_and(|value| value == "true")
}

pub(crate) fn siyuan_sync_enabled(request: &RunRequest) -> bool {
    request
        .context_hints
        .get("siyuan_sync_enabled")
        .is_some_and(|value| value == "true")
}

fn safe_name(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
