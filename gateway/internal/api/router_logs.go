package api

import (
	"net/http"

	"local-agent/gateway/internal/contracts"
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

func systemInfoHandler(repoRoot string, runtimePort int) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		writeJSON(w, http.StatusOK, map[string]any{
			"status":         "ok",
			"formal_entry":   "desktop launcher -> local web console",
			"system_entry":   "gateway",
			"repo_root":      repoRoot,
			"runtime_status": fetchRuntimeStatus(runtimePort),
		})
	}
}

func logsHandler(eventBus *session.EventBus) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		query, err := decodeLogsQuery(r)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		items := queryLogItems(eventBus, query)
		writeJSON(w, http.StatusOK, LogsResponse{Items: items})
	}
}

func queryLogItems(eventBus *session.EventBus, query logsQuery) []contracts.LogEntry {
	var items []contracts.LogEntry
	if query.View == "runs" {
		items = eventBus.RecentRuns(query.Limit, query.SessionID)
	} else {
		items = eventBus.RecentBy(query.Limit, query.SessionID, query.RunID)
	}
	return applyLogsQueryFilter(items, query)
}

func applyLogsQueryFilter(items []contracts.LogEntry, query logsQuery) []contracts.LogEntry {
	if query.SessionID == "" && query.RunID == "" {
		return items
	}
	filtered := make([]contracts.LogEntry, 0, len(items))
	for _, item := range items {
		if query.SessionID != "" && item.SessionID != query.SessionID {
			continue
		}
		if query.RunID != "" && item.RunID != query.RunID {
			continue
		}
		filtered = append(filtered, item)
	}
	return filtered
}
