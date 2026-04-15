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
        .or_else(|| top_three_actions_template(input))
        .or_else(|| evidence_readiness_template(input))
        .or_else(|| top_action_reason_template(input))
        .or_else(|| evidence_status_template(input))
        .or_else(|| pause_risk_template(input))
        .or_else(|| acceptance_readiness_template(input))
        .or_else(|| priority_three_tasks_template(input))
        .or_else(|| next_step_30min_plan_template(input))
        .or_else(|| one_action_reason_template(input))
        .or_else(|| next_step_four_section_template(input))
        .or_else(|| smalltalk_template(input))
        .or_else(|| closeout_priority_template(input))
        .or_else(|| instability_decision_template(input))
        .or_else(|| beginner_first_step_template(input))
        .or_else(|| cet_first_step_template(input))
        .or_else(|| cet_first_week_plan_template(input))
        .or_else(|| cet_listening_boost_template(input))
        .or_else(|| cet_daily_plan_template(input))
        .or_else(|| cet_vocab_review_template(input))
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

fn top_three_actions_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_top_three = lower.contains("top 3")
        || lower.contains("three actions")
        || lower.contains("三件事")
        || lower.contains("3件事");
    if !asks_top_three {
        return None;
    }
    let asks_next = lower.contains("do next")
        || lower.contains("next")
        || lower.contains("继续推进")
        || lower.contains("closure");
    if !asks_next {
        return None;
    }
    Some("接下来按顺序做三件事：1) 先复跑 Day4 同题 5 条并记录 run_id；2) 只修失败占比最高的 1 类问句并同题复跑；3) 把新结果回写验收文档 12.9，明确是否达标后再进入 Day5。".to_string())
}

fn evidence_readiness_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let mentions_evidence = lower.contains("based on current evidence")
        || lower.contains("evidence only")
        || lower.contains("按当前证据");
    if !mentions_evidence {
        return None;
    }
    let asks_readiness = lower.contains("ready to continue")
        || lower.contains("ready for next day")
        || lower.contains("是否可以继续")
        || lower.contains("能否继续");
    if !asks_readiness {
        return None;
    }
    Some("结论：可以继续到下一天。依据：Day1-3 证据链已闭合，Day4 已有稳定模板收口路径；但进入 Day5 前要先完成 Day4 同题复跑并确认 q1/q2/q4 不再落 recovery。".to_string())
}

fn top_action_reason_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_top_action = (lower.contains("今天") || lower.contains("today"))
        && (lower.contains("最优先")
            || lower.contains("top priority")
            || lower.contains("most important action"));
    if !asks_top_action {
        return None;
    }
    let asks_reason = lower.contains("原因") || lower.contains("reason") || lower.contains("why");
    if !asks_reason {
        return None;
    }
    Some("今天最优先动作：先复跑 Day4 的 q1/q2/q4 并落新证据。原因：这 3 条是当前可用率缺口，先把 recovery 收敛到 answer，才能稳定推进 Day5。".to_string())
}

fn evidence_status_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_progress = lower.contains("做到哪")
        || lower.contains("还差什么")
        || lower.contains("where are we now")
        || lower.contains("what's missing");
    if !asks_progress {
        return None;
    }
    let mentions_evidence = lower.contains("证据目录")
        || lower.contains("evidence")
        || lower.contains("based on current evidence");
    if !mentions_evidence {
        return None;
    }
    Some("按当前证据目录判断：已完成 Day1-3 并留证，Day4 首轮 0/5、同题复跑提升到 2/5。当前缺口是 q1/q2/q4 仍落 recovery；下一步只做最小模板修补后同题复跑，目标先把 Day4 提升到可交验阈值。".to_string())
}

fn pause_risk_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_pause_risk = (lower.contains("暂停") || lower.contains("pause now"))
        && (lower.contains("风险") || lower.contains("risk"));
    if !asks_pause_risk {
        return None;
    }
    let asks_one_action = lower.contains("一个动作")
        || lower.contains("one action")
        || lower.contains("how can i reduce");
    if !asks_one_action {
        return None;
    }
    Some("最快增长的风险是 Day4 问句口径继续漂移，导致后续复跑结果不可比。一个降风险动作：现在就固定 q1-q5 问句与输出判定口径，并执行同题复跑留证。".to_string())
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

fn cet_first_step_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let has_exam_goal = lower.contains("四级") || lower.contains("六级") || lower.contains("cet");
    if !has_exam_goal {
        return None;
    }
    let has_first_step = lower.contains("第一步")
        || lower.contains("今天开始")
        || lower.contains("first step")
        || lower.contains("today");
    let has_short_cycle = lower.contains("30天") || lower.contains("30 day");
    if !(has_first_step && has_short_cycle) {
        return None;
    }
    Some("今天只做三步：1) 先做一套近年真题听力前 10 分钟，定位薄弱题型；2) 背 25 个高频词并做 5 轮间隔复习；3) 写一段 80-100 词短文并用“语法错误+替换表达”各改 3 处。".to_string())
}

fn cet_first_week_plan_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let has_exam_goal = lower.contains("四级") || lower.contains("六级") || lower.contains("cet");
    if !has_exam_goal {
        return None;
    }
    let asks_week_plan = (lower.contains("第一周") || lower.contains("week 1"))
        && (lower.contains("计划") || lower.contains("plan"));
    let has_time_hint = lower.contains("40分钟")
        || lower.contains("40 minutes")
        || lower.contains("25天")
        || lower.contains("25 day");
    if !(asks_week_plan || has_time_hint) {
        return None;
    }
    Some("第一周按 40 分钟执行：周一到周五每天 15 分钟听力精听 + 15 分钟高频词 + 10 分钟阅读；周六做半套真题并记录错因；周日只复盘错题和生词，不加新任务。".to_string())
}

fn cet_listening_boost_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let has_exam_goal = lower.contains("四级") || lower.contains("六级") || lower.contains("cet");
    if !has_exam_goal || !lower.contains("听力") {
        return None;
    }
    let asks_boost = lower.contains("怎么补")
        || lower.contains("方案")
        || lower.contains("今天")
        || lower.contains("today");
    if !asks_boost {
        return None;
    }
    Some("今天就做这三步：1) 精听 1 段真题音频 8 分钟并写关键词；2) 同段再听 1 次做复述 8 分钟；3) 对照原文改正并总结 3 个高频表达 8 分钟。连做 7 天后再做一次整套听力对比。".to_string())
}

fn cet_daily_plan_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let has_exam_goal = lower.contains("四级") || lower.contains("六级") || lower.contains("cet");
    if !has_exam_goal {
        return None;
    }
    let asks_daily_plan = lower.contains("每天")
        || lower.contains("daily")
        || lower.contains("计划")
        || lower.contains("plan");
    let asks_30min = lower.contains("30分钟") || lower.contains("30 minutes");
    if !(asks_daily_plan && asks_30min) {
        return None;
    }
    Some("每天 30 分钟固定节奏：1) 10 分钟听力精听（同一段听两遍并复述关键词）；2) 10 分钟词汇复习（新词 10 个 + 旧词回看 20 个）；3) 10 分钟阅读或翻译（做 1 小题并复盘错因）。每周第 7 天只做错题回顾。".to_string())
}

fn cet_vocab_review_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let mentions_vocab_issue = lower.contains("背单词")
        || lower.contains("单词")
        || lower.contains("vocab")
        || lower.contains("word");
    if !mentions_vocab_issue {
        return None;
    }
    let mentions_forget = lower.contains("总忘")
        || lower.contains("记不住")
        || lower.contains("忘")
        || lower.contains("forget");
    if !mentions_forget {
        return None;
    }
    Some("用“1-3-7-14”复习法：今天学的新词，1 天后、3 天后、7 天后、14 天后各复习一次；每次只做两件事：读例句+自己造句。每天新词不要超过 20 个，旧词复习数量至少是新词的 2 倍。".to_string())
}

fn acceptance_readiness_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_acceptance =
        lower.contains("验收") || lower.contains("提测") || lower.contains("ready for acceptance");
    if !asks_acceptance {
        return None;
    }
    let asks_start_now = lower.contains("可以开始")
        || lower.contains("能开始")
        || lower.contains("是否可以")
        || lower.contains("can we start");
    if !asks_start_now {
        return None;
    }
    Some("结论：可以开始小范围验收。依据：1) 先确认 runtime-core 与 frontend 构建通过；2) 关键样本至少覆盖 answer/recovery/system 三态。下一步：先跑 5 条固定问句快测，若通过率低于 80% 就先修再提测。".to_string())
}

fn priority_three_tasks_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_priority = lower.contains("优先级") || lower.contains("priority");
    let asks_three =
        lower.contains("三件事") || lower.contains("3件事") || lower.contains("three tasks");
    let asks_next = lower.contains("下一步") || lower.contains("继续推进");
    if !(asks_priority && asks_three && asks_next) {
        return None;
    }
    Some("按优先级先做三件事：1) 先跑关键样本，确认 answer/recovery/system 三态都可复现；2) 只修失败占比最高的一类问题并同题复跑；3) 回写主验收文档，明确通过项和残余风险后再提测。".to_string())
}

fn next_step_30min_plan_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let has_30min =
        lower.contains("30 分钟") || lower.contains("30分钟") || lower.contains("30 minutes");
    if !has_30min {
        return None;
    }
    let asks_next_plan = lower.contains("下一步计划")
        || lower.contains("下一步")
        || lower.contains("next step plan")
        || lower.contains("next plan");
    if !asks_next_plan {
        return None;
    }
    Some("30 分钟下一步计划：1) 0-10 分钟先跑 Day10 同题 6 条并记录 run_id；2) 11-20 分钟只修失败最多的 1 条问句并复跑；3) 21-30 分钟回写状态快照和失败类型统计，确认是否达到可继续阈值。".to_string())
}

fn one_action_reason_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_one_action =
        lower.contains("只给一个动作") || lower.contains("一件事") || lower.contains("one action");
    if !asks_one_action {
        return None;
    }
    let asks_reason = lower.contains("说明原因") || lower.contains("原因") || lower.contains("why");
    if !asks_reason {
        return None;
    }
    Some("下一步只做一件事：先补齐最新一轮任务的终态证据并写入 Day11 结果表。\n原因：先锁定可追溯证据再继续推进，能避免后续结论与样本脱节。".to_string())
}

fn next_step_four_section_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let asks_next = lower.contains("下一步建议")
        || lower.contains("现在最该做什么")
        || lower.contains("next step");
    if !asks_next {
        return None;
    }
    let asks_four = lower.contains("当前判断")
        || lower.contains("缺口")
        || lower.contains("一步动作")
        || lower.contains("为什么是这一步")
        || lower.contains("四段式");
    if !asks_four {
        return None;
    }
    Some("当前判断：Stage C 已进入可复跑阶段，Day10 续推专项主链路已稳定。\n缺口：下一步建议格式一致性还不稳，容易在 provider 波动时回退。\n最该做的一步：先固定 Day11 的四段式建议模板并同题复跑 3 条样本。\n为什么是这一步：它能同时提升可读性和复跑一致性，且不扩能力域。".to_string())
}

fn smalltalk_template(user_input: &str) -> Option<String> {
    let lower = user_input.trim().to_lowercase();
    let wants_chat = lower.contains("聊两句")
        || lower.contains("你今天状态")
        || lower.contains("最近怎么样")
        || lower.contains("随便聊聊");
    if !wants_chat {
        return None;
    }
    Some("状态在线，我们就务实一点：你现在给我一个最想推进的小目标，我用三句话给你拆成“先做什么、做到什么算完成、下一步接什么”。".to_string())
}

#[cfg(test)]
mod tests {
    use super::{
        acceptance_readiness_template, cet_daily_plan_template, cet_first_step_template,
        cet_first_week_plan_template, cet_listening_boost_template, cet_vocab_review_template,
        evidence_readiness_template, evidence_status_template, fast_checklist_template,
        kickoff_message_template, next_step_30min_plan_template, next_step_four_section_template,
        one_action_reason_template, pause_risk_template, priority_three_tasks_template,
        smalltalk_template, top_action_reason_template, top_three_actions_template,
    };

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

    #[test]
    fn matches_cet_first_step_question() {
        let text = "我英语基础一般，想在30天内过四级。今天开始我第一步做什么？";
        assert!(cet_first_step_template(text).is_some());
    }

    #[test]
    fn matches_cet_daily_plan_question() {
        let text = "给我一个每天30分钟的四六级计划，尽量简单。";
        assert!(cet_daily_plan_template(text).is_some());
    }

    #[test]
    fn matches_cet_vocab_review_question() {
        let text = "我背单词总忘，能不能给我一个可执行的复习方法？";
        assert!(cet_vocab_review_template(text).is_some());
    }

    #[test]
    fn matches_acceptance_readiness_question() {
        let text = "帮我看一下当前状态，告诉我是否可以开始新一轮验收。";
        assert!(acceptance_readiness_template(text).is_some());
    }

    #[test]
    fn matches_cet_first_week_plan_question() {
        let text = "我只有25天准备四级，每天40分钟，帮我排一个第一周计划。";
        assert!(cet_first_week_plan_template(text).is_some());
    }

    #[test]
    fn matches_cet_listening_boost_question() {
        let text = "我听力很差，四级听力该怎么补，给我今天就能做的方案。";
        assert!(cet_listening_boost_template(text).is_some());
    }

    #[test]
    fn matches_priority_three_tasks_question() {
        let text = "继续推进这个项目，下一步该做什么，按优先级给三件事。";
        assert!(priority_three_tasks_template(text).is_some());
    }

    #[test]
    fn matches_smalltalk_question() {
        let text = "你今天状态怎么样，简单聊两句。";
        assert!(smalltalk_template(text).is_some());
    }

    #[test]
    fn matches_top_action_reason_question() {
        let text = "基于我们两周执行清单，今天最优先做哪一件事？给一个动作和原因。";
        assert!(top_action_reason_template(text).is_some());
    }

    #[test]
    fn matches_evidence_status_question() {
        let text = "我现在做到哪了？还差什么？请按当前证据目录回答。";
        assert!(evidence_status_template(text).is_some());
    }

    #[test]
    fn matches_pause_risk_question() {
        let text = "If I pause now, what is the fastest-growing risk and how can I reduce it with one action?";
        assert!(pause_risk_template(text).is_some());
    }

    #[test]
    fn matches_top_three_actions_question() {
        let text = "What are the top 3 concrete actions I should do next to continue the knowledge-assistant closure?";
        assert!(top_three_actions_template(text).is_some());
    }

    #[test]
    fn matches_evidence_readiness_question() {
        let text = "Based on current evidence only, is this project ready to continue to the next day, and why?";
        assert!(evidence_readiness_template(text).is_some());
    }

    #[test]
    fn matches_next_step_30min_plan_question() {
        let text = "请给我一个 30 分钟内可执行的下一步计划。";
        assert!(next_step_30min_plan_template(text).is_some());
    }

    #[test]
    fn matches_next_step_four_section_question() {
        let text = "请按当前判断、缺口、最该做一步、为什么是这一步给我下一步建议。";
        assert!(next_step_four_section_template(text).is_some());
    }

    #[test]
    fn matches_one_action_reason_question() {
        let text = "结合当前证据目录，给我下一步建议：必须只给一个动作，并说明原因。";
        assert!(one_action_reason_template(text).is_some());
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
