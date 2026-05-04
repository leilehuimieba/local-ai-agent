package api

import (
	"encoding/json"
	"net/http"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/mcp"
	"local-agent/gateway/internal/state"
)

func registerSettingsRoutes(
	mux *http.ServeMux,
	repoRoot string,
	cfg config.AppConfig,
	settingsStore *state.SettingsStore,
	mgr *mcp.Manager,
) {
	mux.HandleFunc("/api/v1/settings", settingsHandler(repoRoot, cfg, settingsStore, mgr))
	mux.HandleFunc("/api/v1/settings/diagnostics/check", diagnosticsCheckHandler(repoRoot, cfg, settingsStore, mgr))
	mux.HandleFunc("/api/v1/settings/diagnostics/remediate/logs", diagnosticsLogsRemediationHandler(repoRoot))
	mux.HandleFunc("/api/v1/settings/diagnostics/remediate/frontend-dist", diagnosticsFrontendRemediationHandler(repoRoot))
	mux.HandleFunc("/api/v1/settings/diagnostics/remediate/gateway", diagnosticsGatewayRemediationHandler(repoRoot, cfg.GatewayPort))
	mux.HandleFunc("/api/v1/settings/diagnostics/remediate/config", diagnosticsConfigRemediationHandler(repoRoot))
	mux.HandleFunc("/api/v1/settings/external-connections/action", externalConnectionActionHandler(repoRoot, cfg, settingsStore))
}

func settingsHandler(repoRoot string, cfg config.AppConfig, store *state.SettingsStore, mgr *mcp.Manager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if err := applySettingsUpdate(w, r, repoRoot, store, mgr); err != nil {
			return
		}
		writeJSON(w, http.StatusOK, buildSettingsResponse(repoRoot, cfg, store, mgr))
	}
}

func applySettingsUpdate(w http.ResponseWriter, r *http.Request, repoRoot string, store *state.SettingsStore, mgr *mcp.Manager) error {
	if r.Method != http.MethodPost {
		return nil
	}
	var payload settingsUpdatePayload
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return err
	}
	if err := updateSettingsStore(store, payload); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return err
	}
	if err := applyDirectorySettings(store, payload); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return err
	}
	if err := applyMCPSettings(repoRoot, payload, mgr); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return err
	}
	return nil
}

type settingsUpdatePayload struct {
	Mode                   string             `json:"mode"`
	Model                  contracts.ModelRef `json:"model"`
	WorkspaceID            string             `json:"workspace_id"`
	DirectoryPromptEnabled *bool              `json:"directory_prompt_enabled"`
	ShowRiskLevel          *bool              `json:"show_risk_level"`
	RevokeDirectoryRoot    string             `json:"revoke_directory_root"`
	AddDirectoryName       string             `json:"add_directory_name"`
	AddDirectoryPath       string             `json:"add_directory_path"`
	EmbeddingProviderID    string             `json:"embedding_provider_id"`
	AddMCPName             string             `json:"add_mcp_name"`
	AddMCPURL              string             `json:"add_mcp_url"`
	RemoveMCPID            string             `json:"remove_mcp_id"`
	MCPPolicyServerID      string             `json:"mcp_policy_server_id"`
	MCPPolicyToolName      string             `json:"mcp_policy_tool_name"`
	MCPPolicyAllowed       *bool              `json:"mcp_policy_allowed"`
	MCPPolicyRiskLevel     string             `json:"mcp_policy_risk_level"`
	MCPPolicyConfirm       *bool              `json:"mcp_policy_requires_confirmation"`
}

func updateSettingsStore(store *state.SettingsStore, payload settingsUpdatePayload) error {
	return store.UpdateFull(payload.Mode, payload.Model, payload.WorkspaceID, payload.DirectoryPromptEnabled, payload.ShowRiskLevel, payload.EmbeddingProviderID)
}

func applyDirectorySettings(store *state.SettingsStore, payload settingsUpdatePayload) error {
	if payload.AddDirectoryPath != "" {
		if err := store.AddCustomDirectory(payload.AddDirectoryName, payload.AddDirectoryPath); err != nil {
			return err
		}
	}
	if payload.RevokeDirectoryRoot != "" {
		store.RevokeDirectoryApproval(payload.RevokeDirectoryRoot)
	}
	return nil
}
