package api

import (
	"encoding/json"
	"net/http"

	"local-agent/gateway/internal/service"
)

func registerReleaseRoutes(mux *http.ServeMux, repoRoot string) {
	mux.HandleFunc("/api/v1/release/run", releaseRunHandler(repoRoot))
}

func releaseRunHandler(repoRoot string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		var req service.ReleaseRunRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "invalid json body", http.StatusBadRequest)
			return
		}
		result, err := service.RunReleaseStep(repoRoot, req)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		writeReleaseResponse(w, result)
	}
}

func writeReleaseResponse(w http.ResponseWriter, result service.ReleaseRunResult) {
	status := http.StatusOK
	if result.Status != "passed" {
		status = http.StatusInternalServerError
	}
	writeJSON(w, status, result)
}
