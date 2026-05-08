use crate::knowledge_store::KnowledgeRecord;
use crate::memory::MemoryEntry;

pub(super) fn is_runtime_generated_memory(item: &MemoryEntry) -> bool {
    let project_answer = is_project_answer_memory(item);
    let tool_trace = is_tool_trace_memory(item);
    let fallback = is_fallback_memory(item);
    item.source_type == "runtime"
        && (project_answer
            || tool_trace
            || fallback
            || is_low_value_runtime_lesson(item)
            || is_legacy_preference_noise(item))
}

pub(super) fn is_runtime_generated_knowledge(item: &KnowledgeRecord) -> bool {
    let project_answer =
        item.title.contains("项目说明") || item.summary.contains("已基于项目文档片段完成一次项目说明回答");
    item.source_type == "runtime" && item.source.starts_with("run:") && project_answer
}

fn is_project_answer_memory(item: &MemoryEntry) -> bool {
    let kind = item.kind == "project_knowledge" || item.kind == "workspace_summary";
    let generated = item.title.contains("项目说明") || item.summary.contains("已基于项目文档片段完成一次项目说明回答");
    kind && generated
}

fn is_tool_trace_memory(item: &MemoryEntry) -> bool {
    item.kind == "lesson_learned"
        && (item.title.contains("导出知识到思源")
            || item.title.contains("检索思源笔记")
            || item.title.contains("读取思源正文")
            || item.title.contains("复用已存在思源知识")
            || item.summary.contains("知识已导出到思源目录")
            || item.summary.contains("已返回思源笔记摘要")
            || item.summary.contains("思源正文读取成功")
            || item.summary.contains("命中已存在思源导出"))
}

fn is_fallback_memory(item: &MemoryEntry) -> bool {
    item.kind == "lesson_learned" && (is_garbled_reply(&item.content) || is_capability_fallback(&item.content))
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

fn is_low_value_runtime_lesson(item: &MemoryEntry) -> bool {
    let generic = item.kind == "lesson_learned"
        && item.title.contains("基于会话压缩摘要继续回答。")
        && item.summary.contains("已从最近")
        && item.summary.contains("完成一次模型回答");
    let tool_trace = item.kind == "lesson_learned"
        && (item.title.contains("读取文件：")
            || item.title.contains("执行命令：")
            || item.summary.contains("文件读取成功")
            || item.summary.contains("命令执行成功"));
    generic || tool_trace
}

fn is_legacy_preference_noise(item: &MemoryEntry) -> bool {
    item.kind == "preference" && item.title.trim().is_empty() && !item.verified
}
