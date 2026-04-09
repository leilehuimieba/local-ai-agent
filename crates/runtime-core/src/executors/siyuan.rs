use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::execution::ActionExecution;
use crate::knowledge::KnowledgeHit;
use crate::knowledge_store::{
    KnowledgeRecord, append_knowledge_record, find_reusable_siyuan_record, search_knowledge_records,
};
use crate::paths::{
    siyuan_auto_write_enabled, siyuan_export_dir, siyuan_root_dir, siyuan_sync_enabled,
};
use crate::text::summarize_text;
use std::fs;
use std::path::{Path, PathBuf};

const CACHE_DEPENDS_FS: &str = "思源能力依赖实时配置与文件系统状态，不使用回答缓存。";
const CACHE_WRITE_REASON: &str = "思源写入属于实时副作用动作，不使用回答缓存。";

pub(crate) fn execute_siyuan_search(request: &RunRequest, query: &str) -> ActionExecution {
    let Some(root) = siyuan_root_dir(request) else {
        return missing_siyuan_action("当前未配置思源根目录。");
    };
    let hits = search_siyuan_notes(request, &root, query);
    if hits.is_empty() {
        return ActionExecution::bypass_ok(
            format!("检索思源笔记：{}", query),
            "没有命中思源笔记摘要。".to_string(),
            format!("当前没有找到与 `{}` 相关的思源笔记。", query),
            "先查询思源索引与正文，未命中时直接返回空结果说明。".to_string(),
            CACHE_DEPENDS_FS,
        );
    }
    ActionExecution::bypass_ok(
        format!("检索思源笔记：{}", query),
        "已返回思源笔记摘要。".to_string(),
        format!("思源检索结果：\n{}", render_hits(hits)),
        "综合思源索引与正文检索结果返回相关笔记摘要。".to_string(),
        CACHE_DEPENDS_FS,
    )
}

pub(crate) fn execute_siyuan_read(request: &RunRequest, path: &str) -> ActionExecution {
    let Some(root) = siyuan_root_dir(request) else {
        return missing_siyuan_action("当前未配置思源根目录。");
    };
    let target = resolve_siyuan_path(&root, path);
    match fs::read_to_string(&target) {
        Ok(content) => ActionExecution::bypass_ok(
            format!("读取思源正文：{}", target.display()),
            format!("思源正文读取成功，摘要：{}", summarize_text(&content)),
            format!(
                "思源正文读取完成：{}\n{}",
                target.display(),
                summarize_text(&content)
            ),
            "直接读取指定思源文档并压缩成可展示摘要。".to_string(),
            CACHE_DEPENDS_FS,
        ),
        Err(error) => ActionExecution::bypass_fail(
            format!("读取思源正文：{}", path),
            format!("思源正文读取失败：{}", error),
            format!("思源正文读取失败：{}", error),
            "指定思源文档读取失败，按系统错误直接返回。".to_string(),
            CACHE_DEPENDS_FS,
        ),
    }
}

pub(crate) fn execute_siyuan_write(request: &RunRequest) -> ActionExecution {
    if !siyuan_auto_write_enabled(request) {
        return missing_siyuan_action("当前未开启思源自动写入。");
    }
    let Some(path) = planned_siyuan_export_path(request) else {
        return missing_siyuan_action("当前未配置思源导出目录。");
    };
    let Ok(sync_hint) = export_siyuan_note(request, &path) else {
        return failed_siyuan_write("思源导出失败");
    };
    ActionExecution::bypass_ok(
        format!("导出知识到思源：{}", path.display()),
        "知识已导出到思源目录。".to_string(),
        format!("思源导出完成：{}\n{}", path.display(), sync_hint),
        "将当前输入整理为思源文档并同步写入知识索引。".to_string(),
        CACHE_WRITE_REASON,
    )
}

fn planned_siyuan_export_path(request: &RunRequest) -> Option<PathBuf> {
    let title = summarize_text(&request.user_input);
    reusable_siyuan_path(request, &title, &title).or_else(|| create_siyuan_path(request))
}

fn export_siyuan_note(request: &RunRequest, path: &Path) -> Result<&'static str, String> {
    let content = format!(
        "# {}\n\n{}",
        request.user_input,
        summarize_text(&request.user_input)
    );
    ensure_parent_dir(path).map_err(|error| error.to_string())?;
    fs::write(path, content).map_err(|error| error.to_string())?;
    append_siyuan_record(request, path)?;
    Ok(if siyuan_sync_enabled(request) {
        "已开启摘要回写。"
    } else {
        "摘要回写未开启。"
    })
}

fn missing_siyuan_action(message: &str) -> ActionExecution {
    ActionExecution::bypass_fail(
        "执行思源能力".to_string(),
        message.to_string(),
        message.to_string(),
        "思源能力缺少必要配置，无法进入实际执行。".to_string(),
        CACHE_DEPENDS_FS,
    )
}

fn failed_siyuan_write(message: &str) -> ActionExecution {
    ActionExecution::bypass_fail(
        "导出知识到思源".to_string(),
        format!("思源写入失败：{}", message),
        format!("思源写入失败：{}", message),
        "思源导出阶段发生系统错误，直接按失败收口。".to_string(),
        CACHE_WRITE_REASON,
    )
}

fn search_siyuan_notes(request: &RunRequest, root: &Path, query: &str) -> Vec<KnowledgeHit> {
    let indexed = search_siyuan_index_hits(request, query);
    if !indexed.is_empty() {
        return indexed;
    }
    let mut scored = collect_siyuan_paths(root)
        .into_iter()
        .filter_map(|path| score_siyuan_file(path, query))
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| right.0.cmp(&left.0));
    scored.into_iter().take(3).map(|(_, hit)| hit).collect()
}

fn search_siyuan_index_hits(request: &RunRequest, query: &str) -> Vec<KnowledgeHit> {
    let mut scored = search_knowledge_records(request)
        .into_iter()
        .filter(|record| record.source_type == "siyuan")
        .filter_map(|record| score_siyuan_record(record, query))
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| right.0.cmp(&left.0));
    scored.into_iter().take(3).map(|(_, hit)| hit).collect()
}

fn score_siyuan_record(record: KnowledgeRecord, query: &str) -> Option<(i32, KnowledgeHit)> {
    let haystack = format!("{} {} {}", record.title, record.summary, record.content);
    let score = crate::text::score_text(query, &haystack)
        + crate::knowledge::knowledge_path_priority(&record.source);
    (score > 0).then_some((
        score,
        KnowledgeHit {
            path: record.source,
            snippet: record.summary,
            source_type: "siyuan_index".to_string(),
            source_label: "用户确认知识".to_string(),
            knowledge_type: record.knowledge_type,
            confidence: if record.verified {
                "高（用户沉淀确认）".to_string()
            } else {
                "中（待进一步验证）".to_string()
            },
            updated_at: record.updated_at,
            reason: "思源索引命中".to_string(),
        },
    ))
}

fn collect_siyuan_paths(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_siyuan_files(root, &mut files, 0);
    files
}

fn collect_siyuan_files(root: &Path, files: &mut Vec<PathBuf>, depth: usize) {
    if depth > 4 || !root.exists() {
        return;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_siyuan_files(&path, files, depth + 1);
            continue;
        }
        if path.extension().and_then(|item| item.to_str()) == Some("md") {
            files.push(path);
        }
    }
}

fn score_siyuan_file(path: PathBuf, query: &str) -> Option<(i32, KnowledgeHit)> {
    let content = fs::read_to_string(&path).ok()?;
    let snippet = summarize_text(&content);
    let path_text = path.display().to_string();
    let haystack = format!("{} {}", path_text, content);
    let score = crate::text::score_text(query, &haystack)
        + crate::knowledge::knowledge_path_priority(&path_text);
    (score > 0).then_some((
        score,
        KnowledgeHit {
            path: path_text,
            snippet,
            source_type: "siyuan_file".to_string(),
            source_label: "用户确认知识".to_string(),
            knowledge_type: "user_curated".to_string(),
            confidence: "高（用户沉淀确认）".to_string(),
            updated_at: String::new(),
            reason: "思源正文命中".to_string(),
        },
    ))
}

fn resolve_siyuan_path(root: &Path, raw: &str) -> PathBuf {
    let path = Path::new(raw);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn reusable_siyuan_path(request: &RunRequest, title: &str, summary: &str) -> Option<PathBuf> {
    let record = find_reusable_siyuan_record(request, title, summary)?;
    let path = PathBuf::from(record.source);
    path.exists().then_some(path)
}

fn create_siyuan_path(request: &RunRequest) -> Option<PathBuf> {
    let export_dir = siyuan_export_dir(request)?;
    Some(export_dir.join(format!(
        "{}-{}.md",
        request.workspace_ref.workspace_id, request.run_id
    )))
}

fn append_siyuan_record(request: &RunRequest, path: &Path) -> Result<(), String> {
    let title = summarize_text(&request.user_input);
    let summary = summarize_text(&request.user_input);
    let record = KnowledgeRecord {
        id: format!("siyuan-note-{}", timestamp_now()),
        knowledge_type: "document_digest".to_string(),
        title,
        summary,
        content: format!("思源导出知识：{}", request.user_input),
        tags: vec![
            "siyuan".to_string(),
            request.workspace_ref.workspace_id.clone(),
        ],
        source: path.display().to_string(),
        source_type: "siyuan".to_string(),
        verified: true,
        workspace_id: request.workspace_ref.workspace_id.clone(),
        priority: 1,
        archived: false,
        created_at: timestamp_now(),
        updated_at: timestamp_now(),
    };
    if has_same_siyuan_content(request, &record) {
        return Ok(());
    }
    append_knowledge_record(request, &record)
}

fn has_same_siyuan_content(request: &RunRequest, record: &KnowledgeRecord) -> bool {
    find_reusable_siyuan_record(request, &record.title, &record.summary).is_some()
}

fn ensure_parent_dir(path: &Path) -> Result<(), std::io::Error> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    fs::create_dir_all(parent)
}

fn render_hits(hits: Vec<KnowledgeHit>) -> String {
    hits.into_iter()
        .enumerate()
        .map(|(index, hit)| {
            format!(
                "{}. {}\n   {}\n   来源类型：{}\n   命中理由：{}",
                index + 1,
                hit.path,
                hit.snippet,
                hit.source_type,
                hit.reason
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
