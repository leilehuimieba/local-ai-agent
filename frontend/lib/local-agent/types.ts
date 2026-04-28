// Runtime Types
export type RunState = "idle" | "running" | "awaiting_confirmation" | "completed" | "failed"
export type ConnectionState = "connected" | "connecting" | "closed" | "disconnected"
export type ViewType = "task" | "logs" | "knowledge" | "settings"
export type AgentMode = "observer" | "standard" | "full_access"

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
}

export interface Workspace {
  id: string
  name: string
  root_path: string
}

export interface Provider {
  provider_id: string
  display_name: string
  base_url: string
  api_key?: string
  embedding_model?: string
  status?: "active" | "inactive" | "error"
}

export interface Settings {
  mode: AgentMode
  model: Model
  workspace: Workspace
  embedding_provider_id: string
  providers: Provider[]
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


