use crate::contracts::{GitCommitSummary, GitSnapshot, RepoContextSnapshot, WorkspaceDocSummary};
use crate::text::summarize_text;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const DOC_READ_LIMIT: usize = 4096;
const RECENT_COMMIT_LIMIT: usize = 5;

#[derive(Clone, Debug)]
pub struct RepoContextLoadResult {
    pub snapshot: RepoContextSnapshot,
    pub degraded: bool,
    pub error_count: u32,
}

pub(crate) fn load_repo_context(workspace_root: &Path) -> RepoContextLoadResult {
    let mut warnings = Vec::new();
    let git_available = command_succeeds(None, &["--version"]);
    let repo_root = if git_available {
        match run_git(Some(workspace_root), &["rev-parse", "--show-toplevel"]) {
            Ok(output) if !output.is_empty() => Some(output),
            Ok(_) => {
                warnings.push("Git 可用，但未识别到仓库根路径。".to_string());
                None
            }
            Err(message) => {
                warnings.push(message);
                None
            }
        }
    } else {
        warnings.push("当前环境未检测到 Git，已按非仓库工作区降级。".to_string());
        None
    };

    let git_snapshot = if let Some(root) = repo_root.as_ref() {
        Some(load_git_snapshot(Path::new(root), &mut warnings))
    } else {
        None
    };

    let doc_base = repo_root
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root.to_path_buf());
    let doc_summaries = load_doc_summaries(&doc_base, &mut warnings);
    let degraded = !warnings.is_empty();
    let error_count = warnings.len() as u32;

    RepoContextLoadResult {
        snapshot: RepoContextSnapshot {
            workspace_root: workspace_root.display().to_string(),
            repo_root,
            git_available,
            git_snapshot,
            doc_summaries,
            warnings,
            collected_at: timestamp_now(),
        },
        degraded,
        error_count,
    }
}

pub(crate) fn repo_context_summary(snapshot: &RepoContextSnapshot) -> String {
    let mut parts = vec![format!(
        "当前工作区根路径为 `{}`。",
        snapshot.workspace_root
    )];

    if let Some(repo_root) = snapshot.repo_root.as_deref() {
        parts.push(format!("Git 仓库根路径识别为 `{}`。", repo_root));
    } else if snapshot.git_available {
        parts.push("当前工作区未位于 Git 仓库中。".to_string());
    } else {
        parts.push("当前环境未检测到 Git，已按非仓库目录继续。".to_string());
    }

    if let Some(git_snapshot) = snapshot.git_snapshot.as_ref() {
        let branch = git_snapshot
            .current_branch
            .as_deref()
            .unwrap_or("未识别当前分支");
        let default_branch = git_snapshot
            .default_branch
            .as_deref()
            .unwrap_or("未识别默认分支");
        let dirty = if git_snapshot.is_dirty {
            "存在未提交修改"
        } else {
            "工作树干净"
        };
        parts.push(format!(
            "当前分支 `{}`，默认分支候选 `{}`，{}。",
            branch, default_branch, dirty
        ));
        if !git_snapshot.recent_commits.is_empty() {
            let commit_titles = git_snapshot
                .recent_commits
                .iter()
                .take(3)
                .map(|item| item.short_message.as_str())
                .collect::<Vec<_>>()
                .join("；");
            parts.push(format!(
                "最近提交摘要包括：{}。",
                summarize_text(&commit_titles)
            ));
        }
    }

    if snapshot.doc_summaries.is_empty() {
        parts.push("未命中高价值说明文件。".to_string());
    } else {
        let doc_labels = snapshot
            .doc_summaries
            .iter()
            .map(|item| item.path.as_str())
            .collect::<Vec<_>>()
            .join("、");
        parts.push(format!(
            "命中了 {} 个高价值说明文件：{}。",
            snapshot.doc_summaries.len(),
            summarize_text(&doc_labels)
        ));
    }

    if !snapshot.warnings.is_empty() {
        parts.push(format!(
            "存在 {} 条降级说明：{}。",
            snapshot.warnings.len(),
            summarize_text(&snapshot.warnings.join("；"))
        ));
    }

    summarize_text(&parts.join(" "))
}

pub(crate) fn repo_context_metadata(
    load_result: &RepoContextLoadResult,
) -> BTreeMap<String, String> {
    let snapshot = &load_result.snapshot;
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "workspace_root".to_string(),
        snapshot.workspace_root.clone(),
    );
    metadata.insert(
        "git_available".to_string(),
        bool_to_string(snapshot.git_available),
    );
    metadata.insert(
        "repo_context_status".to_string(),
        if load_result.degraded {
            "degraded".to_string()
        } else {
            "ready".to_string()
        },
    );
    metadata.insert(
        "doc_hits".to_string(),
        snapshot.doc_summaries.len().to_string(),
    );
    metadata.insert(
        "repo_context_warning_count".to_string(),
        load_result.error_count.to_string(),
    );
    metadata.insert(
        "repo_context_collected_at".to_string(),
        snapshot.collected_at.clone(),
    );
    metadata.insert(
        "repo_context_summary".to_string(),
        repo_context_summary(snapshot),
    );

    if let Some(repo_root) = snapshot.repo_root.as_ref() {
        metadata.insert("repo_root".to_string(), repo_root.clone());
    }
    if !snapshot.doc_summaries.is_empty() {
        metadata.insert(
            "doc_paths".to_string(),
            snapshot
                .doc_summaries
                .iter()
                .map(|item| item.path.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
    if !snapshot.warnings.is_empty() {
        metadata.insert(
            "repo_context_warnings".to_string(),
            snapshot.warnings.join("\n"),
        );
    }
    if let Some(git_snapshot) = snapshot.git_snapshot.as_ref() {
        metadata.insert(
            "git_dirty".to_string(),
            bool_to_string(git_snapshot.is_dirty),
        );
        if let Some(current_branch) = git_snapshot.current_branch.as_ref() {
            metadata.insert("current_branch".to_string(), current_branch.clone());
        }
        if let Some(default_branch) = git_snapshot.default_branch.as_ref() {
            metadata.insert("default_branch".to_string(), default_branch.clone());
        }
    }

    metadata
}

fn load_git_snapshot(repo_root: &Path, warnings: &mut Vec<String>) -> GitSnapshot {
    let current_branch = match run_git(Some(repo_root), &["branch", "--show-current"]) {
        Ok(output) if !output.is_empty() => Some(output),
        Ok(_) => None,
        Err(message) => {
            warnings.push(message);
            None
        }
    };

    let default_branch = load_default_branch(repo_root, warnings);
    let is_dirty = match run_git(
        Some(repo_root),
        &["status", "--porcelain", "--untracked-files=all"],
    ) {
        Ok(output) => !output.is_empty(),
        Err(message) => {
            warnings.push(message);
            false
        }
    };

    let recent_commits = match run_git(
        Some(repo_root),
        &["log", "-n", "5", "--pretty=format:%H%x1f%s%x1f%an%x1f%cI"],
    ) {
        Ok(output) => parse_recent_commits(&output),
        Err(message) => {
            warnings.push(message);
            Vec::new()
        }
    };

    GitSnapshot {
        current_branch,
        default_branch,
        is_dirty,
        recent_commits,
    }
}

fn load_default_branch(repo_root: &Path, _warnings: &mut Vec<String>) -> Option<String> {
    match run_git(
        Some(repo_root),
        &["symbolic-ref", "--short", "refs/remotes/origin/HEAD"],
    ) {
        Ok(output) if !output.is_empty() => {
            return output.rsplit('/').next().map(|item| item.to_string());
        }
        Ok(_) => {}
        Err(_) => {}
    }

    for branch in ["main", "master"] {
        if command_succeeds(
            Some(repo_root),
            &["show-ref", "--verify", &format!("refs/heads/{branch}")],
        ) {
            return Some(branch.to_string());
        }
    }

    None
}

fn parse_recent_commits(raw: &str) -> Vec<GitCommitSummary> {
    raw.lines()
        .filter_map(|line| {
            let mut fields = line.split('\u{1f}');
            let commit_id = fields.next()?.trim().to_string();
            let short_message = fields.next()?.trim().to_string();
            let author = fields
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string);
            let timestamp = fields
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string);
            if commit_id.is_empty() || short_message.is_empty() {
                return None;
            }
            Some(GitCommitSummary {
                commit_id,
                short_message,
                author,
                timestamp,
            })
        })
        .take(RECENT_COMMIT_LIMIT)
        .collect()
}

fn load_doc_summaries(base_dir: &Path, warnings: &mut Vec<String>) -> Vec<WorkspaceDocSummary> {
    candidate_docs()
        .into_iter()
        .filter_map(|(relative_path, kind)| {
            let absolute_path = base_dir.join(relative_path);
            if !absolute_path.is_file() {
                return None;
            }

            match summarize_doc_file(&absolute_path) {
                Ok((summary, truncated)) => Some(WorkspaceDocSummary {
                    path: relative_path.to_string(),
                    kind: kind.to_string(),
                    exists: true,
                    summary,
                    truncated,
                }),
                Err(error) => {
                    warnings.push(format!("说明文件 `{}` 摘要失败：{}", relative_path, error));
                    None
                }
            }
        })
        .collect()
}

fn summarize_doc_file(path: &Path) -> Result<(String, bool), String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    let truncated = bytes.len() > DOC_READ_LIMIT;
    let readable = String::from_utf8_lossy(&bytes[..bytes.len().min(DOC_READ_LIMIT)]);
    Ok((summarize_text(&readable), truncated))
}

fn candidate_docs() -> Vec<(&'static str, &'static str)> {
    vec![
        ("CLAUDE.md", "agent_instruction"),
        ("AGENTS.md", "agent_instruction"),
        ("README.md", "readme"),
        ("README.zh-CN.md", "readme"),
        ("docs/README.md", "readme"),
        (".github/copilot-instructions.md", "project_instruction"),
        ("docs/ARCHITECTURE.md", "project_instruction"),
    ]
}

fn command_succeeds(current_dir: Option<&Path>, args: &[&str]) -> bool {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(dir) = current_dir {
        command.current_dir(dir);
    }

    command
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn run_git(current_dir: Option<&Path>, args: &[&str]) -> Result<String, String> {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(dir) = current_dir {
        command.current_dir(dir);
    }

    let output = command
        .output()
        .map_err(|error| format!("Git 命令 `{}` 启动失败：{}", args.join(" "), error))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let detail = if stderr.is_empty() {
            "未知错误".to_string()
        } else {
            stderr
        };
        Err(format!(
            "Git 命令 `{}` 执行失败：{}",
            args.join(" "),
            detail
        ))
    }
}

fn bool_to_string(value: bool) -> String {
    if value {
        "true".to_string()
    } else {
        "false".to_string()
    }
}

fn timestamp_now() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
