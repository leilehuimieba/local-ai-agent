use crate::contracts::RunRequest;
use crate::knowledge_store::{KnowledgeRecord, search_knowledge_records};
use crate::paths::{repo_root, siyuan_root_dir, siyuan_sync_enabled};
use crate::text::{extract_snippet, score_text};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub(crate) struct KnowledgeHit {
    pub path: String,
    pub snippet: String,
    pub source_type: String,
    pub source_label: String,
    pub knowledge_type: String,
    pub confidence: String,
    pub updated_at: String,
    pub reason: String,
}

pub(crate) fn search_knowledge(
    request: &RunRequest,
    query: &str,
    limit: usize,
) -> Vec<KnowledgeHit> {
    let file_hits = search_file_knowledge(request, query, limit);
    let mut stored_hits = search_stored_knowledge(request, query, limit);
    if stored_hits.is_empty() && siyuan_sync_enabled(request) {
        stored_hits = search_siyuan_index(request, query, limit);
    }
    stored_hits.extend(file_hits);
    stored_hits.into_iter().take(limit).collect()
}

fn search_file_knowledge(request: &RunRequest, query: &str, limit: usize) -> Vec<KnowledgeHit> {
    let mut scored = collect_knowledge_files(request)
        .into_iter()
        .filter_map(|path| score_knowledge_file(path, query))
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| right.0.cmp(&left.0));
    scored.into_iter().map(|(_, hit)| hit).take(limit).collect()
}

fn collect_knowledge_files(request: &RunRequest) -> Vec<PathBuf> {
    let mut seen = BTreeSet::new();
    let mut files = Vec::new();
    for root in knowledge_roots(request) {
        collect_search_files(&root, &mut files, &mut seen, 0);
    }
    files
}

fn knowledge_roots(request: &RunRequest) -> Vec<PathBuf> {
    let mut roots = vec![PathBuf::from(&request.workspace_ref.root_path)];
    let repo_docs = repo_root(request).join("docs");
    if repo_docs.exists() {
        roots.insert(0, repo_docs);
    }
    roots
}

fn score_knowledge_file(path: PathBuf, query: &str) -> Option<(i32, KnowledgeHit)> {
    let content = fs::read_to_string(&path).ok()?;
    let path_text = path.to_string_lossy().to_string();
    let score = knowledge_file_score(query, &path_text, &content);
    (score > 0).then_some((score, file_knowledge_hit(query, &content, path_text)))
}

fn file_knowledge_hit(query: &str, content: &str, path_text: String) -> KnowledgeHit {
    KnowledgeHit {
        reason: knowledge_reason(&path_text),
        path: path_text,
        snippet: extract_snippet(content, query),
        source_type: "workspace_file".to_string(),
        source_label: "文档知识".to_string(),
        knowledge_type: "document_reference".to_string(),
        confidence: "高（工作区文档直接命中）".to_string(),
        updated_at: String::new(),
    }
}

fn knowledge_file_score(query: &str, path_text: &str, content: &str) -> i32 {
    let mut score = score_text(query, &format!("{} {}", path_text, content));
    score += path_priority(path_text);
    score
}

fn search_stored_knowledge(request: &RunRequest, query: &str, limit: usize) -> Vec<KnowledgeHit> {
    let mut scored = search_knowledge_records(request)
        .into_iter()
        .filter(|record| !record.archived)
        .filter(|record| !is_recursive_record(record))
        .filter_map(|record| {
            let haystack = stored_haystack(&record);
            let score = score_text(query, &haystack) + path_priority(&record.source);
            (score > 0).then_some((score, stored_knowledge_hit(record)))
        })
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| right.0.cmp(&left.0));
    scored.into_iter().map(|(_, hit)| hit).take(limit).collect()
}

fn stored_haystack(record: &KnowledgeRecord) -> String {
    format!(
        "{} {} {} {} {} {} {}",
        record.title,
        record.summary,
        record.content,
        record.source,
        record.source_type,
        source_label(&record.source_type),
        confidence_label(&record.source_type, record.verified)
    )
}

fn stored_knowledge_hit(record: KnowledgeRecord) -> KnowledgeHit {
    KnowledgeHit {
        path: record.source.clone(),
        snippet: record.summary.clone(),
        source_type: record.source_type.clone(),
        source_label: source_label(&record.source_type),
        knowledge_type: record.knowledge_type.clone(),
        confidence: confidence_label(&record.source_type, record.verified),
        updated_at: record.updated_at.clone(),
        reason: knowledge_reason(&record.source),
    }
}

pub(crate) fn is_recursive_record(record: &crate::knowledge_store::KnowledgeRecord) -> bool {
    record.source.starts_with("run:")
        && (record.summary.contains("文件：run:") || record.content.contains("文件：run:"))
}

fn path_priority(path_text: &str) -> i32 {
    let path = path_text.replace('\\', "/").to_lowercase();
    if path.ends_with("/readme.md") || path == "readme.md" {
        return 48;
    }
    if path.contains("/docs/readme.md") {
        return 42;
    }
    if path.contains("/docs/06-development/") {
        return 28;
    }
    if path.contains("/docs/02-architecture/") || path.contains("/docs/03-runtime/") {
        return 18;
    }
    if path.contains("/docs/07-test/") {
        return -32;
    }
    0
}

pub(crate) fn knowledge_path_priority(path_text: &str) -> i32 {
    path_priority(path_text)
}

pub(crate) fn knowledge_reason(path_text: &str) -> String {
    let score = path_priority(path_text);
    if score >= 40 {
        return "README 主入口优先".to_string();
    }
    if score >= 28 {
        return "06-development 开发文档优先".to_string();
    }
    if score < 0 {
        return "07-test 已降权，仅作验收参考".to_string();
    }
    "按内容相关性命中".to_string()
}

fn source_label(source_type: &str) -> String {
    match source_type {
        "runtime" => "运行时沉淀知识".to_string(),
        "siyuan" | "siyuan_file" => "用户确认知识".to_string(),
        "workspace_file" => "文档知识".to_string(),
        _ => "本地知识".to_string(),
    }
}

fn confidence_label(source_type: &str, verified: bool) -> String {
    if source_type == "workspace_file" {
        return "高（直接文件命中）".to_string();
    }
    if source_type == "siyuan" || source_type == "siyuan_file" {
        return "高（用户沉淀确认）".to_string();
    }
    if verified {
        "高（已验证沉淀）".to_string()
    } else {
        "中（待进一步验证）".to_string()
    }
}

fn search_siyuan_index(request: &RunRequest, query: &str, limit: usize) -> Vec<KnowledgeHit> {
    let Some(root) = siyuan_root_dir(request) else {
        return Vec::new();
    };
    let mut files = Vec::new();
    collect_search_files(&root, &mut files, &mut BTreeSet::new(), 0);
    let mut scored = files
        .into_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(&path).ok()?;
            let path_text = path.to_string_lossy().to_string();
            let score = score_text(query, &format!("{} {}", path_text, content));
            (score > 0).then_some((
                score,
                KnowledgeHit {
                    path: path_text,
                    snippet: extract_snippet(&content, query),
                    source_type: "siyuan_file".to_string(),
                    source_label: "用户确认知识".to_string(),
                    knowledge_type: "user_curated".to_string(),
                    confidence: "高（用户沉淀确认）".to_string(),
                    updated_at: String::new(),
                    reason: "思源知识命中".to_string(),
                },
            ))
        })
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| right.0.cmp(&left.0));
    scored.into_iter().map(|(_, hit)| hit).take(limit).collect()
}

fn collect_search_files(
    root: &Path,
    files: &mut Vec<PathBuf>,
    seen: &mut BTreeSet<String>,
    depth: usize,
) {
    if depth > 4 || !root.exists() {
        return;
    }

    let root_key = root.to_string_lossy().to_string();
    if !seen.insert(root_key) {
        return;
    }

    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_lowercase();
        if path.is_dir() {
            if matches!(
                name.as_str(),
                ".git" | "node_modules" | "target" | "dist" | "tmp" | "logs" | "data"
            ) {
                continue;
            }
            collect_search_files(&path, files, seen, depth + 1);
            continue;
        }

        let Some(ext) = path.extension().and_then(|item| item.to_str()) else {
            continue;
        };
        if !matches!(ext.to_lowercase().as_str(), "md" | "txt" | "json" | "toml") {
            continue;
        }
        files.push(path);
        if files.len() >= 240 {
            return;
        }
    }
}
