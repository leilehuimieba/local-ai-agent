package api

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"sync/atomic"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/memory"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/session"
	"local-agent/gateway/internal/state"
)

type ChatHandler struct {
	repoRoot          string
	appConfig         config.AppConfig
	runtimeClient     *runtimeclient.Client
	eventBus          *session.EventBus
	settingsStore     *state.SettingsStore
	confirmationStore *state.ConfirmationStore
	credentialStore   *state.ProviderCredentialStore
	runtimeStore      *state.RuntimeProviderStore
	memoryStore       *memory.Store
}

type ChatRunRequest struct {
	SessionID    string              `json:"session_id"`
	UserInput    string              `json:"user_input"`
	Mode         string              `json:"mode"`
	Model        config.ModelRef     `json:"model"`
	Workspace    config.WorkspaceRef `json:"workspace"`
	ContextHints map[string]string   `json:"context_hints,omitempty"`
}

type ChatRetryRequest struct {
	SessionID    string `json:"session_id"`
	RunID        string `json:"run_id"`
	CheckpointID string `json:"checkpoint_id,omitempty"`
}

type ChatRunAccepted struct {
	Accepted      bool   `json:"accepted"`
	SessionID     string `json:"session_id"`
	RunID         string `json:"run_id"`
	InitialStatus string `json:"initial_status"`
}

var idCounter uint64

func NewChatHandler(
	repoRoot string,
	cfg config.AppConfig,
	runtimeClient *runtimeclient.Client,
	eventBus *session.EventBus,
	settingsStore *state.SettingsStore,
	confirmationStore *state.ConfirmationStore,
	credentialStore *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) *ChatHandler {
	return &ChatHandler{
		repoRoot:          repoRoot,
		appConfig:         cfg,
		runtimeClient:     runtimeClient,
		eventBus:          eventBus,
		settingsStore:     settingsStore,
		confirmationStore: confirmationStore,
		credentialStore:   credentialStore,
		runtimeStore:      runtimeStore,
		memoryStore:       memory.NewStore(repoRoot),
	}
}

func (h *ChatHandler) Run(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	payload, err := decodeRunPayload(r)
	if err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return
	}
	if payload.UserInput == "" {
		http.Error(w, "user_input is required", http.StatusBadRequest)
		return
	}
	runRequest, err := h.buildRunRequest(payload)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	go h.execute(runRequest)
	writeJSON(w, http.StatusAccepted, ChatRunAccepted{
		Accepted:      true,
		SessionID:     runRequest.SessionID,
		RunID:         runRequest.RunID,
		InitialStatus: "accepted",
	})
}

func decodeRunPayload(r *http.Request) (ChatRunRequest, error) {
	var payload ChatRunRequest
	err := json.NewDecoder(r.Body).Decode(&payload)
	return payload, err
}

func (h *ChatHandler) buildRunRequest(payload ChatRunRequest) (contracts.RunRequest, error) {
	sessionID, mode, model, workspace, firstSeen, err := h.resolveRunContext(payload)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	providerRef, err := h.resolveProviderRef(model.ProviderID)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	return contracts.RunRequest{
		RequestID:              newID("request"),
		RunID:                  newID("run"),
		SessionID:              sessionID,
		TraceID:                newID("trace"),
		UserInput:              payload.UserInput,
		Mode:                   mode,
		ModelRef:               model,
		ProviderRef:            providerRef,
		WorkspaceRef:           workspace,
		ContextHints:           h.withKnowledgeHints(runContextHints(payload.ContextHints, h.repoRoot, firstSeen)),
		ResumeFromCheckpointID: "",
		ResumeStrategy:         "",
	}, nil
}

func (h *ChatHandler) resolveRunContext(payload ChatRunRequest) (string, string, config.ModelRef, config.WorkspaceRef, bool, error) {
	sessionID := payload.SessionID
	if sessionID == "" {
		sessionID = newID("session")
	}
	currentMode, currentModel, _, currentWorkspace, _, directoryPromptEnabled, _, _ := h.settingsStore.Snapshot()
	mode := payload.Mode
	if mode == "" {
		mode = currentMode
	}
	model := payload.Model
	if model.ModelID == "" {
		model = currentModel
	}
	workspace, err := h.resolveWorkspace(payload.Workspace, currentWorkspace)
	if err != nil {
		return "", "", config.ModelRef{}, config.WorkspaceRef{}, false, err
	}
	firstSeen := directoryPromptEnabled && !h.settingsStore.IsWorkspaceApproved(workspace.WorkspaceID)
	return sessionID, mode, model, workspace, firstSeen, nil
}

func (h *ChatHandler) resolveWorkspace(input config.WorkspaceRef, fallback config.WorkspaceRef) (config.WorkspaceRef, error) {
	if input.WorkspaceID == "" {
		return fallback, nil
	}
	workspace, ok := h.settingsStore.WorkspaceByID(input.WorkspaceID)
	if !ok {
		return config.WorkspaceRef{}, fmt.Errorf("workspace not found")
	}
	return workspace, nil
}

func runContextHints(source map[string]string, repoRoot string, firstSeen bool) map[string]string {
	hints := make(map[string]string)
	for key, value := range source {
		hints[key] = value
	}
	hints["repo_root"] = repoRoot
	hints["workspace_first_seen"] = fmt.Sprintf("%t", firstSeen)
	return hints
}

func (h *ChatHandler) withKnowledgeHints(hints map[string]string) map[string]string {
	hints["siyuan_root"] = h.appConfig.Siyuan.RootDir
	hints["siyuan_export_dir"] = h.appConfig.Siyuan.ExportDir
	hints["siyuan_auto_write_enabled"] = fmt.Sprintf("%t", h.appConfig.Siyuan.AutoWriteEnabled)
	hints["siyuan_sync_enabled"] = fmt.Sprintf("%t", h.appConfig.Siyuan.SyncEnabled)
	return hints
}

func (h *ChatHandler) resolveProviderRef(providerID string) (contracts.ProviderRef, error) {
	if ref, ok := runtimeProviderRef(h.runtimeStore, providerID); ok {
		return ref, nil
	}
	if ref, ok := credentialProviderRef(h.appConfig, h.credentialStore, providerID); ok {
		return ref, nil
	}
	if ref, ok := configProviderRef(h.appConfig, providerID); ok {
		return ref, nil
	}
	return contracts.ProviderRef{}, fmt.Errorf("provider %s 缺少可用凭据，请先保存并应用或检查配置", providerID)
}

func runtimeProviderRef(store *state.RuntimeProviderStore, providerID string) (contracts.ProviderRef, bool) {
	record, ok := store.Get(providerID)
	if !ok || record.Status != "applied" || record.APIKey == "" {
		return contracts.ProviderRef{}, false
	}
	return runtimeRecordRef(record), true
}

func credentialProviderRef(cfg config.AppConfig, store *state.ProviderCredentialStore, providerID string) (contracts.ProviderRef, bool) {
	record, ok := store.Get(providerID)
	if !ok || !record.HasCredential || record.APIKey == "" {
		return contracts.ProviderRef{}, false
	}
	provider, ok := catalogProvider(cfg, providerID)
	if !ok {
		return contracts.ProviderRef{}, false
	}
	return credentialRecordRef(provider, record), true
}

func configProviderRef(cfg config.AppConfig, providerID string) (contracts.ProviderRef, bool) {
	provider, ok := catalogProvider(cfg, providerID)
	if !ok || provider.APIKey == "" {
		return contracts.ProviderRef{}, false
	}
	return providerConfigRef(provider, provider.APIKey), true
}

func catalogProvider(cfg config.AppConfig, providerID string) (config.ProviderConfig, bool) {
	for _, item := range cfg.Providers {
		if item.ProviderID == providerID {
			return item, true
		}
	}
	return config.ProviderConfig{}, false
}

func providerConfigRef(provider config.ProviderConfig, apiKey string) contracts.ProviderRef {
	return contracts.ProviderRef{
		ProviderID: provider.ProviderID, DisplayName: provider.DisplayName, BaseURL: provider.BaseURL,
		ChatCompletionsPath: provider.ChatCompletionsPath, ModelsPath: provider.ModelsPath, APIKey: apiKey,
	}
}

func runtimeRecordRef(record state.RuntimeProviderRecord) contracts.ProviderRef {
	return contracts.ProviderRef{
		ProviderID: record.ProviderID, DisplayName: record.DisplayName, BaseURL: record.BaseURL,
		ChatCompletionsPath: record.ChatCompletionsPath, ModelsPath: record.ModelsPath, APIKey: record.APIKey,
	}
}

func credentialRecordRef(provider config.ProviderConfig, record state.ProviderCredentialRecord) contracts.ProviderRef {
	return contracts.ProviderRef{
		ProviderID: provider.ProviderID, DisplayName: firstNonEmptyValue(record.DisplayName, provider.DisplayName),
		BaseURL:             firstNonEmptyValue(record.BaseURL, provider.BaseURL),
		ChatCompletionsPath: firstNonEmptyValue(record.ChatCompletionsPath, provider.ChatCompletionsPath),
		ModelsPath:          firstNonEmptyValue(record.ModelsPath, provider.ModelsPath), APIKey: record.APIKey,
	}
}

func firstNonEmptyValue(values ...string) string {
	for _, value := range values {
		if value != "" {
			return value
		}
	}
	return ""
}

func (h *ChatHandler) Confirm(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	decision, err := decodeConfirmationDecision(r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	pending, status, err := h.pendingConfirmation(decision)
	if err != nil {
		http.Error(w, err.Error(), status)
		return
	}
	if decision.Decision == "approve" {
		h.approveConfirmation(w, decision, pending)
		return
	}
	h.closeConfirmation(w, decision, pending)
}

func (h *ChatHandler) Retry(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	payload, err := decodeRetryPayload(r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	runRequest, err := h.buildRetryRunRequest(payload)
	if err != nil {
		writeRetryError(w, err)
		return
	}
	go h.execute(runRequest)
	writeJSON(w, http.StatusAccepted, ChatRunAccepted{
		Accepted:      true,
		SessionID:     runRequest.SessionID,
		RunID:         runRequest.RunID,
		InitialStatus: "accepted",
	})
}

func decodeRetryPayload(r *http.Request) (ChatRetryRequest, error) {
	var payload ChatRetryRequest
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		return ChatRetryRequest{}, fmt.Errorf("invalid json body")
	}
	if payload.SessionID == "" || payload.RunID == "" {
		return ChatRetryRequest{}, fmt.Errorf("session_id and run_id are required")
	}
	return payload, nil
}

func (h *ChatHandler) buildRetryRunRequest(payload ChatRetryRequest) (contracts.RunRequest, error) {
	record, err := h.retryCheckpoint(payload)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	request := record.Request
	providerRef, err := h.resolveProviderRef(request.ModelRef.ProviderID)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	request.RequestID = newID("request")
	request.RunID = newID("run")
	request.TraceID = newID("trace")
	request.SessionID = payload.SessionID
	request.ProviderRef = providerRef
	request.ConfirmationDecision = nil
	request.ContextHints = h.withKnowledgeHints(copyContextHints(request.ContextHints))
	applyRetryCheckpointResume(&request, record.CheckpointID)
	return request, nil
}

func decodeConfirmationDecision(r *http.Request) (contracts.ConfirmationDecision, error) {
	var decision contracts.ConfirmationDecision
	if err := json.NewDecoder(r.Body).Decode(&decision); err != nil {
		return contracts.ConfirmationDecision{}, fmt.Errorf("invalid json body")
	}
	if decision.ConfirmationID == "" || decision.RunID == "" {
		return contracts.ConfirmationDecision{}, fmt.Errorf("confirmation_id and run_id are required")
	}
	switch decision.Decision {
	case "approve", "reject", "cancel":
		return decision, nil
	default:
		return contracts.ConfirmationDecision{}, fmt.Errorf("invalid decision")
	}
}

func (h *ChatHandler) pendingConfirmation(
	decision contracts.ConfirmationDecision,
) (state.PendingConfirmation, int, error) {
	pending, ok := h.confirmationStore.Get(decision.ConfirmationID)
	if !ok {
		return state.PendingConfirmation{}, http.StatusNotFound, fmt.Errorf("confirmation not found")
	}
	if pending.Request.RunID != decision.RunID {
		return state.PendingConfirmation{}, http.StatusBadRequest, fmt.Errorf("run_id does not match confirmation")
	}
	return pending, http.StatusOK, nil
}

func (h *ChatHandler) approveConfirmation(
	w http.ResponseWriter,
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) {
	h.confirmationStore.Delete(decision.ConfirmationID)
	if pending.Confirmation.Kind == "workspace_access" && decision.Remember {
		h.settingsStore.ApproveWorkspace(pending.Request.WorkspaceRef.WorkspaceID)
	}
	if pending.Request.ContextHints == nil {
		pending.Request.ContextHints = make(map[string]string)
	}
	pending.Request.ContextHints["workspace_first_seen"] = "false"
	pending.Request.ConfirmationDecision = &decision
	applyCheckpointResume(&pending.Request, pending.CheckpointID)
	go h.execute(pending.Request)
	writeConfirmationResponse(w, http.StatusAccepted, decision)
}

func (h *ChatHandler) closeConfirmation(
	w http.ResponseWriter,
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) {
	if taken, ok := h.confirmationStore.Take(decision.ConfirmationID); ok {
		pending = taken
	}
	h.publishConfirmationClosure(decision, pending)
	writeConfirmationResponse(w, http.StatusOK, decision)
}

func writeConfirmationResponse(
	w http.ResponseWriter,
	status int,
	decision contracts.ConfirmationDecision,
) {
	writeJSON(w, status, map[string]any{
		"accepted":        true,
		"confirmation_id": decision.ConfirmationID,
		"decision":        decision.Decision,
	})
}

func (h *ChatHandler) retryCheckpoint(
	payload ChatRetryRequest,
) (state.RuntimeCheckpointRecord, error) {
	store := state.NewRuntimeCheckpointStore(h.repoRoot)
	record, err := store.FindRetryable(payload.RunID, payload.SessionID, payload.CheckpointID)
	if err != nil {
		return state.RuntimeCheckpointRecord{}, err
	}
	return validateRetryCheckpoint(payload, record)
}

func validateRetryCheckpoint(
	payload ChatRetryRequest,
	record state.RuntimeCheckpointRecord,
) (state.RuntimeCheckpointRecord, error) {
	if record.SessionID != payload.SessionID || record.RunID != payload.RunID {
		return state.RuntimeCheckpointRecord{}, fmt.Errorf("checkpoint 与当前会话或运行不匹配")
	}
	if !record.Resumable || record.ResumeReason != "retryable_failure" {
		return state.RuntimeCheckpointRecord{}, fmt.Errorf("当前 checkpoint 不支持失败重试")
	}
	return record, nil
}

func (h *ChatHandler) publishConfirmationClosure(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) {
	h.eventBus.Publish(h.confirmationMemoryEvent(decision, pending))
	h.eventBus.Publish(rejectedConfirmationEvent(decision, pending))
}

func (h *ChatHandler) confirmationMemoryEvent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) contracts.RunEvent {
	entry, ok, reason := confirmationMemoryEntry(decision, pending)
	if !ok {
		return skippedConfirmationMemoryEvent(decision, pending, reason)
	}
	written, err := h.memoryStore.Save(entry)
	if err != nil {
		return skippedConfirmationMemoryEvent(decision, pending, err.Error())
	}
	if !written {
		return skippedConfirmationMemoryEvent(decision, pending, "命中重复风险确认治理记录，跳过写入。")
	}
	return writtenConfirmationMemoryEvent(decision, pending, entry)
}

func rejectedConfirmationEvent(decision contracts.ConfirmationDecision, pending state.PendingConfirmation) contracts.RunEvent {
	summary := rejectionSummary(decision.Decision)
	return contracts.RunEvent{
		EventID:          newID("event"),
		Kind:             "run_event",
		Source:           "gateway",
		AgentID:          "primary",
		AgentLabel:       "主智能体",
		EventType:        "run_finished",
		TraceID:          pending.Request.TraceID,
		SessionID:        pending.Request.SessionID,
		RunID:            pending.Request.RunID,
		Sequence:         99,
		Timestamp:        timestampNow(),
		Stage:            "Finish",
		Summary:          summary,
		Detail:           summary,
		RiskLevel:        pending.Confirmation.RiskLevel,
		ConfirmationID:   decision.ConfirmationID,
		CompletionStatus: rejectionStatus(decision.Decision),
		CompletionReason: rejectionReason(decision.Decision),
		Metadata:         rejectionMetadata(decision, pending, summary),
	}
}

func rejectionSummary(decision string) string {
	if decision == "reject" {
		return "用户拒绝了本次高风险动作，任务按确认结果结束。"
	}
	return "用户取消了本次高风险动作确认，任务按确认结果结束。"
}

func rejectionMetadata(decision contracts.ConfirmationDecision, pending state.PendingConfirmation, summary string) map[string]string {
	return map[string]string{
		"confirmation_id":   decision.ConfirmationID,
		"decision":          decision.Decision,
		"decision_note":     decision.Note,
		"completion_status": rejectionStatus(decision.Decision),
		"completion_reason": rejectionReason(decision.Decision),
		"result_summary":    summary,
		"final_answer":      summary,
		"kind":              pending.Confirmation.Kind,
		"risk_level":        pending.Confirmation.RiskLevel,
		"task_title":        pending.Confirmation.ActionSummary,
		"record_type":       "confirmation_result",
		"source_type":       "gateway",
		"next_step":         "任务已结束",
	}
}

func rejectionStatus(decision string) string {
	if decision == "reject" {
		return "rejected"
	}
	return "cancelled"
}

func rejectionReason(decision string) string {
	if decision == "reject" {
		return "用户明确拒绝了当前高风险动作。"
	}
	return "用户取消了当前高风险动作确认。"
}

func confirmationMemoryEntry(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) (memory.Entry, bool, string) {
	if pending.Confirmation.Kind != "high_risk_action" {
		return memory.Entry{}, false, "当前确认类型仅记录日志，不沉淀长期记忆。"
	}
	now := timestampNow()
	return memory.Entry{
		ID: confirmationMemoryID(decision.Decision), Kind: confirmationMemoryKind(decision.Decision),
		Title:   confirmationMemoryTitle(decision.Decision),
		Summary: confirmationMemoryTitle(decision.Decision),
		Content: confirmationMemoryContent(decision, pending), Scope: pending.Request.WorkspaceRef.Name,
		WorkspaceID: pending.Request.WorkspaceRef.WorkspaceID, SessionID: pending.Request.SessionID,
		SourceRunID: pending.Request.RunID, Source: "gateway_confirmation", SourceType: "gateway",
		SourceTitle:        pending.Confirmation.ActionSummary,
		SourceEventType:    confirmationEventType(decision.Decision),
		SourceArtifactPath: firstTargetPath(pending), GovernanceVersion: memory.MemoryGovernanceVersion,
		GovernanceReason: confirmationGovernanceReason(decision.Decision),
		GovernanceSource: confirmationGovernanceSource(decision.Decision), GovernanceAt: now,
		Verified: true, Priority: confirmationPriority(decision.Decision), Archived: false,
		ArchivedAt: "", CreatedAt: now, UpdatedAt: now, Timestamp: now,
	}, true, ""
}

func confirmationMemoryID(decision string) string {
	return fmt.Sprintf("memory-confirmation-%s-%s", decision, timestampNow())
}

func confirmationMemoryKind(decision string) string {
	if decision == "reject" {
		return "lesson_learned"
	}
	return "workflow_pattern"
}

func confirmationMemoryTitle(decision string) string {
	if decision == "reject" {
		return "失败教训：高风险动作被用户拒绝时应先缩小范围并提供更安全替代。"
	}
	return "流程模式：高风险动作在信息不足时应先取消，并补充范围说明后再继续。"
}

func confirmationMemoryContent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) string {
	return fmt.Sprintf(
		"decision=%s; note=%s; action=%s; risk_level=%s; reason=%s; impact_scope=%s; target_paths=%s; alternatives=%s",
		decision.Decision,
		decision.Note,
		pending.Confirmation.ActionSummary,
		pending.Confirmation.RiskLevel,
		pending.Confirmation.Reason,
		pending.Confirmation.ImpactScope,
		joinValues(pending.Confirmation.TargetPaths),
		joinValues(pending.Confirmation.Alternatives),
	)
}

func firstTargetPath(pending state.PendingConfirmation) string {
	if len(pending.Confirmation.TargetPaths) == 0 {
		return ""
	}
	return pending.Confirmation.TargetPaths[0]
}

func joinValues(values []string) string {
	if len(values) == 0 {
		return ""
	}
	return fmt.Sprintf("%v", values)
}

func confirmationGovernanceReason(decision string) string {
	if decision == "reject" {
		return "用户拒绝高风险动作后，已沉淀为正式失败教训。"
	}
	return "用户取消高风险动作确认后，已沉淀为可复用流程模式。"
}

func confirmationGovernanceSource(decision string) string {
	if decision == "reject" {
		return "gateway_confirmation_reject"
	}
	return "gateway_confirmation_cancel"
}

func confirmationEventType(decision string) string {
	if decision == "reject" {
		return "confirmation_rejected"
	}
	return "confirmation_cancelled"
}

func confirmationPriority(decision string) int {
	if decision == "reject" {
		return 65
	}
	return 55
}

func skippedConfirmationMemoryEvent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
	reason string,
) contracts.RunEvent {
	return contracts.RunEvent{
		EventID: newID("event"), Kind: "run_event", Source: "gateway",
		RecordType: "confirmation_result", SourceType: "gateway",
		AgentID: "primary", AgentLabel: "主智能体", EventType: "memory_write_skipped",
		TraceID: pending.Request.TraceID, SessionID: pending.Request.SessionID,
		RunID: pending.Request.RunID, Sequence: 98, Timestamp: timestampNow(), Stage: "Finish",
		Summary: "跳过写入", Detail: reason, RiskLevel: pending.Confirmation.RiskLevel,
		ConfirmationID: decision.ConfirmationID, ArtifactPath: firstTargetPath(pending),
		Metadata: confirmationMemoryMetadata(decision, pending, memory.Entry{
			Kind: "confirmation_result", SourceType: "gateway", GovernanceVersion: memory.MemoryGovernanceVersion,
			GovernanceReason: reason, GovernanceSource: "gateway_confirmation_skip", GovernanceAt: timestampNow(),
			SourceEventType: "memory_write_skipped", SourceArtifactPath: firstTargetPath(pending),
		}, "long_term_memory"),
	}
}

func writtenConfirmationMemoryEvent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
	entry memory.Entry,
) contracts.RunEvent {
	return contracts.RunEvent{
		EventID: newID("event"), Kind: "run_event", Source: "gateway",
		RecordType: entry.Kind, SourceType: entry.SourceType, AgentID: "primary",
		AgentLabel: "主智能体", EventType: "memory_written", TraceID: pending.Request.TraceID,
		SessionID: pending.Request.SessionID, RunID: pending.Request.RunID, Sequence: 98,
		Timestamp: timestampNow(), Stage: "Finish", Summary: entry.Title,
		Detail: "风险确认治理结果已写入长期记忆。", ResultSummary: entry.Summary,
		ArtifactPath: entry.SourceArtifactPath, RiskLevel: pending.Confirmation.RiskLevel,
		ConfirmationID: decision.ConfirmationID,
		Metadata:       confirmationMemoryMetadata(decision, pending, entry, "long_term_memory"),
	}
}

func confirmationMemoryMetadata(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
	entry memory.Entry,
	layer string,
) map[string]string {
	return map[string]string{
		"layer": layer, "record_type": entry.Kind, "source_type": "gateway",
		"memory_kind": entry.Kind, "reason": entry.GovernanceReason,
		"decision": decision.Decision, "decision_note": decision.Note,
		"confirmation_id": decision.ConfirmationID, "kind": pending.Confirmation.Kind,
		"risk_level": pending.Confirmation.RiskLevel, "task_title": pending.Confirmation.ActionSummary,
		"artifact_path": firstTargetPath(pending), "next_step": "任务已结束",
		"governance_status":    confirmationGovernanceStatus(entry),
		"memory_action":        confirmationMemoryAction(entry),
		"governance_version":   entry.GovernanceVersion,
		"governance_reason":    entry.GovernanceReason,
		"governance_source":    entry.GovernanceSource,
		"governance_at":        entry.GovernanceAt,
		"source_event_type":    entry.SourceEventType,
		"source_artifact_path": entry.SourceArtifactPath,
		"archive_reason":       entry.ArchiveReason,
	}
}

func confirmationGovernanceStatus(entry memory.Entry) string {
	if entry.Archived {
		return "archived"
	}
	if entry.SourceEventType == "memory_write_skipped" {
		return "skipped"
	}
	return "written"
}

func confirmationMemoryAction(entry memory.Entry) string {
	if entry.Archived {
		return "archive"
	}
	if entry.SourceEventType == "memory_write_skipped" {
		return "skip"
	}
	return "write"
}

func (h *ChatHandler) Stream(w http.ResponseWriter, r *http.Request) {
	sessionID := r.URL.Query().Get("session_id")
	if sessionID == "" {
		http.Error(w, "session_id is required", http.StatusBadRequest)
		return
	}

	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "streaming unsupported", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	for _, item := range h.eventBus.Snapshot(sessionID) {
		writeSSE(w, item)
	}
	flusher.Flush()

	stream, cancel := h.eventBus.Subscribe(sessionID)
	defer cancel()

	heartbeat := time.NewTicker(15 * time.Second)
	defer heartbeat.Stop()

	for {
		select {
		case <-r.Context().Done():
			return
		case item, ok := <-stream:
			if !ok {
				return
			}
			writeSSE(w, item)
			flusher.Flush()
		case <-heartbeat.C:
			_, _ = fmt.Fprint(w, ": keep-alive\n\n")
			flusher.Flush()
		}
	}
}

func (h *ChatHandler) execute(runRequest contracts.RunRequest) {
	ctx, cancel := context.WithTimeout(context.Background(), 45*time.Second)
	defer cancel()

	response, err := h.runtimeClient.Run(ctx, runRequest)
	if err != nil {
		h.publishRuntimeFailure(runRequest, err.Error())
		return
	}

	if response.ConfirmationRequest != nil {
		h.confirmationStore.Save(state.PendingConfirmation{
			Request:      runRequest,
			Confirmation: *response.ConfirmationRequest,
			CheckpointID: stringValue(response.Result.CheckpointID),
		})
	}

	for _, item := range response.Events {
		h.publishRuntimeEvent(item)
		time.Sleep(120 * time.Millisecond)
	}
}

func (h *ChatHandler) publishRuntimeEvent(event contracts.RunEvent) {
	h.eventBus.Publish(event)
}

func (h *ChatHandler) publishRuntimeFailure(runRequest contracts.RunRequest, errorText string) {
	h.eventBus.Publish(runtimeFailureEvent(runRequest, errorText))
	h.eventBus.Publish(runtimeFailureFinishEvent(runRequest, errorText))
}

func runtimeFailureEvent(runRequest contracts.RunRequest, errorText string) contracts.RunEvent {
	return contracts.RunEvent{
		EventID:       newID("event"),
		Kind:          "run_event",
		Source:        "gateway",
		AgentID:       "primary",
		AgentLabel:    "主智能体",
		EventType:     "run_failed",
		TraceID:       runRequest.TraceID,
		SessionID:     runRequest.SessionID,
		RunID:         runRequest.RunID,
		Sequence:      1,
		Timestamp:     timestampNow(),
		Stage:         "Failed",
		Summary:       "运行时调用失败",
		Detail:        errorText,
		ResultSummary: "Gateway 未能拿到 Runtime 返回结果。",
		Metadata: map[string]string{
			"error_code":     "runtime_unavailable",
			"error_message":  errorText,
			"error_source":   "gateway",
			"retryable":      "true",
			"result_summary": "Gateway 未能拿到 Runtime 返回结果。",
			"task_title":     runRequest.UserInput,
			"next_step":      "等待运行时恢复后重试",
		},
	}
}

func runtimeFailureFinishEvent(runRequest contracts.RunRequest, errorText string) contracts.RunEvent {
	return contracts.RunEvent{
		EventID:    newID("event"),
		Kind:       "run_event",
		Source:     "gateway",
		AgentID:    "primary",
		AgentLabel: "主智能体",
		EventType:  "run_finished",
		TraceID:    runRequest.TraceID,
		SessionID:  runRequest.SessionID,
		RunID:      runRequest.RunID,
		Sequence:   2,
		Timestamp:  timestampNow(),
		Stage:      "Finish",
		Summary:    "任务因运行时不可达而结束",
		Detail:     errorText,
		Metadata: map[string]string{
			"error_code":    "runtime_unavailable",
			"error_message": errorText,
			"error_source":  "gateway",
			"final_answer":  "运行时当前不可达，本次任务未能执行。请先检查 Runtime 进程后重试。",
			"task_title":    runRequest.UserInput,
			"next_step":     "任务已结束",
		},
	}
}

func applyCheckpointResume(request *contracts.RunRequest, checkpointID string) {
	if checkpointID == "" {
		return
	}
	request.ResumeFromCheckpointID = checkpointID
	request.ResumeStrategy = "after_confirmation"
}

func applyRetryCheckpointResume(request *contracts.RunRequest, checkpointID string) {
	if checkpointID == "" {
		return
	}
	request.ResumeFromCheckpointID = checkpointID
	request.ResumeStrategy = "retry_failure"
}

func copyContextHints(source map[string]string) map[string]string {
	if source == nil {
		return map[string]string{}
	}
	target := make(map[string]string, len(source))
	for key, value := range source {
		target[key] = value
	}
	return target
}

func writeRetryError(w http.ResponseWriter, err error) {
	if errors.Is(err, state.ErrRuntimeCheckpointNotFound()) {
		http.Error(w, "未找到可重试 checkpoint", http.StatusNotFound)
		return
	}
	http.Error(w, err.Error(), http.StatusBadRequest)
}

func stringValue(value *string) string {
	if value == nil {
		return ""
	}
	return *value
}

func writeSSE(w http.ResponseWriter, item contracts.RunEvent) {
	payload, _ := json.Marshal(item)
	_, _ = fmt.Fprintf(w, "event: run_event\n")
	_, _ = fmt.Fprintf(w, "data: %s\n\n", payload)
}

func newID(prefix string) string {
	counter := atomic.AddUint64(&idCounter, 1)
	return fmt.Sprintf("%s-%d-%d", prefix, time.Now().UnixMilli(), counter)
}

func timestampNow() string {
	return fmt.Sprintf("%d", time.Now().UnixMilli())
}
