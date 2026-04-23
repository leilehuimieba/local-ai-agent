use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::execution::ActionExecution;
use crate::memory_layer::{digest_layer_phrase, digest_layer_summary};
use crate::memory::{MemoryEntry, append_memory_entry, search_memory_entries};
use crate::memory_recall::recall_memory_digest;
use crate::text::summarize_text;

const CACHE_WRITE_REASON: &str = "记忆写入属于实时副作用动作，不使用回答缓存。";
const CACHE_RECALL_REASON: &str = "记忆召回依赖实时存储状态，不使用回答缓存。";

pub(crate) fn execute_memory_write(
    request: &RunRequest,
    kind: &str,
    summary: &str,
    content: &str,
) -> ActionExecution {
    let entry = build_memory_entry(request, kind, summary, content);
    match append_memory_entry(request, &entry) {
        Ok(()) => ok_write(&entry),
        Err(error) => fail(
            format!("写入长期记忆：{}", summary),
            format!("记忆写入失败：{}", error),
            format!("记忆写入失败：{}", error),
            "长期记忆写入失败，按存储错误直接返回。",
            CACHE_WRITE_REASON,
        ),
    }
}

pub(crate) fn execute_memory_recall(request: &RunRequest, query: &str) -> ActionExecution {
    let entries = search_memory_entries(request, query, 3);
    let digest = recall_memory_digest(request, query, 3);
    if entries.is_empty() {
        return ok(
            format!("按需召回记忆：{}", query),
            format!("没有找到相关长期记忆。（{}）", digest_layer_phrase(&digest)),
            format!("当前没有找到与 `{}` 相关的长期记忆。", query),
            &format!(
                "先检索长期记忆索引，未命中时直接返回空结果说明；本次召回层为{}。",
                digest_layer_phrase(&digest)
            ),
            CACHE_RECALL_REASON,
        );
    }
    ok(
        format!("按需召回记忆：{}", query),
        format!("已召回 {} 条相关记忆。（{}）", entries.len(), digest_layer_phrase(&digest)),
        format!(
            "已召回相关长期记忆。\n召回层：{}\n\n{}",
            digest_layer_summary(&digest),
            render_entries(&entries)
        ),
        &format!(
            "按查询词检索长期记忆，并返回前几条高相关结果；本次召回层为{}。",
            digest_layer_phrase(&digest)
        ),
        CACHE_RECALL_REASON,
    )
}

fn render_entries(entries: &[MemoryEntry]) -> String {
    entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            format!(
                "{}. [{}] {}\n   {}",
                index + 1,
                entry.kind,
                entry.summary,
                summarize_text(&entry.content)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn build_memory_entry(
    request: &RunRequest,
    kind: &str,
    summary: &str,
    content: &str,
) -> MemoryEntry {
    let now = timestamp_now();
    MemoryEntry {
        id: memory_id(),
        kind: kind.to_string(),
        title: summary.to_string(),
        summary: summary.to_string(),
        content: content.to_string(),
        scope: request.workspace_ref.name.clone(),
        workspace_id: request.workspace_ref.workspace_id.clone(),
        session_id: request.session_id.clone(),
        source_run_id: request.run_id.clone(),
        source: memory_source(request),
        source_type: "runtime".to_string(),
        source_title: summary.to_string(),
        source_event_type: "memory_written".to_string(),
        source_artifact_path: String::new(),
        governance_version: String::new(),
        governance_reason: String::new(),
        governance_source: String::new(),
        governance_at: String::new(),
        archive_reason: String::new(),
        verified: true,
        priority: 0,
        archived: false,
        archived_at: String::new(),
        created_at: now.clone(),
        updated_at: now.clone(),
        timestamp: now,
    }
}

fn memory_id() -> String {
    format!("memory-{}", timestamp_now())
}

fn memory_source(request: &RunRequest) -> String {
    format!("run:{}", request.run_id)
}

fn ok_write(entry: &MemoryEntry) -> ActionExecution {
    ActionExecution::bypass_ok_with_memory_summary(
        format!("写入长期记忆：{}", entry.summary),
        format!("已写入 `{}` 记忆：{}", entry.kind, entry.summary),
        format!(
            "记忆写入完成。\n类型：{}\n摘要：{}\n内容摘要：{}",
            entry.kind,
            entry.summary,
            summarize_text(&entry.content)
        ),
        "按用户指定内容构造长期记忆记录并写入本地主存储。".to_string(),
        CACHE_WRITE_REASON,
    )
}

fn ok(
    action_summary: String,
    result_summary: String,
    final_answer: String,
    reasoning_summary: &str,
    cache_reason: &str,
) -> ActionExecution {
    ActionExecution::bypass_ok(
        action_summary,
        result_summary,
        final_answer,
        reasoning_summary.to_string(),
        cache_reason,
    )
}

fn fail(
    action_summary: String,
    result_summary: String,
    final_answer: String,
    reasoning_summary: &str,
    cache_reason: &str,
) -> ActionExecution {
    ActionExecution::bypass_fail(
        action_summary,
        result_summary,
        final_answer,
        reasoning_summary.to_string(),
        cache_reason,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{ModelRef, ProviderRef, WorkspaceRef};
    use crate::memory::MemoryEntry;
    use crate::sqlite_store::write_memory_entry_sqlite;
    use std::collections::BTreeMap;

    #[test]
    fn memory_recall_result_mentions_object_aware_layers() {
        let request = sample_request("对象摘要");
        write_memory_entry_sqlite(&request, &sample_entry("对象摘要")).unwrap();
        let result = execute_memory_recall(&request, "对象摘要");
        assert!(result.result_summary.contains("system views + current memory object"));
        assert!(result.final_answer.contains("召回层：system views + current memory object"));
        assert!(result.reasoning_summary.contains("本次召回层为system views + current memory object"));
    }

    #[test]
    fn memory_recall_without_object_hits_mentions_system_view_layer() {
        let request = sample_request("未命中");
        let result = execute_memory_recall(&request, "未命中");
        assert!(result.result_summary.contains("system views"));
        assert!(result.reasoning_summary.contains("本次召回层为system views"));
    }

    fn sample_request(user_input: &str) -> RunRequest {
        let root = std::env::temp_dir().join(format!(
            "memory-executor-{}",
            crate::events::timestamp_now()
        ));
        std::fs::create_dir_all(&root).unwrap();
        RunRequest {
            request_id: "request-test".to_string(),
            run_id: "run-test".to_string(),
            session_id: "session-test".to_string(),
            trace_id: "trace-test".to_string(),
            user_input: user_input.to_string(),
            mode: "standard".to_string(),
            model_ref: ModelRef {
                provider_id: "p".to_string(),
                model_id: "m".to_string(),
                display_name: "model".to_string(),
            },
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-test".to_string(),
                name: "workspace".to_string(),
                root_path: root.display().to_string(),
                is_active: true,
            },
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }

    fn sample_entry(summary: &str) -> MemoryEntry {
        MemoryEntry {
            id: "memory-object".to_string(),
            kind: "project_rule".to_string(),
            title: "rule-object".to_string(),
            summary: summary.to_string(),
            content: format!("content-{summary}"),
            scope: "workspace".to_string(),
            workspace_id: "workspace-test".to_string(),
            session_id: "session-test".to_string(),
            source_run_id: "run-test".to_string(),
            source: "run:run-test".to_string(),
            source_type: "runtime".to_string(),
            source_title: "rule-object".to_string(),
            source_event_type: "run_finished".to_string(),
            source_artifact_path: String::new(),
            governance_version: "v1".to_string(),
            governance_reason: "测试".to_string(),
            governance_source: "test".to_string(),
            governance_at: "1".to_string(),
            archive_reason: String::new(),
            verified: true,
            priority: 12,
            archived: false,
            archived_at: String::new(),
            created_at: "1001".to_string(),
            updated_at: "1001".to_string(),
            timestamp: "1001".to_string(),
        }
    }
}
