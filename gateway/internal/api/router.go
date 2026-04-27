package api

import (
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/knowledge"
	"local-agent/gateway/internal/memory"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/session"
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
	Model                  config.ModelRef          `json:"model"`
	AvailableModels        []config.ModelRef        `json:"available_models"`
	Providers              []ProviderOption         `json:"providers"`
	Workspace              config.WorkspaceRef      `json:"workspace"`
	AvailableWorkspaces    []config.WorkspaceRef    `json:"available_workspaces"`
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

func NewRouter(
	repoRoot string,
	cfg config.AppConfig,
	runtimeClient *runtimeclient.Client,
	eventBus *session.EventBus,
	settingsStore *state.SettingsStore,
	confirmationStore *state.ConfirmationStore,
	credentialStore *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) http.Handler {
	mux := http.NewServeMux()
	chat := NewChatHandler(repoRoot, cfg, runtimeClient, eventBus, settingsStore, confirmationStore, credentialStore, runtimeStore)
	memoryDeps := memoryRouteDeps{store: memory.NewStore(repoRoot), state: settingsStore}
	registerCoreRoutes(mux, cfg)
	registerProvidersRoutes(mux, cfg, credentialStore, runtimeStore)
	registerLearningRoutes(mux, memoryDeps)
	registerSettingsRoutes(mux, repoRoot, cfg, settingsStore)
	registerLogsRoutes(mux, repoRoot, cfg.RuntimePort, eventBus)
	registerReleaseRoutes(mux, repoRoot)
	registerMemoryRoutes(mux, memoryDeps)
	registerChatRoutes(mux, chat)
	knowledge.NewHandler(repoRoot).RegisterRoutes(mux, settingsStore, repoRoot, cfg)
	mux.Handle("/", spaHandler(repoRoot))
	return mux
}

func registerCoreRoutes(mux *http.ServeMux, cfg config.AppConfig) {
	mux.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		writeJSON(w, http.StatusOK, map[string]any{
			"status":  "ok",
			"app":     cfg.AppName,
			"gateway": cfg.GatewayPort,
		})
	})
}

func settingsHandler(repoRoot string, cfg config.AppConfig, store *state.SettingsStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if err := applySettingsUpdate(w, r, store); err != nil {
			return
		}
		writeJSON(w, http.StatusOK, buildSettingsResponse(repoRoot, cfg, store))
	}
}

func applySettingsUpdate(w http.ResponseWriter, r *http.Request, store *state.SettingsStore) error {
	if r.Method != http.MethodPost {
		return nil
	}
	var payload struct {
		Mode                   string          `json:"mode"`
		Model                  config.ModelRef `json:"model"`
		WorkspaceID            string          `json:"workspace_id"`
		DirectoryPromptEnabled *bool           `json:"directory_prompt_enabled"`
		ShowRiskLevel          *bool           `json:"show_risk_level"`
		RevokeDirectoryRoot    string          `json:"revoke_directory_root"`
	}
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return err
	}
	if err := store.Update(payload.Mode, payload.Model, payload.WorkspaceID, payload.DirectoryPromptEnabled, payload.ShowRiskLevel); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return err
	}
	if payload.RevokeDirectoryRoot != "" {
		store.RevokeDirectoryApproval(payload.RevokeDirectoryRoot)
	}
	return nil
}

func systemInfoHandler(repoRoot string, runtimePort int) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		writeJSON(w, http.StatusOK, map[string]any{
			"status":         "ok",
			"formal_entry":   "desktop launcher -> local web console",
			"system_entry":   "gateway",
			"repo_root":      repoRoot,
			"runtime_status": fetchRuntimeStatus(runtimePort),
		})
	}
}

func logsHandler(eventBus *session.EventBus) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		query, err := decodeLogsQuery(r)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		items := queryLogItems(eventBus, query)
		writeJSON(w, http.StatusOK, LogsResponse{Items: items})
	}
}

func queryLogItems(eventBus *session.EventBus, query logsQuery) []contracts.LogEntry {
	var items []contracts.LogEntry
	if query.View == "runs" {
		items = eventBus.RecentRuns(query.Limit, query.SessionID)
	} else {
		items = eventBus.RecentBy(query.Limit, query.SessionID, query.RunID)
	}
	return applyLogsQueryFilter(items, query)
}

func applyLogsQueryFilter(items []contracts.LogEntry, query logsQuery) []contracts.LogEntry {
	if query.SessionID == "" && query.RunID == "" {
		return items
	}
	filtered := make([]contracts.LogEntry, 0, len(items))
	for _, item := range items {
		if query.SessionID != "" && item.SessionID != query.SessionID {
			continue
		}
		if query.RunID != "" && item.RunID != query.RunID {
			continue
		}
		filtered = append(filtered, item)
	}
	return filtered
}

func (deps memoryRouteDeps) handleMemories(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	workspaceID, ok := currentWorkspaceID(deps.state)
	if !ok {
		http.Error(w, "workspace not found", http.StatusNotFound)
		return
	}
	writeMemoryList(w, deps, workspaceID)
}

func (deps memoryRouteDeps) handleMemoryDelete(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	payload, err := decodeMemoryDeletePayload(r)
	if err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return
	}
	workspaceID, ok := currentWorkspaceID(deps.state)
	if !ok {
		http.Error(w, "workspace not found", http.StatusNotFound)
		return
	}
	if err := deps.store.Delete(workspaceID, payload.MemoryID); err != nil {
		writeMemoryDeleteError(w, err)
		return
	}
	writeMemoryList(w, deps, workspaceID)
}

func decodeMemoryDeletePayload(r *http.Request) (struct {
	MemoryID string `json:"memory_id"`
}, error) {
	var payload struct {
		MemoryID string `json:"memory_id"`
	}
	err := json.NewDecoder(r.Body).Decode(&payload)
	return payload, err
}

func writeMemoryDeleteError(w http.ResponseWriter, err error) {
	if err == sql.ErrNoRows {
		http.Error(w, "memory not found", http.StatusNotFound)
		return
	}
	http.Error(w, err.Error(), http.StatusInternalServerError)
}

func writeMemoryList(w http.ResponseWriter, deps memoryRouteDeps, workspaceID string) {
	items, err := deps.store.List(workspaceID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	writeJSON(w, http.StatusOK, contracts.MemoryListResponse{Items: toContractMemories(items)})
}

func currentWorkspaceID(store *state.SettingsStore) (string, bool) {
	_, _, _, workspace, _, _, _, _ := store.Snapshot()
	if workspace.WorkspaceID == "" {
		return "", false
	}
	return workspace.WorkspaceID, true
}

func externalConnectionActionHandler(repoRoot string, cfg config.AppConfig, store *state.SettingsStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, err := decodeExternalConnectionAction(w, r)
		if err != nil {
			return
		}
		response, status := executeExternalConnectionAction(repoRoot, cfg, store, payload)
		writeJSON(w, status, response)
	}
}

func decodeExternalConnectionAction(w http.ResponseWriter, r *http.Request) (ExternalConnectionActionRequest, error) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ExternalConnectionActionRequest{}, errors.New("method not allowed")
	}
	var payload ExternalConnectionActionRequest
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return ExternalConnectionActionRequest{}, err
	}
	return payload, nil
}

func executeExternalConnectionAction(repoRoot string, cfg config.AppConfig, store *state.SettingsStore, payload ExternalConnectionActionRequest) (ExternalConnectionActionResponse, int) {
	if err := validateExternalConnectionAction(payload); err != nil {
		return ExternalConnectionActionResponse{SlotID: payload.SlotID, Action: payload.Action, OK: false, Message: err.Error()}, http.StatusBadRequest
	}
	settings := buildSettingsResponse(repoRoot, cfg, store)
	slot, ok := findExternalConnection(settings.ExternalConnections, payload.SlotID)
	if !ok {
		return ExternalConnectionActionResponse{SlotID: payload.SlotID, Action: payload.Action, OK: false, Message: "slot_id 不存在"}, http.StatusNotFound
	}
	if !supportsExternalConnectionAction(payload.SlotID) {
		return buildExternalConnectionActionResponse(payload, slot, false, "当前不支持该连接动作", settings.ExternalConnections), http.StatusOK
	}
	return buildExternalConnectionActionResponse(payload, slot, slot.Status == "active", slot.NextStep, settings.ExternalConnections), http.StatusOK
}

func validateExternalConnectionAction(payload ExternalConnectionActionRequest) error {
	if payload.SlotID == "" {
		return errors.New("slot_id is required")
	}
	if payload.Action == "" {
		return errors.New("action is required")
	}
	if payload.Action != "validate" && payload.Action != "recheck" {
		return errors.New("action must be validate or recheck")
	}
	return nil
}

func buildExternalConnectionActionResponse(payload ExternalConnectionActionRequest, slot ExternalConnectionSlot, ok bool, message string, slots []ExternalConnectionSlot) ExternalConnectionActionResponse {
	slotCopy := slot
	return ExternalConnectionActionResponse{
		SlotID: payload.SlotID, Action: payload.Action, OK: ok, Message: message,
		UpdatedSlot: &slotCopy, ExternalConnections: slots,
	}
}

func findExternalConnection(slots []ExternalConnectionSlot, slotID string) (ExternalConnectionSlot, bool) {
	for _, slot := range slots {
		if slot.SlotID == slotID {
			return slot, true
		}
	}
	return ExternalConnectionSlot{}, false
}

func supportsExternalConnectionAction(slotID string) bool {
	return slotID == "local_files_project" || slotID == "local_notes_knowledge"
}

func applyConnectionCheck(slot ExternalConnectionSlot, err error) ExternalConnectionSlot {
	if err == nil {
		return slot
	}
	slot.Status = "limited"
	slot.NextStep = err.Error()
	return slot
}

func validateLocalFilesProject(repoRoot string, workspace config.WorkspaceRef) error {
	return errors.Join(
		checkAccessibleDirectory("项目根目录", repoRoot),
		checkAccessibleDirectory("工作区根目录", workspace.RootPath),
	)
}

func validateLocalNotesKnowledge(repoRoot string, cfg config.AppConfig) error {
	return errors.Join(
		checkAccessibleDirectory("知识库目录", filepath.Join(repoRoot, "data", "knowledge_base")),
		checkAccessibleDirectory("思源根目录", cfg.Siyuan.RootDir),
		checkAccessibleDirectory("思源导出目录", cfg.Siyuan.ExportDir),
	)
}

func checkAccessibleDirectory(label string, path string) error {
	if path == "" {
		return fmt.Errorf("%s未配置", label)
	}
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("%s不可用: %w", label, err)
	}
	if !info.IsDir() {
		return fmt.Errorf("%s不是目录: %s", label, path)
	}
	_, err = os.ReadDir(path)
	if err != nil {
		return fmt.Errorf("%s不可访问: %w", label, err)
	}
	return nil
}

func diagnosticsCheckHandler(repoRoot string, cfg config.AppConfig, store *state.SettingsStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		settings := buildSettingsResponse(repoRoot, cfg, store)
		diagnostics := settings.Diagnostics
		writeJSON(w, http.StatusOK, DiagnosticsCheckResponse{
			CheckedAt: diagnostics.CheckedAt, OverallOK: len(diagnostics.Errors) == 0,
			Diagnostics: diagnostics, Warnings: diagnostics.Warnings, Errors: diagnostics.Errors,
		})
	}
}

func finalizeDiagnostics(status DiagnosticsStatus) DiagnosticsStatus {
	status.Warnings = diagnosticsWarnings(status)
	status.Errors = diagnosticsErrors(status)
	return status
}

func diagnosticsWarnings(status DiagnosticsStatus) []string {
	var warnings []string
	appendIfMissing(&warnings, !status.SettingsPathExists, "设置快照文件尚未生成。")
	appendIfMissing(&warnings, !status.RunLogPathExists, "运行日志文件尚未生成。")
	appendIfMissing(&warnings, !status.EventLogPathExists, "事件日志文件尚未生成。")
	appendIfMissing(&warnings, !status.StorageRootExists, "存储根目录尚未生成。")
	appendIfMissing(&warnings, !status.WorkingMemoryDirExists, "短期工作记忆目录尚未生成。")
	appendIfMissing(&warnings, !status.KnowledgeBasePathExists, "知识库目录尚未生成。")
	return warnings
}

func diagnosticsErrors(status DiagnosticsStatus) []string {
	var errors []string
	appendIfMissing(&errors, !status.RepoRootExists, "仓库根目录不存在或不可访问。")
	appendIfMissing(&errors, !status.RuntimeReachable, "Runtime 当前不可达。")
	appendIfMissing(&errors, !status.SiyuanRootExists && status.SiyuanSyncEnabled, "思源根目录不存在或不可访问。")
	appendIfMissing(&errors, !status.SiyuanExportDirExists && status.SiyuanAutoWriteEnabled, "思源导出目录不存在或不可访问。")
	return errors
}

func appendIfMissing(items *[]string, condition bool, message string) {
	if condition {
		*items = append(*items, message)
	}
}

func toContractMemories(items []memory.Entry) []contracts.MemoryEntry {
	result := make([]contracts.MemoryEntry, 0, len(items))
	for _, item := range items {
		result = append(result, contracts.MemoryEntry{
			ID:                 item.ID,
			Kind:               item.Kind,
			Title:              item.Title,
			Summary:            item.Summary,
			Content:            item.Content,
			Reason:             item.Reason,
			Scope:              item.Scope,
			WorkspaceID:        item.WorkspaceID,
			SessionID:          item.SessionID,
			SourceRunID:        item.SourceRunID,
			Source:             item.Source,
			SourceType:         item.SourceType,
			SourceTitle:        item.SourceTitle,
			SourceEventType:    item.SourceEventType,
			SourceArtifactPath: item.SourceArtifactPath,
			Verified:           item.Verified,
			Priority:           item.Priority,
			Archived:           item.Archived,
			ArchivedAt:         item.ArchivedAt,
			CreatedAt:          item.CreatedAt,
			UpdatedAt:          item.UpdatedAt,
			Timestamp:          item.Timestamp,
		})
	}
	return result
}

func providerOptions(items []config.ProviderConfig) []ProviderOption {
	options := make([]ProviderOption, 0, len(items))
	for _, item := range items {
		options = append(options, ProviderOption{
			ProviderID:          item.ProviderID,
			DisplayName:         item.DisplayName,
			BaseURL:             item.BaseURL,
			ChatCompletionsPath: item.ChatCompletionsPath,
			ModelsPath:          item.ModelsPath,
		})
	}
	return options
}

func spaHandler(repoRoot string) http.Handler {
	distDir := filepath.Join(repoRoot, "frontend", "dist")
	indexFile := filepath.Join(distDir, "index.html")
	fileServer := http.FileServer(http.Dir(distDir))

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if _, err := os.Stat(indexFile); err != nil {
			w.Header().Set("Content-Type", "text/html; charset=utf-8")
			_, _ = fmt.Fprint(w, `<!doctype html><html lang="zh-CN"><head><meta charset="utf-8"><title>本地智能体</title></head><body style="font-family:Segoe UI,Microsoft YaHei,sans-serif;padding:32px"><h1>前端尚未构建</h1><p>请先在 <code>frontend/</code> 下执行 <code>npm install</code> 和 <code>npm run build</code>。</p></body></html>`)
			return
		}

		requestPath := filepath.Join(distDir, filepath.Clean(r.URL.Path))
		if info, err := os.Stat(requestPath); err == nil && !info.IsDir() {
			fileServer.ServeHTTP(w, r)
			return
		}

		http.ServeFile(w, r, indexFile)
	})
}

func fetchRuntimeStatus(runtimePort int) RuntimeStatus {
	for attempt := 0; attempt < 3; attempt++ {
		status, ok := requestRuntimeStatus(runtimePort)
		if ok {
			return status
		}
		time.Sleep(120 * time.Millisecond)
	}
	return RuntimeStatus{OK: false, Name: "runtime-host", Version: "unreachable"}
}

func requestRuntimeStatus(runtimePort int) (RuntimeStatus, bool) {
	client := http.Client{Timeout: time.Second}
	resp, err := client.Get(fmt.Sprintf("http://127.0.0.1:%d/health", runtimePort))
	if err != nil {
		return RuntimeStatus{}, false
	}
	defer resp.Body.Close()
	var payload RuntimeStatus
	if err := json.NewDecoder(resp.Body).Decode(&payload); err != nil {
		return RuntimeStatus{OK: false, Name: "runtime-host", Version: "invalid-response"}, true
	}
	return payload, true
}

func writeJSON(w http.ResponseWriter, status int, payload any) {
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(payload)
}
