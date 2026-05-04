// Runtime Types
export type RunState = "idle" | "running" | "awaiting_confirmation" | "completed" | "failed"
export type ConnectionState = "connected" | "connecting" | "closed" | "disconnected"
export type ViewType = "task" | "logs" | "knowledge" | "settings"
export type AgentMode = "observe" | "standard" | "full_access"

// Result Block Types
export interface CodeBlock {
  type: "code"
  language: string
  content: string
}

export interface TextBlock {
  type: "text"
  content: string
}

export interface DataGridBlock {
  type: "data_grid"
  headers: string[]
  rows: string[][]
}

export interface ListBlock {
  type: "list"
  items: string[]
}

export type ResultBlock = CodeBlock | TextBlock | DataGridBlock | ListBlock

// Message Types
export interface Message {
  id: string
  role: "user" | "assistant"
  content: string
  blocks?: ResultBlock[]
  timestamp: string
  isStreaming?: boolean
}

// Event Types
export interface RuntimeEvent {
  event_id: string
  event_type: string
  stage: string
  summary: string
  timestamp: string
  run_id?: string
  session_id?: string
  sequence?: number
  trace_id?: string
  detail?: string
  metadata?: Record<string, unknown>
  details?: Record<string, unknown>
}

// Confirmation Types
export interface Confirmation {
  confirmation_id: string
  run_id: string
  risk_level: "low" | "medium" | "high" | "critical"
  action_summary: string
  reason: string
  target_paths: string[]
  hazards: string[]
  alternatives: string[]
}

// Runtime State
export interface RuntimeState {
  sessionId: string
  currentRunId: string
  currentTaskTitle: string
  runState: RunState
  connectionState: ConnectionState
  messages: Message[]
  events: RuntimeEvent[]
  confirmation: Confirmation | null
  composeValue: string
  criticalError: string | null
  submitError: string | null
}

// Settings Types
export interface Model {
  model_id: string
  display_name: string
  provider_id: string
  enabled?: boolean
  available?: boolean
}

export interface Workspace {
  workspace_id: string
  name: string
  root_path: string
  is_active?: boolean
}

export interface Provider {
  provider_id: string
  display_name: string
  base_url: string
  chat_completions_path?: string
  models_path?: string
  credential_kind?: string
  supports_test?: boolean
  editable?: boolean
  embedding_model?: string
  credential_status?: ProviderCredentialStatus
  status?: "active" | "inactive" | "error"
}

export interface ProviderCredentialStatus {
  has_credential: boolean
  api_key_masked?: string
  updated_at?: string
  last_test_status?: string
  last_test_message?: string
  last_test_at?: string
  apply_status: string
  applied_at?: string
  pending_reload: boolean
}

export interface DirectoryApproval {
  approval_id: string
  workspace_id: string
  name: string
  root_path: string
  created_at?: string
}

export interface RuntimeStatusInfo {
  ok: boolean
  name: string
  version: string
}

export interface EmbeddingInfo {
  provider_id: string
  model_name: string
}

export interface MCPTool {
  name: string
  description: string
  inputSchema?: Record<string, unknown>
  server_id?: string
  allowed: boolean
  risk_level: "low" | "medium" | "high" | "critical"
  requires_confirmation: boolean
  audit_enabled: boolean
  policy_source: string
}

export interface MCPServerInfo {
  id: string
  name: string
  type: string
  url: string
  enabled: boolean
  ready: boolean
  tool_count: number
  allowed_tool_count: number
  blocked_tool_count: number
  requires_policy: boolean
}

export interface MCPInfo {
  servers: MCPServerInfo[]
  tools: MCPTool[]
}

export interface Settings {
  mode: AgentMode
  model: Model
  workspace: Workspace
  embedding_provider_id: string
  providers: Provider[]
  active_provider_id?: string
  available_models: Model[]
  available_workspaces: Workspace[]
  approved_directories: DirectoryApproval[]
  directory_prompt_enabled: boolean
  show_risk_level: boolean
  ports: Record<string, number>
  runtime_status?: RuntimeStatusInfo
  embedding?: EmbeddingInfo
  mcp?: MCPInfo
}

// Knowledge Types
export interface KnowledgeItem {
  id: string
  title: string
  summary: string
  content: string
  category: string
  tags: string[]
  citationCount: number
  source: string
  createdAt: string
  updatedAt: string
}

export interface KnowledgeCategory {
  id: string
  name: string
  color: string
  count: number
}

// Log Types
export type LogStatus = "completed" | "failed" | "running"

export interface LogRun {
  run_id: string
  session_id: string
  title: string
  status: LogStatus
  started_at: string
  completed_at?: string
  duration_ms: number
  event_count: number
}

export interface LogRunDetails {
  run: LogRun
  events: RuntimeEvent[]
  tool_calls: ToolCall[]
  validation: ValidationResult[]
  risks: Risk[]
  metadata: Record<string, unknown>
  context: Record<string, unknown>
}

export interface ToolCall {
  id: string
  name: string
  arguments: Record<string, unknown>
  result?: unknown
  duration_ms: number
  status: "success" | "error"
}

export interface ValidationResult {
  id: string
  type: string
  passed: boolean
  message: string
}

export interface Risk {
  id: string
  level: "low" | "medium" | "high" | "critical"
  description: string
  mitigation?: string
}

// Memory Types
export interface Memory {
  id: string
  kind: string
  title: string
  summary: string
  content: string
  createdAt: string
  sourceRunId?: string
}
