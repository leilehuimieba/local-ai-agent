use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const RUNTIME_NAME: &str = "runtime-host";
pub const RUNTIME_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug)]
pub struct RuntimeSnapshot {
    pub state: &'static str,
    pub current_run_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelRef {
    pub provider_id: String,
    pub model_id: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ProviderRef {
    pub provider_id: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub chat_completions_path: String,
    #[serde(default)]
    pub models_path: String,
    #[serde(default)]
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceRef {
    pub workspace_id: String,
    pub name: String,
    pub root_path: String,
    pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepoContextSnapshot {
    pub workspace_root: String,
    pub repo_root: Option<String>,
    pub git_available: bool,
    pub git_snapshot: Option<GitSnapshot>,
    #[serde(default)]
    pub doc_summaries: Vec<WorkspaceDocSummary>,
    #[serde(default)]
    pub warnings: Vec<String>,
    pub collected_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GitSnapshot {
    pub current_branch: Option<String>,
    pub default_branch: Option<String>,
    pub is_dirty: bool,
    #[serde(default)]
    pub recent_commits: Vec<GitCommitSummary>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GitCommitSummary {
    pub commit_id: String,
    pub short_message: String,
    pub author: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceDocSummary {
    pub path: String,
    pub kind: String,
    pub exists: bool,
    pub summary: String,
    pub truncated: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilitySpec {
    pub capability_id: String,
    pub display_name: String,
    pub domain: String,
    pub risk_level: String,
    pub input_schema: String,
    pub output_kind: String,
    pub side_effect_level: String,
    #[serde(default)]
    pub supports_modes: Vec<String>,
    pub verification_policy: String,
    #[serde(default)]
    pub connector_slot: String,
    pub source_kind: String,
    #[serde(default)]
    pub requires_confirmation: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityListResponse {
    #[serde(default)]
    pub items: Vec<CapabilitySpec>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectorSlotSpec {
    pub slot_id: String,
    pub display_name: String,
    pub priority: u8,
    pub status: String,
    pub scope: String,
    #[serde(default)]
    pub current_capabilities: Vec<String>,
    #[serde(default)]
    pub supported_actions: Vec<String>,
    pub boundary: String,
    pub next_step: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectorListResponse {
    #[serde(default)]
    pub items: Vec<ConnectorSlotSpec>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmationDecision {
    pub confirmation_id: String,
    pub run_id: String,
    pub decision: String,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub remember: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmationRequest {
    pub confirmation_id: String,
    pub run_id: String,
    pub risk_level: String,
    pub action_summary: String,
    pub reason: String,
    pub impact_scope: String,
    pub target_paths: Vec<String>,
    pub reversible: bool,
    pub hazards: Vec<String>,
    pub alternatives: Vec<String>,
    pub kind: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub error_code: String,
    pub message: String,
    pub summary: String,
    pub retryable: bool,
    pub source: String,
    pub stage: String,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunRequest {
    pub request_id: String,
    pub run_id: String,
    pub session_id: String,
    pub trace_id: String,
    pub user_input: String,
    pub mode: String,
    pub model_ref: ModelRef,
    #[serde(default)]
    pub provider_ref: ProviderRef,
    pub workspace_ref: WorkspaceRef,
    #[serde(default)]
    pub context_hints: BTreeMap<String, String>,
    #[serde(default)]
    pub resume_from_checkpoint_id: String,
    #[serde(default)]
    pub resume_strategy: String,
    pub confirmation_decision: Option<ConfirmationDecision>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RuntimeContextSnapshot {
    #[serde(default)]
    pub workspace_root: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub session_summary: String,
    #[serde(default)]
    pub memory_digest: String,
    #[serde(default)]
    pub knowledge_digest: String,
    #[serde(default)]
    pub tool_preview: String,
    #[serde(default)]
    pub reasoning_summary: String,
    #[serde(default)]
    pub cache_status: String,
    #[serde(default)]
    pub cache_reason: String,
    #[serde(default)]
    pub assembly_profile: String,
    #[serde(default)]
    pub includes_session: bool,
    #[serde(default)]
    pub includes_memory: bool,
    #[serde(default)]
    pub includes_knowledge: bool,
    #[serde(default)]
    pub includes_tool_preview: bool,
    #[serde(default)]
    pub skill_injection_enabled: bool,
    #[serde(default)]
    pub max_skill_level: String,
    #[serde(default)]
    pub injected_skill_level: String,
    #[serde(default)]
    pub injected_skill_ids: String,
    #[serde(default)]
    pub evidence_refs: String,
    #[serde(default)]
    pub phase_label: String,
    #[serde(default)]
    pub selection_reason: String,
    #[serde(default)]
    pub prefers_artifact_context: bool,
    #[serde(default)]
    pub artifact_hint: String,
    #[serde(default)]
    pub observation_injection: String,
    #[serde(default)]
    pub observation_references: String,
    #[serde(default)]
    pub observation_budget_total: usize,
    #[serde(default)]
    pub observation_budget_used: usize,
    #[serde(default)]
    pub observation_budget_hit: bool,
    #[serde(default)]
    pub observation_budget_total_tokens: usize,
    #[serde(default)]
    pub observation_budget_used_tokens: usize,
    #[serde(default)]
    pub observation_budget_hit_tokens: bool,
    #[serde(default)]
    pub prompt_static: String,
    #[serde(default)]
    pub prompt_project: String,
    #[serde(default)]
    pub prompt_dynamic: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ToolCallSnapshot {
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub risk_level: String,
    #[serde(default)]
    pub input_schema: String,
    #[serde(default)]
    pub output_kind: String,
    #[serde(default)]
    pub requires_confirmation: bool,
    #[serde(default)]
    pub arguments_json: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct VerificationSnapshot {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub passed: bool,
    #[serde(default)]
    pub policy: String,
    #[serde(default)]
    pub evidence: Vec<String>,
    #[serde(default)]
    pub skill_hit_effective: bool,
    #[serde(default)]
    pub skill_hit_reason: String,
    #[serde(default)]
    pub guard_downgraded: bool,
    #[serde(default)]
    pub guard_decision_ref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunEvent {
    pub event_id: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub record_type: String,
    #[serde(default)]
    pub source_type: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub agent_label: String,
    pub event_type: String,
    #[serde(default)]
    pub trace_id: String,
    pub session_id: String,
    pub run_id: String,
    pub sequence: u32,
    pub timestamp: String,
    pub stage: String,
    pub summary: String,
    #[serde(default)]
    pub detail: String,
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub tool_display_name: String,
    #[serde(default)]
    pub tool_category: String,
    #[serde(default)]
    pub output_kind: String,
    #[serde(default)]
    pub result_summary: String,
    #[serde(default)]
    pub artifact_path: String,
    #[serde(default)]
    pub risk_level: String,
    #[serde(default)]
    pub confirmation_id: String,
    #[serde(default)]
    pub final_answer: String,
    #[serde(default)]
    pub completion_status: String,
    #[serde(default)]
    pub completion_reason: String,
    #[serde(default)]
    pub verification_summary: String,
    #[serde(default)]
    pub checkpoint_written: bool,
    #[serde(default)]
    pub context_snapshot: Option<RuntimeContextSnapshot>,
    #[serde(default)]
    pub tool_call_snapshot: Option<ToolCallSnapshot>,
    #[serde(default)]
    pub verification_snapshot: Option<VerificationSnapshot>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunResult {
    #[serde(default)]
    pub request_id: String,
    pub run_id: String,
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub source: String,
    pub status: String,
    pub final_answer: String,
    pub summary: String,
    pub error: Option<ErrorInfo>,
    pub memory_write_summary: Option<String>,
    pub final_stage: String,
    #[serde(default)]
    pub checkpoint_id: Option<String>,
    #[serde(default)]
    pub resumable: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeRunResponse {
    pub events: Vec<RunEvent>,
    pub result: RunResult,
    pub confirmation_request: Option<ConfirmationRequest>,
}

impl RuntimeSnapshot {
    pub fn idle() -> Self {
        Self {
            state: "Analyze",
            current_run_id: None,
        }
    }
}
