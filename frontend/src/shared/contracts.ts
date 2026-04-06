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

export type ChatMessage = {
  id: string;
  role: "user" | "assistant";
  content: string;
  runId?: string;
};
