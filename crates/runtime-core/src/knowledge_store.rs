use crate::paths::knowledge_base_file_path;
use crate::sqlite_store::{list_knowledge_records_sqlite, write_knowledge_record_sqlite};
use crate::storage::{append_jsonl, read_jsonl};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct KnowledgeRecord {
    pub id: String,
    pub knowledge_type: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,
    pub source: String,
    #[serde(default)]
    pub source_type: String,
    pub verified: bool,
    pub workspace_id: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub archived: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub(crate) struct KnowledgeSkip {
    pub reason: String,
}

pub(crate) fn append_knowledge_record(
    request: &crate::contracts::RunRequest,
    record: &KnowledgeRecord,
) -> Result<(), String> {
    if let Some(skip) = should_skip_knowledge_record(record) {
        return Err(skip.reason);
    }
    write_knowledge_record_sqlite(request, record)?;
    append_jsonl(knowledge_base_file_path(request), record)
}

pub(crate) fn search_knowledge_records(
    request: &crate::contracts::RunRequest,
) -> Vec<KnowledgeRecord> {
    let items = list_knowledge_records_sqlite(request);
    if items.is_empty() {
        return read_jsonl::<KnowledgeRecord>(&knowledge_base_file_path(request));
    }
    items
}

pub(crate) fn has_knowledge_record(
    request: &crate::contracts::RunRequest,
    record: &KnowledgeRecord,
) -> bool {
    search_knowledge_records(request)
        .into_iter()
        .any(|current| same_record(&current, record))
}

pub(crate) fn find_reusable_siyuan_record(
    request: &crate::contracts::RunRequest,
    title: &str,
    summary: &str,
) -> Option<KnowledgeRecord> {
    search_knowledge_records(request)
        .into_iter()
        .filter(|record| record.source_type == "siyuan")
        .find(|record| same_siyuan_content(record, title, summary))
}

fn same_record(current: &KnowledgeRecord, target: &KnowledgeRecord) -> bool {
    same_identity(current, target) || same_summary(current, target)
}

fn same_identity(current: &KnowledgeRecord, target: &KnowledgeRecord) -> bool {
    current.workspace_id == target.workspace_id
        && current.title == target.title
        && current.source == target.source
        && current.source_type == target.source_type
}

fn same_summary(current: &KnowledgeRecord, target: &KnowledgeRecord) -> bool {
    current.workspace_id == target.workspace_id
        && current.title == target.title
        && current.summary == target.summary
        && current.source_type == target.source_type
}

fn same_siyuan_content(record: &KnowledgeRecord, title: &str, summary: &str) -> bool {
    record.title == title && record.summary == summary
}

pub(crate) fn should_skip_knowledge_record(record: &KnowledgeRecord) -> Option<KnowledgeSkip> {
    let runtime_generated = record.source_type == "runtime" && record.source.starts_with("run:");
    let project_answer = record.title.contains("项目说明")
        || record
            .summary
            .contains("已基于项目文档片段完成一次项目说明回答");
    if runtime_generated && project_answer {
        return Some(KnowledgeSkip {
            reason: "命中低价值运行时知识治理规则：项目说明回显不进入知识层。".to_string(),
        });
    }
    if record.summary.chars().count() < 20 {
        return Some(KnowledgeSkip {
            reason: "命中低价值知识拦截：摘要过短，缺少跨任务复用价值。".to_string(),
        });
    }
    if !record.verified {
        return Some(KnowledgeSkip {
            reason: "命中低价值知识拦截：当前结果未验证通过。".to_string(),
        });
    }
    None
}
