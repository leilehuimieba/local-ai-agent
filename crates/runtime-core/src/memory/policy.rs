use super::MemoryEntry;
use crate::memory_schema::MEMORY_GOVERNANCE_VERSION;

pub(crate) fn normalized_memory_entry(entry: &MemoryEntry) -> MemoryEntry {
    let mut item = entry.clone();
    let source = derived_governance_source(&item);
    let reason = derived_governance_reason(&item);
    item.governance_version = choose_text(&[&item.governance_version, MEMORY_GOVERNANCE_VERSION]);
    item.governance_source = choose_text(&[&item.governance_source, &source]);
    item.governance_reason = choose_text(&[&item.governance_reason, &reason]);
    item.governance_at = choose_text(&[&item.governance_at, &item.updated_at, &item.created_at, &item.timestamp]);
    item.archive_reason = normalize_archive_reason(&item);
    item.memory_write_layer = choose_text(&[&item.memory_write_layer, &derived_write_layer(&item)]);
    item.memory_write_decision = choose_text(&[&item.memory_write_decision, &derived_write_decision(&item)]);
    item.memory_write_reason = choose_text(&[&item.memory_write_reason, &derived_write_reason(&item)]);
    item.memory_duplicate_strategy = choose_text(&[&item.memory_duplicate_strategy, "none"]);
    item
}

pub(super) fn should_skip_memory_entry(query_text: &str, entry: &MemoryEntry) -> bool {
    is_recursive_memory(entry)
        || is_path_only_memory(entry)
        || is_low_value_runtime_memory(entry)
        || is_test_memory_noise(query_text, entry)
}

pub(super) fn should_archive_memory_entry(entry: &MemoryEntry) -> bool {
    !is_seed_memory(entry) && is_low_value_runtime_memory(entry)
}

fn derived_write_layer(entry: &MemoryEntry) -> String {
    match entry.kind.as_str() {
        "workspace_summary" => "working_only".to_string(),
        "lesson_learned" | "task_outcome" => "episodic_memory".to_string(),
        "preference" | "workflow_preference" | "project_rule" | "workflow_pattern" => {
            "semantic_or_procedural_memory".to_string()
        }
        _ if entry.source_type == "seed" => "semantic_or_procedural_memory".to_string(),
        _ => "semantic_or_procedural_memory".to_string(),
    }
}

fn derived_write_decision(entry: &MemoryEntry) -> String {
    if derived_write_layer(entry) == "working_only" {
        "rejected".to_string()
    } else {
        "accepted".to_string()
    }
}

fn derived_write_reason(entry: &MemoryEntry) -> String {
    match derived_write_layer(entry).as_str() {
        "working_only" => "当前内容仅保留 working_only，不进入长期层。".to_string(),
        "episodic_memory" => "当前条目按过程经验进入 episodic_memory。".to_string(),
        _ => "当前条目具备跨任务复用价值，进入 semantic_or_procedural_memory。".to_string(),
    }
}

fn derived_governance_source(entry: &MemoryEntry) -> String {
    match entry.source_type.as_str() {
        "seed" => "seed_baseline".to_string(),
        "runtime" if entry.source_event_type == "memory_written" => "runtime_manual_write".to_string(),
        "runtime" if entry.source_event_type == "run_failed" => "runtime_failure_lesson".to_string(),
        "runtime" if entry.source_event_type == "run_finished" => "runtime_finish_memory".to_string(),
        "runtime" if entry.source_event_type == "verification_completed" => "runtime_verified_memory".to_string(),
        "runtime" => "runtime_memory".to_string(),
        _ => "memory_append".to_string(),
    }
}

fn derived_governance_reason(entry: &MemoryEntry) -> String {
    match entry.source_type.as_str() {
        "seed" => "基线记忆已按当前治理版本固化。".to_string(),
        "runtime" if entry.source_event_type == "memory_written" => "用户显式写入长期记忆。".to_string(),
        "runtime" if entry.source_event_type == "run_failed" => "失败教训已纳入长期记忆治理。".to_string(),
        "runtime" if entry.source_event_type == "run_finished" => "任务结果已按长期记忆治理规则沉淀。".to_string(),
        "runtime" if entry.source_event_type == "verification_completed" => "验证通过后已沉淀长期记忆。".to_string(),
        _ => "记忆记录已按当前治理版本写入。".to_string(),
    }
}

fn normalize_archive_reason(entry: &MemoryEntry) -> String {
    if !entry.archived {
        return String::new();
    }
    choose_text(&[&entry.archive_reason, "当前记录已标记为归档。"])
}

fn choose_text(values: &[&str]) -> String {
    values
        .iter()
        .find_map(|value| {
            let text = value.trim();
            (!text.is_empty()).then_some(text.to_string())
        })
        .unwrap_or_default()
}

fn is_recursive_memory(entry: &MemoryEntry) -> bool {
    entry.summary.contains("文件：run:") || entry.content.contains("文件：run:")
}

fn is_path_only_memory(entry: &MemoryEntry) -> bool {
    looks_like_path_only(&entry.content) || looks_like_path_only(&entry.summary)
}

fn is_low_value_runtime_memory(entry: &MemoryEntry) -> bool {
    is_runtime_project_answer_memory(entry) || is_runtime_tool_trace_memory(entry) || is_runtime_fallback_memory(entry)
}

fn is_test_memory_noise(query_text: &str, entry: &MemoryEntry) -> bool {
    is_project_query(query_text) && is_test_doc_memory(entry) && !is_seed_memory(entry)
}

fn is_project_query(query_text: &str) -> bool {
    query_text.contains("项目") || query_text.contains("说明") || query_text.contains("做什么")
}

fn is_test_doc_memory(entry: &MemoryEntry) -> bool {
    entry.summary.contains("docs\\07-test")
        || entry.summary.contains("docs/07-test")
        || entry.content.contains("docs\\07-test")
        || entry.content.contains("docs/07-test")
}

pub(super) fn is_seed_memory(entry: &MemoryEntry) -> bool {
    entry.source_run_id.starts_with("seed:")
}

fn is_runtime_project_answer_memory(entry: &MemoryEntry) -> bool {
    let project_answer = entry.kind == "project_knowledge" || entry.kind == "workspace_summary";
    let runtime_source = entry.source_type == "runtime";
    let generated =
        entry.title.contains("项目说明") || entry.summary.contains("已基于项目文档片段完成一次项目说明回答");
    project_answer && runtime_source && generated
}

fn is_runtime_tool_trace_memory(entry: &MemoryEntry) -> bool {
    let trace_title = entry.title.contains("导出知识到思源")
        || entry.title.contains("检索思源笔记")
        || entry.title.contains("读取思源正文")
        || entry.title.contains("复用已存在思源知识");
    let trace_summary = entry.summary.contains("知识已导出到思源目录")
        || entry.summary.contains("已返回思源笔记摘要")
        || entry.summary.contains("思源正文读取成功")
        || entry.summary.contains("命中已存在思源导出");
    entry.kind == "lesson_learned" && entry.source_type == "runtime" && (trace_title || trace_summary)
}

fn is_runtime_fallback_memory(entry: &MemoryEntry) -> bool {
    entry.kind == "lesson_learned"
        && entry.source_type == "runtime"
        && (is_garbled_reply(&entry.content) || is_capability_fallback(&entry.content))
}

fn is_garbled_reply(content: &str) -> bool {
    content.contains("显示为乱码")
        || content.contains("无法识别为有效的文字或指令")
        || content.contains("无法准确识别您想要表达的意思")
}

fn is_capability_fallback(content: &str) -> bool {
    content.contains("无法打开你的计算机")
        || content.contains("无法控制你的计算机硬件")
        || content.contains("如果你有工作区内的文件管理")
}

fn looks_like_path_only(value: &str) -> bool {
    let text = value.trim();
    (text.contains(":\\") || text.contains(":/")) && !text.contains('。') && !text.contains('，') && !text.contains(' ')
}
