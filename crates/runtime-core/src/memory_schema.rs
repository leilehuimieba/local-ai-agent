use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum MemoryKind {
    Preference,
    ProjectRule,
    WorkspaceSummary,
    WorkflowPattern,
    ProjectKnowledge,
    LessonLearned,
    DailyNote,
    TaskOutcome,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct StructuredMemoryEntry {
    pub id: String,
    #[serde(default, alias = "kind")]
    pub memory_type: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub source_run_id: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub source_type: String,
    #[serde(default)]
    pub source_title: String,
    #[serde(default)]
    pub source_event_type: String,
    #[serde(default)]
    pub source_artifact_path: String,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub archived_at: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub timestamp: String,
}

pub(crate) fn normalize_memory_kind(value: &str) -> MemoryKind {
    match value.trim().to_lowercase().as_str() {
        "preference" | "workflow_preference" | "user_preference" | "偏好" => {
            MemoryKind::Preference
        }
        "project_rule" | "项目规则" => MemoryKind::ProjectRule,
        "workspace_summary" | "工作区摘要" => MemoryKind::WorkspaceSummary,
        "workflow_pattern" | "流程模式" => MemoryKind::WorkflowPattern,
        "daily_note" | "daily" | "日报" | "每日记录" => MemoryKind::DailyNote,
        "lesson" | "lesson_learned" | "经验" | "教训" => MemoryKind::LessonLearned,
        "task_outcome" | "任务结果" => MemoryKind::TaskOutcome,
        _ => MemoryKind::ProjectKnowledge,
    }
}

pub(crate) fn memory_kind_label(kind: &MemoryKind) -> &'static str {
    match kind {
        MemoryKind::Preference => "preference",
        MemoryKind::ProjectRule => "project_rule",
        MemoryKind::WorkspaceSummary => "workspace_summary",
        MemoryKind::WorkflowPattern => "workflow_pattern",
        MemoryKind::ProjectKnowledge => "project_knowledge",
        MemoryKind::LessonLearned => "lesson_learned",
        MemoryKind::DailyNote => "daily_note",
        MemoryKind::TaskOutcome => "task_outcome",
    }
}

pub(crate) fn canonical_kind(value: &str) -> String {
    let kind = normalize_memory_kind(value);
    memory_kind_label(&kind).to_string()
}

pub(crate) fn canonical_kind_for_record(value: &str, title: &str, summary: &str) -> String {
    if looks_like_preference_record(title, summary) {
        return "preference".to_string();
    }
    canonical_kind(value)
}

fn looks_like_preference_record(title: &str, summary: &str) -> bool {
    title.contains("用户偏好：")
        || summary.contains("用户偏好：")
        || summary.contains("prefer chinese replies")
        || summary.contains("继续使用中文")
}
