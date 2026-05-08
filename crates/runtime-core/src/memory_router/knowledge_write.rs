use super::{MemoryAuditTrail, MemoryWriteOutcome};
use crate::capabilities::ToolExecutionTrace;
use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::knowledge_store::{
    KnowledgeRecord, append_knowledge_record, find_reusable_siyuan_record, has_knowledge_record,
    should_skip_knowledge_record,
};
use crate::memory_schema::MEMORY_GOVERNANCE_VERSION;
use crate::paths::{knowledge_base_file_path, siyuan_auto_write_enabled, siyuan_export_dir, siyuan_sync_enabled};
use crate::text::summarize_text;
use crate::verify::VerificationReport;
use std::fs;

pub(super) fn write_knowledge_record(
    request: &RunRequest,
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> MemoryWriteOutcome {
    let Some(record) = build_knowledge_record(request, trace, report) else {
        return skipped_record_outcome(
            "knowledge_base",
            "document_digest",
            "当前结果不满足知识沉淀条件，未进入知识层。",
        );
    };
    if has_knowledge_record(request, &record) {
        return skipped_record_outcome("knowledge_base", &record.knowledge_type, "命中重复知识条目，跳过写入。");
    }
    if should_skip_recursive_record(&record) {
        return skipped_record_outcome(
            "knowledge_base",
            &record.knowledge_type,
            "检测到知识递归污染风险，跳过写入。",
        );
    }
    append_or_skip_record(request, &record)
}

fn append_or_skip_record(request: &RunRequest, record: &KnowledgeRecord) -> MemoryWriteOutcome {
    if let Some(skip) = should_skip_knowledge_record(record) {
        return skipped_record_outcome("knowledge_base", &record.knowledge_type, &skip.reason);
    }
    match append_knowledge_record(request, record) {
        Ok(()) => knowledge_write_outcome(request, record),
        Err(error) => skipped_record_outcome("knowledge_base", &record.knowledge_type, &error),
    }
}

fn build_knowledge_record(
    request: &RunRequest,
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> Option<KnowledgeRecord> {
    let kind = knowledge_type(trace, report)?;
    Some(KnowledgeRecord {
        id: format!("knowledge-{}", timestamp_now()),
        knowledge_type: kind,
        title: summarize_text(&trace.action_summary),
        summary: knowledge_summary(trace),
        content: summarize_text(&trace.result.final_answer),
        tags: vec![trace.tool.tool_name.clone(), request.workspace_ref.workspace_id.clone()],
        source: format!("run:{}", request.run_id),
        source_type: "runtime".to_string(),
        verified: report.outcome.passed,
        workspace_id: request.workspace_ref.workspace_id.clone(),
        priority: 0,
        archived: false,
        created_at: timestamp_now(),
        updated_at: timestamp_now(),
    })
}

pub(super) fn knowledge_summary(trace: &ToolExecutionTrace) -> String {
    let summary = summarize_text(&trace.result.summary);
    if trace.tool.tool_name == "knowledge_search" || summary.chars().count() >= 20 {
        return summary;
    }
    let fallback = summarize_text(&trace.result.final_answer);
    if fallback.chars().count() >= 20 {
        return fallback;
    }
    summary
}

pub(super) fn knowledge_type(trace: &ToolExecutionTrace, report: &VerificationReport) -> Option<String> {
    if !report.outcome.passed {
        return None;
    }
    match trace.tool.tool_name.as_str() {
        "knowledge_search" => Some("knowledge_recall".to_string()),
        "project_answer" => Some("project_status".to_string()),
        "read_siyuan_note" | "search_siyuan_notes" => Some("user_curated".to_string()),
        "agent_resolve" => Some("workflow_pattern".to_string()),
        _ => None,
    }
}

fn should_skip_recursive_record(record: &KnowledgeRecord) -> bool {
    let summary = record.summary.as_str();
    summary.contains("文件：run:")
        || summary.contains("source\":\"run:")
        || summary.contains("已基于项目文档片段完成一次项目说明回答：文件：run:")
}

fn skipped_record_outcome(layer: &str, record_type: &str, reason: &str) -> MemoryWriteOutcome {
    MemoryWriteOutcome {
        event_type: skipped_event_type(layer),
        layer: layer.to_string(),
        record_type: record_type.to_string(),
        source_type: String::new(),
        title: skipped_title(layer),
        summary: summarize_text(reason),
        reason: reason.to_string(),
        audit: skipped_audit(skipped_event_type(layer), "runtime_skip_guard", reason),
    }
}

fn skipped_event_type(layer: &str) -> &'static str {
    if layer == "knowledge_base" {
        "knowledge_write_skipped"
    } else {
        "memory_write_skipped"
    }
}

fn skipped_title(layer: &str) -> String {
    if layer == "knowledge_base" {
        "跳过知识写入".to_string()
    } else {
        "跳过写入".to_string()
    }
}

fn knowledge_write_outcome(request: &RunRequest, record: &KnowledgeRecord) -> MemoryWriteOutcome {
    let mut summary = format!("知识条目已写入 {}", knowledge_base_file_path(request).display());
    let mut reason = "当前任务形成了稳定且可复用的摘要结果。".to_string();
    if let Some(path) = maybe_export_siyuan(request, record) {
        summary = format!("{summary}；思源已同步 {}", path.display());
        reason = synced_reason(request);
    }
    MemoryWriteOutcome {
        event_type: "knowledge_written",
        layer: "knowledge_base".to_string(),
        record_type: record.knowledge_type.clone(),
        source_type: "runtime".to_string(),
        title: record.title.clone(),
        summary,
        reason,
        audit: knowledge_audit(record),
    }
}

fn synced_reason(request: &RunRequest) -> String {
    if siyuan_sync_enabled(request) {
        return "当前任务形成了稳定摘要，已写入 SQLite 并同步到思源。".to_string();
    }
    "当前任务形成了稳定且可复用的摘要结果。".to_string()
}

fn maybe_export_siyuan(request: &RunRequest, record: &KnowledgeRecord) -> Option<std::path::PathBuf> {
    if !siyuan_auto_write_enabled(request) {
        return None;
    }
    if let Some(path) = reusable_siyuan_path(request, record) {
        return Some(path);
    }
    export_record_to_siyuan(request, record)
}

fn export_record_to_siyuan(request: &RunRequest, record: &KnowledgeRecord) -> Option<std::path::PathBuf> {
    let export_dir = siyuan_export_dir(request)?;
    let path = export_dir.join(format!("{}-{}.md", request.workspace_ref.workspace_id, record.id));
    let content = format!("# {}\n\n{}\n\n{}", record.title, record.summary, record.content);
    let parent = path.parent()?;
    fs::create_dir_all(parent).ok()?;
    fs::write(&path, content).ok()?;
    write_siyuan_index(request, record, &path).ok()?;
    Some(path)
}

fn write_siyuan_index(request: &RunRequest, record: &KnowledgeRecord, path: &std::path::Path) -> Result<(), String> {
    let siyuan_record = KnowledgeRecord {
        id: format!("siyuan-{}", record.id),
        knowledge_type: record.knowledge_type.clone(),
        title: record.title.clone(),
        summary: record.summary.clone(),
        content: summarize_text(&record.content),
        tags: record.tags.clone(),
        source: path.display().to_string(),
        source_type: "siyuan".to_string(),
        verified: record.verified,
        workspace_id: request.workspace_ref.workspace_id.clone(),
        priority: record.priority + 1,
        archived: false,
        created_at: record.created_at.clone(),
        updated_at: timestamp_now(),
    };
    if has_knowledge_record(request, &siyuan_record) {
        return Ok(());
    }
    append_knowledge_record(request, &siyuan_record)
}

fn reusable_siyuan_path(request: &RunRequest, record: &KnowledgeRecord) -> Option<std::path::PathBuf> {
    let current = find_reusable_siyuan_record(request, &record.title, &record.summary)?;
    let path = std::path::PathBuf::from(current.source);
    path.exists().then_some(path)
}

fn skipped_audit(event_type: &str, source: &str, reason: &str) -> MemoryAuditTrail {
    MemoryAuditTrail {
        governance_status: "skipped".to_string(),
        memory_action: "skip".to_string(),
        governance_version: MEMORY_GOVERNANCE_VERSION.to_string(),
        governance_reason: summarize_text(reason),
        governance_source: source.to_string(),
        governance_at: timestamp_now(),
        memory_write_layer: String::new(),
        memory_write_decision: "rejected".to_string(),
        memory_write_reason: summarize_text(reason),
        memory_duplicate_strategy: "none".to_string(),
        source_event_type: event_type.to_string(),
        source_artifact_path: String::new(),
        archive_reason: String::new(),
    }
}

fn knowledge_audit(record: &KnowledgeRecord) -> MemoryAuditTrail {
    MemoryAuditTrail {
        governance_status: "written".to_string(),
        memory_action: "write".to_string(),
        governance_version: MEMORY_GOVERNANCE_VERSION.to_string(),
        governance_reason: "稳定知识摘要已进入知识层并保留来源信息。".to_string(),
        governance_source: "knowledge_base_write".to_string(),
        governance_at: record.updated_at.clone(),
        memory_write_layer: "knowledge_base".to_string(),
        memory_write_decision: "accepted".to_string(),
        memory_write_reason: "稳定知识摘要已进入知识层。".to_string(),
        memory_duplicate_strategy: "none".to_string(),
        source_event_type: "knowledge_written".to_string(),
        source_artifact_path: String::new(),
        archive_reason: String::new(),
    }
}
