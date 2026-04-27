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
	"local-agent/gateway/internal/memory"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/service"
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
	executionRegistry *service.ExecutionRegistry
}

type ChatRunRequest struct {
	RequestID       string              `json:"request_id,omitempty"`
	RunID           string              `json:"run_id,omitempty"`
	SessionID       string              `json:"session_id"`
	TraceID         string              `json:"trace_id,omitempty"`
	UserInput       string              `json:"user_input"`
	Mode            string              `json:"mode"`
	Model           contracts.ModelRef     `json:"model"`
	Workspace       contracts.WorkspaceRef `json:"workspace"`
	ContextHints    map[string]string   `json:"context_hints,omitempty"`
	KnowledgeBaseID string              `json:"knowledge_base_id,omitempty"`
}

type ChatRetryRequest struct {
	SessionID    string `json:"session_id"`
	RunID        string `json:"run_id"`
	CheckpointID string `json:"checkpoint_id,omitempty"`
}

type ChatRunAccepted struct {
	Accepted        bool   `json:"accepted"`
	SessionID       string `json:"session_id"`
	RunID           string `json:"run_id"`
	RequestID       string `json:"request_id"`
	TraceID         string `json:"trace_id"`
	InitialStatus   string `json:"initial_status"`
	EntryID         string `json:"entry_id"`
	ProtocolVersion string `json:"protocol_version"`
	StreamEndpoint  string `json:"stream_endpoint"`
	LogsEndpoint    string `json:"logs_endpoint"`
	ConfirmEndpoint string `json:"confirm_endpoint"`
	RetryEndpoint   string `json:"retry_endpoint"`
	CancelEndpoint  string `json:"cancel_endpoint"`
}

var idCounter uint64

const (
	chatEntryID         = "gateway.chat.entry1"
	chatProtocolVersion = "v1"
)

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
		executionRegistry: service.NewExecutionRegistry(),
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
	go service.Execute(runRequest, h.runtimeClient, h.eventBus, h.confirmationStore, h.executionRegistry)
	writeJSON(w, http.StatusAccepted, newChatRunAccepted(runRequest))
}

func decodeRunPayload(r *http.Request) (ChatRunRequest, error) {
	var payload ChatRunRequest
	err := json.NewDecoder(r.Body).Decode(&payload)
	return payload, err
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
	go service.Execute(runRequest, h.runtimeClient, h.eventBus, h.confirmationStore, h.executionRegistry)
	writeJSON(w, http.StatusAccepted, newChatRunAccepted(runRequest))
}

func (h *ChatHandler) Stream(w http.ResponseWriter, r *http.Request) {
	sessionID, flusher, ok := validateStreamRequest(w, r)
	if !ok {
		return
	}
	setStreamHeaders(w)
	h.flushSnapshot(sessionID, w, flusher)
	stream, cancel := h.eventBus.Subscribe(sessionID)
	defer cancel()
	streamSessionEvents(r.Context(), w, flusher, stream)
}

func validateStreamRequest(w http.ResponseWriter, r *http.Request) (string, http.Flusher, bool) {
	sessionID := r.URL.Query().Get("session_id")
	if sessionID == "" {
		http.Error(w, "session_id is required", http.StatusBadRequest)
		return "", nil, false
	}
	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "streaming unsupported", http.StatusInternalServerError)
		return "", nil, false
	}
	return sessionID, flusher, true
}

func setStreamHeaders(w http.ResponseWriter) {
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")
}

func (h *ChatHandler) flushSnapshot(sessionID string, w http.ResponseWriter, flusher http.Flusher) {
	for _, item := range h.eventBus.Snapshot(sessionID) {
		writeSSE(w, item)
	}
	flusher.Flush()
}

func streamSessionEvents(ctx context.Context, w http.ResponseWriter, flusher http.Flusher, stream <-chan contracts.RunEvent) {
	heartbeat := time.NewTicker(15 * time.Second)
	defer heartbeat.Stop()
	for {
		select {
		case <-ctx.Done():
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

func newChatRunAccepted(runRequest contracts.RunRequest) ChatRunAccepted {
	return ChatRunAccepted{
		Accepted:        true,
		SessionID:       runRequest.SessionID,
		RunID:           runRequest.RunID,
		RequestID:       runRequest.RequestID,
		TraceID:         runRequest.TraceID,
		InitialStatus:   "accepted",
		EntryID:         chatEntryID,
		ProtocolVersion: chatProtocolVersion,
		StreamEndpoint:  "/api/v1/events/stream?session_id={session_id}",
		LogsEndpoint:    "/api/v1/logs?session_id={session_id}&run_id={run_id}",
		ConfirmEndpoint: "/api/v1/chat/confirm",
		RetryEndpoint:   "/api/v1/chat/retry",
		CancelEndpoint:  "/api/v1/chat/cancel",
	}
}
