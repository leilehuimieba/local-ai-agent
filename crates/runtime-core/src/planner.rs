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
    if let Some(action) = explicit_action(trimmed) {
        return action;
    }
    natural_language_action(
        trimmed,
        has_session_context(envelope),
        has_project_context(envelope),
    )
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
    if let Some(action) = fuzzy_action(input) {
        return action;
    }
    if is_capability_question(input) {
        return PlannedAction::Explain;
    }
    if is_learning_continuation_question(input) {
        return PlannedAction::ContextAnswer;
    }
    if has_project_material && is_project_status_question(input) {
        return PlannedAction::ProjectAnswer;
    }
    if should_default_to_context_answer(input) {
        return PlannedAction::ContextAnswer;
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
        PlannedAction::AgentResolve
    }
}

fn should_default_to_context_answer(input: &str) -> bool {
    let lower = input.trim().to_lowercase();
    let asks_question = mentions_any(
        &lower,
        &[
            "？",
            "?",
            "怎么",
            "如何",
            "什么",
            "哪",
            "吗",
            "请给我",
            "请用",
            "清单",
            "方案",
            "计划",
            "步骤",
            "三步",
            "模板",
            "复盘",
            "顺序",
            "建议",
            "结论",
            "依据",
            "风险",
            "判断",
            "第一步",
        ],
    );
    let asks_execution = mentions_any(&lower, &["打开", "启动", "运行", "删除"]);
    asks_question && !asks_execution
}

fn fuzzy_action(input: &str) -> Option<PlannedAction> {
    let lower = input.trim().to_lowercase();
    if should_use_context_for_evidence_status(&lower) {
        return Some(PlannedAction::ContextAnswer);
    }
    if should_use_context_for_pause_risk(&lower) {
        return Some(PlannedAction::ContextAnswer);
    }
    if should_use_context_for_next_step_four_section(&lower) {
        return Some(PlannedAction::ContextAnswer);
    }
    if should_use_context_for_fast_checklist(&lower) {
        return Some(PlannedAction::ContextAnswer);
    }
    if should_use_context_for_kickoff_message(&lower) {
        return Some(PlannedAction::ContextAnswer);
    }
    if should_use_context_for_acceptance_readiness(&lower) {
        return Some(PlannedAction::ContextAnswer);
    }
    if should_use_context_for_priority_three_tasks(&lower) {
        return Some(PlannedAction::ContextAnswer);
    }
    if should_use_context_for_smalltalk(&lower) {
        return Some(PlannedAction::ContextAnswer);
    }
    if should_open_calculator(&lower) {
        return Some(PlannedAction::RunCommand {
            command: calculator_command(),
        });
    }
    None
}

fn should_use_context_for_fast_checklist(input: &str) -> bool {
    let has_time_limit = mentions_any(
        input,
        &[
            "30 minutes",
            "20 minutes",
            "15 minutes",
            "30分钟",
            "20分钟",
            "15分钟",
        ],
    );
    let has_checklist_intent = mentions_any(
        input,
        &[
            "checklist",
            "practical checklist",
            "execute now",
            "quick checklist",
            "执行清单",
            "清单",
        ],
    );
    has_time_limit && has_checklist_intent
}

fn should_use_context_for_kickoff_message(input: &str) -> bool {
    let has_kickoff_intent = mentions_any(
        input,
        &[
            "kickoff message",
            "restart quickly",
            "tomorrow morning",
            "明早",
            "启动语",
            "开场提醒",
        ],
    );
    let asks_short_output = mentions_any(
        input,
        &["one short", "short message", "一句", "简短", "不超过两句"],
    );
    has_kickoff_intent && asks_short_output
}

fn should_use_context_for_acceptance_readiness(input: &str) -> bool {
    let asks_acceptance = mentions_any(
        input,
        &["验收", "提测", "ready for acceptance", "ready to validate"],
    );
    if !asks_acceptance {
        return false;
    }
    mentions_any(
        input,
        &["可以开始", "能开始", "现在是否", "whether", "can we start"],
    )
}

fn should_use_context_for_priority_three_tasks(input: &str) -> bool {
    let asks_priority = mentions_any(input, &["优先级", "priority"]);
    if !asks_priority {
        return false;
    }
    let asks_three = mentions_any(input, &["三件事", "3件事", "three things", "three tasks"]);
    let asks_next = mentions_any(input, &["下一步", "next step", "继续推进"]);
    asks_three && asks_next
}

fn should_use_context_for_smalltalk(input: &str) -> bool {
    mentions_any(
        input,
        &[
            "聊两句",
            "你今天状态",
            "最近怎么样",
            "随便聊聊",
            "casual chat",
        ],
    )
}

fn should_use_context_for_evidence_status(input: &str) -> bool {
    let asks_progress = mentions_any(
        input,
        &[
            "做到哪",
            "还差什么",
            "where are we now",
            "what's missing",
            "current status",
        ],
    );
    if !asks_progress {
        return false;
    }
    mentions_any(
        input,
        &["证据目录", "evidence", "based on current evidence"],
    )
}

fn should_use_context_for_pause_risk(input: &str) -> bool {
    let asks_pause_risk =
        mentions_any(input, &["暂停", "pause now"]) && mentions_any(input, &["风险", "risk"]);
    if !asks_pause_risk {
        return false;
    }
    mentions_any(input, &["一个动作", "one action", "how can i reduce"])
}

fn should_use_context_for_next_step_four_section(input: &str) -> bool {
    let asks_next = mentions_any(input, &["下一步建议", "现在最该做什么", "next step"]);
    if !asks_next {
        return false;
    }
    mentions_any(
        input,
        &["当前判断", "缺口", "一步动作", "为什么是这一步", "四段式"],
    )
}

fn should_open_calculator(input: &str) -> bool {
    let mentions_calc = mentions_any(
        input,
        &[
            "计算器",
            "calc",
            "calculator",
            "打开计算器",
            "启动计算器",
            "打开一下计算器",
        ],
    );
    let mentions_open = mentions_any(
        input,
        &["打开", "启动", "运行", "帮我打开", "帮我启动", "帮我运行"],
    );
    mentions_calc && (mentions_open || input.trim() == "计算器" || input.trim() == "calc")
}

fn calculator_command() -> String {
    if cfg!(target_os = "windows") {
        "start calc".to_string()
    } else if cfg!(target_os = "macos") {
        "open -a Calculator".to_string()
    } else {
        "gnome-calculator".to_string()
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
    let _ = has_session_context;
    mentions_any(
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
    has_project_material
        && !is_learning_continuation_question(input)
        && (is_project_question(input) || is_project_status_question(input))
}

fn is_capability_question(input: &str) -> bool {
    let lower = input.trim().to_lowercase();
    let capability_words = mentions_any(
        &lower,
        &[
            "你能做什么",
            "支持什么",
            "有哪些能力",
            "能力边界",
            "如何使用",
            "可用能力",
        ],
    );
    if capability_words {
        return true;
    }
    lower.contains("怎么用")
        && mentions_any(
            &lower,
            &["你", "这个助手", "本地智能体", "这个系统", "这些能力"],
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

fn is_learning_continuation_question(input: &str) -> bool {
    let lower = input.trim().to_lowercase();
    let learning_words = mentions_any(
        &lower,
        &[
            "学习",
            "复习",
            "掌握",
            "巩固",
            "知识点",
            "学习建议",
            "待巩固",
        ],
    );
    let continue_words = mentions_any(
        &lower,
        &[
            "上次做到哪",
            "还差什么",
            "下一步做什么",
            "下一步",
            "建议先做",
            "掌握到哪",
        ],
    );
    learning_words && continue_words
}

fn has_project_context(envelope: &RuntimeContextEnvelope) -> bool {
    let repo_summary = envelope.project_block.repo_summary.trim();
    let doc_summary = envelope.project_block.doc_summary.trim();
    !repo_summary.is_empty()
        || (!doc_summary.is_empty() && !doc_summary.starts_with("当前没有命中高价值说明文件"))
}

fn has_session_context(envelope: &RuntimeContextEnvelope) -> bool {
    envelope.dynamic_block.session_summary.trim() != "当前会话还没有可复用的压缩摘要。"
}

#[cfg(test)]
mod tests {
    use super::{PlannedAction, plan_action_with_context};
    use crate::context_builder::{
        DynamicPromptBlock, ProjectPromptBlock, RuntimeContextEnvelope, StaticPromptBlock,
    };

    fn static_block() -> StaticPromptBlock {
        StaticPromptBlock {
            role_prompt: String::new(),
            mode_prompt: String::new(),
        }
    }

    fn project_block(repo_summary: &str, doc_summary: &str) -> ProjectPromptBlock {
        ProjectPromptBlock {
            workspace_root: "D:/repo".to_string(),
            repo_summary: repo_summary.to_string(),
            doc_summary: doc_summary.to_string(),
        }
    }

    fn dynamic_block(user_input: &str, session_summary: &str) -> DynamicPromptBlock {
        DynamicPromptBlock {
            user_input: user_input.to_string(),
            assembly_profile: "test".to_string(),
            includes_session: true,
            includes_memory: false,
            includes_knowledge: false,
            includes_tool_preview: false,
            phase_label: "test".to_string(),
            selection_reason: "test".to_string(),
            prefers_artifact_context: false,
            session_summary: session_summary.to_string(),
            memory_digest: String::new(),
            knowledge_digest: String::new(),
            tool_preview: String::new(),
            artifact_hint: String::new(),
            reasoning_summary: String::new(),
            cache_status: "cold".to_string(),
            cache_reason: String::new(),
        }
    }

    fn envelope(
        user_input: &str,
        session_summary: &str,
        repo_summary: &str,
        doc_summary: &str,
    ) -> RuntimeContextEnvelope {
        RuntimeContextEnvelope {
            user_input: user_input.to_string(),
            mode: "standard".to_string(),
            workspace_root: "D:/repo".to_string(),
            static_block: static_block(),
            project_block: project_block(repo_summary, doc_summary),
            dynamic_block: dynamic_block(user_input, session_summary),
        }
    }

    #[test]
    fn plans_explicit_action_over_natural_language() {
        let env = envelope(
            "read: README.md",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "当前没有命中高价值说明文件。",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ReadFile { .. }
        ));
    }

    #[test]
    fn plans_context_answer_for_continue_words() {
        let env = envelope(
            "可以继续",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "当前没有命中高价值说明文件。",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_project_answer_for_status_questions_with_material() {
        let env = envelope(
            "项目进度到哪了？",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "docs/README.md: 项目说明",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ProjectAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_learning_status_questions() {
        let env = envelope(
            "继续复习 Rust 所有权和借用。我现在掌握到哪了，还差什么，下一步做什么？",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "docs/README.md: 项目说明",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_smalltalk_without_context() {
        let env = envelope(
            "你好，随便聊聊",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "当前没有命中高价值说明文件。",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_question_without_context() {
        let env = envelope(
            "我只有30分钟，想做一轮上线前回归，你给我一个按分钟拆分的执行清单。",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "当前没有命中高价值说明文件。",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_plan_question_without_context() {
        let env = envelope(
            "我今晚只剩20分钟，给我一个只做收口的三步计划。",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "当前没有命中高价值说明文件。",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_template_question_without_context() {
        let env = envelope(
            "给我一个今晚就能执行的复盘模板，不超过五行。",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "当前没有命中高价值说明文件。",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_open_calculator_from_natural_language() {
        let env = envelope(
            "帮我打开计算器",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "当前没有命中高价值说明文件。",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::RunCommand { .. }
        ));
    }

    #[test]
    fn plans_context_answer_for_english_fast_checklist() {
        let env = envelope(
            "I only have 30 minutes. Give me a practical checklist I can execute now.",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "当前没有命中高价值说明文件。",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_english_kickoff_message() {
        let env = envelope(
            "Write one short kickoff message for tomorrow morning so I can restart quickly.",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "当前没有命中高价值说明文件。",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_acceptance_readiness_question() {
        let env = envelope(
            "帮我先看一下当前仓库状态，然后告诉我是否可以开始新一轮验收。",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "docs/README.md: 项目说明",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_priority_three_tasks_question() {
        let env = envelope(
            "继续推进这个项目，下一步该做什么，按优先级给三件事。",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "docs/README.md: 项目说明",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_smalltalk() {
        let env = envelope(
            "你今天状态怎么样，简单聊两句。",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "docs/README.md: 项目说明",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_evidence_status_question() {
        let env = envelope(
            "我现在做到哪了？还差什么？请按当前证据目录回答。",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "docs/README.md: 项目说明",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }

    #[test]
    fn plans_context_answer_for_pause_risk_question() {
        let env = envelope(
            "如果我现在暂停，最快增长的风险是什么？怎么用一个动作降风险？",
            "当前会话还没有可复用的压缩摘要。",
            "",
            "docs/README.md: 项目说明",
        );
        assert!(matches!(
            plan_action_with_context(&env),
            PlannedAction::ContextAnswer
        ));
    }
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
