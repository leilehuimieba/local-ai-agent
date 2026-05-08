// Runtime Types
export type RunState = "idle" | "running" | "awaiting_confirmation" | "completed" | "failed"
export type ConnectionState = "connected" | "connecting" | "closed" | "disconnected"
export type ViewType = "task" | "logs" | "knowledge" | "settings"
export type AgentMode = "observe" | "standard" | "full_access"
export type MainlineRiskLevel = "green" | "yellow" | "orange" | "red"
export type MainlineProbabilityState = "known" | "unknown"
export type KeyEvidenceSourceType = "mock_exam" | "real_exam"
export type MainlineSwitchReviewStatus = "idle" | "approved" | "rejected"
export type TimeBudgetLevel = "ample" | "steady" | "tight" | "critical"
export type EveningReviewStatus = "pending" | "submitted" | "late_allowed" | "reroute"
export type LateEvidenceDecision = "none" | "make_up_yesterday" | "continue_today"
export type MainlineExecutionStatus = "idle" | "executing" | "completed" | "closed"
export type PersonalizedTaskType =
  | "vocabulary"
  | "mock_exam"
  | "reading"
  | "listening"
  | "writing_translation"
  | "professional_skill"
  | "algorithm"
  | "other"

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
  risk_level: "low" | "medium" | "high" | "critical" | "irreversible"
  action_summary: string
  reason: string
  target_paths: string[]
  hazards: string[]
  alternatives: string[]
  tool_name?: string
  tool_arguments_json?: string
  patch_preview_report_json?: string
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

export interface EvidencePacket {
  didWhat: string
  resultSummary: string
  blockers: string
  nextAdjustment: string
}

export interface KeyEvidenceEntry {
  sourceType: KeyEvidenceSourceType
  completedDate: string
  durationMinutes: string
  underExamCondition: boolean | null
  totalScore: string
  listeningScore: string
  readingScore: string
  writingTranslationScore: string
  notes: string
}

export interface KeyEvidenceRecord {
  sourceType: KeyEvidenceSourceType
  completedDate: string
  durationMinutes: number
  underExamCondition: boolean
  totalScore: number
  listeningScore: number
  readingScore: number
  writingTranslationScore: number
  notes: string
  submittedAt: string
  probabilityAfter: number | null
}

export interface MainlineSnapshot {
  currentGoalLabel: string
  probabilityValue: number | null
  probabilityState: MainlineProbabilityState
  riskLevel: MainlineRiskLevel
  evidenceExpired: boolean
  lastCriticalEvidenceAt: string | null
  latestAdjustment: string
}

export interface TemporaryMainlineForm {
  temporaryGoalLabel: string
  reason: string
  dueDate: string
  urgent: boolean
  important: boolean
  insistAfterReject: boolean
}

export interface TemporaryMainlineHistory {
  temporaryGoalLabel: string
  originalGoalLabel: string
  reason: string
  dueDate: string
  approvedByReview: boolean
  userInsisted: boolean
  switchedAt: string
  restoredAt: string | null
}

export interface TemporaryMainlineState {
  isActive: boolean
  reviewStatus: MainlineSwitchReviewStatus
  reviewFeedback: string | null
  currentTemporaryGoal: string | null
  originalSnapshot: MainlineSnapshot | null
  currentSwitchStartedAt: string | null
  activeRecordId: string | null
  history: TemporaryMainlineHistory[]
}

export interface PersonalizedFollowupEntry {
  taskType: PersonalizedTaskType
  isCoreTask: boolean
  completed: boolean | null
  hasShortTermGain: boolean | null
  resultNote: string
}

export interface PersonalizedFollowupRecord {
  taskType: PersonalizedTaskType
  isCoreTask: boolean
  completed: boolean
  hasShortTermGain: boolean
  resultNote: string
  submittedAt: string
}

export interface PersonalizedFollowupInsight {
  updatedAt: string | null
  basedOnCount: number
  preferredTaskType: PersonalizedTaskType | null
  boostTaskType: PersonalizedTaskType | null
  riskTaskType: PersonalizedTaskType | null
  recommendation: string
}

export interface TimeBlockDraft {
  startTime: string
  endTime: string
  label: string
}

export interface TimeBlockItem {
  startTime: string
  endTime: string
  label: string
  durationMinutes: number
}

export interface TimeBudgetEntry {
  coreTaskLabel: string
  budgetChangeNote: string
  timeBlockDraft: TimeBlockDraft
}

export interface TimeBudgetInsight {
  updatedAt: string | null
  totalAvailableMinutes: number
  level: TimeBudgetLevel
  recommendation: string
}

export interface EveningReviewState {
  scheduledLabel: string
  status: EveningReviewStatus
  statusText: string
  guidance: string
  deadlineLabel: string | null
  branchDecision: LateEvidenceDecision
  branchText: string
  lastReviewedAt: string | null
  lastSubmittedForDate: string | null
}

export interface FollowthroughState {
  lastActionLabel: string | null
  nextActionLabel: string
  nextActionHelper: string
  planUsesLatestEvidence: boolean | null
}

export interface ExecutionState {
  status: MainlineExecutionStatus
  currentTaskLabel: string | null
  todayCoreTaskCompleted: boolean
  todayClosed: boolean
  statusText: string
  helperText: string
  activeDate: string | null
  needsReopen: boolean
  staleFromDate: string | null
}

export interface NextDayPlanState {
  generatedAt: string | null
  ready: boolean
  coreTaskLabel: string
  supportTaskLabel: string
  rationale: string
  restoreNote: string | null
  targetDate: string | null
  basedOnEvidenceDate: string | null
  needsRefresh: boolean
  statusText: string
  refreshReason: string | null
  todayTaskLabel: string | null
  takeoverStatus: "idle" | "pending_today" | "taken_over" | "completed" | "closed"
  takeoverHint: string | null
}

export interface PlanHistoryItem {
  targetDate: string
  taskLabel: string
  status: "taken_over" | "completed" | "closed"
  statusText: string
  updatedAt: string
  reconciled: boolean
  reconcileText: string
}

export interface MainlineShellState {
  currentGoalLabel: string
  probabilityValue: number | null
  probabilityState: MainlineProbabilityState
  riskLevel: MainlineRiskLevel
  evidenceExpired: boolean
  evidenceSubmitted: boolean
  lastCriticalEvidenceAt: string | null
  latestAdjustment: string
  keyEvidenceFeedback: string | null
  switchFeedback: string | null
  evidencePacket: EvidencePacket
  keyEvidenceEntry: KeyEvidenceEntry
  keyEvidenceHistory: KeyEvidenceRecord[]
  switchForm: TemporaryMainlineForm
  temporaryMainline: TemporaryMainlineState
  personalizedFeedback: string | null
  personalizedEntry: PersonalizedFollowupEntry
  personalizedHistory: PersonalizedFollowupRecord[]
  personalizedInsight: PersonalizedFollowupInsight
  timeBudgetFeedback: string | null
  timeBudgetEntry: TimeBudgetEntry
  timeBlocks: TimeBlockItem[]
  timeBudgetInsight: TimeBudgetInsight
  eveningReview: EveningReviewState
  followthrough: FollowthroughState
  execution: ExecutionState
  nextDayPlan: NextDayPlanState
  todayPlanStatusText: string
  todayPlanReconcileText: string | null
  recentPlanHistory: PlanHistoryItem[]
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

export interface MCPAuditRecord {
  audit_id: string
  timestamp: string
  server_id: string
  tool_name: string
  session_id?: string
  run_id?: string
  trace_id?: string
  allowed: boolean
  risk_level: string
  requires_confirmation: boolean
  audit_enabled: boolean
  policy_source: string
  arguments_hash: string
  outcome: string
  error_code?: string
  error_message?: string
  elapsed_ms: number
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
