package api

import (
	"context"
	"net/http"
	"time"

	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/state"
)

type catalogRouteDeps struct {
	runtimeClient *runtimeclient.Client
	state         *state.SettingsStore
}

func registerCatalogRoutes(mux *http.ServeMux, runtimeClient *runtimeclient.Client, settingsStore *state.SettingsStore) {
	deps := catalogRouteDeps{runtimeClient: runtimeClient, state: settingsStore}
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
	payload, err := deps.runtimeClient.Capabilities(ctx, deps.capabilityMode(r))
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

func (deps catalogRouteDeps) capabilityMode(r *http.Request) string {
	mode := r.URL.Query().Get("mode")
	if mode != "" {
		return mode
	}
	currentMode, _, _, _, _, _, _, _ := deps.state.Snapshot()
	return currentMode
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
