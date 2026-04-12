use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::paths::resolve_workspace_path;
use crate::text::summarize_text;
use std::fs;
use std::path::Path;

const CACHE_READ_REASON: &str = "文件读取结果依赖实时文件内容，不使用回答缓存。";
const CACHE_WRITE_REASON: &str = "文件写入属于实时副作用动作，不使用回答缓存。";
const CACHE_DELETE_REASON: &str = "删除属于实时副作用动作，不使用回答缓存。";
const CACHE_LIST_REASON: &str = "目录浏览依赖实时文件系统状态，不使用回答缓存。";

pub(crate) fn execute_file_read(request: &RunRequest, path: &str) -> ActionExecution {
    let resolved = match resolve_path(request, "读取文件", path, CACHE_READ_REASON) {
        Ok(resolved) => resolved,
        Err(outcome) => return outcome,
    };
    match fs::read_to_string(&resolved) {
        Ok(content) => ok_file_read(&resolved, &content),
        Err(error) => fail(
            format!("读取文件：{}", path),
            format!("文件读取失败：{}", error),
            format!("文件读取失败：{}", error),
            "目标文件读取失败，按错误直接返回。",
            CACHE_READ_REASON,
        ),
    }
}

pub(crate) fn execute_file_write(
    request: &RunRequest,
    path: &str,
    content: &str,
) -> ActionExecution {
    let resolved = match resolve_path(request, "写入文件", path, CACHE_WRITE_REASON) {
        Ok(resolved) => resolved,
        Err(outcome) => return outcome,
    };
    if let Err(error) = ensure_parent_dir(&resolved) {
        return fail_create_parent_dir(&resolved, &error.to_string(), CACHE_WRITE_REASON);
    }
    if let Err(error) = fs::write(&resolved, content) {
        return fail_write_file(&resolved, &error.to_string(), CACHE_WRITE_REASON);
    }
    ok_file_write(&resolved, content)
}

pub(crate) fn execute_delete_path(request: &RunRequest, path: &str) -> ActionExecution {
    let resolved = match resolve_path(request, "删除路径", path, CACHE_DELETE_REASON) {
        Ok(resolved) => resolved,
        Err(outcome) => return outcome,
    };
    let result = if resolved.is_dir() {
        fs::remove_dir_all(&resolved)
    } else {
        fs::remove_file(&resolved)
    };
    match result {
        Ok(()) => ok_file_delete(&resolved),
        Err(error) => fail(
            format!("删除路径：{}", resolved.display()),
            format!("删除失败：{}", error),
            format!("删除失败：{}", error),
            "删除阶段失败，直接按系统错误返回。",
            CACHE_DELETE_REASON,
        ),
    }
}

pub(crate) fn execute_list_files(request: &RunRequest, path: Option<&str>) -> ActionExecution {
    let base_path = path.unwrap_or(".");
    let Ok(resolved) = resolve_workspace_path(&request.workspace_ref.root_path, base_path) else {
        return invalid_path(
            "列出目录",
            base_path,
            "目标路径越界或解析失败",
            CACHE_LIST_REASON,
        );
    };
    match fs::read_dir(&resolved) {
        Ok(entries) => {
            let joined = preview_dir_entries(entries);
            ok(
                format!("列出目录：{}", resolved.display()),
                format!("目录列举成功：{}", joined),
                format!("目录内容：{}\n{}", resolved.display(), joined),
                "读取目标目录首批条目并压缩为目录预览。",
                CACHE_LIST_REASON,
            )
        }
        Err(error) => fail(
            format!("列出目录：{}", resolved.display()),
            format!("目录列举失败：{}", error),
            format!("目录列举失败：{}", error),
            "目录读取失败，按系统错误直接返回。",
            CACHE_LIST_REASON,
        ),
    }
}

fn preview_dir_entries(entries: fs::ReadDir) -> String {
    let mut names = Vec::new();
    for entry in entries.flatten().take(20) {
        names.push(entry.file_name().to_string_lossy().to_string());
    }
    if names.is_empty() {
        "目录为空。".to_string()
    } else {
        names.join(", ")
    }
}

fn ensure_parent_dir(resolved: &Path) -> Result<(), std::io::Error> {
    let Some(parent) = resolved.parent() else {
        return Ok(());
    };
    fs::create_dir_all(parent)
}

fn invalid_path(action: &str, path: &str, summary: &str, cache_reason: &str) -> ActionExecution {
    fail(
        format!("{}：{}", action, path),
        summary.to_string(),
        summary.to_string(),
        summary,
        cache_reason,
    )
}

fn resolve_path(
    request: &RunRequest,
    action: &str,
    path: &str,
    cache_reason: &str,
) -> Result<std::path::PathBuf, ActionExecution> {
    let normalized = normalize_explicit_path(path);
    if let Some(reason) = invalid_explicit_path_reason(&normalized) {
        return Err(invalid_path(action, &normalized, reason, cache_reason));
    }
    resolve_workspace_path(&request.workspace_ref.root_path, &normalized)
        .map_err(|_| invalid_path(action, &normalized, "目标路径越界或解析失败", cache_reason))
}

fn normalize_explicit_path(path: &str) -> String {
    path.trim().replace('\\', "/")
}

fn invalid_explicit_path_reason(path: &str) -> Option<&'static str> {
    if path.is_empty() {
        return Some("目标路径为空，请提供可读取的文件路径。");
    }
    if has_encoding_placeholder(path) {
        return Some(
            "目标路径包含 `?`，疑似发生编码丢失；请改用 ASCII 路径或先 list 目录后复制路径重试。",
        );
    }
    let bad = ['*', '"', '<', '>', '|', '\0'];
    path.chars()
        .any(|ch| bad.contains(&ch))
        .then_some("目标路径包含非法字符；请使用标准文件路径（不要包含 * \" < > |）。")
}

fn has_encoding_placeholder(path: &str) -> bool {
    path.chars()
        .any(|ch| ch == '?' || ch == '？' || ch == '\u{FFFD}' || ch.is_control())
}

fn ok_file_read(resolved: &Path, content: &str) -> ActionExecution {
    let summary = summarize_text(content);
    ok(
        format!("读取文件：{}", resolved.display()),
        format!("文件读取成功，摘要：{}", summary),
        format!(
            "文件读取完成：{}\n内容摘要：{}",
            resolved.display(),
            summary
        ),
        "直接读取目标文件，并将原文压缩成可展示摘要。",
        CACHE_READ_REASON,
    )
}

fn ok_file_write(resolved: &Path, content: &str) -> ActionExecution {
    let count = content.chars().count();
    let summary = summarize_text(content);
    ok(
        format!("写入文件：{}", resolved.display()),
        format!("文件写入成功，共写入 {} 个字符。", count),
        format!(
            "文件写入完成：{}\n写入字符数：{}\n内容摘要：{}",
            resolved.display(),
            count,
            summary
        ),
        "先校验工作区路径，再直接写入目标文件并返回摘要。",
        CACHE_WRITE_REASON,
    )
}

fn ok_file_delete(resolved: &Path) -> ActionExecution {
    ok(
        format!("删除路径：{}", resolved.display()),
        "目标路径已删除。".to_string(),
        format!("删除完成：{}", resolved.display()),
        "按目标类型执行删除，并将删除结果直接回传。",
        CACHE_DELETE_REASON,
    )
}

fn fail_create_parent_dir(resolved: &Path, error: &str, cache_reason: &str) -> ActionExecution {
    fail(
        format!("写入文件：{}", resolved.display()),
        format!("目录创建失败：{}", error),
        format!("写入前创建目录失败：{}", error),
        "写入前置目录创建失败，未进入文件写入阶段。",
        cache_reason,
    )
}

fn fail_write_file(resolved: &Path, error: &str, cache_reason: &str) -> ActionExecution {
    fail(
        format!("写入文件：{}", resolved.display()),
        format!("文件写入失败：{}", error),
        format!("文件写入失败：{}", error),
        "文件写入过程中出现系统错误，直接按失败收口。",
        cache_reason,
    )
}

fn ok(
    action_summary: String,
    result_summary: String,
    final_answer: String,
    reasoning_summary: &str,
    cache_reason: &str,
) -> ActionExecution {
    ActionExecution::bypass_ok(
        action_summary,
        result_summary,
        final_answer,
        reasoning_summary.to_string(),
        cache_reason,
    )
}

fn fail(
    action_summary: String,
    result_summary: String,
    final_answer: String,
    reasoning_summary: &str,
    cache_reason: &str,
) -> ActionExecution {
    ActionExecution::bypass_fail(
        action_summary,
        result_summary,
        final_answer,
        reasoning_summary.to_string(),
        cache_reason,
    )
}

#[cfg(test)]
mod tests {
    use super::{invalid_explicit_path_reason, normalize_explicit_path};

    #[test]
    fn normalizes_explicit_path_slashes_and_spaces() {
        let value = normalize_explicit_path("  docs\\README.md  ");
        assert_eq!(value, "docs/README.md");
    }

    #[test]
    fn rejects_question_mark_path_hint() {
        let reason = invalid_explicit_path_reason("docs/???.md");
        assert!(reason.is_some());
    }

    #[test]
    fn rejects_replacement_char_path_hint() {
        let reason = invalid_explicit_path_reason("docs/\u{FFFD}\u{FFFD}.md");
        assert!(reason.is_some());
    }
}
