package contracts

import "local-agent/gateway/internal/config"

type ProviderRef struct {
	ProviderID          string `json:"provider_id"`
	DisplayName         string `json:"display_name"`
	BaseURL             string `json:"base_url"`
	ChatCompletionsPath string `json:"chat_completions_path"`
	ModelsPath          string `json:"models_path"`
	APIKey              string `json:"api_key"`
}

type ErrorInfo struct {
	ErrorCode string            `json:"error_code"`
	Message   string            `json:"message"`
	Summary   string            `json:"summary"`
	Retryable bool              `json:"retryable"`
	Source    string            `json:"source"`
	Stage     string            `json:"stage"`
	Metadata  map[string]string `json:"metadata,omitempty"`
}

type RunRequest struct {
	RequestID            string                `json:"request_id"`
	RunID                string                `json:"run_id"`
	SessionID            string                `json:"session_id"`
	TraceID              string                `json:"trace_id"`
	UserInput            string                `json:"user_input"`
	Mode                 string                `json:"mode"`
	ModelRef             config.ModelRef       `json:"model_ref"`
	ProviderRef          ProviderRef           `json:"provider_ref"`
	WorkspaceRef         config.WorkspaceRef   `json:"workspace_ref"`
	ContextHints         map[string]string     `json:"context_hints,omitempty"`
	ResumeFromCheckpointID string              `json:"resume_from_checkpoint_id,omitempty"`
	ResumeStrategy       string                `json:"resume_strategy,omitempty"`
	ConfirmationDecision *ConfirmationDecision `json:"confirmation_decision,omitempty"`
}

type RuntimeContextSnapshot struct {
	WorkspaceRoot    string `json:"workspace_root,omitempty"`
	Mode             string `json:"mode,omitempty"`
	SessionSummary   string `json:"session_summary,omitempty"`
	MemoryDigest     string `json:"memory_digest,omitempty"`
	KnowledgeDigest  string `json:"knowledge_digest,omitempty"`
	ToolPreview      string `json:"tool_preview,omitempty"`
	ReasoningSummary string `json:"reasoning_summary,omitempty"`
	CacheStatus      string `json:"cache_status,omitempty"`
	CacheReason      string `json:"cache_reason,omitempty"`
	AssemblyProfile  string `json:"assembly_profile,omitempty"`
	IncludesSession  bool   `json:"includes_session,omitempty"`
	IncludesMemory   bool   `json:"includes_memory,omitempty"`
	IncludesKnowledge bool  `json:"includes_knowledge,omitempty"`
	IncludesToolPreview bool `json:"includes_tool_preview,omitempty"`
	PromptStatic     string `json:"prompt_static,omitempty"`
	PromptProject    string `json:"prompt_project,omitempty"`
	PromptDynamic    string `json:"prompt_dynamic,omitempty"`
}

type ToolCallSnapshot struct {
	ToolName             string `json:"tool_name,omitempty"`
	DisplayName          string `json:"display_name,omitempty"`
	Category             string `json:"category,omitempty"`
	RiskLevel            string `json:"risk_level,omitempty"`
	InputSchema          string `json:"input_schema,omitempty"`
	OutputKind           string `json:"output_kind,omitempty"`
	RequiresConfirmation bool   `json:"requires_confirmation,omitempty"`
	ArgumentsJSON        string `json:"arguments_json,omitempty"`
}

type VerificationSnapshot struct {
	Code    string `json:"code,omitempty"`
	Summary string `json:"summary,omitempty"`
	Passed  bool   `json:"passed,omitempty"`
	Policy  string `json:"policy,omitempty"`
	Evidence []string `json:"evidence,omitempty"`
}

type RepoContextSnapshot struct {
	WorkspaceRoot string                `json:"workspace_root"`
	RepoRoot      *string               `json:"repo_root,omitempty"`
	GitAvailable  bool                  `json:"git_available"`
	GitSnapshot   *GitSnapshot          `json:"git_snapshot,omitempty"`
	DocSummaries  []WorkspaceDocSummary `json:"doc_summaries,omitempty"`
	Warnings      []string              `json:"warnings,omitempty"`
	CollectedAt   string                `json:"collected_at"`
}

type GitSnapshot struct {
	CurrentBranch *string            `json:"current_branch,omitempty"`
	DefaultBranch *string            `json:"default_branch,omitempty"`
	IsDirty       bool               `json:"is_dirty"`
	RecentCommits []GitCommitSummary `json:"recent_commits,omitempty"`
}

type GitCommitSummary struct {
	CommitID     string  `json:"commit_id"`
	ShortMessage string  `json:"short_message"`
	Author       *string `json:"author,omitempty"`
	Timestamp    *string `json:"timestamp,omitempty"`
}

type WorkspaceDocSummary struct {
	Path      string `json:"path"`
	Kind      string `json:"kind"`
	Exists    bool   `json:"exists"`
	Summary   string `json:"summary"`
	Truncated bool   `json:"truncated"`
}

type CapabilitySpec struct {
	CapabilityID         string   `json:"capability_id"`
	DisplayName          string   `json:"display_name"`
	Domain               string   `json:"domain"`
	RiskLevel            string   `json:"risk_level"`
	InputSchema          string   `json:"input_schema"`
	OutputKind           string   `json:"output_kind"`
	SideEffectLevel      string   `json:"side_effect_level"`
	SupportsModes        []string `json:"supports_modes,omitempty"`
	VerificationPolicy   string   `json:"verification_policy"`
	ConnectorSlot        string   `json:"connector_slot,omitempty"`
	SourceKind           string   `json:"source_kind"`
	RequiresConfirmation bool     `json:"requires_confirmation,omitempty"`
}

type CapabilityListResponse struct {
	Items []CapabilitySpec `json:"items"`
}

type ConnectorSlotSpec struct {
	SlotID              string   `json:"slot_id"`
	DisplayName         string   `json:"display_name"`
	Priority            int      `json:"priority"`
	Status              string   `json:"status"`
	Scope               string   `json:"scope"`
	CurrentCapabilities []string `json:"current_capabilities,omitempty"`
	SupportedActions    []string `json:"supported_actions,omitempty"`
	Boundary            string   `json:"boundary"`
	NextStep            string   `json:"next_step"`
}

type ConnectorListResponse struct {
	Items []ConnectorSlotSpec `json:"items"`
}

type RunResult struct {
	RequestID          string     `json:"request_id,omitempty"`
	RunID              string     `json:"run_id"`
	SessionID          string     `json:"session_id,omitempty"`
	TraceID            string     `json:"trace_id,omitempty"`
	Kind               string     `json:"kind,omitempty"`
	Source             string     `json:"source,omitempty"`
	Status             string     `json:"status"`
	FinalAnswer        string     `json:"final_answer"`
	Summary            string     `json:"summary"`
	Error              *ErrorInfo `json:"error,omitempty"`
	MemoryWriteSummary *string    `json:"memory_write_summary,omitempty"`
	FinalStage         string     `json:"final_stage"`
	CheckpointID       *string    `json:"checkpoint_id,omitempty"`
	Resumable          *bool      `json:"resumable,omitempty"`
}

type RunEvent struct {
	EventID              string                  `json:"event_id"`
	Kind                 string                  `json:"kind,omitempty"`
	Source               string                  `json:"source,omitempty"`
	RecordType           string                  `json:"record_type,omitempty"`
	SourceType           string                  `json:"source_type,omitempty"`
	AgentID              string                  `json:"agent_id,omitempty"`
	AgentLabel           string                  `json:"agent_label,omitempty"`
	EventType            string                  `json:"event_type"`
	TraceID              string                  `json:"trace_id,omitempty"`
	SessionID            string                  `json:"session_id"`
	RunID                string                  `json:"run_id"`
	Sequence             int                     `json:"sequence"`
	Timestamp            string                  `json:"timestamp"`
	Stage                string                  `json:"stage"`
	Summary              string                  `json:"summary"`
	Detail               string                  `json:"detail,omitempty"`
	ToolName             string                  `json:"tool_name,omitempty"`
	ToolDisplayName      string                  `json:"tool_display_name,omitempty"`
	ToolCategory         string                  `json:"tool_category,omitempty"`
	OutputKind           string                  `json:"output_kind,omitempty"`
	ResultSummary        string                  `json:"result_summary,omitempty"`
	ArtifactPath         string                  `json:"artifact_path,omitempty"`
	RiskLevel            string                  `json:"risk_level,omitempty"`
	ConfirmationID       string                  `json:"confirmation_id,omitempty"`
	FinalAnswer          string                  `json:"final_answer,omitempty"`
	CompletionStatus     string                  `json:"completion_status,omitempty"`
	CompletionReason     string                  `json:"completion_reason,omitempty"`
	VerificationSummary  string                  `json:"verification_summary,omitempty"`
	CheckpointWritten    bool                    `json:"checkpoint_written,omitempty"`
	ContextSnapshot      *RuntimeContextSnapshot `json:"context_snapshot,omitempty"`
	ToolCallSnapshot     *ToolCallSnapshot       `json:"tool_call_snapshot,omitempty"`
	VerificationSnapshot *VerificationSnapshot   `json:"verification_snapshot,omitempty"`
	Metadata             map[string]string       `json:"metadata,omitempty"`
}

type ConfirmationRequest struct {
	ConfirmationID string   `json:"confirmation_id"`
	RunID          string   `json:"run_id"`
	RiskLevel      string   `json:"risk_level"`
	ActionSummary  string   `json:"action_summary"`
	Reason         string   `json:"reason"`
	ImpactScope    string   `json:"impact_scope"`
	TargetPaths    []string `json:"target_paths"`
	Reversible     bool     `json:"reversible"`
	Hazards        []string `json:"hazards"`
	Alternatives   []string `json:"alternatives"`
	Kind           string   `json:"kind"`
}

type ConfirmationDecision struct {
	ConfirmationID string `json:"confirmation_id"`
	RunID          string `json:"run_id"`
	Decision       string `json:"decision"`
	Note           string `json:"note,omitempty"`
	Remember       bool   `json:"remember,omitempty"`
}

type LogEntry struct {
	LogID                string                  `json:"log_id"`
	SessionID            string                  `json:"session_id"`
	RunID                string                  `json:"run_id"`
	Timestamp            string                  `json:"timestamp"`
	Level                string                  `json:"level"`
	Category             string                  `json:"category"`
	Source               string                  `json:"source"`
	RecordType           string                  `json:"record_type,omitempty"`
	SourceType           string                  `json:"source_type,omitempty"`
	AgentID              string                  `json:"agent_id,omitempty"`
	AgentLabel           string                  `json:"agent_label,omitempty"`
	EventType            string                  `json:"event_type,omitempty"`
	Stage                string                  `json:"stage,omitempty"`
	Summary              string                  `json:"summary"`
	Detail               string                  `json:"detail,omitempty"`
	ToolName             string                  `json:"tool_name,omitempty"`
	ToolDisplayName      string                  `json:"tool_display_name,omitempty"`
	ToolCategory         string                  `json:"tool_category,omitempty"`
	OutputKind           string                  `json:"output_kind,omitempty"`
	ResultSummary        string                  `json:"result_summary,omitempty"`
	ArtifactPath         string                  `json:"artifact_path,omitempty"`
	RiskLevel            string                  `json:"risk_level,omitempty"`
	ConfirmationID       string                  `json:"confirmation_id,omitempty"`
	FinalAnswer          string                  `json:"final_answer,omitempty"`
	CompletionStatus     string                  `json:"completion_status,omitempty"`
	CompletionReason     string                  `json:"completion_reason,omitempty"`
	VerificationSummary  string                  `json:"verification_summary,omitempty"`
	ContextSnapshot      *RuntimeContextSnapshot `json:"context_snapshot,omitempty"`
	ToolCallSnapshot     *ToolCallSnapshot       `json:"tool_call_snapshot,omitempty"`
	VerificationSnapshot *VerificationSnapshot   `json:"verification_snapshot,omitempty"`
	Error                *ErrorInfo              `json:"error,omitempty"`
	Metadata             map[string]string       `json:"metadata,omitempty"`
}

type RuntimeRunResponse struct {
	Events              []RunEvent           `json:"events"`
	Result              RunResult            `json:"result"`
	ConfirmationRequest *ConfirmationRequest `json:"confirmation_request,omitempty"`
}

type MemoryEntry struct {
	ID                 string `json:"id"`
	Kind               string `json:"kind"`
	MemoryKind         string `json:"memory_kind,omitempty"`
	Title              string `json:"title"`
	Summary            string `json:"summary"`
	Content            string `json:"content"`
	Reason             string `json:"reason"`
	Scope              string `json:"scope"`
	WorkspaceID        string `json:"workspace_id"`
	SessionID          string `json:"session_id"`
	SourceRunID        string `json:"source_run_id"`
	Source             string `json:"source"`
	SourceType         string `json:"source_type"`
	SourceTitle        string `json:"source_title,omitempty"`
	SourceEventType    string `json:"source_event_type,omitempty"`
	SourceArtifactPath string `json:"source_artifact_path,omitempty"`
	GovernanceVersion  string `json:"governance_version,omitempty"`
	GovernanceReason   string `json:"governance_reason,omitempty"`
	GovernanceSource   string `json:"governance_source,omitempty"`
	GovernanceAt       string `json:"governance_at,omitempty"`
	GovernanceStatus   string `json:"governance_status,omitempty"`
	MemoryAction       string `json:"memory_action,omitempty"`
	ArchiveReason      string `json:"archive_reason,omitempty"`
	Verified           bool   `json:"verified"`
	Priority           int    `json:"priority"`
	Archived           bool   `json:"archived"`
	ArchivedAt         string `json:"archived_at,omitempty"`
	CreatedAt          string `json:"created_at"`
	UpdatedAt          string `json:"updated_at"`
	Timestamp          string `json:"timestamp"`
}

type MemoryListResponse struct {
	Items []MemoryEntry `json:"items"`
}
