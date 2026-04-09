use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::knowledge::search_knowledge;

const CACHE_REASON: &str = "知识检索依赖实时知识库状态，不使用回答缓存。";

pub(crate) fn execute_knowledge_search(request: &RunRequest, query: &str) -> ActionExecution {
    let hits = search_knowledge(request, query, 3);
    if hits.is_empty() {
        return ActionExecution::bypass_ok(
            format!("检索本地知识：{}", query),
            "没有找到相关知识内容。".to_string(),
            format!("当前没有在本地知识源中找到与 `{}` 相关的内容。", query),
            "先检索本地知识源，未命中时直接返回空结果说明。".to_string(),
            CACHE_REASON,
        );
    }
    ActionExecution::bypass_ok(
        format!("检索本地知识：{}", query),
        format!("已从本地知识源返回 {} 条摘要结果。", hits.len()),
        format!("本地知识检索结果：\n{}", render_hits(&hits)),
        "综合 README、开发文档和知识索引返回高相关知识片段。".to_string(),
        CACHE_REASON,
    )
}

fn render_hits(hits: &[crate::knowledge::KnowledgeHit]) -> String {
    hits
        .iter()
        .enumerate()
        .map(|(index, hit)| {
            format!(
                "{}. {}\n   {}\n   来源分类：{}\n   来源类型：{}\n   知识类型：{}\n   可信度：{}\n   更新时间：{}\n   命中理由：{}",
                index + 1,
                hit.path,
                hit.snippet,
                hit.source_label,
                hit.source_type,
                hit.knowledge_type,
                hit.confidence,
                blank_value(&hit.updated_at),
                hit.reason
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn blank_value(value: &str) -> &str {
    if value.trim().is_empty() {
        "未提供"
    } else {
        value
    }
}
