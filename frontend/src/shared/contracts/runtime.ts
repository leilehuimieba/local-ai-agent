import type {
  ErrorInfo,
  ModelRef,
  ProviderRef,
  WorkspaceRef,
} from "./base";

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
  arguments_json?: string;
};

export type VerificationSnapshot = {
  code?: string;
  summary?: string;
  passed?: boolean;
  policy?: string;
  evidence?: string[];
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

export type LogsResponse = {
  items: LogEntry[];
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
