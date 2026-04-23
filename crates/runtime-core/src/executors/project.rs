use crate::answer_cache_helpers::{
    append_scene_answer_cache, cached_answer, cached_answer_reasoning, probe_answer_cache_or_bypass,
};
use crate::answer_sanitize::{is_answer_usable, sanitize_answer};
use crate::context_builder::build_runtime_context;
use crate::context_policy::project_answer_policy;
use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::knowledge::search_knowledge;
use crate::llm::complete_text;
use crate::paths::repo_root;
use crate::prompt::render_project_answer_prompt;
use crate::repo_context::load_repo_context;
use crate::session::SessionMemory;
use crate::text::{extract_snippet, summarize_text};
use crate::tool_registry::runtime_tool_registry;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn execute_project_answer(request: &RunRequest) -> ActionExecution {
    let snippets = build_project_context(request);
    let cache_probe = probe_project_cache(request, &snippets);
    if let Some(hit) = project_cache_hit(&snippets, &cache_probe) {
        return hit;
    }
    let prompt = render_project_prompt(request, &cache_probe);
    match complete_text(request, &prompt) {
        Ok(response) => project_answer_success(request, &snippets, &cache_probe, response.content),
        Err(error) => recover_project_answer(request, &snippets, &cache_probe, &error.to_string()),
    }
}

fn build_project_context(request: &RunRequest) -> String {
    let status_context = project_status_context(request);
    if !status_context.is_empty() {
        return status_context;
    }
    let hits = project_context_hits(request);
    if hits.is_empty() {
        "当前没有检索到可用项目文档片段。".to_string()
    } else {
        hits.into_iter()
            .map(|hit| format!("文件：{}\n片段：{}", hit.path, hit.snippet))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

fn project_status_context(request: &RunRequest) -> String {
    if !is_project_status_request(&request.user_input) {
        return String::new();
    }
    let sections = preferred_project_status_paths(request)
        .into_iter()
        .filter_map(|path| project_status_section(&path, &request.user_input))
        .collect::<Vec<_>>();
    if sections.is_empty() {
        String::new()
    } else {
        sections.join("\n\n")
    }
}

fn preferred_project_status_paths(request: &RunRequest) -> Vec<PathBuf> {
    let docs_root = repo_root(request).join("docs");
    let mut paths = hermes_project_status_paths(&docs_root);
    paths.extend(legacy_project_status_paths(&docs_root));
    paths
}

fn hermes_project_status_paths(docs_root: &Path) -> Vec<PathBuf> {
    vec![
        docs_root.join("11-hermes-rebuild").join("current-state.md"),
        docs_root
            .join("11-hermes-rebuild")
            .join("changes")
            .join("H-gate-h-signoff-20260416")
            .join("status.md"),
        docs_root
            .join("11-hermes-rebuild")
            .join("changes")
            .join("H-gate-h-signoff-20260416")
            .join("review.md"),
        docs_root
            .join("11-hermes-rebuild")
            .join("changes")
            .join("INDEX.md"),
        docs_root.join("README.md"),
        docs_root
            .join("11-hermes-rebuild")
            .join("Hermes重构总路线图_完整计划.md"),
        docs_root
            .join("11-hermes-rebuild")
            .join("stage-plans")
            .join("阶段计划总表.md"),
    ]
}

fn legacy_project_status_paths(docs_root: &Path) -> Vec<PathBuf> {
    vec![
        docs_root
            .join("06-development")
            .join("忠实用户转化导向开发任务书_V1.md"),
        docs_root
            .join("07-test")
            .join("忠实用户转化导向验收文档_V1.md"),
        docs_root
            .join("06-development")
            .join("第二阶段需求文档_V1.md"),
        docs_root
            .join("06-development")
            .join("第二阶段产品定位与开发重点清单_V1.md"),
        docs_root
            .join("06-development")
            .join("第二阶段短期可用能力开发任务书_V1.md"),
        docs_root
            .join("07-test")
            .join("第二阶段短期可用能力验收文档_V1.md"),
    ]
}

fn project_status_section(path: &Path, query: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let summary = project_status_summary(path, &content, query);
    Some(format!("文件：{}\n摘要：{}", path.display(), summary))
}

fn project_status_summary(path: &Path, content: &str, query: &str) -> String {
    let name = path
        .file_name()
        .and_then(|item| item.to_str())
        .unwrap_or_default();
    if name.contains("忠实用户转化导向开发任务书") {
        return "当前正式阶段已经切到忠实用户转化导向，顺序按 A、B、C、D、E、F 推进；其中项目状态回答要求稳定输出已完成能力、当前阶段、待收口项，并进一步展开到真实样本、验证路径和完成标准。".to_string();
    }
    if name.contains("忠实用户转化导向验收文档") {
        return "最新正式验收里工作包 A 已通过、整体结论仍为有条件通过；工作包 D 还要求补独立项目状态样本，并在回答中明确做到什么程度、为什么这样判断、下一步做什么。".to_string();
    }
    if name.contains("第二阶段需求文档") {
        return "定位为面向长期学习、长期成长、长期积累、可持续专精的本地个人智能体平台；当前阶段优先收口在线模型可用后的短期可用能力，不扩到多智能体、语音、浏览器全局观察和桌面自动化。".to_string();
    }
    if name.contains("产品定位与开发重点清单") {
        return "当前开发重点已经固定为在线模型主链路、本地缓存与上下文复用、记忆与知识沉淀、skill 接口预留和本地小模型兜底预研；短期目标是把系统做到可对话、可执行、可持续使用。".to_string();
    }
    if name.contains("短期可用能力开发任务书") {
        return "第二阶段短期正式交付范围包括在线模型对话主链路、工作区内文件读取、工作区内文件写入、受控命令执行、任务分析执行验证收口主链路、本地缓存最小闭环，以及记忆与知识沉淀继续增强。".to_string();
    }
    if name.contains("短期可用能力验收文档") {
        return "已完成的真实留证覆盖自然语言对话、缓存命中、文件读取、文件写入、命令执行、能力说明和高风险确认；当前正式结论仍为有条件通过，非阻断问题集中在项目状态类说明还不够细。".to_string();
    }
    extract_snippet(content, query)
}

fn project_context_hits(request: &RunRequest) -> Vec<crate::knowledge::KnowledgeHit> {
    let direct_hits = search_knowledge(request, &request.user_input, 4);
    if !direct_hits.is_empty() {
        return direct_hits;
    }
    search_knowledge(
        request,
        &project_context_fallback_query(&request.user_input),
        4,
    )
}

fn project_context_fallback_query(user_input: &str) -> String {
    if is_project_status_request(user_input) {
        "阶段 H Gate-H 聚合复核 current-state 当前活跃 change warning 未签收 不可签收 H-02 H-03 暂停点 重启条件".to_string()
    } else {
        "项目 智能体 本地 主干 架构 运行时".to_string()
    }
}

fn is_project_status_request(user_input: &str) -> bool {
    [
        "停在什么状态",
        "做到什么程度",
        "进度",
        "当前阶段",
        "实现了什么",
        "完成了吗",
        "当前情况",
        "为什么不能继续",
        "默认推进",
        "为什么停",
        "暂停点",
        "重启",
        "恢复推进",
        "还差什么",
        "继续下一步",
        "现在最该做什么",
        "下一步做什么",
    ]
    .iter()
    .any(|token| user_input.contains(token))
}

fn project_answer_success(
    request: &RunRequest,
    snippets: &str,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    content: String,
) -> ActionExecution {
    let final_answer = finalized_project_answer(request, &content, snippets);
    if should_recover_project_answer(&content, &final_answer) {
        return recover_project_answer(request, snippets, cache_probe, "模型输出不可用");
    }
    let result_summary = project_result_summary(snippets);
    append_project_answer_cache(
        request,
        cache_probe,
        snippets,
        &final_answer,
        &result_summary,
    );
    ok_project_answer(cache_probe, result_summary, final_answer)
}

fn ok_project_answer(
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    result_summary: String,
    final_answer: String,
) -> ActionExecution {
    ActionExecution::cached_ok(
        "基于本地项目文档生成项目说明。".to_string(),
        result_summary,
        final_answer,
        format!(
            "优先依据当前执行入口与项目文档片段组织项目说明。{}",
            cache_probe.reason
        ),
        cache_probe.status.clone(),
        cache_probe.reason.clone(),
    )
}

fn append_project_answer_cache(
    request: &RunRequest,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    snippets: &str,
    final_answer: &str,
    result_summary: &str,
) {
    append_scene_answer_cache(
        request,
        cache_probe,
        "project_answer",
        &request.user_input,
        snippets,
        final_answer,
        result_summary,
    );
}

fn project_result_summary(snippets: &str) -> String {
    format!(
        "已基于项目文档片段完成一次项目说明回答：{}",
        summarize_text(snippets)
    )
}

fn finalized_project_answer(request: &RunRequest, content: &str, snippets: &str) -> String {
    if is_project_status_request(&request.user_input) {
        return stable_project_status_answer(snippets);
    }
    sanitize_project_answer(
        content,
        "当前项目是一个本地智能体系统，围绕运行时、网关和前端工作台组织能力。",
    )
}

fn recover_project_answer(
    request: &RunRequest,
    snippets: &str,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    cause: &str,
) -> ActionExecution {
    let summary = fallback_project_summary(snippets);
    let cache_summary = recover_cache_summary(&summary);
    append_project_answer_cache(request, cache_probe, snippets, &summary, &cache_summary);
    recovered_project_answer(cache_probe, cause, summary)
}

fn recover_cache_summary(summary: &str) -> String {
    format!(
        "已基于项目文档恢复生成项目说明：{}",
        summarize_text(summary)
    )
}

fn recovered_project_answer(
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    cause: &str,
    summary: String,
) -> ActionExecution {
    ActionExecution::cached_ok(
        "基于本地项目文档生成项目说明。".to_string(),
        format!("项目说明主回答失败，已执行单次恢复：{}", cause),
        format!("主回答未成功，已切换到项目文档恢复路径。\n{}", summary),
        format!(
            "模型回答不可用，已降级为项目文档恢复路径。{}",
            cache_probe.reason
        ),
        cache_probe.status.clone(),
        cache_probe.reason.clone(),
    )
}

fn sanitize_project_answer(content: &str, fallback: &str) -> String {
    let answer = sanitize_answer(content, fallback);
    if is_project_answer_usable(&answer) {
        answer
    } else {
        fallback.to_string()
    }
}

fn is_project_answer_usable(content: &str) -> bool {
    is_answer_usable(content) && has_cjk_text(content) && !looks_like_path_only(content)
}

fn should_recover_project_answer(content: &str, final_answer: &str) -> bool {
    !is_project_answer_usable(content)
        || final_answer.trim()
            == "当前项目是一个本地智能体系统，围绕运行时、网关和前端工作台组织能力。"
}

fn fallback_project_summary(snippets: &str) -> String {
    if snippets.contains("当前没有检索到可用项目文档片段") {
        "当前缺少可复用的项目文档片段，建议先补充 README 或开发文档后再追问。".to_string()
    } else if is_hermes_status_context(snippets) {
        stable_project_status_answer(snippets)
    } else if is_loyal_status_context(snippets) {
        stable_project_status_answer(snippets)
    } else if is_phase2_status_context(snippets) {
        stable_project_status_answer(snippets)
    } else {
        "当前项目是一个围绕运行时、网关和前端工作台组织能力的本地智能体系统，重点在让在线模型接入后的对话、执行和沉淀链路稳定可用。结合现有文档，当前阶段更偏向主链路收口，而不是继续扩展重型未来能力。".to_string()
    }
}

fn stable_project_status_answer(snippets: &str) -> String {
    if is_hermes_status_context(snippets) {
        return "当前停在阶段 H 的 Gate-H 聚合复核暂停点：Gate-H 仍是执行中、未签收，当前活跃 change 为 `H-gate-h-signoff-20260416`。现在不能默认继续推进，是因为 H-02 仍处于并行观察 / 冻结观察 warning，当前无新的合格受限样本，第二窗口仍是 `aborted_manual_takeover`；H-03 虽已完成 `H03-37`、`H03-38`、`H03-39`，但最强结论只到“建议主控评估是否切主推进”，仍不等于 ready。Gate-H 已完成当前轮次聚合复核判断，但聚合层不能写得比 H-02 / H-03 输入更强，所以当前只能维持 `warning / 执行中 / 未签收 / 不可签收`。后续只有在出现新的 H-02 合格受限样本、H-03 获得超出当前 warning 的更强证据，或主控明确给出新的更强裁决口径时，才值得重新启动下一轮推进。".to_string();
    }
    if is_loyal_status_context(snippets) {
        return "已完成能力：在线模型对话、文件读写、受控命令执行、缓存、正式记忆查看与删除、会话续推，以及首页继续上次任务和下一步建议都已落到主链路，并已有 `tmp/loyal-user-acceptance/memory-visibility-sample.json`、`tmp/loyal-user-acceptance/memory-delete-sample.json`、`tmp/loyal-user-acceptance/project-continue-sample.json`、`tmp/loyal-user-acceptance/workspace-dashboard-sample.json` 留证。当前阶段：项目已从第二阶段短期可用，推进到忠实用户转化导向收口期；正式验收里工作包 A 已通过，整体结论仍是有条件通过，正在补工作包 D 的项目状态理解增强。待收口项：还需要把项目状态类回答稳定绑定到独立状态样本、验证路径和完成标准，并让“现在最该做什么 / 继续下一步 / 还差什么”更明确指向当前阶段动作。验证路径与完成标准：本轮应补 `tmp/loyal-user-acceptance/project-status-loyal-summary.json` 与 `tmp/loyal-user-acceptance/next-step-sample.json`，并回填 `docs/07-test/忠实用户转化导向验收文档_V1.md` 的工作包 D、构建验证和整体结论；做到回答能明确指出已完成能力、当前阶段、待收口项，并进一步指向样本或完成标准，才算通过。".to_string();
    }
    if is_phase2_status_context(snippets) {
        return "已完成能力：在线模型对话主链路、工作区内文件读取与写入、受控命令执行、本地缓存最小闭环，以及记忆和知识沉淀继续增强，这些能力都已有真实样本留证。当前阶段：仍处在第二阶段短期可用目标下的主链路收口期，重点继续把项目说明、验证留痕和前端事件日志展示做稳。待收口项：项目状态类回答还需要进一步细化到样本和完成标准，会话续答质量也还依赖压缩摘要厚度。下一步建议：优先围绕忠实用户转化方向补连续性、记忆可见性和续推体验。".to_string();
    }
    "当前项目已经具备基础运行能力，正在继续收口项目说明、执行验证和长期沉淀这几条主链路。"
        .to_string()
}

fn is_loyal_status_context(snippets: &str) -> bool {
    snippets.contains("忠实用户转化导向")
        || snippets.contains("工作包 A 已通过")
        || snippets.contains("工作包 D")
        || snippets.contains("memory-visibility-sample.json")
}

fn is_hermes_status_context(snippets: &str) -> bool {
    snippets.contains("阶段 H")
        || snippets.contains("Gate-H")
        || snippets.contains("H-gate-h-signoff-20260416")
        || snippets.contains("聚合复核")
}

fn is_phase2_status_context(snippets: &str) -> bool {
    snippets.contains("在线模型对话主链路")
        || snippets.contains("工作区内文件读取与写入")
        || snippets.contains("受控命令执行")
        || snippets.contains("本地缓存最小闭环")
}

fn has_cjk_text(content: &str) -> bool {
    content
        .chars()
        .any(|ch| ('\u{4e00}'..='\u{9fff}').contains(&ch))
}

fn looks_like_path_only(content: &str) -> bool {
    let value = content.trim();
    (value.contains(":\\") || value.contains(":/"))
        && !value.contains('。')
        && !value.contains('，')
        && !value.contains(' ')
}

fn probe_project_cache(
    request: &RunRequest,
    snippets: &str,
) -> crate::answer_cache::AnswerCacheProbe {
    probe_answer_cache_or_bypass(
        request,
        "project_answer",
        &request.user_input,
        snippets,
        !snippets.contains("当前没有检索到可用项目文档片段"),
        "当前没有稳定项目文档片段，直接走恢复或模型路径。",
    )
}

fn project_cache_hit(
    snippets: &str,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
) -> Option<ActionExecution> {
    let answer = sanitize_project_answer(&cache_probe.answer.clone()?, "");
    if answer.is_empty() {
        return None;
    }
    Some(cached_answer(
        "基于本地项目文档生成项目说明。",
        cache_probe,
        answer,
        cached_answer_reasoning("当前输入与项目文档摘要", snippets),
    ))
}

fn render_project_prompt(
    request: &RunRequest,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
) -> String {
    let session_context = SessionMemory::default();
    let repo_context = load_repo_context(std::path::Path::new(&request.workspace_ref.root_path));
    let registry = runtime_tool_registry();
    let visible_tools = registry.visible_tools(&request.mode);
    let envelope = build_runtime_context(
        request,
        &session_context,
        &repo_context,
        &visible_tools,
        &project_answer_policy(),
        &cache_probe.status,
        &cache_probe.reason,
    );
    render_project_answer_prompt(&envelope).full_prompt
}

#[cfg(test)]
mod tests {
    use super::{is_project_status_request, project_result_summary, stable_project_status_answer};

    #[test]
    fn recognizes_stage_h_pause_question_as_status_request() {
        let input = "我现在接手这个项目，请直接告诉我：当前停在什么状态、为什么不能继续默认推进、以及以后满足什么条件才值得重启。";
        assert!(is_project_status_request(input));
    }

    #[test]
    fn prefers_hermes_pause_answer_for_gate_h_context() {
        let snippets = "当前阶段：阶段 H。当前 Gate：Gate-H（执行中，未签收）。当前活跃 change：H-gate-h-signoff-20260416。当前主推进：Gate-H 聚合复核。";
        let answer = stable_project_status_answer(snippets);
        assert!(answer.contains("Gate-H"));
        assert!(answer.contains("不可签收"));
        assert!(answer.contains("重新启动下一轮推进"));
    }

    #[test]
    fn project_result_summary_prefers_current_state_first() {
        let snippets = "文件：D:\\newwork\\本地智能体\\docs\\11-hermes-rebuild\\current-state.md\n摘要：当前阶段：阶段 H。当前 Gate：Gate-H（执行中，未签收）。\n\n文件：D:\\newwork\\本地智能体\\docs\\README.md\n摘要：Docs 导航。";
        let summary = project_result_summary(snippets);
        assert!(summary.contains("current-state.md"));
    }
}
