import type {
  ModelRef,
  ProviderRef,
  WorkspaceRef,
} from "./base";

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

export type DirectoryApproval = {
  approval_id: string;
  workspace_id: string;
  name: string;
  root_path: string;
  created_at?: string;
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

export type EmbeddingInfo = {
  provider_id: string;
  model_name: string;
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
  embedding: EmbeddingInfo;
};
