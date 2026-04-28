package api

import (
	"encoding/json"
	"net/http"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/state"
)

func providersHandler(
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		writeJSON(w, http.StatusOK, buildProviderSettingsResponse(cfg, credentials, runtimeStore))
	}
}
func providerTestHandler(cfg config.AppConfig) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, provider, ok := decodeProviderTest(w, r, cfg)
		if !ok {
			return
		}
		writeJSON(w, http.StatusOK, testProvider(cfg, provider, payload))
	}
}
func providerSaveHandler(
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, provider, ok := decodeProviderSave(w, r, cfg)
		if !ok {
			return
		}
		status, response := saveProvider(provider, payload, credentials, runtimeStore)
		writeJSON(w, status, response)
	}
}
func providerApplyHandler(
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, provider, ok := decodeProviderApply(w, r, cfg)
		if !ok {
			return
		}
		status, response := applyProvider(provider, payload, credentials, runtimeStore)
		writeJSON(w, status, response)
	}
}
func providerRemoveHandler(
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, provider, ok := decodeProviderRemove(w, r, cfg)
		if !ok {
			return
		}
		status, response := removeProvider(provider, payload, credentials, runtimeStore)
		writeJSON(w, status, response)
	}
}
func decodeProviderTest(
	w http.ResponseWriter,
	r *http.Request,
	cfg config.AppConfig,
) (ProviderTestRequest, config.ProviderConfig, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ProviderTestRequest{}, config.ProviderConfig{}, false
	}
	var payload ProviderTestRequest
	if !decodeJSONBody(w, r, &payload) || payload.APIKey == "" {
		http.Error(w, "provider_id and api_key are required", http.StatusBadRequest)
		return ProviderTestRequest{}, config.ProviderConfig{}, false
	}
	provider, ok := findProvider(cfg, payload.ProviderID)
	if !ok {
		http.Error(w, "provider not found", http.StatusBadRequest)
	}
	return payload, provider, ok
}
func decodeProviderSave(
	w http.ResponseWriter,
	r *http.Request,
	cfg config.AppConfig,
) (ProviderSaveRequest, config.ProviderConfig, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ProviderSaveRequest{}, config.ProviderConfig{}, false
	}
	var payload ProviderSaveRequest
	if !decodeJSONBody(w, r, &payload) || payload.APIKey == "" {
		http.Error(w, "provider_id and api_key are required", http.StatusBadRequest)
		return ProviderSaveRequest{}, config.ProviderConfig{}, false
	}
	provider, ok := findProvider(cfg, payload.ProviderID)
	if !ok {
		http.Error(w, "provider not found", http.StatusBadRequest)
	}
	return payload, provider, ok
}
func decodeProviderApply(
	w http.ResponseWriter,
	r *http.Request,
	cfg config.AppConfig,
) (ProviderApplyRequest, config.ProviderConfig, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ProviderApplyRequest{}, config.ProviderConfig{}, false
	}
	var payload ProviderApplyRequest
	if !decodeJSONBody(w, r, &payload) || payload.ProviderID == "" {
		http.Error(w, "provider_id is required", http.StatusBadRequest)
		return ProviderApplyRequest{}, config.ProviderConfig{}, false
	}
	provider, ok := findProvider(cfg, payload.ProviderID)
	if !ok {
		http.Error(w, "provider not found", http.StatusBadRequest)
	}
	return payload, provider, ok
}
func decodeProviderRemove(
	w http.ResponseWriter,
	r *http.Request,
	cfg config.AppConfig,
) (ProviderRemoveRequest, config.ProviderConfig, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ProviderRemoveRequest{}, config.ProviderConfig{}, false
	}
	var payload ProviderRemoveRequest
	if !decodeJSONBody(w, r, &payload) || payload.ProviderID == "" {
		http.Error(w, "provider_id is required", http.StatusBadRequest)
		return ProviderRemoveRequest{}, config.ProviderConfig{}, false
	}
	provider, ok := findProvider(cfg, payload.ProviderID)
	if !ok {
		http.Error(w, "provider not found", http.StatusBadRequest)
	}
	return payload, provider, ok
}
func decodeJSONBody(w http.ResponseWriter, r *http.Request, target any) bool {
	if err := json.NewDecoder(r.Body).Decode(target); err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return false
	}
	return true
}
