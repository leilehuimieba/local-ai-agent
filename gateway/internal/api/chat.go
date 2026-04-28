package api

import (
	"net/http"

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
