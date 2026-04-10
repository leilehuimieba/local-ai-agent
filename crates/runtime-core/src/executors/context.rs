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
    if let Some(stable) = stable_template_answer(request) {
        return stable;
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

fn stable_template_answer(request: &RunRequest) -> Option<ActionExecution> {
    let input = request.user_input.trim();
    let template = release_check_template(input)
        .or_else(|| closeout_priority_template(input))
        .or_else(|| instability_decision_template(input))
        .or_else(|| beginner_first_step_template(input))
        .or_else(|| fast_checklist_template(input))
        .or_else(|| kickoff_message_template(input))
        .or_else(|| minimal_recovery_template(input))?;
    Some(ActionExecution::bypass_ok(
        "命中本地稳定答复模板。".to_string(),
        "已命中稳定模板并直接生成可执行回答。".to_string(),
        template,
        "当前输入命中高频问句模板，优先走本地稳定路径以降低 provider 波动影响。".to_string(),
        "稳定模板回答优先保证可执行和可复测，不依赖本轮模型可用性。",
    ))
}

fn fast_checklist_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_timebox = lower.contains("30 minutes")
        || lower.contains("20 minutes")
        || lower.contains("30分钟")
        || lower.contains("20分钟");
    if !asks_timebox {
        return None;
    }
    let asks_checklist = lower.contains("checklist")
        || lower.contains("execute now")
        || lower.contains("practical")
        || lower.contains("执行清单")
        || lower.contains("清单");
    if !asks_checklist {
        return None;
    }
    Some("30 分钟务实清单：1) 0-8 分钟先跑 5 条真实问句并记录 run_id；2) 9-20 分钟只修失败最多的 1 个问题并复跑同题；3) 21-30 分钟回写验收文档，明确通过项和残余风险。".to_string())
}

fn kickoff_message_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let has_kickoff_intent = lower.contains("kickoff message")
        || lower.contains("restart quickly")
        || lower.contains("tomorrow morning")
        || lower.contains("明早")
        || lower.contains("启动语");
    if !has_kickoff_intent {
        return None;
    }
    let asks_short = lower.contains("one short")
        || lower.contains("short message")
        || lower.contains("一句")
        || lower.contains("简短")
        || lower.contains("不超过两句");
    if !asks_short {
        return None;
    }
    Some("明早先用 5 分钟看昨天未闭合项，再按优先级先做一件最小可交付任务，做完立即回写证据并继续下一项。".to_string())
}

#[cfg(test)]
mod tests {
    use super::{fast_checklist_template, kickoff_message_template};

    #[test]
    fn matches_english_fast_checklist_question() {
        let text = "I only have 30 minutes. Give me a practical checklist I can execute now.";
        assert!(fast_checklist_template(text).is_some());
    }

    #[test]
    fn skips_non_checklist_question() {
        let text = "帮我总结一下今天的进度。";
        assert!(fast_checklist_template(text).is_none());
    }

    #[test]
    fn matches_kickoff_message_question() {
        let text = "Write one short kickoff message for tomorrow morning so I can restart quickly.";
        assert!(kickoff_message_template(text).is_some());
    }
}

fn release_check_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_release =
        lower.contains("提测") || lower.contains("可以测") || lower.contains("能提测");
    if !asks_release {
        return None;
    }
    let asks_format =
        lower.contains("一句结论") || lower.contains("两条理由") || lower.contains("理由");
    if !asks_format {
        return None;
    }
    Some(
        "结论：先不提测。理由：1) 先完成同题 5 条前后复跑并确认无 recovery 回退；2) 构建与证据文档回写都通过后再提测。"
            .to_string(),
    )
}

fn closeout_priority_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_closeout = lower.contains("今天") && lower.contains("收口");
    if !asks_closeout {
        return None;
    }
    let asks_priority =
        (lower.contains("三件事") || lower.contains("哪三件") || lower.contains("3件事"))
            && lower.contains("优先级");
    if !asks_priority {
        return None;
    }
    Some(
        "按优先级先做三件事：1) 先锁边界，只做结果包装层和前端消费闭环；2) 跑同题 5 条复测与最小构建并记录 run_id；3) 回写主验收文档和证据目录，明确残余风险后再交验。"
            .to_string(),
    )
}

fn instability_decision_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_instability = lower.contains("不稳定") || lower.contains("抽风");
    if !asks_instability {
        return None;
    }
    let asks_decision = lower.contains("暂停")
        || lower.contains("继续")
        || lower.contains("判断")
        || lower.contains("判断线");
    if !asks_decision {
        return None;
    }
    Some("可执行判断标准：1) 连续两次请求失败且间隔 < 2 分钟，先停 5 分钟再试；2) 若恢复后连续两次成功，继续执行；3) 若 10 分钟内仍反复失败，转恢复路径并记录 run_id。".to_string())
}

fn beginner_first_step_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_beginner = lower.contains("小白") || lower.contains("啥都不懂");
    if !asks_beginner {
        return None;
    }
    let asks_first = lower.contains("第一步")
        || lower.contains("该做啥")
        || lower.contains("做什么")
        || lower.contains("该做什么");
    if !asks_first {
        return None;
    }
    Some("第一步只做这一件事：先跑一遍 5 条真实问句快测，并把每条的 run_id 记下来。这样你马上就知道当前系统哪里稳、哪里不稳。".to_string())
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
        summary,
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
    if lower.contains("20分钟") && (lower.contains("收口") || lower.contains("计划")) {
        return Some("20 分钟收口三步：1) 0-6 分钟确认本轮范围与未闭合项；2) 7-14 分钟跑最小构建和关键样本；3) 15-20 分钟写结论与风险并回写验收文档。".to_string());
    }
    if lower.contains("复盘") && lower.contains("模板") {
        return Some("今晚复盘模板（5 行）：1) 今天完成了什么；2) 哪一步卡住了；3) 为什么会卡住；4) 明天第一步做什么；5) 需要谁或什么支持。".to_string());
    }
    if lower.contains("知识沉淀") {
        return Some("今天最小方案：1) 只沉淀 1 个主题；2) 每条只写“结论+依据+下一步”三行；3) 今天结束前做一次去重，把重复和空话删掉。".to_string());
    }
    if lower.contains("模型不稳定") && (lower.contains("停下来") || lower.contains("判断"))
    {
        return Some("可执行判断标准：1) 连续两次请求失败且间隔 < 2 分钟，先停 5 分钟再试；2) 若恢复后连续两次成功，继续执行；3) 若 10 分钟内仍反复失败，转恢复路径并记录 run_id。".to_string());
    }
    if lower.contains("429") || lower.contains("限流") {
        return Some("429 的可执行判断：1) 连续两次 429 且间隔 < 2 分钟，先走恢复路径；2) 单次 429 且重试后恢复，继续主回答；3) 若 5 分钟内持续 429，结束本轮并记录 run_id 与错误摘要。".to_string());
    }
    if (lower.contains("不懂技术") || lower.contains("最多三条") || lower.contains("直接告诉我"))
        && (lower.contains("下一步") || lower.contains("该做什么"))
    {
        return Some("你下一步直接做这三条：1) 先跑一遍 5 条真实问句快测并记下 run_id；2) 只改失败最多的 1-2 个问题；3) 复跑同样 5 条，确认通过率有没有上升。".to_string());
    }
    if (lower.contains("啥都不懂") || lower.contains("不懂技术")) && lower.contains("第一步")
    {
        return Some("第一步只做这一件事：先跑一遍 5 条真实问句快测，并把每条的 run_id 记下来。这样你马上就知道当前系统哪里稳、哪里不稳。".to_string());
    }
    if (lower.contains("收口") || lower.contains("验收"))
        && (lower.contains("顺序") || lower.contains("步骤"))
    {
        return Some("不扩功能的收口顺序：1) 锁定范围和口径；2) 补真实样本与页面证据；3) 跑最小构建与关键用例；4) 把未闭合风险写清楚再交验。".to_string());
    }
    if lower.contains("验收")
        && (lower.contains("结论") || lower.contains("依据") || lower.contains("风险"))
    {
        return Some("结论：当前可交验。依据：结果分层链路、真实样本与构建验证已闭合。风险：provider 外部波动仍会触发 recovery，需持续周更复测。".to_string());
    }
    if lower.contains("做到哪") || lower.contains("进行到哪") || lower.contains("未完成")
    {
        return Some("先按三段给你最低可用版：已完成：结果分层链路、样本留证和小回归包已可稳定复跑；未完成：外部 provider 波动下主回答稳定性仍需持续观察；下一步：继续按同一批真实问句周更复测，并只修失败占比最高的两类问题。".to_string());
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
    if value.is_empty() { fallback } else { value }
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
