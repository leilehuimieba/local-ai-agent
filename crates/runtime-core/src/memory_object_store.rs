use crate::contracts::RunRequest;
use crate::memory::MemoryEntry;
use crate::sqlite_store::{
    list_memory_object_aliases_sqlite, list_memory_object_versions_sqlite,
    rollback_memory_object_sqlite, sync_memory_object_entry_sqlite,
};
use crate::text::summarize_text;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MemoryObjectVersion {
    pub object_id: String,
    pub version_id: String,
    pub canonical_uri: String,
    pub summary: String,
    pub content: String,
    pub is_current: bool,
    pub restored_from_version_id: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct MemoryObjectDiff {
    pub object_id: String,
    pub from_version_id: String,
    pub to_version_id: String,
    pub summary_changed: bool,
    pub content_changed: bool,
    pub from_summary: String,
    pub to_summary: String,
    pub from_content_excerpt: String,
    pub to_content_excerpt: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct MemoryObjectRollbackResult {
    pub object_id: String,
    pub target_version_id: String,
    pub restored_version_id: String,
    pub canonical_uri: String,
}

#[allow(dead_code)]
pub(crate) fn sync_memory_object_entry(
    request: &RunRequest,
    entry: &MemoryEntry,
) -> Result<MemoryObjectVersion, String> {
    sync_memory_object_entry_sqlite(request, entry)
}

#[allow(dead_code)]
pub(crate) fn list_memory_object_versions(
    request: &RunRequest,
    object_id: &str,
) -> Vec<MemoryObjectVersion> {
    list_memory_object_versions_sqlite(request, object_id)
}

#[allow(dead_code)]
pub(crate) fn get_memory_object_history(
    request: &RunRequest,
    object_id: &str,
) -> Vec<MemoryObjectVersion> {
    list_memory_object_versions(request, object_id)
}

#[allow(dead_code)]
pub(crate) fn list_memory_object_aliases(request: &RunRequest, object_id: &str) -> Vec<String> {
    list_memory_object_aliases_sqlite(request, object_id)
}

#[allow(dead_code)]
pub(crate) fn diff_memory_object_versions(
    request: &RunRequest,
    object_id: &str,
    from_version_id: &str,
    to_version_id: &str,
) -> Result<MemoryObjectDiff, String> {
    let versions = list_memory_object_versions(request, object_id);
    let from = find_version(&versions, from_version_id)?;
    let to = find_version(&versions, to_version_id)?;
    Ok(build_version_diff(object_id, from, to))
}

#[allow(dead_code)]
pub(crate) fn rollback_memory_object(
    request: &RunRequest,
    object_id: &str,
    target_version_id: &str,
) -> Result<MemoryObjectRollbackResult, String> {
    rollback_memory_object_sqlite(request, object_id, target_version_id)
}

fn find_version<'a>(
    versions: &'a [MemoryObjectVersion],
    version_id: &str,
) -> Result<&'a MemoryObjectVersion, String> {
    versions
        .iter()
        .find(|item| item.version_id == version_id)
        .ok_or_else(|| format!("未找到 memory object 版本：{version_id}"))
}

fn build_version_diff(
    object_id: &str,
    from: &MemoryObjectVersion,
    to: &MemoryObjectVersion,
) -> MemoryObjectDiff {
    MemoryObjectDiff {
        object_id: object_id.to_string(),
        from_version_id: from.version_id.clone(),
        to_version_id: to.version_id.clone(),
        summary_changed: from.summary != to.summary,
        content_changed: from.content != to.content,
        from_summary: from.summary.clone(),
        to_summary: to.summary.clone(),
        from_content_excerpt: summarize_text(&from.content),
        to_content_excerpt: summarize_text(&to.content),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use crate::memory::MemoryEntry;
    use crate::sqlite_store::list_memory_entries_sqlite;
    use std::collections::BTreeMap;

    #[test]
    fn sync_versions_share_object_and_update_current() {
        let request = sample_request();
        let first = sync_memory_object_entry(&request, &sample_entry("v1", "第一版")).unwrap();
        let second = sync_memory_object_entry(&request, &sample_entry("v2", "第二版")).unwrap();
        let versions = list_memory_object_versions(&request, &first.object_id);
        assert_eq!(first.object_id, second.object_id);
        assert_eq!(versions.len(), 2);
        assert!(
            versions
                .iter()
                .any(|item| item.version_id == second.version_id && item.is_current)
        );
    }

    #[test]
    fn sync_creates_alias_for_object() {
        let request = sample_request();
        let version = sync_memory_object_entry(&request, &sample_entry("v3", "规则版")).unwrap();
        let aliases = list_memory_object_aliases(&request, &version.object_id);
        assert!(aliases.iter().any(|item| item == &version.canonical_uri));
    }

    #[test]
    fn history_lists_versions_newest_first() {
        let request = sample_request();
        let first = sync_memory_object_entry(&request, &sample_entry("v1", "第一版")).unwrap();
        let second = sync_memory_object_entry(&request, &sample_entry("v2", "第二版")).unwrap();
        let history = get_memory_object_history(&request, &first.object_id);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].version_id, second.version_id);
        assert!(history[0].is_current);
        assert_eq!(history[1].version_id, first.version_id);
    }

    #[test]
    fn diff_reports_summary_and_content_changes() {
        let request = sample_request();
        let first = sync_memory_object_entry(&request, &sample_entry("v1", "第一版")).unwrap();
        let second = sync_memory_object_entry(&request, &sample_entry("v2", "第二版")).unwrap();
        let diff = diff_memory_object_versions(
            &request,
            &first.object_id,
            &first.version_id,
            &second.version_id,
        )
        .unwrap();
        assert!(diff.summary_changed);
        assert!(diff.content_changed);
        assert!(diff.to_content_excerpt.contains("第二版"));
    }

    #[test]
    fn rollback_creates_restored_current_version() {
        let request = sample_request();
        let first = sync_memory_object_entry(&request, &sample_entry("v1", "第一版")).unwrap();
        sync_memory_object_entry(&request, &sample_entry("v2", "第二版")).unwrap();
        let result = rollback_memory_object(&request, &first.object_id, &first.version_id).unwrap();
        let history = get_memory_object_history(&request, &first.object_id);
        assert_eq!(history.len(), 3);
        assert_eq!(history[0].version_id, result.restored_version_id);
        assert!(history[0].is_current);
        assert_eq!(history[0].summary, first.summary);
        assert_eq!(history[0].restored_from_version_id, first.version_id);
    }

    #[test]
    fn rollback_keeps_aliases_and_restores_legacy_recall() {
        let request = sample_request();
        let first = sync_memory_object_entry(&request, &sample_entry("v1", "第一版")).unwrap();
        sync_memory_object_entry(&request, &sample_entry("v2", "第二版")).unwrap();
        let before = list_memory_object_aliases(&request, &first.object_id);
        rollback_memory_object(&request, &first.object_id, &first.version_id).unwrap();
        let after = list_memory_object_aliases(&request, &first.object_id);
        let recalled = list_memory_entries_sqlite(&request);
        let restored = recalled
            .into_iter()
            .find(|item| item.source_type == "governed_rollback")
            .unwrap();
        assert_eq!(before, after);
        assert_eq!(restored.summary, "第一版");
    }

    fn sample_request() -> RunRequest {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "memory-object-store-{}-{}",
            COUNTER.fetch_add(1, Ordering::SeqCst),
            crate::events::timestamp_now()
        ));
        std::fs::create_dir_all(&root).unwrap();
        RunRequest {
            request_id: "request-test".to_string(),
            run_id: "run-test".to_string(),
            session_id: "session-test".to_string(),
            trace_id: "trace-test".to_string(),
            user_input: "测试".to_string(),
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

    fn sample_entry(id: &str, summary: &str) -> MemoryEntry {
        let order = id.trim_start_matches('v').parse::<u32>().unwrap_or(0);
        let timestamp = format!("100{order}");
        MemoryEntry {
            id: id.to_string(),
            kind: "project_rule".to_string(),
            title: "规则对象".to_string(),
            summary: summary.to_string(),
            content: format!("内容-{summary}"),
            scope: "workspace".to_string(),
            workspace_id: "workspace-test".to_string(),
            session_id: "session-test".to_string(),
            source_run_id: "run-test".to_string(),
            source: "run:run-test".to_string(),
            source_type: "runtime".to_string(),
            source_title: "规则对象".to_string(),
            source_event_type: "run_finished".to_string(),
            source_artifact_path: String::new(),
            governance_version: "v1".to_string(),
            governance_reason: "测试".to_string(),
            governance_source: "test".to_string(),
            governance_at: "1".to_string(),
            archive_reason: String::new(),
            verified: true,
            priority: 10,
            archived: false,
            archived_at: String::new(),
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
            timestamp,
        }
    }
}
