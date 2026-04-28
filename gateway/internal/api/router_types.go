package api

import (
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/memory"
	"local-agent/gateway/internal/state"
)

type RuntimeStatus struct {
	OK      bool   `json:"ok"`
	Name    string `json:"name"`
	Version string `json:"version"`
}

type SettingsResponse struct {
	AppName                string                   `json:"app_name"`
	Mode                   string                   `json:"mode"`
	Model                  contracts.ModelRef       `json:"model"`
	AvailableModels        []contracts.ModelRef     `json:"available_models"`
	Providers              []ProviderOption         `json:"providers"`
	Workspace              contracts.WorkspaceRef   `json:"workspace"`
	AvailableWorkspaces    []contracts.WorkspaceRef `json:"available_workspaces"`
	ApprovedDirectories    []DirectoryApproval      `json:"approved_directories"`
	DirectoryPromptEnabled bool                     `json:"directory_prompt_enabled"`
	ShowRiskLevel          bool                     `json:"show_risk_level"`
	Ports                  map[string]int           `json:"ports"`
	RuntimeStatus          RuntimeStatus            `json:"runtime_status"`
	MemoryPolicy           MemoryPolicyStatus       `json:"memory_policy"`
	Diagnostics            DiagnosticsStatus        `json:"diagnostics"`
	ExternalConnections    []ExternalConnectionSlot `json:"external_connections"`
}

type MemoryPolicyStatus struct {
	Enabled             bool   `json:"enabled"`
	RecallStrategy      string `json:"recall_strategy"`
	WriteStrategy       string `json:"write_strategy"`
	CleanupStrategy     string `json:"cleanup_strategy"`
	StorageRoot         string `json:"storage_root"`
	SQLitePath          string `json:"sqlite_path"`
	WorkingMemoryDir    string `json:"working_memory_dir"`
	LongTermMemoryPath  string `json:"long_term_memory_path"`
	KnowledgeBasePath   string `json:"knowledge_base_path"`
	LongTermMemoryCount int    `json:"long_term_memory_count"`
	KnowledgeCount      int    `json:"knowledge_count"`
	WorkingMemoryFiles  int    `json:"working_memory_files"`
}

type DiagnosticsStatus struct {
	CheckedAt               string   `json:"checked_at"`
	RepoRoot                string   `json:"repo_root"`
	RepoRootExists          bool     `json:"repo_root_exists"`
	StorageRoot             string   `json:"storage_root"`
	StorageRootExists       bool     `json:"storage_root_exists"`
	SettingsPath            string   `json:"settings_path"`
	SettingsPathExists      bool     `json:"settings_path_exists"`
	RunLogPath              string   `json:"run_log_path"`
	RunLogPathExists        bool     `json:"run_log_path_exists"`
	EventLogPath            string   `json:"event_log_path"`
	EventLogPathExists      bool     `json:"event_log_path_exists"`
	WorkingMemoryDirExists  bool     `json:"working_memory_dir_exists"`
	KnowledgeBasePathExists bool     `json:"knowledge_base_path_exists"`
	RuntimeReachable        bool     `json:"runtime_reachable"`
	RuntimeVersion          string   `json:"runtime_version"`
	ProviderCount           int      `json:"provider_count"`
	ModelCount              int      `json:"model_count"`
	WorkspaceCount          int      `json:"workspace_count"`
	ApprovedDirectoryCount  int      `json:"approved_directory_count"`
	SiyuanRoot              string   `json:"siyuan_root"`
	SiyuanRootExists        bool     `json:"siyuan_root_exists"`
	SiyuanExportDir         string   `json:"siyuan_export_dir"`
	SiyuanExportDirExists   bool     `json:"siyuan_export_dir_exists"`
	SiyuanAutoWriteEnabled  bool     `json:"siyuan_auto_write_enabled"`
	SiyuanSyncEnabled       bool     `json:"siyuan_sync_enabled"`
	Warnings                []string `json:"warnings"`
	Errors                  []string `json:"errors"`
}

type ExternalConnectionSlot struct {
	SlotID           string   `json:"slot_id"`
	DisplayName      string   `json:"display_name"`
	Priority         int      `json:"priority"`
	Status           string   `json:"status"`
	Scope            string   `json:"scope"`
	CurrentTools     []string `json:"current_tools"`
	SupportedActions []string `json:"supported_actions,omitempty"`
	Boundary         string   `json:"boundary"`
	NextStep         string   `json:"next_step"`
}

type DirectoryApproval struct {
	ApprovalID  string `json:"approval_id"`
	WorkspaceID string `json:"workspace_id"`
	Name        string `json:"name"`
	RootPath    string `json:"root_path"`
	CreatedAt   string `json:"created_at,omitempty"`
}

type LogsResponse struct {
	Items []contracts.LogEntry `json:"items"`
}

type memoryRouteDeps struct {
	store *memory.Store
	state *state.SettingsStore
}

type ProviderOption struct {
	ProviderID          string `json:"provider_id"`
	DisplayName         string `json:"display_name"`
	BaseURL             string `json:"base_url"`
	ChatCompletionsPath string `json:"chat_completions_path"`
	ModelsPath          string `json:"models_path"`
}

type ExternalConnectionActionRequest struct {
	SlotID string `json:"slot_id"`
	Action string `json:"action"`
}

type ExternalConnectionActionResponse struct {
	SlotID              string                   `json:"slot_id"`
	Action              string                   `json:"action"`
	OK                  bool                     `json:"ok"`
	Message             string                   `json:"message"`
	UpdatedSlot         *ExternalConnectionSlot  `json:"updated_slot,omitempty"`
	ExternalConnections []ExternalConnectionSlot `json:"external_connections,omitempty"`
}

type DiagnosticsCheckResponse struct {
	CheckedAt   string            `json:"checked_at"`
	OverallOK   bool              `json:"overall_ok"`
	Diagnostics DiagnosticsStatus `json:"diagnostics"`
	Warnings    []string          `json:"warnings"`
	Errors      []string          `json:"errors"`
}
