use crate::paths::{external_memory_audit_path, knowledge_base_file_path, repo_root};
use crate::sensitive_data::contains_sensitive_text;
use crate::sqlite_store::{list_knowledge_records_sqlite, write_knowledge_record_sqlite};
use crate::storage::{append_jsonl, read_jsonl};
use serde::{Deserialize, Serialize};
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
    append_jsonl(knowledge_base_file_path(request), record)?;
    if let Err(error) = sync_cortex_ingest(request, record) {
        eprintln!("external cortex ingest skipped: {error}");
    }
    Ok(())
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

fn record_contains_sensitive_data(record: &KnowledgeRecord) -> bool {
    contains_sensitive_text(&record.title)
        || contains_sensitive_text(&record.summary)
        || contains_sensitive_text(&record.content)
        || record.tags.iter().any(|tag| contains_sensitive_text(tag))
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
    if record_contains_sensitive_data(record) {
        return Some(KnowledgeSkip {
            reason: "命中敏感信息清洗规则：疑似密钥或隐私字段，禁止入库。".to_string(),
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

#[derive(Clone, Debug, Deserialize)]
struct CortexFlag {
    enabled: bool,
    #[serde(default)]
    server_url: String,
    #[serde(default)]
    agent_id: String,
}

#[derive(Serialize)]
struct CortexIngestPayload<'a> {
    user_message: &'a str,
    assistant_message: &'a str,
    agent_id: &'a str,
}

#[derive(Serialize)]
struct CortexAuditRecord {
    trace_id: String,
    agent_id: String,
    source: String,
    duration_ms: u64,
    status: String,
    attempts: u8,
    error: String,
}

fn sync_cortex_ingest(
    request: &crate::contracts::RunRequest,
    record: &KnowledgeRecord,
) -> Result<(), String> {
    if !should_sync_to_cortex(record) {
        return Ok(());
    }
    let Some(flag) = read_cortex_flag(request) else {
        return Ok(());
    };
    if !flag.enabled {
        return Ok(());
    }
    let token = std::env::var("CORTEX_AUTH_TOKEN").unwrap_or_default();
    if token.trim().is_empty() {
        return Err("CORTEX_AUTH_TOKEN 未配置，跳过外部写入。".to_string());
    }
    post_cortex_ingest(request, &flag, &token, record)
}

fn should_sync_to_cortex(record: &KnowledgeRecord) -> bool {
    if record.source_type != "runtime" {
        return false;
    }
    record.knowledge_type == "workflow_pattern"
        || record.tags.iter().any(|tag| tag == "agent_resolve")
        || record.tags.iter().any(|tag| tag == "result_mode")
}

fn read_cortex_flag(request: &crate::contracts::RunRequest) -> Option<CortexFlag> {
    let path = repo_root(request)
        .join("data")
        .join("settings")
        .join("external-memory-cortex.json");
    let text = fs::read_to_string(path).ok()?;
    serde_json::from_str::<CortexFlag>(&text).ok()
}

fn post_cortex_ingest(
    request: &crate::contracts::RunRequest,
    flag: &CortexFlag,
    token: &str,
    record: &KnowledgeRecord,
) -> Result<(), String> {
    let server_url = if flag.server_url.trim().is_empty() {
        "http://127.0.0.1:21100".to_string()
    } else {
        flag.server_url.clone()
    };
    let agent_id = if flag.agent_id.trim().is_empty() {
        "default".to_string()
    } else {
        flag.agent_id.clone()
    };
    let payload = CortexIngestPayload {
        user_message: &record.summary,
        assistant_message: &record.content,
        agent_id: &agent_id,
    };
    let body_path = write_temp_body(&payload)?;
    let started = Instant::now();
    let result = retry_cortex_ingest(&server_url, token, &body_path);
    let _ = fs::remove_file(body_path);
    write_cortex_ingest_audit(
        request,
        &agent_id,
        record,
        started.elapsed().as_millis(),
        &result,
    );
    result.map(|_| ())
}

fn retry_cortex_ingest(server_url: &str, token: &str, body_path: &Path) -> Result<u8, String> {
    retry_cortex_ingest_operation(|_| run_cortex_curl(server_url, token, body_path))
}

fn retry_cortex_ingest_operation<F>(mut run: F) -> Result<u8, String>
where
    F: FnMut(u8) -> Result<(), String>,
{
    let mut last_error = String::new();
    for attempt in 1..=CORTEX_RETRY_ATTEMPTS {
        match run(attempt) {
            Ok(()) => return Ok(attempt),
            Err(error) => {
                last_error = format!("attempt={attempt}/{CORTEX_RETRY_ATTEMPTS}, {error}");
            }
        }
    }
    Err(format!("cortex ingest retry exhausted, {last_error}"))
}

fn write_cortex_ingest_audit(
    request: &crate::contracts::RunRequest,
    agent_id: &str,
    record: &KnowledgeRecord,
    duration_ms: u128,
    result: &Result<u8, String>,
) {
    let status = if result.is_ok() { "ok" } else { "failed" };
    let payload = CortexAuditRecord {
        trace_id: request.trace_id.clone(),
        agent_id: agent_id.to_string(),
        source: record.source.clone(),
        duration_ms: duration_ms as u64,
        status: status.to_string(),
        attempts: result.as_ref().copied().unwrap_or(CORTEX_RETRY_ATTEMPTS),
        error: result.as_ref().err().cloned().unwrap_or_default(),
    };
    let _ = append_jsonl(external_memory_audit_path(request), &payload);
}

fn write_temp_body(payload: &CortexIngestPayload<'_>) -> Result<PathBuf, String> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|item| item.as_millis())
        .unwrap_or_default();
    let path = std::env::temp_dir().join(format!("cortex-ingest-{stamp}.json"));
    let body = serde_json::to_string(payload).map_err(|error| error.to_string())?;
    fs::write(&path, body).map_err(|error| error.to_string())?;
    Ok(path)
}

fn run_cortex_curl(server_url: &str, token: &str, body_path: &Path) -> Result<(), String> {
    let uri = format!("{}/api/v1/ingest", server_url.trim_end_matches('/'));
    let mut cmd = Command::new(curl_bin());
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let output = cmd
        .args(["-s", "-o", null_sink(), "-w", "%{http_code}"])
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
    let code = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if output.status.success() && (code == "200" || code == "201") {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(format!(
        "cortex ingest failed, status={code}, stderr={stderr}"
    ))
}

fn curl_bin() -> &'static str {
    if cfg!(target_os = "windows") {
        "curl.exe"
    } else {
        "curl"
    }
}

fn null_sink() -> &'static str {
    if cfg!(target_os = "windows") {
        "NUL"
    } else {
        "/dev/null"
    }
}

#[cfg(test)]
mod tests {
    use super::{
        append_knowledge_record, retry_cortex_ingest_operation, search_knowledge_records,
        should_skip_knowledge_record, should_sync_to_cortex, KnowledgeRecord,
    };
    use crate::paths::external_memory_audit_path;

    #[test]
    fn sync_enabled_for_agent_resolve_workflow_pattern() {
        let record = KnowledgeRecord {
            id: "k1".to_string(),
            knowledge_type: "workflow_pattern".to_string(),
            title: "title".to_string(),
            summary: "summary long enough for sync".to_string(),
            content: "content".to_string(),
            tags: vec!["agent_resolve".to_string(), "main".to_string()],
            source: "run:1".to_string(),
            source_type: "runtime".to_string(),
            verified: true,
            workspace_id: "main".to_string(),
            priority: 0,
            archived: false,
            created_at: "1".to_string(),
            updated_at: "1".to_string(),
        };
        assert!(should_sync_to_cortex(&record));
    }

    #[test]
    fn sync_disabled_for_non_runtime_records() {
        let record = KnowledgeRecord {
            id: "k2".to_string(),
            knowledge_type: "workflow_pattern".to_string(),
            title: "title".to_string(),
            summary: "summary".to_string(),
            content: "content".to_string(),
            tags: vec!["agent_resolve".to_string()],
            source: "manual".to_string(),
            source_type: "manual".to_string(),
            verified: true,
            workspace_id: "main".to_string(),
            priority: 0,
            archived: false,
            created_at: "1".to_string(),
            updated_at: "1".to_string(),
        };
        assert!(!should_sync_to_cortex(&record));
    }

    #[test]
    fn ingest_audit_contains_required_fields() {
        let mut request = crate::query_engine_testkit::testkit::sample_request("retryable_failure");
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|item| item.as_millis())
            .unwrap_or_default();
        let repo_root = std::env::temp_dir().join(format!("runtime-core-ingest-audit-{stamp}"));
        request
            .context_hints
            .insert("repo_root".to_string(), repo_root.display().to_string());
        let record = KnowledgeRecord {
            id: "k3".to_string(),
            knowledge_type: "workflow_pattern".to_string(),
            title: "title".to_string(),
            summary: "summary long enough for ingest audit".to_string(),
            content: "content".to_string(),
            tags: vec!["agent_resolve".to_string()],
            source: "run:99".to_string(),
            source_type: "runtime".to_string(),
            verified: true,
            workspace_id: "main".to_string(),
            priority: 0,
            archived: false,
            created_at: "1".to_string(),
            updated_at: "1".to_string(),
        };
        super::write_cortex_ingest_audit(&request, "default", &record, 15, &Ok(2));
        let log = std::fs::read_to_string(external_memory_audit_path(&request)).expect("audit log");
        assert!(log.contains("\"trace_id\":\"trace-1\""));
        assert!(log.contains("\"agent_id\":\"default\""));
        assert!(log.contains("\"source\":\"run:99\""));
        assert!(log.contains("\"duration_ms\":15"));
        let _ = std::fs::remove_dir_all(repo_root);
    }

    #[test]
    fn retry_ingest_operation_returns_success_attempt() {
        let mut calls = 0_u8;
        let result = retry_cortex_ingest_operation(|_| {
            calls += 1;
            if calls < 3 {
                return Err("transient".to_string());
            }
            Ok(())
        });
        assert_eq!(result, Ok(3));
    }

    #[test]
    fn retry_ingest_operation_keeps_timeout_error_trace() {
        let result = retry_cortex_ingest_operation(|_| Err("Operation timed out".to_string()));
        let error = result.expect_err("should fail");
        assert!(error.contains("retry exhausted"));
        assert!(error.contains("attempt=3/3"));
        assert!(error.contains("Operation timed out"));
    }

    #[test]
    fn append_knowledge_record_keeps_local_data_when_external_sync_fails() {
        let mut request = crate::query_engine_testkit::testkit::sample_request("retryable_failure");
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|item| item.as_millis())
            .unwrap_or_default();
        let repo_root = std::env::temp_dir().join(format!("runtime-core-ingest-fallback-{stamp}"));
        request
            .context_hints
            .insert("repo_root".to_string(), repo_root.display().to_string());
        let settings_dir = repo_root.join("data").join("settings");
        let _ = std::fs::create_dir_all(&settings_dir);
        let flag =
            "{\"enabled\":true,\"server_url\":\"http://127.0.0.1:65535\",\"agent_id\":\"default\"}";
        let _ = std::fs::write(settings_dir.join("external-memory-cortex.json"), flag);
        let record = runtime_record("k4", "fallback record summary for local write path");
        assert!(append_knowledge_record(&request, &record).is_ok());
        let stored = search_knowledge_records(&request);
        assert!(stored.iter().any(|item| item.id == "k4"));
        let _ = std::fs::remove_dir_all(repo_root);
    }

    #[test]
    fn sensitive_record_is_skipped() {
        let mut record = runtime_record("k5", "summary long enough for sensitive check");
        record.content = "authorization: bearer sk-test-token".to_string();
        let skip = should_skip_knowledge_record(&record).expect("sensitive should be skipped");
        assert!(skip.reason.contains("敏感信息"));
    }

    #[test]
    fn email_record_is_skipped() {
        let mut record = runtime_record("k7", "summary long enough for email sensitive check");
        record.content = "contact: user.demo@example.com".to_string();
        let skip = should_skip_knowledge_record(&record).expect("email should be skipped");
        assert!(skip.reason.contains("敏感信息"));
    }

    #[test]
    fn phone_record_is_skipped() {
        let mut record = runtime_record("k8", "summary long enough for phone sensitive check");
        record.content = "联系电话 13800138000".to_string();
        let skip = should_skip_knowledge_record(&record).expect("phone should be skipped");
        assert!(skip.reason.contains("敏感信息"));
    }

    #[test]
    fn id_card_record_is_skipped() {
        let mut record = runtime_record("k9", "summary long enough for id-card sensitive check");
        record.content = "身份证 11010519491231002X".to_string();
        let skip = should_skip_knowledge_record(&record).expect("id card should be skipped");
        assert!(skip.reason.contains("敏感信息"));
    }

    #[test]
    fn normal_record_not_skipped() {
        let record = runtime_record("k6", "summary long enough without secret marker");
        assert!(should_skip_knowledge_record(&record).is_none());
    }

    fn runtime_record(id: &str, summary: &str) -> KnowledgeRecord {
        KnowledgeRecord {
            id: id.to_string(),
            knowledge_type: "workflow_pattern".to_string(),
            title: "title".to_string(),
            summary: summary.to_string(),
            content: "content".to_string(),
            tags: vec!["agent_resolve".to_string()],
            source: "run:1".to_string(),
            source_type: "runtime".to_string(),
            verified: true,
            workspace_id: "main".to_string(),
            priority: 0,
            archived: false,
            created_at: "1".to_string(),
            updated_at: "1".to_string(),
        }
    }
}
