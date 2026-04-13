use crate::contracts::RunRequest;
use crate::knowledge_store::{KnowledgeRecord, search_knowledge_records};
use crate::paths::{external_memory_audit_path, repo_root, siyuan_root_dir, siyuan_sync_enabled};
use crate::storage::append_jsonl;
use crate::text::{extract_snippet, score_text};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;
const CORTEX_RETRY_ATTEMPTS: u8 = 3;
const CORTEX_CONNECT_TIMEOUT_SECONDS: &str = "2";
const CORTEX_MAX_TIMEOUT_SECONDS: &str = "6";

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
    let local_hits = search_local_knowledge(request, query, limit);
    let remain = limit.saturating_sub(local_hits.len());
    let external_hits = if remain > 0 {
        search_external_knowledge(request, query, remain)
    } else {
        Vec::new()
    };
    merge_knowledge_hits(local_hits, external_hits, limit)
}

fn merge_knowledge_hits(
    local_hits: Vec<KnowledgeHit>,
    external_hits: Vec<KnowledgeHit>,
    limit: usize,
) -> Vec<KnowledgeHit> {
    let mut hits = local_hits;
    hits.extend(external_hits);
    dedupe_hits(hits).into_iter().take(limit).collect()
}

fn dedupe_hits(hits: Vec<KnowledgeHit>) -> Vec<KnowledgeHit> {
    let mut seen = BTreeSet::new();
    hits.into_iter()
        .filter(|hit| seen.insert(hit_dedupe_key(hit)))
        .collect()
}

fn hit_dedupe_key(hit: &KnowledgeHit) -> String {
    let snippet = normalize_hit_text(&hit.snippet);
    let anchor = if snippet.chars().count() >= 12 {
        snippet
    } else {
        format!("{}|{}", normalize_hit_text(&hit.path), snippet)
    };
    format!("{}|{}", hit.knowledge_type, anchor)
}

fn normalize_hit_text(text: &str) -> String {
    text.to_lowercase()
        .split_whitespace()
        .collect::<String>()
        .chars()
        .take(160)
        .collect()
}

fn search_local_knowledge(request: &RunRequest, query: &str, limit: usize) -> Vec<KnowledgeHit> {
    let file_hits = search_file_knowledge(request, query, limit);
    let mut stored_hits = search_stored_knowledge(request, query, limit);
    if stored_hits.is_empty() && siyuan_sync_enabled(request) {
        stored_hits = search_siyuan_index(request, query, limit);
    }
    stored_hits.extend(file_hits);
    stored_hits.into_iter().take(limit).collect()
}

#[derive(Clone, Debug, Deserialize)]
struct CortexFlag {
    enabled: bool,
    #[serde(default)]
    server_url: String,
    #[serde(default)]
    agent_id: String,
}

#[derive(Clone, Debug, Deserialize)]
struct CortexRecallResponse {
    #[serde(default)]
    memories: Vec<CortexRecallMemory>,
}

#[derive(Clone, Debug, Deserialize)]
struct CortexRecallMemory {
    #[serde(default)]
    id: String,
    #[serde(default)]
    content: String,
    #[serde(default)]
    category: String,
    #[serde(default)]
    created_at: String,
}

#[derive(Serialize)]
struct CortexRecallAuditRecord {
    trace_id: String,
    agent_id: String,
    source: String,
    duration_ms: u64,
    status: String,
    attempts: u8,
    error: String,
}

fn search_external_knowledge(request: &RunRequest, query: &str, limit: usize) -> Vec<KnowledgeHit> {
    let Some(flag) = read_cortex_flag(request) else {
        return Vec::new();
    };
    if !flag.enabled {
        return Vec::new();
    }
    let token = std::env::var("CORTEX_AUTH_TOKEN").unwrap_or_default();
    if token.trim().is_empty() {
        return Vec::new();
    }
    cortex_result_or_empty(recall_cortex_hits(request, &flag, &token, query, limit))
}

fn cortex_result_or_empty(result: Result<Vec<KnowledgeHit>, String>) -> Vec<KnowledgeHit> {
    result.unwrap_or_default()
}

fn read_cortex_flag(request: &RunRequest) -> Option<CortexFlag> {
    let path = repo_root(request)
        .join("data")
        .join("settings")
        .join("external-memory-cortex.json");
    let text = fs::read_to_string(path).ok()?;
    serde_json::from_str::<CortexFlag>(&text).ok()
}

fn recall_cortex_hits(
    request: &RunRequest,
    flag: &CortexFlag,
    token: &str,
    query: &str,
    limit: usize,
) -> Result<Vec<KnowledgeHit>, String> {
    let body_path = write_recall_body(flag, query, limit)?;
    let started = Instant::now();
    let retry = retry_cortex_recall(flag, token, &body_path);
    let attempts = retry
        .as_ref()
        .map(|(_, attempt)| *attempt)
        .unwrap_or(CORTEX_RETRY_ATTEMPTS);
    let result = retry.and_then(|(body, _)| parse_cortex_recall_hits(&body));
    let _ = fs::remove_file(body_path);
    write_cortex_recall_audit(request, flag, query, started.elapsed().as_millis(), attempts, &result);
    result
}

fn retry_cortex_recall(flag: &CortexFlag, token: &str, body_path: &Path) -> Result<(String, u8), String> {
    let mut last_error = String::new();
    for attempt in 1..=CORTEX_RETRY_ATTEMPTS {
        match run_cortex_recall(flag, token, body_path) {
            Ok(body) => return Ok((body, attempt)),
            Err(error) => {
                last_error = format!("attempt={attempt}/{CORTEX_RETRY_ATTEMPTS}, {error}");
            }
        }
    }
    Err(format!("cortex recall retry exhausted, {last_error}"))
}

fn write_cortex_recall_audit(
    request: &RunRequest,
    flag: &CortexFlag,
    query: &str,
    duration_ms: u128,
    attempts: u8,
    result: &Result<Vec<KnowledgeHit>, String>,
) {
    let payload = CortexRecallAuditRecord {
        trace_id: request.trace_id.clone(),
        agent_id: resolve_agent_id(flag),
        source: recall_source(query),
        duration_ms: duration_ms as u64,
        status: if result.is_ok() { "ok" } else { "failed" }.to_string(),
        attempts,
        error: result.as_ref().err().cloned().unwrap_or_default(),
    };
    let _ = append_jsonl(external_memory_audit_path(request), &payload);
}

fn resolve_agent_id(flag: &CortexFlag) -> String {
    if flag.agent_id.trim().is_empty() {
        return "default".to_string();
    }
    flag.agent_id.clone()
}

fn sensitive_query_marker(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let markers = [
        "authorization: bearer",
        "api_key",
        "token=",
        "password=",
        "secret=",
        "sk-",
        "ak-",
    ];
    markers.iter().any(|marker| lower.contains(marker))
}

fn recall_source(query: &str) -> String {
    if sensitive_query_marker(query) {
        return "knowledge_search:[REDACTED]".to_string();
    }
    let snippet = query.chars().take(80).collect::<String>();
    format!("knowledge_search:{snippet}")
}

fn write_recall_body(flag: &CortexFlag, query: &str, limit: usize) -> Result<PathBuf, String> {
    let agent_id = if flag.agent_id.trim().is_empty() {
        "default".to_string()
    } else {
        flag.agent_id.clone()
    };
    let body = serde_json::json!({
        "query": query,
        "agent_id": agent_id,
        "top_k": limit,
    });
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|item| item.as_millis())
        .unwrap_or_default();
    let path = std::env::temp_dir().join(format!("cortex-recall-{stamp}.json"));
    let content = serde_json::to_string(&body).map_err(|error| error.to_string())?;
    fs::write(&path, content).map_err(|error| error.to_string())?;
    Ok(path)
}

fn run_cortex_recall(flag: &CortexFlag, token: &str, body_path: &Path) -> Result<String, String> {
    let server_url = if flag.server_url.trim().is_empty() {
        "http://127.0.0.1:21100"
    } else {
        flag.server_url.trim()
    };
    let uri = format!("{}/api/v1/recall", server_url.trim_end_matches('/'));
    let mut cmd = Command::new(curl_bin());
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let output = cmd
        .args(["-sS", "-f"])
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg(format!("Authorization: Bearer {token}"))
        .args(["--connect-timeout", CORTEX_CONNECT_TIMEOUT_SECONDS])
        .args(["--max-time", CORTEX_MAX_TIMEOUT_SECONDS])
        .arg("--data-binary")
        .arg(format!("@{}", body_path.display()))
        .arg(uri)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!("cortex recall failed, stderr={}", String::from_utf8_lossy(&output.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_cortex_recall_hits(body: &str) -> Result<Vec<KnowledgeHit>, String> {
    let response = serde_json::from_str::<CortexRecallResponse>(body).map_err(|e| e.to_string())?;
    let hits = response
        .memories
        .into_iter()
        .map(|memory| KnowledgeHit {
            path: format!("cortex://{}", memory.id),
            snippet: memory.content,
            source_type: "cortex".to_string(),
            source_label: "外部增强记忆".to_string(),
            knowledge_type: cortex_category_or_default(&memory.category),
            confidence: "中（外部 recall 补充）".to_string(),
            updated_at: memory.created_at,
            reason: "本地命中不足，补充外部召回".to_string(),
        })
        .collect::<Vec<_>>();
    Ok(hits)
}

fn cortex_category_or_default(category: &str) -> String {
    if category.trim().is_empty() {
        return "external_memory".to_string();
    }
    category.to_string()
}

fn curl_bin() -> &'static str {
    if cfg!(target_os = "windows") {
        "curl.exe"
    } else {
        "curl"
    }
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

#[cfg(test)]
mod tests {
    use crate::paths::external_memory_audit_path;
    use super::{
        KnowledgeHit, cortex_result_or_empty, dedupe_hits, merge_knowledge_hits,
        parse_cortex_recall_hits, recall_source,
    };

    #[test]
    fn maps_cortex_recall_payload_to_hits() {
        let body = r#"{
          "memories":[
            {"id":"m1","content":"CET4 shadowing improves listening","category":"fact","created_at":"2026-04-13T10:00:00Z"}
          ]
        }"#;
        let hits = parse_cortex_recall_hits(body).expect("payload should parse");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].path, "cortex://m1");
        assert_eq!(hits[0].source_type, "cortex");
        assert_eq!(hits[0].knowledge_type, "fact");
    }

    #[test]
    fn dedupe_prefers_local_hit_when_external_is_duplicate() {
        let local = KnowledgeHit {
            path: "run:1".to_string(),
            snippet: "CET4 shadowing improves listening".to_string(),
            source_type: "runtime".to_string(),
            source_label: "运行时沉淀知识".to_string(),
            knowledge_type: "workflow_pattern".to_string(),
            confidence: "高".to_string(),
            updated_at: "2026-04-13T10:00:00Z".to_string(),
            reason: "local".to_string(),
        };
        let external = KnowledgeHit {
            path: "cortex://m1".to_string(),
            snippet: "CET4 shadowing improves listening".to_string(),
            source_type: "cortex".to_string(),
            source_label: "外部增强记忆".to_string(),
            knowledge_type: "workflow_pattern".to_string(),
            confidence: "中".to_string(),
            updated_at: "2026-04-13T10:00:01Z".to_string(),
            reason: "external".to_string(),
        };
        let hits = dedupe_hits(vec![local, external]);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].source_type, "runtime");
    }

    #[test]
    fn dedupe_keeps_distinct_hits() {
        let first = KnowledgeHit {
            path: "run:1".to_string(),
            snippet: "CET4 shadowing improves listening".to_string(),
            source_type: "runtime".to_string(),
            source_label: "运行时沉淀知识".to_string(),
            knowledge_type: "workflow_pattern".to_string(),
            confidence: "高".to_string(),
            updated_at: "2026-04-13T10:00:00Z".to_string(),
            reason: "local".to_string(),
        };
        let second = KnowledgeHit {
            path: "cortex://m2".to_string(),
            snippet: "Vocabulary chunks boost reading score".to_string(),
            source_type: "cortex".to_string(),
            source_label: "外部增强记忆".to_string(),
            knowledge_type: "workflow_pattern".to_string(),
            confidence: "中".to_string(),
            updated_at: "2026-04-13T10:00:01Z".to_string(),
            reason: "external".to_string(),
        };
        let hits = dedupe_hits(vec![first, second]);
        assert_eq!(hits.len(), 2);
    }

    #[test]
    fn recall_audit_contains_required_fields() {
        let mut request = crate::query_engine_testkit::testkit::sample_request("retryable_failure");
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|item| item.as_millis())
            .unwrap_or_default();
        let repo_root = std::env::temp_dir().join(format!("runtime-core-audit-{stamp}"));
        request.context_hints.insert("repo_root".to_string(), repo_root.display().to_string());
        let flag = super::CortexFlag { enabled: true, server_url: String::new(), agent_id: "default".to_string() };
        super::write_cortex_recall_audit(&request, &flag, "cet4 listening", 12, 2, &Ok(Vec::new()));
        let log = std::fs::read_to_string(external_memory_audit_path(&request)).expect("audit log");
        assert!(log.contains("\"trace_id\":\"trace-1\""));
        assert!(log.contains("\"agent_id\":\"default\""));
        assert!(log.contains("\"source\":\"knowledge_search:cet4 listening\""));
        assert!(log.contains("\"duration_ms\":12"));
        let _ = std::fs::remove_dir_all(repo_root);
    }

    #[test]
    fn recall_source_redacts_sensitive_query() {
        let source = recall_source("please use api_key=sk-test-123 to recall");
        assert_eq!(source, "knowledge_search:[REDACTED]");
    }

    #[test]
    fn recall_merge_prefers_local_when_limit_is_full() {
        let local = vec![test_hit("run:1", "local one"), test_hit("run:2", "local two")];
        let external = vec![test_hit("cortex://1", "external one")];
        let hits = merge_knowledge_hits(local, external, 2);
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].path, "run:1");
        assert_eq!(hits[1].path, "run:2");
    }

    #[test]
    fn recall_merge_adds_external_when_local_not_enough() {
        let local = vec![test_hit("run:1", "local one")];
        let external = vec![test_hit("cortex://1", "external one")];
        let hits = merge_knowledge_hits(local, external, 2);
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[1].path, "cortex://1");
    }

    #[test]
    fn recall_degrades_to_empty_when_external_fails() {
        let hits = cortex_result_or_empty(Err("network".to_string()));
        assert!(hits.is_empty());
    }

    fn test_hit(path: &str, snippet: &str) -> KnowledgeHit {
        KnowledgeHit {
            path: path.to_string(),
            snippet: snippet.to_string(),
            source_type: "runtime".to_string(),
            source_label: "测试".to_string(),
            knowledge_type: "workflow_pattern".to_string(),
            confidence: "中".to_string(),
            updated_at: "1".to_string(),
            reason: "test".to_string(),
        }
    }
}
