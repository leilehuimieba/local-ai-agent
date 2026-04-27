package api

import (
	"net/http"

	"local-agent/gateway/internal/service"
)

func diagnosticsLogsRemediationHandler(repoRoot string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		writeJSON(w, http.StatusOK, service.RemediateLogsWritable(repoRoot))
	}
}

func diagnosticsFrontendRemediationHandler(repoRoot string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		writeJSON(w, http.StatusOK, service.RemediateFrontendDist(repoRoot))
	}
}

func diagnosticsGatewayRemediationHandler(repoRoot string, port int) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		writeJSON(w, http.StatusOK, service.RemediateGatewayUnreachable(repoRoot, port))
	}
}

func diagnosticsConfigRemediationHandler(repoRoot string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		writeJSON(w, http.StatusOK, service.InspectConfigRemediation(repoRoot))
	}
}
