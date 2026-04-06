package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"sync/atomic"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
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
}

type ChatRunRequest struct {
	SessionID    string              `json:"session_id"`
	UserInput    string              `json:"user_input"`
	Mode         string              `json:"mode"`
	Model        config.ModelRef     `json:"model"`
	Workspace    config.WorkspaceRef `json:"workspace"`
	ContextHints map[string]string   `json:"context_hints,omitempty"`
}

type ChatRunAccepted struct {
	Accepted      bool   `json:"accepted"`
	SessionID     string `json:"session_id"`
	RunID         string `json:"run_id"`
	InitialStatus string `json:"initial_status"`
}

var idCounter uint64

func NewChatHandler(repoRoot string, cfg config.AppConfig, runtimeClient *runtimeclient.Client, eventBus *session.EventBus, settingsStore *state.SettingsStore, confirmationStore *state.ConfirmationStore) *ChatHandler {
	return &ChatHandler{
		repoRoot:          repoRoot,
		appConfig:         cfg,
		runtimeClient:     runtimeClient,
		eventBus:          eventBus,
		settingsStore:     settingsStore,
		confirmationStore: confirmationStore,
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
	return contracts.RunRequest{
		RequestID:    newID("request"),
		RunID:        newID("run"),
		SessionID:    sessionID,
		TraceID:      newID("trace"),
		UserInput:    payload.UserInput,
		Mode:         mode,
		ModelRef:     model,
		ProviderRef:  providerRef(h.appConfig, model.ProviderID),
		WorkspaceRef: workspace,
		ContextHints: h.withKnowledgeHints(runContextHints(payload.ContextHints, h.repoRoot, firstSeen)),
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

func providerRef(cfg config.AppConfig, providerID string) contracts.ProviderRef {
	for _, item := range cfg.Providers {
		if item.ProviderID == providerID {
			return contracts.ProviderRef{
				ProviderID:          item.ProviderID,
				DisplayName:         item.DisplayName,
				BaseURL:             item.BaseURL,
				ChatCompletionsPath: item.ChatCompletionsPath,
				ModelsPath:          item.ModelsPath,
				APIKey:              item.APIKey,
			}
		}
	}
	return contracts.ProviderRef{ProviderID: providerID}
}

func (h *ChatHandler) Confirm(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var decision contracts.ConfirmationDecision
	if err := json.NewDecoder(r.Body).Decode(&decision); err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return
	}

	if decision.ConfirmationID == "" || decision.RunID == "" {
		http.Error(w, "confirmation_id and run_id are required", http.StatusBadRequest)
		return
	}

	switch decision.Decision {
	case "approve", "reject", "cancel":
	default:
		http.Error(w, "invalid decision", http.StatusBadRequest)
		return
	}

	pending, ok := h.confirmationStore.Get(decision.ConfirmationID)
	if !ok {
		http.Error(w, "confirmation not found", http.StatusNotFound)
		return
	}

	if pending.Request.RunID != decision.RunID {
		http.Error(w, "run_id does not match confirmation", http.StatusBadRequest)
		return
	}

	switch decision.Decision {
	case "approve":
		h.confirmationStore.Delete(decision.ConfirmationID)
		if pending.Confirmation.Kind == "workspace_access" && decision.Remember {
			h.settingsStore.ApproveWorkspace(pending.Request.WorkspaceRef.WorkspaceID)
		}
		if pending.Request.ContextHints == nil {
			pending.Request.ContextHints = make(map[string]string)
		}
		pending.Request.ContextHints["workspace_first_seen"] = "false"
		pending.Request.ConfirmationDecision = &decision
		go h.execute(pending.Request)
		writeJSON(w, http.StatusAccepted, map[string]any{
			"accepted":        true,
			"confirmation_id": decision.ConfirmationID,
			"decision":        decision.Decision,
		})
	case "reject", "cancel":
		h.confirmationStore.Delete(decision.ConfirmationID)
		h.eventBus.Publish(rejectedConfirmationEvent(decision, pending))
		writeJSON(w, http.StatusOK, map[string]any{
			"accepted":        true,
			"confirmation_id": decision.ConfirmationID,
			"decision":        decision.Decision,
		})
	}
}

func rejectedConfirmationEvent(decision contracts.ConfirmationDecision, pending state.PendingConfirmation) contracts.RunEvent {
	summary := rejectionSummary(decision.Decision)
	return contracts.RunEvent{
		EventID:        newID("event"),
		Kind:           "run_event",
		Source:         "gateway",
		AgentID:        "primary",
		AgentLabel:     "主智能体",
		EventType:      "run_finished",
		TraceID:        pending.Request.TraceID,
		SessionID:      pending.Request.SessionID,
		RunID:          pending.Request.RunID,
		Sequence:       99,
		Timestamp:      timestampNow(),
		Stage:          "Finish",
		Summary:        summary,
		Detail:         summary,
		RiskLevel:      pending.Confirmation.RiskLevel,
		ConfirmationID: decision.ConfirmationID,
		Metadata:       rejectionMetadata(decision, pending, summary),
	}
}

func rejectionSummary(decision string) string {
	if decision == "reject" {
		return "用户拒绝了本次高风险动作"
	}
	return "用户取消了本次确认动作"
}

func rejectionMetadata(decision contracts.ConfirmationDecision, pending state.PendingConfirmation, summary string) map[string]string {
	return map[string]string{
		"confirmation_id": decision.ConfirmationID,
		"decision":        decision.Decision,
		"final_answer":    summary,
		"kind":            pending.Confirmation.Kind,
		"risk_level":      pending.Confirmation.RiskLevel,
		"task_title":      pending.Confirmation.ActionSummary,
		"next_step":       "任务已结束",
	}
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
