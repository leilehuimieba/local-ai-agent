use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::paths::resolve_workspace_path;
use std::fs;
use std::path::{Path, PathBuf};

const CACHE_REASON: &str = "patch 应用属于实时文件副作用动作，不使用回答缓存。";

pub(crate) fn execute_apply_patch(request: &RunRequest, diff: &str, dry_run: bool) -> ActionExecution {
    match apply_patch_flow(request, diff, dry_run) {
        Ok(report) => ok_patch(report, dry_run),
        Err(error) => fail_patch(error),
    }
}

pub(crate) fn preview_apply_patch_report(request: &RunRequest, diff: &str) -> String {
    match apply_patch_flow(request, diff, true) {
        Ok(report) => report.raw_output(),
        Err(error) => error.raw_output(),
    }
}

fn apply_patch_flow(request: &RunRequest, diff: &str, dry_run: bool) -> Result<PatchReport, PatchFailure> {
    let patches = parse_unified_diff(diff)?;
    let mut changes = Vec::new();
    for patch in patches {
        changes.push(build_change(request, patch)?);
    }
    ensure_unique_targets(&changes)?;
    if !dry_run {
        write_all_changes(&changes)?;
    }
    Ok(PatchReport { dry_run, changes })
}

fn parse_unified_diff(diff: &str) -> Result<Vec<FilePatch>, PatchFailure> {
    let lines = diff.lines().map(str::to_string).collect::<Vec<_>>();
    let mut patches = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        if !lines[index].starts_with("--- ") {
            index += 1;
            continue;
        }
        let (patch, next) = parse_file_patch(&lines, index)?;
        patches.push(patch);
        index = next;
    }
    patches.extend(parse_pure_renames(&lines)?);
    (!patches.is_empty())
        .then_some(patches)
        .ok_or_else(|| PatchFailure::parse("diff 缺少文件块。"))
}

fn parse_file_patch(lines: &[String], start: usize) -> Result<(FilePatch, usize), PatchFailure> {
    let old_path = header_path(&lines[start], "--- ")?;
    let next = start + 1;
    if next >= lines.len() || !lines[next].starts_with("+++ ") {
        return Err(PatchFailure::parse("diff 文件块缺少 +++ 文件路径。"));
    }
    let new_path = header_path(&lines[next], "+++ ")?;
    let end = next_file_start(lines, next + 1);
    let hunks = parse_hunks(&lines[next + 1..end])?;
    Ok((
        FilePatch {
            old_path: normalize_diff_path(&old_path),
            new_path: normalize_diff_path(&new_path),
            hunks,
        },
        end,
    ))
}

fn header_path(line: &str, marker: &str) -> Result<String, PatchFailure> {
    let value = line.trim_start_matches(marker).trim();
    value
        .split_whitespace()
        .next()
        .map(str::to_string)
        .ok_or_else(|| PatchFailure::parse("diff 文件路径为空。"))
}

fn next_file_start(lines: &[String], start: usize) -> usize {
    lines[start..]
        .iter()
        .position(|line| line.starts_with("--- ") || line.starts_with("diff --git "))
        .map(|offset| start + offset)
        .unwrap_or(lines.len())
}

fn parse_pure_renames(lines: &[String]) -> Result<Vec<FilePatch>, PatchFailure> {
    let mut patches = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        if !lines[index].starts_with("rename from ") {
            index += 1;
            continue;
        }
        let end = next_git_section(lines, index + 1);
        if !section_has_file_header(&lines[index..end]) {
            patches.push(pure_rename_patch(&lines[index..end])?);
        }
        index = end;
    }
    Ok(patches)
}

fn next_git_section(lines: &[String], start: usize) -> usize {
    lines[start..]
        .iter()
        .position(|line| line.starts_with("diff --git "))
        .map(|offset| start + offset)
        .unwrap_or(lines.len())
}

fn section_has_file_header(lines: &[String]) -> bool {
    lines.iter().any(|line| line.starts_with("--- "))
}

fn pure_rename_patch(lines: &[String]) -> Result<FilePatch, PatchFailure> {
    let old_path = rename_path(lines, "rename from ")?;
    let new_path = rename_path(lines, "rename to ")?;
    Ok(FilePatch {
        old_path: Some(old_path),
        new_path: Some(new_path),
        hunks: Vec::new(),
    })
}

fn rename_path(lines: &[String], marker: &str) -> Result<String, PatchFailure> {
    lines
        .iter()
        .find_map(|line| line.strip_prefix(marker))
        .map(|path| path.trim().to_string())
        .filter(|path| !path.is_empty())
        .ok_or_else(|| PatchFailure::parse("rename patch 缺少路径。"))
}

fn normalize_diff_path(path: &str) -> Option<String> {
    if path == "/dev/null" {
        return None;
    }
    Some(strip_diff_prefix(path).to_string())
}

fn strip_diff_prefix(path: &str) -> &str {
    path.strip_prefix("b/")
        .or_else(|| path.strip_prefix("a/"))
        .unwrap_or(path)
}

fn parse_hunks(lines: &[String]) -> Result<Vec<Hunk>, PatchFailure> {
    let mut hunks = Vec::new();
    let mut current = Vec::new();
    let mut collecting = false;
    for line in lines {
        if line.starts_with("@@") {
            push_hunk(&mut hunks, &mut current)?;
            collecting = true;
        } else if collecting && is_patch_line(line) {
            current.push(line.clone());
        }
    }
    push_hunk(&mut hunks, &mut current)?;
    (!hunks.is_empty())
        .then_some(hunks)
        .ok_or_else(|| PatchFailure::parse("diff 缺少 hunk。"))
}

fn push_hunk(hunks: &mut Vec<Hunk>, current: &mut Vec<String>) -> Result<(), PatchFailure> {
    if current.is_empty() {
        return Ok(());
    }
    hunks.push(build_hunk(current)?);
    current.clear();
    Ok(())
}

fn is_patch_line(line: &str) -> bool {
    line.starts_with(' ') || line.starts_with('-') || line.starts_with('+') || line.starts_with('\\')
}

fn build_hunk(lines: &[String]) -> Result<Hunk, PatchFailure> {
    let mut old_lines = Vec::new();
    let mut new_lines = Vec::new();
    for line in lines {
        apply_hunk_line(line, &mut old_lines, &mut new_lines)?;
    }
    Ok(Hunk { old_lines, new_lines })
}

fn apply_hunk_line(line: &str, old_lines: &mut Vec<String>, new_lines: &mut Vec<String>) -> Result<(), PatchFailure> {
    let rest = line.get(1..).unwrap_or("").to_string();
    match line.chars().next().unwrap_or(' ') {
        ' ' => push_both(old_lines, new_lines, rest),
        '-' => old_lines.push(rest),
        '+' => new_lines.push(rest),
        '\\' => {}
        _ => return Err(PatchFailure::parse("diff hunk 包含未知行。")),
    }
    Ok(())
}

fn push_both(old_lines: &mut Vec<String>, new_lines: &mut Vec<String>, value: String) {
    old_lines.push(value.clone());
    new_lines.push(value);
}

fn apply_hunks(path: &Path, original: &str, hunks: &[Hunk]) -> Result<String, PatchFailure> {
    if hunks.is_empty() {
        return Ok(original.to_string());
    }
    let source = split_lines(original);
    let mut output = Vec::new();
    let mut cursor = 0;
    for hunk in hunks {
        let index = find_hunk(path, &source, &hunk.old_lines, cursor)?;
        output.extend_from_slice(&source[cursor..index]);
        output.extend(hunk.new_lines.clone());
        cursor = index + hunk.old_lines.len();
    }
    output.extend_from_slice(&source[cursor..]);
    Ok(output.join("\n"))
}

fn split_lines(value: &str) -> Vec<String> {
    value.split('\n').map(str::to_string).collect()
}

fn find_hunk(path: &Path, source: &[String], needle: &[String], start: usize) -> Result<usize, PatchFailure> {
    (start..=source.len().saturating_sub(needle.len()))
        .find(|index| source[*index..].starts_with(needle))
        .ok_or_else(|| PatchFailure::conflict(path, "diff 上下文与当前文件不匹配，已拒绝应用。"))
}

fn build_change(request: &RunRequest, patch: FilePatch) -> Result<PatchChange, PatchFailure> {
    let kind = patch.kind()?;
    let old_path = resolve_optional_path(request, patch.old_path.as_deref())?;
    let new_path = resolve_optional_path(request, patch.new_path.as_deref())?;
    let original = read_original(old_path.as_deref(), &kind)?;
    let target = new_path.as_ref().or(old_path.as_ref()).expect("target");
    let updated = apply_hunks(target, &original, &patch.hunks)?;
    validate_change_target(&kind, old_path.as_deref(), new_path.as_deref(), &updated)?;
    Ok(PatchChange::new(old_path, new_path, kind, original, updated))
}

fn read_original(old_path: Option<&Path>, kind: &PatchKind) -> Result<String, PatchFailure> {
    if matches!(kind, PatchKind::Create) {
        return Ok(String::new());
    }
    let path = old_path.ok_or_else(|| PatchFailure::parse("diff 缺少原始文件路径。"))?;
    fs::read_to_string(path).map_err(|error| PatchFailure::io(path, error))
}

fn resolve_optional_path(request: &RunRequest, path: Option<&str>) -> Result<Option<PathBuf>, PatchFailure> {
    path.map(|value| resolve_workspace_path(&request.workspace_ref.root_path, value))
        .transpose()
        .map_err(PatchFailure::boundary)
}

fn validate_change_target(
    kind: &PatchKind,
    old_path: Option<&Path>,
    new_path: Option<&Path>,
    updated: &str,
) -> Result<(), PatchFailure> {
    match kind {
        PatchKind::Create => ensure_new_target(new_path),
        PatchKind::Delete => ensure_deleted_content(old_path, updated),
        PatchKind::Rename => ensure_rename_target(old_path, new_path),
        PatchKind::Modify => Ok(()),
    }
}

fn ensure_new_target(path: Option<&Path>) -> Result<(), PatchFailure> {
    let path = path.ok_or_else(|| PatchFailure::parse("新增 patch 缺少目标路径。"))?;
    if path.exists() {
        return Err(PatchFailure::conflict(path, "新增文件已存在。"));
    }
    Ok(())
}

fn ensure_deleted_content(path: Option<&Path>, updated: &str) -> Result<(), PatchFailure> {
    if updated.is_empty() {
        return Ok(());
    }
    let path = path.unwrap_or_else(|| Path::new(""));
    Err(PatchFailure::conflict(path, "删除 patch 未移除全部内容。"))
}

fn ensure_rename_target(old_path: Option<&Path>, new_path: Option<&Path>) -> Result<(), PatchFailure> {
    let new_path = new_path.ok_or_else(|| PatchFailure::parse("rename patch 缺少新路径。"))?;
    let old_path = old_path.ok_or_else(|| PatchFailure::parse("rename patch 缺少旧路径。"))?;
    if new_path.exists() && new_path != old_path {
        return Err(PatchFailure::conflict(new_path, "rename 目标文件已存在。"));
    }
    Ok(())
}

fn ensure_unique_targets(changes: &[PatchChange]) -> Result<(), PatchFailure> {
    let mut seen = Vec::new();
    for change in changes {
        for path in change.touched_paths() {
            if seen.contains(&path) {
                return Err(PatchFailure::conflict_text(&path, "同一 patch 重复触达文件。"));
            }
            seen.push(path);
        }
    }
    Ok(())
}

fn write_all_changes(changes: &[PatchChange]) -> Result<(), PatchFailure> {
    let mut applied: Vec<&PatchChange> = Vec::new();
    for change in changes {
        if let Err(error) = write_change(change) {
            rollback_changes(&applied);
            return Err(error.with_rollback(true));
        }
        applied.push(change);
    }
    Ok(())
}

fn write_change(change: &PatchChange) -> Result<(), PatchFailure> {
    let result = match change.kind {
        PatchKind::Create | PatchKind::Modify => write_file_change(change),
        PatchKind::Delete => delete_file_change(change),
        PatchKind::Rename => rename_file_change(change),
    };
    if result.is_err() {
        rollback_change(change);
    }
    result
}

fn write_file_change(change: &PatchChange) -> Result<(), PatchFailure> {
    let path = change.target_path();
    ensure_parent(path)?;
    fs::write(path, &change.updated).map_err(|error| PatchFailure::io(path, error))
}

fn delete_file_change(change: &PatchChange) -> Result<(), PatchFailure> {
    let path = change.old_path.as_ref().expect("delete old path");
    fs::remove_file(path).map_err(|error| PatchFailure::io(path, error))
}

fn rename_file_change(change: &PatchChange) -> Result<(), PatchFailure> {
    let old_path = change.old_path.as_ref().expect("rename old path");
    let new_path = change.new_path.as_ref().expect("rename new path");
    ensure_parent(new_path)?;
    fs::write(new_path, &change.updated).map_err(|error| PatchFailure::io(new_path, error))?;
    fs::remove_file(old_path).map_err(|error| PatchFailure::io(old_path, error))
}

fn ensure_parent(path: &Path) -> Result<(), PatchFailure> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| PatchFailure::io(parent, error))?;
    }
    Ok(())
}

fn rollback_changes(changes: &[&PatchChange]) {
    for change in changes.iter().rev() {
        rollback_change(change);
    }
}

fn rollback_change(change: &PatchChange) {
    match change.kind {
        PatchKind::Create => {
            let _ = fs::remove_file(change.target_path());
        }
        PatchKind::Modify => restore_file(change.target_path(), &change.original),
        PatchKind::Delete => restore_file(change.old_path.as_ref().expect("delete path"), &change.original),
        PatchKind::Rename => rollback_rename(change),
    }
}

fn rollback_rename(change: &PatchChange) {
    let new_path = change.new_path.as_ref().expect("rename new path");
    let old_path = change.old_path.as_ref().expect("rename old path");
    let _ = fs::remove_file(new_path);
    restore_file(old_path, &change.original);
}

fn restore_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, content);
}

fn ok_patch(report: PatchReport, dry_run: bool) -> ActionExecution {
    let raw_output = report.raw_output();
    let final_answer = report.final_answer();
    ActionExecution {
        action_summary: format!("应用 patch：{} 个文件", report.changes.len()),
        result_summary: report.summary(),
        detail_preview: final_answer.clone(),
        final_answer,
        raw_output,
        result_chars: report.result_chars(),
        single_result_budget_chars: 1200,
        single_result_budget_hit: report.result_chars() > 1200,
        success: true,
        memory_write_summary: None,
        reasoning_summary: patch_reasoning(dry_run),
        cache_status: "bypass".to_string(),
        cache_reason: CACHE_REASON.to_string(),
    }
}

fn fail_patch(error: PatchFailure) -> ActionExecution {
    let raw_output = error.raw_output();
    ActionExecution {
        action_summary: "应用 patch".to_string(),
        result_summary: format!("patch 应用失败：{}", error.reason),
        final_answer: error.final_answer(),
        detail_preview: error.final_answer(),
        raw_output,
        result_chars: error.reason.chars().count(),
        single_result_budget_chars: 1200,
        single_result_budget_hit: false,
        success: false,
        memory_write_summary: None,
        reasoning_summary: "patch 解析、路径校验、上下文匹配或写入失败，已按文件级错误收口。".to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: CACHE_REASON.to_string(),
    }
}

struct FilePatch {
    old_path: Option<String>,
    new_path: Option<String>,
    hunks: Vec<Hunk>,
}

impl FilePatch {
    fn kind(&self) -> Result<PatchKind, PatchFailure> {
        match (self.old_path.as_ref(), self.new_path.as_ref()) {
            (None, Some(_)) => Ok(PatchKind::Create),
            (Some(_), None) => Ok(PatchKind::Delete),
            (Some(old), Some(new)) if old != new => Ok(PatchKind::Rename),
            (Some(_), Some(_)) => Ok(PatchKind::Modify),
            _ => Err(PatchFailure::parse("diff 文件块缺少有效路径。")),
        }
    }
}

struct Hunk {
    old_lines: Vec<String>,
    new_lines: Vec<String>,
}

struct PatchReport {
    dry_run: bool,
    changes: Vec<PatchChange>,
}

impl PatchReport {
    fn summary(&self) -> String {
        format!("patch {} 成功，影响 {} 个文件。", self.mode(), self.changes.len())
    }

    fn final_answer(&self) -> String {
        format!("patch {} 完成：\n{}", self.mode(), self.change_lines().join("\n"))
    }

    fn raw_output(&self) -> String {
        serde_json::to_string_pretty(&self.report_json()).unwrap_or_else(|_| "{}".to_string())
    }

    fn report_json(&self) -> serde_json::Value {
        serde_json::json!({ "dry_run": self.dry_run, "changes": self.change_json() })
    }

    fn change_json(&self) -> Vec<serde_json::Value> {
        self.changes.iter().map(PatchChange::as_json).collect()
    }

    fn change_lines(&self) -> Vec<String> {
        self.changes.iter().map(PatchChange::line).collect()
    }

    fn result_chars(&self) -> usize {
        self.final_answer().chars().count()
    }

    fn mode(&self) -> &'static str {
        if self.dry_run { "dry-run" } else { "apply" }
    }
}

struct PatchChange {
    old_path: Option<PathBuf>,
    new_path: Option<PathBuf>,
    kind: PatchKind,
    before_chars: usize,
    after_chars: usize,
    original: String,
    updated: String,
}

impl PatchChange {
    fn new(
        old_path: Option<PathBuf>,
        new_path: Option<PathBuf>,
        kind: PatchKind,
        original: String,
        updated: String,
    ) -> Self {
        Self {
            kind,
            before_chars: original.chars().count(),
            after_chars: updated.chars().count(),
            old_path,
            new_path,
            original,
            updated,
        }
    }

    fn line(&self) -> String {
        format!(
            "- {}: {} ({} -> {} chars)",
            self.kind.as_str(),
            self.display_path(),
            self.before_chars,
            self.after_chars
        )
    }

    fn as_json(&self) -> serde_json::Value {
        serde_json::json!({
            "path": self.display_path(),
            "kind": self.kind.as_str(),
            "before_chars": self.before_chars,
            "after_chars": self.after_chars
        })
    }

    fn display_path(&self) -> String {
        self.target_path().display().to_string()
    }

    fn target_path(&self) -> &Path {
        self.new_path
            .as_deref()
            .or(self.old_path.as_deref())
            .expect("target path")
    }

    fn touched_paths(&self) -> Vec<String> {
        let mut paths = [self.old_path.as_ref(), self.new_path.as_ref()]
            .into_iter()
            .flatten()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>();
        paths.dedup();
        paths
    }
}

#[derive(Clone, Copy)]
enum PatchKind {
    Create,
    Modify,
    Delete,
    Rename,
}

impl PatchKind {
    fn as_str(self) -> &'static str {
        match self {
            PatchKind::Create => "create",
            PatchKind::Modify => "modify",
            PatchKind::Delete => "delete",
            PatchKind::Rename => "rename",
        }
    }
}

struct PatchFailure {
    stage: String,
    path: Option<String>,
    reason: String,
    rollback_attempted: bool,
}

impl PatchFailure {
    fn parse(reason: &str) -> Self {
        Self::new("parse", None, reason)
    }

    fn boundary(reason: String) -> Self {
        Self::new("boundary", None, &reason)
    }

    fn conflict(path: &Path, reason: &str) -> Self {
        Self::new("conflict", Some(path.display().to_string()), reason)
    }

    fn conflict_text(path: &str, reason: &str) -> Self {
        Self::new("conflict", Some(path.to_string()), reason)
    }

    fn io(path: &Path, error: std::io::Error) -> Self {
        Self::new("write", Some(path.display().to_string()), &error.to_string())
    }

    fn new(stage: &str, path: Option<String>, reason: &str) -> Self {
        Self {
            stage: stage.to_string(),
            path,
            reason: reason.to_string(),
            rollback_attempted: false,
        }
    }

    fn with_rollback(mut self, attempted: bool) -> Self {
        self.rollback_attempted = attempted;
        self
    }

    fn final_answer(&self) -> String {
        format!("patch 应用失败：{}{}", self.reason, self.path_suffix())
    }

    fn raw_output(&self) -> String {
        serde_json::to_string_pretty(&self.as_json()).unwrap_or_else(|_| "{}".to_string())
    }

    fn as_json(&self) -> serde_json::Value {
        serde_json::json!({
            "success": false,
            "stage": self.stage,
            "path": self.path,
            "reason": self.reason,
            "rollback_attempted": self.rollback_attempted
        })
    }

    fn path_suffix(&self) -> String {
        self.path
            .as_ref()
            .map(|path| format!("（文件：{path}）"))
            .unwrap_or_default()
    }
}

fn patch_reasoning(dry_run: bool) -> String {
    let mode = if dry_run {
        "dry-run 只生成预览，不写入文件"
    } else {
        "apply 写入已校验文件"
    };
    format!("解析 unified diff，校验路径边界和上下文匹配；{mode}。")
}

#[cfg(test)]
mod tests {
    use super::execute_apply_patch;
    use crate::query_engine_testkit::testkit::sample_request;
    use std::fs;

    #[test]
    fn applies_single_file_unified_diff() {
        let (request, file) = sample_patch_file("hello\nold\nend");
        let result = execute_apply_patch(&request, &diff("sample.txt", "old", "new"), false);
        assert!(result.success);
        assert_eq!(fs::read_to_string(file).unwrap(), "hello\nnew\nend");
    }

    #[test]
    fn dry_run_does_not_write_file() {
        let (request, file) = sample_patch_file("hello\nold\nend");
        let result = execute_apply_patch(&request, &diff("sample.txt", "old", "new"), true);
        assert!(result.success);
        assert_eq!(fs::read_to_string(file).unwrap(), "hello\nold\nend");
    }

    #[test]
    fn rejects_context_mismatch() {
        let (request, _file) = sample_patch_file("hello\nother\nend");
        let result = execute_apply_patch(&request, &diff("sample.txt", "old", "new"), false);
        assert!(!result.success);
        assert!(result.final_answer.contains("上下文"));
    }

    #[test]
    fn rejects_workspace_escape() {
        let (request, _file) = sample_patch_file("hello\nold\nend");
        let result = execute_apply_patch(&request, &diff("../escape.txt", "old", "new"), false);
        assert!(!result.success);
        assert!(result.final_answer.contains("工作区"));
    }

    #[test]
    fn applies_multi_file_patch() {
        let (request, file) = sample_patch_file("hello\nold\nend");
        let second = file.with_file_name("second.txt");
        fs::write(&second, "alpha\nold\nomega").unwrap();
        let result = execute_apply_patch(&request, &multi_diff(), false);
        assert!(result.success);
        assert_eq!(fs::read_to_string(file).unwrap(), "hello\nnew\nend");
        assert_eq!(fs::read_to_string(second).unwrap(), "alpha\nnew\nomega");
        assert!(result.raw_output.contains("\"changes\""));
    }

    #[test]
    fn creates_new_file_from_unified_diff() {
        let (request, file) = sample_patch_file("hello\nold\nend");
        let new_file = file.with_file_name("created.txt");
        let result = execute_apply_patch(&request, &create_diff("created.txt"), false);
        assert!(result.success);
        assert_eq!(fs::read_to_string(new_file).unwrap(), "created\n");
    }

    #[test]
    fn deletes_file_from_unified_diff() {
        let (request, file) = sample_patch_file("hello\nold\nend");
        let result = execute_apply_patch(&request, &delete_diff("sample.txt"), false);
        assert!(result.success);
        assert!(!file.exists());
        assert!(result.raw_output.contains("\"kind\": \"delete\""));
    }

    #[test]
    fn renames_file_without_content_change() {
        let (request, file) = sample_patch_file("hello\nold\nend");
        let renamed = file.with_file_name("renamed.txt");
        let result = execute_apply_patch(&request, &rename_diff("sample.txt", "renamed.txt"), false);
        assert!(result.success);
        assert!(!file.exists());
        assert_eq!(fs::read_to_string(renamed).unwrap(), "hello\nold\nend");
    }

    #[test]
    fn rollback_restores_first_file_when_later_write_fails() {
        let (request, file) = sample_patch_file("hello\nold\nend");
        let blocker = file.with_file_name("blocked");
        fs::write(&blocker, "not a dir").unwrap();
        let result = execute_apply_patch(&request, &rollback_diff(), false);
        assert!(!result.success);
        assert_eq!(fs::read_to_string(file).unwrap(), "hello\nold\nend");
        assert!(result.raw_output.contains("rollback_attempted"));
    }

    fn sample_patch_file(content: &str) -> (crate::contracts::RunRequest, std::path::PathBuf) {
        let root = std::env::temp_dir().join(format!("patch-test-{}", rand_id()));
        fs::create_dir_all(&root).unwrap();
        let file = root.join("sample.txt");
        fs::write(&file, content).unwrap();
        let mut request = sample_request("patch");
        request.workspace_ref.root_path = root.display().to_string();
        (request, file)
    }

    fn diff(path: &str, old: &str, new: &str) -> String {
        format!("--- a/{path}\n+++ b/{path}\n@@ -1,3 +1,3 @@\n hello\n-{old}\n+{new}\n end")
    }

    fn multi_diff() -> String {
        format!("{}\n{}", diff("sample.txt", "old", "new"), second_diff())
    }

    fn second_diff() -> String {
        "--- a/second.txt\n+++ b/second.txt\n@@ -1,3 +1,3 @@\n alpha\n-old\n+new\n omega".to_string()
    }

    fn create_diff(path: &str) -> String {
        format!("--- /dev/null\n+++ b/{path}\n@@ -0,0 +1 @@\n+created")
    }

    fn delete_diff(path: &str) -> String {
        format!("--- a/{path}\n+++ /dev/null\n@@ -1,3 +0,0 @@\n-hello\n-old\n-end")
    }

    fn rename_diff(old_path: &str, new_path: &str) -> String {
        format!("diff --git a/{old_path} b/{new_path}\nrename from {old_path}\nrename to {new_path}")
    }

    fn rollback_diff() -> String {
        format!(
            "{}\n{}",
            diff("sample.txt", "old", "new"),
            create_diff("blocked/child.txt")
        )
    }

    fn rand_id() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
