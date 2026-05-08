mod audit;
mod knowledge_write;
mod read_route;
mod write_policy;

pub(crate) use read_route::{MemoryRouteSelection, select_memory_route};

use crate::capabilities::ToolExecutionTrace;
use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::memory::{MemoryEntry, append_memory_entry, normalized_memory_entry, search_memory_entries};
use crate::paths::{long_term_memory_file_path, working_memory_dir};
use crate::text::summarize_text;
use crate::verify::VerificationReport;

use self::audit::{skipped_entry_audit, working_memory_audit, written_audit};
use self::knowledge_write::write_knowledge_record;
use self::write_policy::{
    append_error_outcome, auto_memory_layer, auto_memory_reason, duplicate_entry_outcome, govern_entry,
    reject_entry_outcome,
};

#[derive(Clone, Debug)]
pub(crate) struct MemoryAuditTrail {
    pub governance_status: String,
    pub memory_action: String,
    pub governance_version: String,
    pub governance_reason: String,
    pub governance_source: String,
    pub governance_at: String,
    pub memory_write_layer: String,
    pub memory_write_decision: String,
    pub memory_write_reason: String,
    pub memory_duplicate_strategy: String,
    pub source_event_type: String,
    pub source_artifact_path: String,
    pub archive_reason: String,
}

#[derive(Clone, Debug)]
pub(crate) struct MemoryWriteOutcome {
    pub event_type: &'static str,
    pub layer: String,
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
        layer: "working_only".to_string(),
        record_type: "session_state".to_string(),
        source_type: "runtime".to_string(),
        title: request.session_id.clone(),
        summary: format!("短期工作记忆已落盘到 {}", working_memory_dir(request).display()),
        reason: "当前任务主循环已完成短期状态更新。".to_string(),
        audit: working_memory_audit(),
    }
}

fn write_long_term_memory(
    request: &RunRequest,
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> MemoryWriteOutcome {
    let mut entry = auto_memory_entry(request, trace, report);
    if !report.outcome.passed {
        return reject_entry_outcome(&mut entry, "working_only", "验证未通过，自动写回退回 working_only。");
    }
    let layer = auto_memory_layer(&entry);
    if layer == "working_only" {
        return reject_entry_outcome(&mut entry, layer, "当前结果更像一次性项目状态回显，不进入长期层。");
    }
    govern_entry(&mut entry, layer, "accepted", auto_memory_reason(layer), "none");
    if has_memory_duplicate(request, &entry) {
        return duplicate_entry_outcome(&mut entry, "命中重复长期记忆，跳过写入。");
    }
    memory_write_result(request, &entry, append_memory_entry(request, &entry))
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
    let mut entry = preference_entry(request)?;
    govern_entry(
        &mut entry,
        "semantic_or_procedural_memory",
        "accepted",
        "用户偏好具备跨任务复用价值，进入 semantic_or_procedural_memory。",
        "none",
    );
    if has_memory_duplicate(request, &entry) {
        return Some(duplicate_entry_outcome(&mut entry, "命中重复用户偏好，跳过写入。"));
    }
    Some(match append_memory_entry(request, &entry) {
        Ok(()) => memory_written_outcome(request, &entry, "用户明确给出了可跨任务复用的长期偏好。"),
        Err(error) => append_error_outcome(&entry, &error),
    })
}

fn write_failure_lesson_memory(
    request: &RunRequest,
    trace: &ToolExecutionTrace,
    report: &VerificationReport,
) -> Option<MemoryWriteOutcome> {
    let mut entry = failure_lesson_entry(request, trace, report)?;
    govern_entry(
        &mut entry,
        "episodic_memory",
        "accepted",
        "失败复盘按过程经验进入 episodic_memory。",
        "none",
    );
    if has_memory_duplicate(request, &entry) {
        return Some(duplicate_entry_outcome(&mut entry, "命中重复失败教训，跳过写入。"));
    }
    Some(match append_memory_entry(request, &entry) {
        Ok(()) => memory_written_outcome(request, &entry, "失败复盘已形成可复用教训。"),
        Err(error) => append_error_outcome(&entry, &error),
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
        memory_write_layer: String::new(),
        memory_write_decision: String::new(),
        memory_write_reason: String::new(),
        memory_duplicate_strategy: String::new(),
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
        memory_write_layer: String::new(),
        memory_write_decision: String::new(),
        memory_write_reason: String::new(),
        memory_duplicate_strategy: String::new(),
        verified: report.outcome.passed,
        priority: 60,
        archived: false,
        archived_at: String::new(),
        created_at: now.clone(),
        updated_at: now.clone(),
        timestamp: now,
    })
}

fn auto_memory_entry(request: &RunRequest, trace: &ToolExecutionTrace, report: &VerificationReport) -> MemoryEntry {
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
        memory_write_layer: String::new(),
        memory_write_decision: String::new(),
        memory_write_reason: String::new(),
        memory_duplicate_strategy: String::new(),
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
    push_preference(&mut parts, user_input, &["用中文回答", "中文回答"], "用中文回答");
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

fn failure_lesson_summary(trace: &ToolExecutionTrace, report: &VerificationReport) -> Option<String> {
    if report.outcome.code == "verified_with_recovery" {
        return Some(format!(
            "失败教训：{} 遇到异常时应执行单次受控恢复并保留恢复留痕。",
            trace.tool.display_name
        ));
    }
    failure_lesson_reason(trace, report).map(|reason| format!("失败教训：{} {}", trace.tool.display_name, reason))
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

fn push_preference(parts: &mut Vec<&'static str>, user_input: &str, keywords: &[&str], label: &'static str) {
    if keywords.iter().any(|keyword| user_input.contains(keyword)) && !parts.contains(&label) {
        parts.push(label);
    }
}

fn failure_lesson_reason(trace: &ToolExecutionTrace, _report: &VerificationReport) -> Option<&'static str> {
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
        _ if text.contains("连接被拒绝") || text.contains("connection refused") || text.contains("127.0.0.1:8898") => {
            "运行时不可达时应先恢复服务再继续主链路。"
        }
        _ => "失败时应保留错误摘要并停止错误沉淀。",
    })
}

fn memory_written_outcome(request: &RunRequest, entry: &MemoryEntry, reason: &str) -> MemoryWriteOutcome {
    let entry = normalized_memory_entry(entry);
    MemoryWriteOutcome {
        event_type: "memory_written",
        layer: entry.memory_write_layer.clone(),
        record_type: entry.kind.clone(),
        source_type: entry.source_type.clone(),
        title: entry.summary.clone(),
        summary: format!("长期记忆已写入 {}", long_term_memory_file_path(request).display()),
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

fn memory_write_result(request: &RunRequest, entry: &MemoryEntry, result: Result<(), String>) -> MemoryWriteOutcome {
    match result {
        Ok(()) => MemoryWriteOutcome {
            event_type: "memory_written",
            layer: normalized_memory_entry(entry).memory_write_layer,
            record_type: entry.kind.clone(),
            source_type: entry.source_type.clone(),
            title: entry.summary.clone(),
            summary: format!("长期记忆已写入 {}", long_term_memory_file_path(request).display()),
            reason: "任务完成后形成了可复用摘要。".to_string(),
            audit: written_audit(&normalized_memory_entry(entry)),
        },
        Err(error) => append_error_outcome(entry, &error),
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

pub(super) fn skipped_memory_outcome(layer: &str, entry: &MemoryEntry, reason: &str) -> MemoryWriteOutcome {
    let entry = normalized_memory_entry(entry);
    MemoryWriteOutcome {
        event_type: skipped_event_type(layer),
        layer: layer.to_string(),
        record_type: entry.kind.clone(),
        source_type: entry.source_type.clone(),
        title: skipped_title(layer),
        summary: summarize_text(reason),
        reason: reason.to_string(),
        audit: skipped_entry_audit(&entry, skipped_event_type(layer), reason),
    }
}

#[cfg(test)]
mod tests;
