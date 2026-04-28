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
  embedding_model?: string;
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
