package api

import (
	"net/http"

	"local-agent/gateway/internal/session"
)

func registerLogsRoutes(
	mux *http.ServeMux,
	repoRoot string,
	runtimePort int,
	eventBus *session.EventBus,
) {
	mux.HandleFunc("/api/v1/system/info", systemInfoHandler(repoRoot, runtimePort))
	mux.HandleFunc("/api/v1/logs", logsHandler(eventBus))
	mux.HandleFunc("/api/v1/artifacts/content", artifactContentHandler(repoRoot))
}
