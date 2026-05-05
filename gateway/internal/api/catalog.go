package api

import (
	"context"
	"net/http"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/mcp"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/service"
	"local-agent/gateway/internal/state"
)

type catalogRouteDeps struct {
	runtimeClient *runtimeclient.Client
	state         *state.SettingsStore
	repoRoot      string
	appConfig     config.AppConfig
	mcpManager    *mcp.Manager
}

func registerCatalogRoutes(
	mux *http.ServeMux,
	repoRoot string,
	cfg config.AppConfig,
	runtimeClient *runtimeclient.Client,
	settingsStore *state.SettingsStore,
	mgr *mcp.Manager,
) {
	deps := catalogRouteDeps{
		runtimeClient: runtimeClient,
		state:         settingsStore,
		repoRoot:      repoRoot,
		appConfig:     cfg,
		mcpManager:    mgr,
	}
	mux.HandleFunc("/api/v1/capabilities", deps.handleCapabilities)
	mux.HandleFunc("/api/v1/connectors", deps.handleConnectors)
}

func (deps catalogRouteDeps) handleCapabilities(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	ctx, cancel := runtimeCatalogContext(r.Context())
	defer cancel()
	payload, err := deps.runtimeClient.CapabilitiesForRequest(ctx, deps.capabilityRequest(r))
	if err != nil {
		writeRuntimeProxyError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, payload)
}

func (deps catalogRouteDeps) handleConnectors(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	ctx, cancel := runtimeCatalogContext(r.Context())
	defer cancel()
	payload, err := deps.runtimeClient.Connectors(ctx)
	if err != nil {
		writeRuntimeProxyError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, payload)
}

func (deps catalogRouteDeps) capabilityRequest(r *http.Request) contracts.RunRequest {
	mode, model, _, workspace, _, _, _, _, _ := deps.state.Snapshot()
	if override := r.URL.Query().Get("mode"); override != "" {
		mode = override
	}
	hints := service.RunContextHints(nil, deps.repoRoot, false)
	hints = service.WithKnowledgeHints(hints, deps.appConfig.Siyuan)
	hints = withMCPToolHints(hints, deps.mcpManager)
	return contracts.RunRequest{
		RequestID:    "capability-request",
		RunID:        "capability-run",
		SessionID:    "capability-session",
		TraceID:      "capability-trace",
		UserInput:    "capability catalog",
		Mode:         mode,
		ModelRef:     model,
		WorkspaceRef: workspace,
		ContextHints: hints,
	}
}

func runtimeCatalogContext(parent context.Context) (context.Context, context.CancelFunc) {
	return context.WithTimeout(parent, 5*time.Second)
}

func writeRuntimeProxyError(w http.ResponseWriter, err error) {
	writeJSON(w, http.StatusBadGateway, map[string]any{
		"error":   "runtime_unavailable",
		"message": err.Error(),
	})
}
