use crate::capabilities::ToolExecutionTrace;
use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::knowledge_store::{
    KnowledgeRecord, append_knowledge_record, find_reusable_siyuan_record, has_knowledge_record,
    should_skip_knowledge_record,
};
use crate::memory::{
    MemoryEntry, append_memory_entry, normalized_memory_entry, search_memory_entries,
};
use crate::memory_schema::MEMORY_GOVERNANCE_VERSION;
use crate::paths::{
    knowledge_base_file_path, long_term_memory_file_path, siyuan_auto_write_enabled,
    siyuan_export_dir, siyuan_sync_enabled, working_memory_dir,
};
use crate::text::summarize_text;
use crate::verify::VerificationReport;
use std::fs;

#[derive(Clone, Debug)]
pub(crate) struct MemoryAuditTrail {
    pub governance_status: String,
    pub memory_action: String,
    pub governance_version: String,
    pub governance_reason: String,
    pub governance_source: String,
    pub governance_at: String,
    pub source_event_type: String,
    pub source_artifact_path: String,
    pub archive_reason: String,
}

#[derive(Clone, Debug)]
pub(crate) struct MemoryWriteOutcome {
    pub event_type: &'static str,
    pub layer: &'static str,
    pub record_type: String,
    pub source_type: String,
    pub title: String,
    pub summary: String,
    pub reason: String,
    pub audit: MemoryAuditTrail,
}

pub(crate) fn evaluate_finish_memory_writes(
    request: &RunRequest,
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> Vec<MemoryWriteOutcome> {
    let mut outcomes = Vec::new();
    outcomes.push(working_memory_outcome(request));
    outcomes.push(write_long_term_memory(request, trace, report));
    if let Some(outcome) = write_preference_memory(request, trace, report) {
        outcomes.push(outcome);
    }
    if let Some(outcome) = write_failure_lesson_memory(request, trace, report) {
        outcomes.push(outcome);
    }
    outcomes.push(write_knowledge_record(request, trace, report));
    outcomes
}

fn working_memory_outcome(request: &RunRequest) -> MemoryWriteOutcome {
    MemoryWriteOutcome {
        event_type: "memory_written",
        layer: "working_memory",
        record_type: "session_state".to_string(),
        source_type: "runtime".to_string(),
        title: request.session_id.clone(),
        summary: format!(
            "短期工作记忆已落盘到 {}",
            working_memory_dir(request).display()
        ),
        reason: "当前任务主循环已完成短期状态更新。".to_string(),
        audit: working_memory_audit(),
    }
}

fn write_long_term_memory(
    request: &RunRequest,
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> MemoryWriteOutcome {
    if !report.outcome.passed {
        return skipped_record_outcome(
            "long_term_memory",
            "lesson_learned",
            "验证未通过，跳过长期记忆写入。",
        );
    }
    let entry = auto_memory_entry(request, trace, report);
    if has_memory_duplicate(request, &entry) {
        return skipped_memory_outcome("long_term_memory", &entry, "命中重复长期记忆，跳过写入。");
    }
    memory_write_result(request, &entry, append_memory_entry(request, &entry))
}

fn write_knowledge_record(
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
        return skipped_record_outcome(
            "knowledge_base",
            &record.knowledge_type,
            "命中重复知识条目，跳过写入。",
        );
    }
    if looks_like_recursive_knowledge(&record) {
        return skipped_record_outcome(
            "knowledge_base",
            &record.knowledge_type,
            "检测到知识递归污染风险，跳过写入。",
        );
    }
    if let Some(skip) = should_skip_knowledge_record(&record) {
        return skipped_record_outcome("knowledge_base", &record.knowledge_type, &skip.reason);
    }
    match append_knowledge_record(request, &record) {
        Ok(()) => knowledge_write_outcome(request, &record),
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
        tags: vec![
            trace.tool.tool_name.clone(),
            request.workspace_ref.workspace_id.clone(),
        ],
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

fn knowledge_summary(trace: &ToolExecutionTrace) -> String {
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

fn knowledge_type(trace: &ToolExecutionTrace, report: &VerificationReport) -> Option<String> {
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

fn infer_memory_kind(tool_name: &str) -> String {
    match tool_name {
        "project_answer" => "workspace_summary".to_string(),
        "knowledge_search" => "workflow_pattern".to_string(),
        _ => "lesson_learned".to_string(),
    }
}

fn has_memory_duplicate(request: &RunRequest, entry: &MemoryEntry) -> bool {
    search_memory_entries(request, &entry.summary, 12)
        .into_iter()
        .any(|current| same_memory(&current, entry))
}

fn write_preference_memory(
    request: &RunRequest,
    _trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> Option<MemoryWriteOutcome> {
    if !report.outcome.passed {
        return None;
    }
    let entry = preference_entry(request)?;
    if has_memory_duplicate(request, &entry) {
        return Some(skipped_memory_outcome(
            "long_term_memory",
            &entry,
            "命中重复用户偏好，跳过写入。",
        ));
    }
    Some(match append_memory_entry(request, &entry) {
        Ok(()) => memory_written_outcome(request, &entry, "用户明确给出了可跨任务复用的长期偏好。"),
        Err(error) => skipped_memory_outcome("long_term_memory", &entry, &error),
    })
}

fn write_failure_lesson_memory(
    request: &RunRequest,
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> Option<MemoryWriteOutcome> {
    let entry = failure_lesson_entry(request, trace, report)?;
    if has_memory_duplicate(request, &entry) {
        return Some(skipped_memory_outcome(
            "long_term_memory",
            &entry,
            "命中重复失败教训，跳过写入。",
        ));
    }
    Some(match append_memory_entry(request, &entry) {
        Ok(()) => memory_written_outcome(request, &entry, "失败复盘已形成可复用教训。"),
        Err(error) => skipped_memory_outcome("long_term_memory", &entry, &error),
    })
}

fn preference_entry(request: &RunRequest) -> Option<MemoryEntry> {
    let kind = preference_kind(&request.user_input)?;
    let summary = preference_summary(&request.user_input)?;
    let now = timestamp_now();
    Some(MemoryEntry {
        id: format!("memory-preference-{}", timestamp_now()),
        kind,
        title: summary.clone(),
        summary,
        content: summarize_text(&request.user_input),
        scope: request.workspace_ref.name.clone(),
        workspace_id: request.workspace_ref.workspace_id.clone(),
        session_id: request.session_id.clone(),
        source_run_id: request.run_id.clone(),
        source: format!("run:{}", request.run_id),
        source_type: "runtime".to_string(),
        source_title: summarize_text(&request.user_input),
        source_event_type: "run_finished".to_string(),
        source_artifact_path: String::new(),
        governance_version: String::new(),
        governance_reason: String::new(),
        governance_source: String::new(),
        governance_at: String::new(),
        archive_reason: String::new(),
        verified: true,
        priority: 80,
        archived: false,
        archived_at: String::new(),
        created_at: now.clone(),
        updated_at: now.clone(),
        timestamp: now,
    })
}

fn failure_lesson_entry(
    request: &RunRequest,
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> Option<MemoryEntry> {
    let summary = failure_lesson_summary(trace, report)?;
    let now = timestamp_now();
    let source = failure_lesson_source(trace);
    Some(MemoryEntry {
        id: format!("memory-lesson-{}", timestamp_now()),
        kind: "lesson_learned".to_string(),
        title: summary.clone(),
        summary,
        content: failure_lesson_content(trace, report),
        scope: request.workspace_ref.name.clone(),
        workspace_id: request.workspace_ref.workspace_id.clone(),
        session_id: request.session_id.clone(),
        source_run_id: request.run_id.clone(),
        source: format!("run:{}", request.run_id),
        source_type: "runtime".to_string(),
        source_title: source.title,
        source_event_type: source.event_type,
        source_artifact_path: source.artifact_path,
        governance_version: String::new(),
        governance_reason: String::new(),
        governance_source: String::new(),
        governance_at: String::new(),
        archive_reason: String::new(),
        verified: report.outcome.passed,
        priority: 60,
        archived: false,
        archived_at: String::new(),
        created_at: now.clone(),
        updated_at: now.clone(),
        timestamp: now,
    })
}

fn auto_memory_entry(
    request: &RunRequest,
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> MemoryEntry {
    let now = timestamp_now();
    let source = memory_source(trace, "verification_completed");
    MemoryEntry {
        id: format!("memory-auto-{}", timestamp_now()),
        kind: infer_memory_kind(&trace.tool.tool_name),
        title: summarize_text(&trace.action_summary),
        summary: summarize_text(&trace.result.summary),
        content: summarize_text(&trace.result.final_answer),
        scope: request.workspace_ref.name.clone(),
        workspace_id: request.workspace_ref.workspace_id.clone(),
        session_id: request.session_id.clone(),
        source_run_id: request.run_id.clone(),
        source: format!("run:{}", request.run_id),
        source_type: "runtime".to_string(),
        source_title: source.title,
        source_event_type: source.event_type,
        source_artifact_path: source.artifact_path,
        governance_version: String::new(),
        governance_reason: String::new(),
        governance_source: String::new(),
        governance_at: String::new(),
        archive_reason: String::new(),
        verified: report.outcome.passed,
        priority: 0,
        archived: false,
        archived_at: String::new(),
        created_at: now.clone(),
        updated_at: now.clone(),
        timestamp: now,
    }
}

fn preference_summary(user_input: &str) -> Option<String> {
    let mut parts = Vec::new();
    push_preference(
        &mut parts,
        user_input,
        &["用中文回答", "中文回答"],
        "用中文回答",
    );
    push_preference(
        &mut parts,
        user_input,
        &["简明扼要", "精简", "不要废话", "简洁直接"],
        "回答简洁直接",
    );
    push_preference(
        &mut parts,
        user_input,
        &["不要在回答末尾总结"],
        "回答末尾不要总结已完成事项",
    );
    push_preference(
        &mut parts,
        user_input,
        &["函数不超过 30 行", "函数不超过30行"],
        "新增或修改函数不超过30行",
    );
    push_preference(
        &mut parts,
        user_input,
        &["不要添加注释", "不要给未修改", "未修改的代码行添加注释"],
        "不要给未修改代码加注释",
    );
    push_preference(
        &mut parts,
        user_input,
        &["按文档要求", "按照文档要求", "必须遵守开发任务书"],
        "严格按文档要求执行",
    );
    push_preference(
        &mut parts,
        user_input,
        &["先验收", "逐项填写", "逐项勾验"],
        "优先按验收清单逐项校验",
    );
    push_preference(
        &mut parts,
        user_input,
        &["每次尽量多做一点"],
        "单轮尽量推进更多有效工作",
    );
    push_preference(
        &mut parts,
        user_input,
        &["不要改太多", "最小改动", "简单点"],
        "优先最小改动实现",
    );
    (!parts.is_empty()).then_some(format!("用户偏好：{}", parts.join("；")))
}

fn failure_lesson_summary(
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> Option<String> {
    if report.outcome.code == "verified_with_recovery" {
        return Some(format!(
            "失败教训：{} 遇到异常时应执行单次受控恢复并保留恢复留痕。",
            trace.tool.display_name
        ));
    }
    failure_lesson_reason(trace, report)
        .map(|reason| format!("失败教训：{} {}", trace.tool.display_name, reason))
}

fn failure_lesson_content(trace: &ToolExecutionTrace, report: &VerificationReport) -> String {
    format!(
        "工具={}；验证={}; 错误码={}; 结果摘要={}; 最终答复={}; artifact_path={}",
        trace.tool.tool_name,
        report.outcome.summary,
        trace.result.error_code.as_deref().unwrap_or(""),
        summarize_text(&trace.result.summary),
        summarize_text(&trace.result.final_answer),
        trace.result.artifact_path.as_deref().unwrap_or(""),
    )
}

fn preference_kind(user_input: &str) -> Option<String> {
    let workflow = [
        "按文档要求",
        "按照文档要求",
        "先验收",
        "逐项填写",
        "逐项勾验",
        "每次尽量多做一点",
    ];
    workflow
        .iter()
        .any(|keyword| user_input.contains(keyword))
        .then_some("workflow_preference".to_string())
        .or_else(|| preference_summary(user_input).map(|_| "preference".to_string()))
}

fn push_preference(
    parts: &mut Vec<&'static str>,
    user_input: &str,
    keywords: &[&str],
    label: &'static str,
) {
    if keywords.iter().any(|keyword| user_input.contains(keyword)) && !parts.contains(&label) {
        parts.push(label);
    }
}

fn failure_lesson_reason(
    trace: &ToolExecutionTrace,
    _report: &VerificationReport,
) -> Option<&'static str> {
    if trace.result.success {
        return None;
    }
    let error = trace.result.error_code.as_deref().unwrap_or("");
    let text = format!("{} {}", trace.result.summary, trace.result.final_answer);
    Some(match () {
        _ if error.contains("provider_not_configured") || text.contains("provider 未配置") => {
            "provider 未配置时应直接收口为配置错误，并阻止继续生成无效回答。"
        }
        _ if text.contains("risk_confirmation_required") || text.contains("需要风险确认") => {
            "命中风险确认时应暂停执行并等待用户确认，不应继续推进主动作。"
        }
        _ if text.contains("runtime-host") && text.contains("占用") => {
            "runtime-host 被占用时应先释放进程再重试构建或启动。"
        }
        _ if text.contains("连接被拒绝")
            || text.contains("connection refused")
            || text.contains("127.0.0.1:8898") =>
        {
            "运行时不可达时应先恢复服务再继续主链路。"
        }
        _ => "失败时应保留错误摘要并停止错误沉淀。",
    })
}

fn memory_written_outcome(
    request: &RunRequest,
    entry: &MemoryEntry,
    reason: &str,
) -> MemoryWriteOutcome {
    let entry = normalized_memory_entry(entry);
    MemoryWriteOutcome {
        event_type: "memory_written",
        layer: "long_term_memory",
        record_type: entry.kind.clone(),
        source_type: entry.source_type.clone(),
        title: entry.summary.clone(),
        summary: format!(
            "长期记忆已写入 {}",
            long_term_memory_file_path(request).display()
        ),
        reason: reason.to_string(),
        audit: written_audit(&entry),
    }
}

fn same_memory(current: &MemoryEntry, target: &MemoryEntry) -> bool {
    current.workspace_id == target.workspace_id
        && current.kind == target.kind
        && current.title == target.title
        && current.summary == target.summary
}

fn looks_like_recursive_knowledge(record: &KnowledgeRecord) -> bool {
    let summary = record.summary.as_str();
    summary.contains("文件：run:")
        || summary.contains("source\":\"run:")
        || summary.contains("已基于项目文档片段完成一次项目说明回答：文件：run:")
}

fn skipped_record_outcome(
    layer: &'static str,
    record_type: &str,
    reason: &str,
) -> MemoryWriteOutcome {
    MemoryWriteOutcome {
        event_type: skipped_event_type(layer),
        layer,
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
    let mut summary = format!(
        "知识条目已写入 {}",
        knowledge_base_file_path(request).display()
    );
    let mut reason = "当前任务形成了稳定且可复用的摘要结果。".to_string();
    if let Some(path) = maybe_export_siyuan(request, record) {
        summary = format!("{summary}；思源已同步 {}", path.display());
        if siyuan_sync_enabled(request) {
            reason = "当前任务形成了稳定摘要，已写入 SQLite 并同步到思源。".to_string();
        }
    }
    MemoryWriteOutcome {
        event_type: "knowledge_written",
        layer: "knowledge_base",
        record_type: record.knowledge_type.clone(),
        source_type: "runtime".to_string(),
        title: record.title.clone(),
        summary,
        reason,
        audit: knowledge_audit(record),
    }
}

fn maybe_export_siyuan(
    request: &RunRequest,
    record: &KnowledgeRecord,
) -> Option<std::path::PathBuf> {
    if !siyuan_auto_write_enabled(request) {
        return None;
    }
    if let Some(path) = reusable_siyuan_path(request, record) {
        return Some(path);
    }
    let export_dir = siyuan_export_dir(request)?;
    let path = export_dir.join(format!(
        "{}-{}.md",
        request.workspace_ref.workspace_id, record.id
    ));
    let content = format!(
        "# {}\n\n{}\n\n{}",
        record.title, record.summary, record.content
    );
    let parent = path.parent()?;
    fs::create_dir_all(parent).ok()?;
    fs::write(&path, content).ok()?;
    write_siyuan_index(request, record, &path).ok()?;
    Some(path)
}

fn write_siyuan_index(
    request: &RunRequest,
    record: &KnowledgeRecord,
    path: &std::path::Path,
) -> Result<(), String> {
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

fn reusable_siyuan_path(
    request: &RunRequest,
    record: &KnowledgeRecord,
) -> Option<std::path::PathBuf> {
    let current = find_reusable_siyuan_record(request, &record.title, &record.summary)?;
    let path = std::path::PathBuf::from(current.source);
    path.exists().then_some(path)
}

struct MemorySource {
    title: String,
    event_type: String,
    artifact_path: String,
}

fn memory_source(trace: &ToolExecutionTrace, event_type: &str) -> MemorySource {
    MemorySource {
        title: memory_source_title(trace),
        event_type: event_type.to_string(),
        artifact_path: trace.result.artifact_path.clone().unwrap_or_default(),
    }
}

fn failure_lesson_source(trace: &ToolExecutionTrace) -> MemorySource {
    memory_source(trace, "run_failed")
}

fn memory_source_title(trace: &ToolExecutionTrace) -> String {
    let task_title = summarize_text(&trace.action_summary);
    let summary = summarize_text(&trace.result.summary);
    first_text(&task_title, "", &summary)
}

fn first_text(primary: &str, secondary: &str, fallback: &str) -> String {
    if !primary.trim().is_empty() {
        return primary.to_string();
    }
    if !secondary.trim().is_empty() {
        return secondary.to_string();
    }
    fallback.to_string()
}

fn memory_write_result(
    request: &RunRequest,
    entry: &MemoryEntry,
    result: Result<(), String>,
) -> MemoryWriteOutcome {
    match result {
        Ok(()) => MemoryWriteOutcome {
            event_type: "memory_written",
            layer: "long_term_memory",
            record_type: entry.kind.clone(),
            source_type: entry.source_type.clone(),
            title: entry.summary.clone(),
            summary: format!(
                "长期记忆已写入 {}",
                long_term_memory_file_path(request).display()
            ),
            reason: "任务完成后形成了可复用摘要。".to_string(),
            audit: written_audit(&normalized_memory_entry(entry)),
        },
        Err(error) => skipped_memory_outcome("long_term_memory", entry, &error),
    }
}

fn skipped_memory_outcome(
    layer: &'static str,
    entry: &MemoryEntry,
    reason: &str,
) -> MemoryWriteOutcome {
    let entry = normalized_memory_entry(entry);
    MemoryWriteOutcome {
        event_type: skipped_event_type(layer),
        layer,
        record_type: entry.kind.clone(),
        source_type: entry.source_type.clone(),
        title: skipped_title(layer),
        summary: summarize_text(reason),
        reason: reason.to_string(),
        audit: skipped_entry_audit(&entry, skipped_event_type(layer), reason),
    }
}

fn working_memory_audit() -> MemoryAuditTrail {
    MemoryAuditTrail {
        governance_status: "written".to_string(),
        memory_action: "write".to_string(),
        governance_version: MEMORY_GOVERNANCE_VERSION.to_string(),
        governance_reason: "短期工作记忆已按当前运行时状态同步落盘。".to_string(),
        governance_source: "runtime_working_memory".to_string(),
        governance_at: timestamp_now(),
        source_event_type: "memory_written".to_string(),
        source_artifact_path: String::new(),
        archive_reason: String::new(),
    }
}

fn written_audit(entry: &MemoryEntry) -> MemoryAuditTrail {
    MemoryAuditTrail {
        governance_status: "written".to_string(),
        memory_action: "write".to_string(),
        governance_version: entry.governance_version.clone(),
        governance_reason: entry.governance_reason.clone(),
        governance_source: entry.governance_source.clone(),
        governance_at: entry.governance_at.clone(),
        source_event_type: entry.source_event_type.clone(),
        source_artifact_path: entry.source_artifact_path.clone(),
        archive_reason: entry.archive_reason.clone(),
    }
}

fn skipped_audit(event_type: &str, source: &str, reason: &str) -> MemoryAuditTrail {
    MemoryAuditTrail {
        governance_status: "skipped".to_string(),
        memory_action: "skip".to_string(),
        governance_version: MEMORY_GOVERNANCE_VERSION.to_string(),
        governance_reason: summarize_text(reason),
        governance_source: source.to_string(),
        governance_at: timestamp_now(),
        source_event_type: event_type.to_string(),
        source_artifact_path: String::new(),
        archive_reason: String::new(),
    }
}

fn skipped_entry_audit(entry: &MemoryEntry, event_type: &str, reason: &str) -> MemoryAuditTrail {
    let source = if entry.governance_source.is_empty() {
        "runtime_skip_guard"
    } else {
        entry.governance_source.as_str()
    };
    let mut audit = skipped_audit(event_type, source, reason);
    audit.source_artifact_path = entry.source_artifact_path.clone();
    audit
}

fn knowledge_audit(record: &KnowledgeRecord) -> MemoryAuditTrail {
    MemoryAuditTrail {
        governance_status: "written".to_string(),
        memory_action: "write".to_string(),
        governance_version: MEMORY_GOVERNANCE_VERSION.to_string(),
        governance_reason: "稳定知识摘要已进入知识层并保留来源信息。".to_string(),
        governance_source: "knowledge_base_write".to_string(),
        governance_at: record.updated_at.clone(),
        source_event_type: "knowledge_written".to_string(),
        source_artifact_path: String::new(),
        archive_reason: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{knowledge_summary, knowledge_type};
    use crate::capabilities::{ToolDefinition, ToolExecutionTrace};
    use crate::verify::{VerificationOutcome, VerificationReport};

    #[test]
    fn knowledge_type_accepts_agent_resolve_when_verified() {
        let trace = sample_trace("agent_resolve", "任务已完成", "可复用流程说明");
        let report = sample_report(true);
        assert_eq!(
            knowledge_type(&trace, &report),
            Some("workflow_pattern".to_string())
        );
    }

    #[test]
    fn knowledge_summary_falls_back_to_final_answer_when_short() {
        let trace = sample_trace(
            "agent_resolve",
            "完成",
            "这是一段可复用的较长知识摘要文本，用于验证回退策略有效。",
        );
        assert_eq!(
            knowledge_summary(&trace),
            "这是一段可复用的较长知识摘要文本，用于验证回退策略有效。"
        );
    }

    fn sample_trace(tool_name: &str, summary: &str, final_answer: &str) -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: sample_tool(tool_name),
            action_summary: "测试动作".to_string(),
            result: sample_result(summary, final_answer),
        }
    }

    fn sample_tool(tool_name: &str) -> ToolDefinition {
        ToolDefinition {
            tool_name: tool_name.to_string(),
            display_name: "测试工具".to_string(),
            category: "agent".to_string(),
            risk_level: "low".to_string(),
            input_schema: "none".to_string(),
            output_kind: "text_preview".to_string(),
            requires_confirmation: false,
        }
    }

    fn sample_result(summary: &str, final_answer: &str) -> crate::capabilities::ToolCallResult {
        crate::capabilities::ToolCallResult {
            summary: summary.to_string(),
            final_answer: final_answer.to_string(),
            artifact_path: None,
            error_code: None,
            elapsed_ms: 10,
            retryable: false,
            success: true,
            memory_write_summary: None,
            reasoning_summary: "测试推理".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: String::new(),
        }
    }

    fn sample_report(passed: bool) -> VerificationReport {
        VerificationReport {
            outcome: VerificationOutcome {
                passed,
                code: "verified".to_string(),
                policy: "check_result_summary".to_string(),
                evidence: vec![],
                summary: "验证".to_string(),
                next_step: "继续".to_string(),
            },
            tool_elapsed_ms: 10,
        }
    }
}
