package api

import (
	"encoding/json"
	"net/http"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/state"
)

func registerSettingsRoutes(
	mux *http.ServeMux,
	repoRoot string,
	cfg config.AppConfig,
	settingsStore *state.SettingsStore,
) {
	mux.HandleFunc("/api/v1/settings", settingsHandler(repoRoot, cfg, settingsStore))
	mux.HandleFunc("/api/v1/settings/diagnostics/check", diagnosticsCheckHandler(repoRoot, cfg, settingsStore))
	mux.HandleFunc("/api/v1/settings/diagnostics/remediate/logs", diagnosticsLogsRemediationHandler(repoRoot))
	mux.HandleFunc("/api/v1/settings/diagnostics/remediate/frontend-dist", diagnosticsFrontendRemediationHandler(repoRoot))
	mux.HandleFunc("/api/v1/settings/diagnostics/remediate/gateway", diagnosticsGatewayRemediationHandler(repoRoot, cfg.GatewayPort))
	mux.HandleFunc("/api/v1/settings/diagnostics/remediate/config", diagnosticsConfigRemediationHandler(repoRoot))
	mux.HandleFunc("/api/v1/settings/external-connections/action", externalConnectionActionHandler(repoRoot, cfg, settingsStore))
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
		Mode                   string             `json:"mode"`
		Model                  contracts.ModelRef `json:"model"`
		WorkspaceID            string             `json:"workspace_id"`
		DirectoryPromptEnabled *bool              `json:"directory_prompt_enabled"`
		ShowRiskLevel          *bool              `json:"show_risk_level"`
		RevokeDirectoryRoot    string             `json:"revoke_directory_root"`
		EmbeddingProviderID    string             `json:"embedding_provider_id"`
	}
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return err
	}
	if err := store.UpdateFull(payload.Mode, payload.Model, payload.WorkspaceID, payload.DirectoryPromptEnabled, payload.ShowRiskLevel, payload.EmbeddingProviderID); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return err
	}
	if payload.RevokeDirectoryRoot != "" {
		store.RevokeDirectoryApproval(payload.RevokeDirectoryRoot)
	}
	return nil
}
