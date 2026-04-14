use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::paths::{artifact_dir, artifact_index_path};
use crate::storage::{append_jsonl, write_artifact};
use crate::text::summarize_text;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ArtifactRecord {
    pub kind: String,
    pub path: String,
    pub preview: String,
    pub content_chars: usize,
    pub created_at: String,
}

pub(crate) fn externalize_text_artifact(
    request: &RunRequest,
    kind: &str,
    content: &str,
) -> Option<ArtifactRecord> {
    if content.chars().count() < 240 {
        return None;
    }
    let file_name = format!("{}-{}.txt", kind, timestamp_now());
    let path = artifact_dir(request).join(PathBuf::from(file_name));
    create_artifact_record(request, kind, content, path).ok()
}

pub(crate) fn externalize_text_artifact_always(
    request: &RunRequest,
    kind: &str,
    content: &str,
) -> Option<ArtifactRecord> {
    if content.trim().is_empty() {
        return None;
    }
    let file_name = format!("{}-{}.txt", kind, timestamp_now());
    let path = artifact_dir(request).join(PathBuf::from(file_name));
    create_artifact_record(request, kind, content, path).ok()
}

pub(crate) fn externalize_json_artifact<T>(
    request: &RunRequest,
    kind: &str,
    value: &T,
) -> Option<ArtifactRecord>
where
    T: Serialize,
{
    let content = serde_json::to_string_pretty(value).ok()?;
    let file_name = format!("{}-{}.json", kind, timestamp_now());
    let path = artifact_dir(request).join(PathBuf::from(file_name));
    create_artifact_record(request, kind, &content, path).ok()
}

fn create_artifact_record(
    request: &RunRequest,
    kind: &str,
    content: &str,
    path: PathBuf,
) -> Result<ArtifactRecord, String> {
    let saved_path = write_artifact(path, content)?;
    let record = ArtifactRecord {
        kind: kind.to_string(),
        path: saved_path,
        preview: summarize_text(content),
        content_chars: content.chars().count(),
        created_at: timestamp_now(),
    };
    append_jsonl(artifact_index_path(request), &record)?;
    Ok(record)
}
