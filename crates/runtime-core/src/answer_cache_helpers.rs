use crate::answer_cache::AnswerCacheProbe;
use crate::execution::ActionExecution;
use crate::text::summarize_text;

pub(crate) fn cached_answer(
    action_summary: &str,
    probe: &AnswerCacheProbe,
    answer: String,
    reasoning_summary: String,
) -> ActionExecution {
    ActionExecution::cached_ok(
        action_summary.to_string(),
        probe
            .summary
            .clone()
            .unwrap_or_else(|| "已复用本地缓存回答。".to_string()),
        answer,
        reasoning_summary,
        probe.status.clone(),
        probe.reason.clone(),
    )
}

pub(crate) fn cached_answer_reasoning(label: &str, context_digest: &str) -> String {
    format!(
        "{}稳定命中缓存，直接复用最近一次可用回答。上下文摘要：{}",
        label,
        summarize_text(context_digest)
    )
}

pub(crate) fn probe_answer_cache_or_bypass(
    request: &RunRequest,
    scene: &str,
    user_input: &str,
    context_digest: &str,
    context_available: bool,
    bypass_reason: &str,
) -> AnswerCacheProbe {
    if !context_available {
        return bypass_probe(bypass_reason);
    }
    probe_answer_cache(request, scene, user_input, context_digest)
}

pub(crate) fn append_scene_answer_cache(
    request: &RunRequest,
    probe: &AnswerCacheProbe,
    scene: &str,
    user_input: &str,
    context_digest: &str,
    answer: &str,
    summary: &str,
) {
    append_answer_cache(
        request,
        probe,
        scene,
        user_input,
        context_digest,
        answer,
        summary,
    );
}
use crate::answer_cache::{append_answer_cache, bypass_probe, probe_answer_cache};
use crate::contracts::RunRequest;
