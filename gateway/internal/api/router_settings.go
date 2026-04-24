package api

import (
	"net/http"

	"local-agent/gateway/internal/config"
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
