use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::paths::answer_cache_file_path;
use crate::storage::{append_jsonl, read_jsonl};
use crate::text::summarize_text;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct AnswerCacheEntry {
    pub cache_key: String,
    pub scene: String,
    pub user_input: String,
    pub context_digest: String,
    pub answer: String,
    pub summary: String,
    pub created_at: String,
}

#[derive(Clone, Debug)]
pub(crate) struct AnswerCacheProbe {
    pub cache_key: String,
    pub status: String,
    pub reason: String,
    pub answer: Option<String>,
    pub summary: Option<String>,
}

pub(crate) fn bypass_probe(reason: &str) -> AnswerCacheProbe {
    AnswerCacheProbe {
        cache_key: String::new(),
        status: "bypass".to_string(),
        reason: reason.to_string(),
        answer: None,
        summary: None,
    }
}

pub(crate) fn probe_answer_cache(
    request: &RunRequest,
    scene: &str,
    user_input: &str,
    context_digest: &str,
) -> AnswerCacheProbe {
    let cache_key = cache_key(scene, user_input, context_digest);
    let entries = read_jsonl::<AnswerCacheEntry>(&answer_cache_file_path(request));
    match entries
        .into_iter()
        .rev()
        .find(|item| item.cache_key == cache_key)
    {
        Some(entry) => hit_probe(cache_key, entry),
        None => miss_probe(cache_key, scene),
    }
}

pub(crate) fn append_answer_cache(
    request: &RunRequest,
    probe: &AnswerCacheProbe,
    scene: &str,
    user_input: &str,
    context_digest: &str,
    answer: &str,
    summary: &str,
) {
    if probe.status == "hit"
        || probe.status == "bypass"
        || probe.cache_key.is_empty()
        || answer.trim().is_empty()
    {
        return;
    }
    let entry = AnswerCacheEntry {
        cache_key: probe.cache_key.clone(),
        scene: scene.to_string(),
        user_input: summarize_text(user_input),
        context_digest: summarize_text(context_digest),
        answer: answer.to_string(),
        summary: summarize_text(summary),
        created_at: timestamp_now(),
    };
    let _ = append_jsonl(answer_cache_file_path(request), &entry);
}

fn cache_key(scene: &str, user_input: &str, context_digest: &str) -> String {
    format!(
        "{}::{}::{}",
        scene,
        normalize_key(user_input),
        normalize_key(context_digest)
    )
}

fn normalize_key(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

fn hit_probe(cache_key: String, entry: AnswerCacheEntry) -> AnswerCacheProbe {
    AnswerCacheProbe {
        cache_key,
        status: "hit".to_string(),
        reason: "命中本地缓存，优先复用最近稳定回答。".to_string(),
        answer: Some(entry.answer),
        summary: Some(entry.summary),
    }
}

fn miss_probe(cache_key: String, scene: &str) -> AnswerCacheProbe {
    AnswerCacheProbe {
        cache_key,
        status: "miss".to_string(),
        reason: format!("当前未命中 `{scene}` 本地缓存，本轮将生成新回答并回写缓存。"),
        answer: None,
        summary: None,
    }
}
