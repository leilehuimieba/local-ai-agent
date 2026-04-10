export type ErrorInfo = {
  error_code: string;
  message: string;
  summary: string;
  retryable: boolean;
  source: string;
  stage: string;
  metadata?: Record<string, string>;
};

export type ModelRef = {
  provider_id: string;
  model_id: string;
  display_name: string;
  enabled?: boolean;
  available?: boolean;
};

export type ProviderRef = {
  provider_id: string;
  display_name: string;
  base_url: string;
  chat_completions_path: string;
  models_path: string;
};

export type ProviderApplyStatus = "not_configured" | "saved_not_applied" | "applied";

export type ProviderTestStatus = "idle" | "success" | "error";

export type ProviderCredentialStatus = {
  has_credential: boolean;
  api_key_masked?: string;
  updated_at?: string;
  last_test_status?: ProviderTestStatus;
  last_test_message?: string;
  last_test_at?: string;
  apply_status: ProviderApplyStatus;
  applied_at?: string;
  pending_reload: boolean;
};

export type ProviderSettingsItem = ProviderRef & {
  credential_kind: string;
  supports_test: boolean;
  editable: boolean;
  credential_status: ProviderCredentialStatus;
};

export type ProviderSettingsResponse = {
  active_provider_id?: string;
  providers: ProviderSettingsItem[];
};

export type ProviderTestRequest = {
  provider_id: string;
  base_url?: string;
  api_key: string;
};

export type ProviderTestResponse = {
  ok: boolean;
  provider_id: string;
  message: string;
  checked_at?: string;
  error_code?: string;
};

export type ProviderSaveRequest = {
  provider_id: string;
  api_key: string;
};

export type ProviderSaveResponse = {
  ok: boolean;
  provider_id: string;
  message: string;
  credential_status: ProviderCredentialStatus;
};

export type ProviderApplyRequest = {
  provider_id: string;
};

export type ProviderApplyResponse = {
  ok: boolean;
  provider_id: string;
  message: string;
  apply_mode: string;
  applied_at?: string;
  restart_required: boolean;
};

export type ProviderRemoveRequest = {
  provider_id: string;
};

export type ProviderRemoveResponse = {
  ok: boolean;
  provider_id: string;
  message: string;
  state_code?: string;
};

export type WorkspaceRef = {
  workspace_id: string;
  name: string;
  root_path: string;
  is_active: boolean;
};

export type GitCommitSummary = {
  commit_id: string;
  short_message: string;
  author?: string;
  timestamp?: string;
};

export type GitSnapshot = {
  current_branch?: string;
  default_branch?: string;
  is_dirty: boolean;
  recent_commits?: GitCommitSummary[];
};

export type WorkspaceDocSummary = {
  path: string;
  kind: string;
  exists: boolean;
  summary: string;
  truncated: boolean;
};

export type CapabilitySpec = {
  capability_id: string;
  display_name: string;
  domain: string;
  risk_level: string;
  input_schema: string;
  output_kind: string;
  side_effect_level: string;
  supports_modes?: string[];
  verification_policy: string;
  connector_slot?: string;
  source_kind: string;
  requires_confirmation?: boolean;
};

export type CapabilityListResponse = {
  items: CapabilitySpec[];
};

export type ConnectorSlotSpec = {
  slot_id: string;
  display_name: string;
  priority: number;
  status: string;
  scope: string;
  current_capabilities?: string[];
  supported_actions?: string[];
  boundary: string;
  next_step: string;
};

export type ConnectorListResponse = {
  items: ConnectorSlotSpec[];
};

export type RepoContextSnapshot = {
  workspace_root: string;
  repo_root?: string;
  git_available: boolean;
  git_snapshot?: GitSnapshot;
  doc_summaries?: WorkspaceDocSummary[];
  warnings?: string[];
  collected_at: string;
};

export type SettingsResponse = {
  app_name: string;
  mode: string;
  model: ModelRef;
  available_models: ModelRef[];
  providers: ProviderRef[];
  workspace: WorkspaceRef;
  available_workspaces: WorkspaceRef[];
  approved_directories: DirectoryApproval[];
  directory_prompt_enabled: boolean;
  show_risk_level: boolean;
  ports: {
    gateway: number;
    runtime: number;
  };
  runtime_status: {
    ok: boolean;
    name: string;
    version: string;
  };
  memory_policy: MemoryPolicyStatus;
  diagnostics: DiagnosticsStatus;
  external_connections: ExternalConnectionSlot[];
};

export type MemoryPolicyStatus = {
  enabled: boolean;
  recall_strategy: string;
  write_strategy: string;
  cleanup_strategy: string;
  storage_root: string;
  sqlite_path: string;
  working_memory_dir: string;
  long_term_memory_path: string;
  knowledge_base_path: string;
  long_term_memory_count: number;
  knowledge_count: number;
  working_memory_files: number;
};

export type DiagnosticsStatus = {
  checked_at?: string;
  repo_root: string;
  storage_root: string;
  settings_path: string;
  run_log_path: string;
  event_log_path: string;
  runtime_reachable: boolean;
  runtime_version: string;
  provider_count: number;
  model_count: number;
  workspace_count: number;
  approved_directory_count: number;
  siyuan_root: string;
  siyuan_export_dir: string;
  siyuan_auto_write_enabled: boolean;
  siyuan_sync_enabled: boolean;
  warnings?: string[];
  errors?: string[];
};

export type ExternalConnectionSlot = {
  slot_id: string;
  display_name: string;
  priority: number;
  status: string;
  scope: string;
  current_tools: string[];
  supported_actions?: Array<"validate" | "recheck">;
  boundary: string;
  next_step: string;
};

export type ExternalConnectionActionRequest = {
  slot_id: string;
  action: "validate" | "recheck";
};

export type ExternalConnectionActionResponse = {
  slot_id: string;
  action: "validate" | "recheck";
  ok: boolean;
  message: string;
  updated_slot?: ExternalConnectionSlot;
  external_connections?: ExternalConnectionSlot[];
};

export type DiagnosticsCheckResponse = {
  checked_at: string;
  overall_ok: boolean;
  diagnostics: DiagnosticsStatus;
  warnings: string[];
  errors: string[];
};

export type DirectoryApproval = {
  approval_id: string;
  workspace_id: string;
  name: string;
  root_path: string;
  created_at?: string;
};

export type LogsResponse = {
  items: LogEntry[];
};

export type MemoryEntry = {
  id: string;
  kind: string;
  memory_kind?: string;
  title: string;
  summary: string;
  content: string;
  reason: string;
  scope: string;
  workspace_id: string;
  session_id: string;
  source_run_id: string;
  source: string;
  source_type: string;
  source_title?: string;
  source_event_type?: string;
  source_artifact_path?: string;
  governance_version?: string;
  governance_reason?: string;
  governance_source?: string;
  governance_at?: string;
  governance_status?: string;
  memory_action?: string;
  archive_reason?: string;
  verified: boolean;
  priority: number;
  archived: boolean;
  archived_at?: string;
  created_at: string;
  updated_at: string;
  timestamp: string;
};

export type MemoryListResponse = {
  items: MemoryEntry[];
};

export type ChatRunAccepted = {
  accepted: boolean;
  session_id: string;
  run_id: string;
  initial_status: string;
};

export type RuntimeRunRequest = {
  request_id: string;
  run_id: string;
  session_id: string;
  trace_id: string;
  user_input: string;
  mode: string;
  model_ref: ModelRef;
  provider_ref: ProviderRef;
  workspace_ref: WorkspaceRef;
  context_hints?: Record<string, string>;
  resume_from_checkpoint_id?: string;
  resume_strategy?: string;
  confirmation_decision?: ConfirmationDecision;
};

export type RunResult = {
  request_id?: string;
  run_id: string;
  session_id?: string;
  trace_id?: string;
  kind?: string;
  source?: string;
  status: string;
  final_answer: string;
  summary: string;
  error?: ErrorInfo;
  memory_write_summary?: string;
  final_stage: string;
  checkpoint_id?: string;
  resumable?: boolean;
};

export type RuntimeRunResponse = {
  events: RunEvent[];
  result: RunResult;
  confirmation_request?: ConfirmationRequest;
};

export type RunEvent = {
  event_id: string;
  kind?: string;
  source?: string;
  record_type?: string;
  source_type?: string;
  agent_id?: string;
  agent_label?: string;
  event_type: string;
  trace_id?: string;
  session_id: string;
  run_id: string;
  sequence: number;
  timestamp: string;
  stage: string;
  summary: string;
  detail?: string;
  tool_name?: string;
  tool_display_name?: string;
  tool_category?: string;
  output_kind?: string;
  result_summary?: string;
  artifact_path?: string;
  risk_level?: string;
  confirmation_id?: string;
  final_answer?: string;
  completion_status?: string;
  completion_reason?: string;
  verification_summary?: string;
  checkpoint_written?: boolean;
  context_snapshot?: RuntimeContextSnapshot;
  tool_call_snapshot?: ToolCallSnapshot;
  verification_snapshot?: VerificationSnapshot;
  metadata?: Record<string, string>;
};

export type RuntimeContextSnapshot = {
  workspace_root?: string;
  mode?: string;
  session_summary?: string;
  memory_digest?: string;
  knowledge_digest?: string;
  tool_preview?: string;
  reasoning_summary?: string;
  cache_status?: string;
  cache_reason?: string;
  assembly_profile?: string;
  includes_session?: boolean;
  includes_memory?: boolean;
  includes_knowledge?: boolean;
  includes_tool_preview?: boolean;
  prompt_static?: string;
  prompt_project?: string;
  prompt_dynamic?: string;
};

export type ToolCallSnapshot = {
  tool_name?: string;
  display_name?: string;
  category?: string;
  risk_level?: string;
  input_schema?: string;
  output_kind?: string;
  requires_confirmation?: boolean;
};

export type VerificationSnapshot = {
  code?: string;
  summary?: string;
  passed?: boolean;
  policy?: string;
  evidence?: string[];
};

export type LogEntry = {
  log_id: string;
  session_id: string;
  run_id: string;
  timestamp: string;
  level: string;
  category: string;
  source: string;
  record_type?: string;
  source_type?: string;
  agent_id?: string;
  agent_label?: string;
  event_type?: string;
  stage?: string;
  summary: string;
  detail?: string;
  tool_name?: string;
  tool_display_name?: string;
  tool_category?: string;
  output_kind?: string;
  result_summary?: string;
  artifact_path?: string;
  risk_level?: string;
  confirmation_id?: string;
  final_answer?: string;
  completion_status?: string;
  completion_reason?: string;
  verification_summary?: string;
  context_snapshot?: RuntimeContextSnapshot;
  tool_call_snapshot?: ToolCallSnapshot;
  verification_snapshot?: VerificationSnapshot;
  error?: ErrorInfo;
  metadata?: Record<string, string>;
};

export type ConfirmationRequest = {
  confirmation_id: string;
  run_id: string;
  risk_level: string;
  action_summary: string;
  reason: string;
  impact_scope: string;
  target_paths: string[];
  reversible: boolean;
  hazards: string[];
  alternatives: string[];
  kind: string;
};

export type ConfirmationDecision = {
  confirmation_id: string;
  run_id: string;
  decision: "approve" | "reject" | "cancel";
  note?: string;
  remember?: boolean;
};

export type ChatMessage = {
  id: string;
  role: "user" | "assistant";
  content: string;
  runId?: string;
};
