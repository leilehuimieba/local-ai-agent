use crate::context_builder::RuntimeContextEnvelope;
use crate::contracts::RepoContextSnapshot;
use crate::memory_schema::canonical_kind;
use crate::session::SessionMemory;

#[derive(Clone, Debug)]
pub(crate) enum PlannedAction {
    RunCommand {
        command: String,
    },
    ReadFile {
        path: String,
    },
    WriteFile {
        path: String,
        content: String,
    },
    DeletePath {
        path: String,
    },
    ListFiles {
        path: Option<String>,
    },
    WriteMemory {
        kind: String,
        summary: String,
        content: String,
    },
    RecallMemory {
        query: String,
    },
    SearchKnowledge {
        query: String,
    },
    SearchSiyuanNotes {
        query: String,
    },
    ReadSiyuanNote {
        path: String,
    },
    WriteSiyuanKnowledge,
    ProjectAnswer,
    ContextAnswer,
    Explain,
    AgentResolve,
}

pub(crate) fn analysis_summary(
    action: &PlannedAction,
    session_context: &SessionMemory,
    repo_context: &RepoContextSnapshot,
) -> String {
    let mut base = match action {
        PlannedAction::RunCommand { .. } => "运行时识别到这是一个命令执行任务。".to_string(),
        PlannedAction::ReadFile { .. } => "运行时识别到这是一个文件读取任务。".to_string(),
        PlannedAction::WriteFile { .. } => "运行时识别到这是一个文件写入任务。".to_string(),
        PlannedAction::DeletePath { .. } => "运行时识别到这是一个删除类动作。".to_string(),
        PlannedAction::ListFiles { .. } => "运行时识别到这是一个工作区目录浏览任务。".to_string(),
        PlannedAction::WriteMemory { .. } => "运行时识别到这是一个长期记忆写入任务。".to_string(),
        PlannedAction::RecallMemory { .. } => "运行时识别到这是一个按需记忆召回任务。".to_string(),
        PlannedAction::SearchKnowledge { .. } => {
            "运行时识别到这是一个本地知识检索任务。".to_string()
        }
        PlannedAction::SearchSiyuanNotes { .. } => {
            "运行时识别到这是一个思源摘要检索任务。".to_string()
        }
        PlannedAction::ReadSiyuanNote { .. } => {
            "运行时识别到这是一个思源正文读取任务。".to_string()
        }
        PlannedAction::WriteSiyuanKnowledge => "运行时识别到这是一个思源知识导出任务。".to_string(),
        PlannedAction::ProjectAnswer => {
            "当前输入更像项目说明类问题，运行时将基于本地文档生成项目回答。".to_string()
        }
        PlannedAction::ContextAnswer => {
            "当前输入未命中动作前缀，运行时将基于会话压缩摘要继续回答。".to_string()
        }
        PlannedAction::Explain => {
            "当前输入不包含已支持的执行前缀，运行时将返回可用能力说明。".to_string()
        }
        PlannedAction::AgentResolve => {
            "当前输入将被交给执行大模型并提供 Tools 调用，尝试使用 Agent 能力执行。".to_string()
        }
    };

    base.push_str(&format!(
        " 当前工作区为 `{}`。",
        repo_context.workspace_root
    ));
    if let Some(git_snapshot) = repo_context.git_snapshot.as_ref() {
        let branch = git_snapshot
            .current_branch
            .as_deref()
            .unwrap_or("未识别分支");
        let dirty_status = if git_snapshot.is_dirty {
            "存在未提交修改"
        } else {
            "工作树干净"
        };
        base.push_str(&format!(
            " 基于当前仓库状态，运行时识别到分支 `{}`，{}。",
            branch, dirty_status
        ));
    } else if repo_context.git_available {
        base.push_str(" 当前工作区不在 Git 仓库中，分析将仅依赖工作区路径与说明文件。");
    } else {
        base.push_str(" 当前环境未检测到 Git，分析将按降级路径继续。");
    }
    if !repo_context.doc_summaries.is_empty() {
        let doc_paths = repo_context
            .doc_summaries
            .iter()
            .map(|item| item.path.as_str())
            .collect::<Vec<_>>()
            .join("、");
        base.push_str(&format!(" 已命中高价值说明文件：{}。", doc_paths));
    }
    if !session_context.compressed_summary.is_empty() {
        base.push_str(&format!(
            " 本次只加载最近 {} 轮压缩摘要，而不是全量历史。",
            session_context.recent_turns.len()
        ));
    }
    base
}

pub(crate) fn plan_action_with_context(envelope: &RuntimeContextEnvelope) -> PlannedAction {
    let trimmed = envelope.user_input.trim();
    // 依然保留非常特殊的说明能力
    if is_capability_question(trimmed) {
        return PlannedAction::Explain;
    }
    // 其余不再尝试各种正则匹配和猜测，如果在 explicit_action 没有命中严格前缀，就丢给大模型处理
    explicit_action(trimmed).unwrap_or(PlannedAction::AgentResolve)
}

fn explicit_action(input: &str) -> Option<PlannedAction> {
    run_command_action(input)
        .or_else(|| read_file_action(input))
        .or_else(|| delete_path_action(input))
        .or_else(|| list_files_action(input))
        .or_else(|| write_file_action(input))
        .or_else(|| write_memory_action(input))
        .or_else(|| recall_memory_action(input))
        .or_else(|| search_knowledge_action(input))
        .or_else(|| search_siyuan_action(input))
        .or_else(|| read_siyuan_action(input))
        .or_else(|| write_siyuan_action(input))
}

fn run_command_action(input: &str) -> Option<PlannedAction> {
    extract_prefixed_value(
        input,
        &["cmd:", "command:", "run command:", "执行命令:", "运行命令:"],
    )
    .map(|command| PlannedAction::RunCommand { command })
}

fn read_file_action(input: &str) -> Option<PlannedAction> {
    extract_prefixed_value(input, &["read:", "read file:", "读取文件:", "查看文件:"])
        .map(|path| PlannedAction::ReadFile { path })
}

fn delete_path_action(input: &str) -> Option<PlannedAction> {
    extract_prefixed_value(
        input,
        &["delete:", "remove:", "删除:", "删除文件:", "移除:"],
    )
    .map(|path| PlannedAction::DeletePath { path })
}

fn list_files_action(input: &str) -> Option<PlannedAction> {
    extract_prefixed_value(
        input,
        &["list:", "列出文件:", "列出目录:", "workspace list:"],
    )
    .map(|path| PlannedAction::ListFiles {
        path: if path.is_empty() { None } else { Some(path) },
    })
}

fn write_file_action(input: &str) -> Option<PlannedAction> {
    extract_write_request(input, &["write:", "create:", "写入文件:", "创建文件:"])
        .map(|(path, content)| PlannedAction::WriteFile { path, content })
}

fn write_memory_action(input: &str) -> Option<PlannedAction> {
    extract_memory_request(input, &["remember:", "memory write:", "记住:", "写入记忆:"]).map(
        |(kind, summary, content)| PlannedAction::WriteMemory {
            kind,
            summary,
            content,
        },
    )
}

fn recall_memory_action(input: &str) -> Option<PlannedAction> {
    extract_prefixed_value(input, &["recall:", "memory:", "回忆:", "检索记忆:"])
        .map(|query| PlannedAction::RecallMemory { query })
}

fn search_knowledge_action(input: &str) -> Option<PlannedAction> {
    extract_prefixed_value(
        input,
        &["knowledge:", "search knowledge:", "检索知识:", "知识检索:"],
    )
    .map(|query| PlannedAction::SearchKnowledge { query })
}

fn search_siyuan_action(input: &str) -> Option<PlannedAction> {
    extract_prefixed_value(input, &["siyuan:", "思源检索:", "搜索思源:"])
        .map(|query| PlannedAction::SearchSiyuanNotes { query })
}

fn read_siyuan_action(input: &str) -> Option<PlannedAction> {
    extract_prefixed_value(input, &["read siyuan:", "读取思源:", "思源正文:"])
        .map(|path| PlannedAction::ReadSiyuanNote { path })
}

fn write_siyuan_action(input: &str) -> Option<PlannedAction> {
    ["write siyuan", "写入思源", "导出思源"]
        .iter()
        .any(|prefix| input.trim().eq_ignore_ascii_case(prefix))
        .then_some(PlannedAction::WriteSiyuanKnowledge)
}

fn natural_language_action(
    input: &str,
    has_session_context: bool,
    has_project_material: bool,
) -> PlannedAction {
    if is_capability_question(input) {
        return PlannedAction::Explain;
    }
    if has_project_material && is_project_status_question(input) {
        return PlannedAction::ProjectAnswer;
    }
    if should_continue_session(input, has_session_context) {
        return PlannedAction::ContextAnswer;
    }
    if should_answer_project(input, has_project_material) {
        return PlannedAction::ProjectAnswer;
    }
    if has_session_context {
        PlannedAction::ContextAnswer
    } else if is_project_status_question(input) && has_project_material {
        PlannedAction::ProjectAnswer
    } else {
        PlannedAction::Explain
    }
}

fn is_project_question(input: &str) -> bool {
    let lower = input.trim().to_lowercase();
    let mentions_project = mentions_any(&lower, &["项目", "工程", "仓库", "代码库", "系统"]);
    let asks_summary = mentions_any(
        &lower,
        &[
            "做什么",
            "干什么",
            "是什么",
            "介绍",
            "说明",
            "总结",
            "概述",
            "简介",
            "当前",
            "现在",
        ],
    );
    mentions_project && asks_summary
}

fn should_continue_session(input: &str, has_session_context: bool) -> bool {
    has_session_context
        && mentions_any(
            &input.trim().to_lowercase(),
            &[
                "继续",
                "刚才",
                "上面",
                "前面",
                "那这个",
                "然后",
                "下一步",
                "接着",
                "延续",
                "上次做到哪",
                "还差什么",
                "继续推进",
            ],
        )
}

fn should_answer_project(input: &str, has_project_material: bool) -> bool {
    has_project_material && (is_project_question(input) || is_project_status_question(input))
}

fn is_capability_question(input: &str) -> bool {
    mentions_any(
        &input.trim().to_lowercase(),
        &[
            "你能做什么",
            "支持什么",
            "有哪些能力",
            "能力边界",
            "怎么用",
            "如何使用",
            "可用能力",
        ],
    )
}

fn is_project_status_question(input: &str) -> bool {
    mentions_any(
        &input.trim().to_lowercase(),
        &[
            "进度",
            "做到什么程度",
            "完成了吗",
            "状态",
            "实现了什么",
            "当前情况",
            "当前阶段",
            "现在做到什么程度",
            "当前做到什么程度",
            "上次做到哪",
            "还差什么",
            "现在最该做什么",
            "继续下一步",
            "下一步做什么",
            "继续推进",
        ],
    )
}

fn has_project_context(envelope: &RuntimeContextEnvelope) -> bool {
    let repo_summary = envelope.project_block.repo_summary.trim();
    let doc_summary = envelope.project_block.doc_summary.trim();
    !repo_summary.is_empty()
        || (!doc_summary.is_empty() && !doc_summary.starts_with("当前没有命中高价值说明文件"))
}

fn mentions_any(input: &str, tokens: &[&str]) -> bool {
    tokens.iter().any(|token| input.contains(token))
}

fn extract_prefixed_value(input: &str, prefixes: &[&str]) -> Option<String> {
    let lower = input.to_lowercase();
    prefixes.iter().find_map(|prefix| {
        if lower.starts_with(&prefix.to_lowercase()) {
            Some(input[prefix.len()..].trim().to_string())
        } else {
            None
        }
    })
}

fn extract_write_request(input: &str, prefixes: &[&str]) -> Option<(String, String)> {
    prefixes.iter().find_map(|prefix| {
        if input.to_lowercase().starts_with(&prefix.to_lowercase()) {
            let remainder = input[prefix.len()..].trim_start();
            let mut lines = remainder.lines();
            let path = lines.next()?.trim().to_string();
            let content = lines.collect::<Vec<_>>().join("\n");
            Some((path, content))
        } else {
            None
        }
    })
}

fn extract_memory_request(input: &str, prefixes: &[&str]) -> Option<(String, String, String)> {
    prefixes.iter().find_map(|prefix| {
        if input.to_lowercase().starts_with(&prefix.to_lowercase()) {
            let remainder = input[prefix.len()..].trim_start();
            let mut lines = remainder.lines();
            let header = lines.next()?.trim();
            let content = lines.collect::<Vec<_>>().join("\n").trim().to_string();
            let (kind, summary) = if let Some((kind, summary)) = header.split_once('|') {
                (normalize_memory_kind(kind), summary.trim().to_string())
            } else {
                ("project_knowledge".to_string(), header.to_string())
            };
            let final_content = if content.is_empty() {
                summary.clone()
            } else {
                content
            };
            Some((kind, summary, final_content))
        } else {
            None
        }
    })
}

pub(crate) fn normalize_mode(mode: &str) -> String {
    match mode.trim().to_lowercase().as_str() {
        "observe" | "observation" => "observe".to_string(),
        "full" | "full_access" | "full-access" => "full_access".to_string(),
        _ => "standard".to_string(),
    }
}

fn normalize_memory_kind(value: &str) -> String {
    canonical_kind(value)
}
