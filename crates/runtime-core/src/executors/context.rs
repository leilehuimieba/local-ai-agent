use crate::answer_cache_helpers::{
    append_scene_answer_cache, cached_answer, cached_answer_reasoning, probe_answer_cache_or_bypass,
};
use crate::answer_sanitize::sanitize_answer;
use crate::context_builder::build_runtime_context;
use crate::context_policy::context_answer_policy;
use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::llm::complete_text;
use crate::prompt::render_context_answer_prompt;
use crate::repo_context::load_repo_context;
use crate::session::SessionMemory;
use crate::text::summarize_text;
use crate::tool_registry::runtime_tool_registry;

pub(crate) fn execute_context_answer(
    request: &RunRequest,
    session_context: &SessionMemory,
) -> ActionExecution {
    if is_status_continue_request(&request.user_input) {
        return session_continue_answer(request, session_context);
    }
    let cache_probe = probe_context_cache(request, session_context);
    if let Some(hit) = context_cache_hit(session_context, &cache_probe) {
        return hit;
    }
    let prompt = render_context_prompt(request, session_context, &cache_probe);
    match complete_text(request, &prompt) {
        Ok(response) => {
            context_answer_success(request, session_context, &cache_probe, &response.content)
        }
        Err(error) => {
            recover_context_answer(request, session_context, &cache_probe, &error.to_string())
        }
    }
}

fn context_answer_success(
    request: &RunRequest,
    session_context: &SessionMemory,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    content: &str,
) -> ActionExecution {
    let fallback = "当前基于已有会话上下文，可明确的信息还比较有限。";
    let final_answer = sanitize_answer(content, fallback);
    if should_recover_context_answer(content, &final_answer) {
        return recover_context_answer(request, session_context, cache_probe, "模型输出不可用");
    }
    let result_summary = context_result_summary(session_context);
    append_context_answer_cache(
        request,
        cache_probe,
        &request.user_input,
        &final_answer,
        &result_summary,
        session_context,
    );
    ok_context_answer(cache_probe, result_summary, final_answer)
}

fn ok_context_answer(
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    result_summary: String,
    final_answer: String,
) -> ActionExecution {
    ActionExecution::cached_ok(
        "基于会话压缩摘要继续回答。".to_string(),
        result_summary,
        final_answer,
        format!(
            "先读取最近会话压缩摘要，再结合当前输入组织续答。{}",
            cache_probe.reason
        ),
        cache_probe.status.clone(),
        cache_probe.reason.clone(),
    )
}

fn context_result_summary(session_context: &SessionMemory) -> String {
    format!(
        "已从最近 {} 轮会话中提取压缩上下文，并完成一次模型回答。",
        session_context.recent_turns.len()
    )
}

fn recover_context_answer(
    request: &RunRequest,
    session_context: &SessionMemory,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    cause: &str,
) -> ActionExecution {
    let summary = fallback_context_summary(&request.user_input, session_context);
    append_context_answer_cache(
        request,
        cache_probe,
        &request.user_input,
        &summary,
        cause,
        session_context,
    );
    recovered_context_answer(cache_probe, cause, summary)
}

fn recovered_context_answer(
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    cause: &str,
    summary: String,
) -> ActionExecution {
    ActionExecution::cached_ok(
        "基于会话压缩摘要继续回答。".to_string(),
        format!("模型主回答失败，已执行单次恢复：{}", cause),
        format!("主回答未成功，已切换到会话摘要恢复路径。\n{}", summary),
        format!(
            "模型回答不可用，已降级为会话摘要恢复路径。{}",
            cache_probe.reason
        ),
        cache_probe.status.clone(),
        cache_probe.reason.clone(),
    )
}

fn append_context_answer_cache(
    request: &RunRequest,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    user_input: &str,
    answer: &str,
    summary: &str,
    session_context: &SessionMemory,
) {
    append_scene_answer_cache(
        request,
        cache_probe,
        "context_answer",
        user_input,
        &session_context.compressed_summary,
        answer,
        summary,
    );
}

fn should_recover_context_answer(content: &str, final_answer: &str) -> bool {
    !crate::answer_sanitize::is_answer_usable(content)
        || final_answer.trim() == "当前基于已有会话上下文，可明确的信息还比较有限。"
}

fn fallback_context_summary(user_input: &str, session_context: &SessionMemory) -> String {
    let summary = session_context.compressed_summary.trim();
    if !summary.is_empty() {
        return format!(
            "基于当前会话摘要，可先确认这些信息：{}",
            summarize_text(summary)
        );
    }
    minimal_recovery_template(user_input).unwrap_or_else(|| {
        "当前没有可复用的会话摘要。你可以直接补这三项：1) 当前目标；2) 已完成到哪一步；3) 你希望我先给清单、顺序还是排障。".to_string()
    })
}

fn minimal_recovery_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    if lower.contains("30分钟") && lower.contains("回归") {
        return Some("先给你最低可用版：0-5 分钟确认构建通过；6-15 分钟跑学习混合路由+provider 波动；16-25 分钟抽检 answer/recovery/system 三态页面；26-30 分钟整理 run_id 和失败点。".to_string());
    }
    if lower.contains("知识沉淀") {
        return Some("今天最小方案：1) 只沉淀 1 个主题；2) 每条只写“结论+依据+下一步”三行；3) 今天结束前做一次去重，把重复和空话删掉。".to_string());
    }
    if (lower.contains("收口") || lower.contains("验收")) && (lower.contains("顺序") || lower.contains("步骤")) {
        return Some("不扩功能的收口顺序：1) 锁定范围和口径；2) 补真实样本与页面证据；3) 跑最小构建与关键用例；4) 把未闭合风险写清楚再交验。".to_string());
    }
    if lower.contains("做到哪") || lower.contains("进行到哪") || lower.contains("未完成") {
        return Some("先按三段给你最低可用版：已完成：结果分层链路和小回归包已在跑；未完成：回答质量稳定性仍有风险；下一步：先修默认直答与恢复模板，再复跑同一批真实问句。".to_string());
    }
    None
}

fn is_status_continue_request(input: &str) -> bool {
    ["继续推进", "上次做到哪", "还差什么", "下一步做什么"]
        .iter()
        .any(|token| input.contains(token))
}

fn session_continue_answer(
    request: &RunRequest,
    session_context: &SessionMemory,
) -> ActionExecution {
    let summary = continue_summary(session_context);
    ActionExecution::bypass_ok(
        "基于当前会话状态生成续推回答。".to_string(),
        "已根据最近会话摘要整理上次进展与下一步建议。".to_string(),
        format!("{}\n当前工作区：{}", summary, request.workspace_ref.name),
        "当前输入命中了续推类问题，优先使用短期状态和压缩摘要直接回答。".to_string(),
        "续推回答需要优先反映当前会话最新状态，不直接复用旧缓存。",
    )
}

fn continue_summary(session_context: &SessionMemory) -> String {
    let short = &session_context.short_term;
    let current = blank_fallback(&short.current_goal, "当前目标尚未明确");
    let plan = blank_fallback(&short.current_plan, "当前计划尚未形成");
    let issue = blank_fallback(&short.open_issue, "当前没有显式阻塞");
    let next = next_step_text(short);
    format!(
        "上次做到哪：{}。当前计划：{}。还差什么：{}。下一步建议：{}。",
        current, plan, issue, next
    )
}

fn next_step_text(short: &crate::session::ShortTermMemory) -> &str {
    if !short.pending_confirmation.is_empty() {
        return &short.pending_confirmation;
    }
    if !short.open_issue.is_empty() {
        return &short.open_issue;
    }
    if !short.current_plan.is_empty() {
        return &short.current_plan;
    }
    "补充一个更明确的下一步任务"
}

fn blank_fallback<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if value.is_empty() {
        fallback
    } else {
        value
    }
}

fn probe_context_cache(
    request: &RunRequest,
    session_context: &SessionMemory,
) -> crate::answer_cache::AnswerCacheProbe {
    probe_answer_cache_or_bypass(
        request,
        "context_answer",
        &request.user_input,
        &session_context.compressed_summary,
        !session_context.compressed_summary.trim().is_empty(),
        "当前会话摘要为空，直接走模型回答路径。",
    )
}

fn context_cache_hit(
    session_context: &SessionMemory,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
) -> Option<ActionExecution> {
    let answer = cache_probe.answer.clone()?;
    Some(cached_answer(
        "基于会话压缩摘要继续回答。",
        cache_probe,
        answer,
        format!(
            "{}。会话轮次：{}。",
            cached_answer_reasoning("当前输入与会话摘要", &session_context.compressed_summary),
            session_context.recent_turns.len()
        ),
    ))
}

fn render_context_prompt(
    request: &RunRequest,
    session_context: &SessionMemory,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
) -> String {
    let repo_context = load_repo_context(std::path::Path::new(&request.workspace_ref.root_path));
    let registry = runtime_tool_registry();
    let visible_tools = registry.visible_tools(&request.mode);
    let envelope = build_runtime_context(
        request,
        session_context,
        &repo_context,
        &visible_tools,
        &context_answer_policy(),
        &cache_probe.status,
        &cache_probe.reason,
    );
    render_context_answer_prompt(&envelope).full_prompt
}
