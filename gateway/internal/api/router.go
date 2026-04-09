package api

import (
	"bufio"
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
	memoryDeps := memoryRouteDeps{
		store: memory.NewStore(repoRoot),
		state: settingsStore,
	}
	registerCoreRoutes(mux, cfg)
	registerProviderSettingsRoutes(mux, cfg, credentialStore, runtimeStore)
	mux.HandleFunc("/api/v1/settings", settingsHandler(repoRoot, cfg, settingsStore))
	mux.HandleFunc("/api/v1/settings/diagnostics/check", diagnosticsCheckHandler(repoRoot, cfg, settingsStore))
	mux.HandleFunc("/api/v1/settings/external-connections/action", externalConnectionActionHandler(repoRoot, cfg, settingsStore))
	mux.HandleFunc("/api/v1/system/info", systemInfoHandler(repoRoot, cfg.RuntimePort))
	mux.HandleFunc("/api/v1/logs", logsHandler(eventBus))
	mux.HandleFunc("/api/v1/memories", memoryDeps.handleMemories)
	mux.HandleFunc("/api/v1/memories/delete", memoryDeps.handleMemoryDelete)
	mux.HandleFunc("/api/v1/chat/run", chat.Run)
	mux.HandleFunc("/api/v1/chat/confirm", chat.Confirm)
	mux.HandleFunc("/api/v1/events/stream", chat.Stream)
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

func buildSettingsResponse(repoRoot string, cfg config.AppConfig, store *state.SettingsStore) SettingsResponse {
	mode, model, models, workspace, workspaces, directoryPromptEnabled, showRiskLevel, approvals := store.Snapshot()
	runtimeStatus := fetchRuntimeStatus(cfg.RuntimePort)
	return SettingsResponse{
		AppName:                cfg.AppName,
		Mode:                   mode,
		Model:                  model,
		AvailableModels:        models,
		Providers:              providerOptions(cfg.Providers),
		Workspace:              workspace,
		AvailableWorkspaces:    workspaces,
		ApprovedDirectories:    approvedDirectories(approvals),
		DirectoryPromptEnabled: directoryPromptEnabled,
		ShowRiskLevel:          showRiskLevel,
		Ports:                  map[string]int{"gateway": cfg.GatewayPort, "runtime": cfg.RuntimePort},
		RuntimeStatus:          runtimeStatus,
		MemoryPolicy:           buildMemoryPolicy(repoRoot, workspace),
		Diagnostics:            buildDiagnostics(repoRoot, cfg, runtimeStatus, len(models), len(workspaces), len(approvals)),
		ExternalConnections:    buildExternalConnections(repoRoot, cfg, workspace),
	}
}

func buildMemoryPolicy(repoRoot string, workspace config.WorkspaceRef) MemoryPolicyStatus {
	storageRoot := filepath.Join(repoRoot, "data")
	longTermPath := filepath.Join(storageRoot, "long_term_memory", fmt.Sprintf("%s.jsonl", workspace.WorkspaceID))
	knowledgePath := filepath.Join(storageRoot, "knowledge_base", fmt.Sprintf("%s.jsonl", workspace.WorkspaceID))
	return MemoryPolicyStatus{
		Enabled:             true,
		RecallStrategy:      "按需召回，使用摘要注入，不做全量记忆加载。",
		WriteStrategy:       "短期工作记忆实时落盘，任务收口阶段补长期记忆与知识沉淀。",
		CleanupStrategy:     "SQLite 主存储保留正式数据，legacy JSONL 在导入与访问时做去重压实。",
		StorageRoot:         storageRoot,
		SQLitePath:          filepath.Join(storageRoot, "storage", "main.db"),
		WorkingMemoryDir:    filepath.Join(storageRoot, "working_memory"),
		LongTermMemoryPath:  longTermPath,
		KnowledgeBasePath:   knowledgePath,
		LongTermMemoryCount: memoryCount(repoRoot, workspace.WorkspaceID),
		KnowledgeCount:      countJSONLLines(knowledgePath),
		WorkingMemoryFiles:  countDirEntries(filepath.Join(storageRoot, "working_memory")),
	}
}

func buildDiagnostics(repoRoot string, cfg config.AppConfig, runtimeStatus RuntimeStatus, modelCount int, workspaceCount int, approvalCount int) DiagnosticsStatus {
	storageRoot := filepath.Join(repoRoot, "data")
	status := DiagnosticsStatus{
		CheckedAt:               fmt.Sprintf("%d", time.Now().UnixMilli()),
		RepoRoot:                repoRoot,
		RepoRootExists:          pathExists(repoRoot),
		StorageRoot:             storageRoot,
		StorageRootExists:       pathExists(storageRoot),
		SettingsPath:            filepath.Join(storageRoot, "settings", "ui-state.json"),
		SettingsPathExists:      pathExists(filepath.Join(storageRoot, "settings", "ui-state.json")),
		RunLogPath:              filepath.Join(repoRoot, "logs", "run-logs.jsonl"),
		RunLogPathExists:        pathExists(filepath.Join(repoRoot, "logs", "run-logs.jsonl")),
		EventLogPath:            filepath.Join(repoRoot, "logs", "run-events.jsonl"),
		EventLogPathExists:      pathExists(filepath.Join(repoRoot, "logs", "run-events.jsonl")),
		WorkingMemoryDirExists:  pathExists(filepath.Join(storageRoot, "working_memory")),
		KnowledgeBasePathExists: pathExists(filepath.Join(storageRoot, "knowledge_base")),
		RuntimeReachable:        runtimeStatus.OK,
		RuntimeVersion:          fmt.Sprintf("%s / %s", runtimeStatus.Name, runtimeStatus.Version),
		ProviderCount:           len(cfg.Providers),
		ModelCount:              modelCount,
		WorkspaceCount:          workspaceCount,
		ApprovedDirectoryCount:  approvalCount,
		SiyuanRoot:              cfg.Siyuan.RootDir,
		SiyuanRootExists:        pathExists(cfg.Siyuan.RootDir),
		SiyuanExportDir:         cfg.Siyuan.ExportDir,
		SiyuanExportDirExists:   pathExists(cfg.Siyuan.ExportDir),
		SiyuanAutoWriteEnabled:  cfg.Siyuan.AutoWriteEnabled,
		SiyuanSyncEnabled:       cfg.Siyuan.SyncEnabled,
	}
	return finalizeDiagnostics(status)
}

func buildExternalConnections(repoRoot string, cfg config.AppConfig, workspace config.WorkspaceRef) []ExternalConnectionSlot {
	return []ExternalConnectionSlot{
		localFilesConnection(repoRoot, workspace),
		localNotesConnection(repoRoot, cfg),
		browserCaptureConnection(),
		personalManagementConnection(),
	}
}

func localFilesConnection(repoRoot string, workspace config.WorkspaceRef) ExternalConnectionSlot {
	slot := makeExternalConnection("local_files_project", "本地文件与项目目录", 1, "active", []string{"workspace_list", "workspace_read", "workspace_write", "workspace_delete", "run_command"}, "继续作为主链路第一优先级，不引入外部 SaaS 依赖。", "本地项目目录校验通过，可继续使用目录读写与命令能力。")
	return applyConnectionCheck(slot, validateLocalFilesProject(repoRoot, workspace))
}

func localNotesConnection(repoRoot string, cfg config.AppConfig) ExternalConnectionSlot {
	slot := makeExternalConnection("local_notes_knowledge", "本地笔记与知识库", 2, "active", []string{"knowledge_search", "search_siyuan_notes", "read_siyuan_note", "write_siyuan_knowledge"}, "继续坚持 SQLite 主索引、思源外挂正文库，不让笔记系统承接高频主存储。", "本地知识库与思源目录校验通过，可继续使用知识读写能力。")
	return applyConnectionCheck(slot, validateLocalNotesKnowledge(repoRoot, cfg))
}

func browserCaptureConnection() ExternalConnectionSlot {
	return makeExternalConnection("browser_capture_ingest", "浏览器摘录与网页入库", 3, "reserved", []string{"knowledge_search", "write_siyuan_knowledge", "project_answer"}, "本阶段只保留知识读写接入口，不提前接重型浏览器插件或云同步。", "当前不支持动作校验，只保留规划位。")
}

func personalManagementConnection() ExternalConnectionSlot {
	return makeExternalConnection("calendar_reminder_management", "日历、提醒与更重的个人管理连接", 4, "reserved", []string{"session_context", "project_answer"}, "当前阶段只保留规划位，不接日历、提醒、任务中心等重连接器。", "当前不支持动作校验，只保留规划位。")
}

func makeExternalConnection(slotID string, displayName string, priority int, status string, currentTools []string, boundary string, nextStep string) ExternalConnectionSlot {
	return ExternalConnectionSlot{
		SlotID: slotID, DisplayName: displayName, Priority: priority, Status: status,
		Scope: "external_connection", CurrentTools: currentTools,
		SupportedActions: supportedExternalConnectionActions(slotID),
		Boundary: boundary, NextStep: nextStep,
	}
}

func supportedExternalConnectionActions(slotID string) []string {
	if supportsExternalConnectionAction(slotID) {
		return []string{"recheck"}
	}
	return nil
}

func countJSONLLines(path string) int {
	file, err := os.Open(path)
	if err != nil {
		return 0
	}
	defer file.Close()
	count := 0
	for scanner := bufio.NewScanner(file); scanner.Scan(); count++ {
	}
	return count
}

func countDirEntries(path string) int {
	items, err := os.ReadDir(path)
	if err != nil {
		return 0
	}
	return len(items)
}

func memoryCount(repoRoot string, workspaceID string) int {
	items, err := memory.NewStore(repoRoot).List(workspaceID)
	if err != nil {
		return 0
	}
	return len(items)
}

func approvedDirectories(items []state.ApprovedDirectoryRecord) []DirectoryApproval {
	approvals := make([]DirectoryApproval, 0, len(items))
	for _, item := range items {
		approvals = append(approvals, DirectoryApproval{
			ApprovalID:  item.ApprovalID,
			WorkspaceID: item.WorkspaceID,
			Name:        item.Name,
			RootPath:    item.RootPath,
			CreatedAt:   item.CreatedAt,
		})
	}
	return approvals
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
		writeJSON(w, http.StatusOK, LogsResponse{Items: eventBus.Recent(120)})
	}
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

func pathExists(path string) bool {
	if path == "" {
		return false
	}
	_, err := os.Stat(path)
	return err == nil
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
