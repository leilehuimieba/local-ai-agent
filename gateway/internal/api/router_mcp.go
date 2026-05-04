package api

import (
	"encoding/json"
	"net/http"
	"strings"
	"time"

	"local-agent/gateway/internal/mcp"
)

func registerMCPRoutes(mux *http.ServeMux, mgr *mcp.Manager, repoRoot string) {
	if mgr == nil {
		return
	}
	mux.HandleFunc("/api/v1/mcp/servers", mcpServersHandler(mgr))
	mux.HandleFunc("/api/v1/mcp/tools", mcpToolsHandler(mgr))
	mux.HandleFunc("/api/v1/mcp/call", mcpCallHandler(mgr, repoRoot))
}

func mcpServersHandler(mgr *mcp.Manager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		writeJSON(w, http.StatusOK, map[string]any{
			"servers": mgr.Status(),
		})
	}
}

func mcpToolsHandler(mgr *mcp.Manager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		writeJSON(w, http.StatusOK, map[string]any{
			"tools": mgr.AllTools(),
		})
	}
}

func mcpCallHandler(mgr *mcp.Manager, repoRoot string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		var payload mcpCallPayload
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			http.Error(w, "invalid json", http.StatusBadRequest)
			return
		}
		if payload.ServerID == "" {
			http.Error(w, "server_id is required", http.StatusBadRequest)
			return
		}
		start := time.Now()
		policy := mcpCallPolicy(mgr, payload.ServerID, payload.Name)
		result, err := mgr.Call(payload.ServerID, payload.Name, payload.Arguments)
		if err != nil {
			outcome := mcpCallErrorOutcome(err)
			writeMCPAudit(repoRoot, newMCPAuditRecord(payload, policy, start, outcome, err))
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		writeMCPAudit(repoRoot, newMCPAuditRecord(payload, policy, start, "success", nil))
		w.Header().Set("Content-Type", "application/json")
		w.Write(result)
	}
}

type mcpCallPayload struct {
	ServerID  string         `json:"server_id"`
	Name      string         `json:"name"`
	Arguments map[string]any `json:"arguments"`
	SessionID string         `json:"session_id"`
	RunID     string         `json:"run_id"`
	TraceID   string         `json:"trace_id"`
}

func mcpCallPolicy(mgr *mcp.Manager, serverID string, name string) mcp.ToolPolicy {
	policy, err := mgr.ToolPolicy(serverID, name)
	if err != nil {
		return mcp.ToolPolicy{RiskLevel: mcp.DefaultRiskLevel, AuditEnabled: true}
	}
	return policy
}

func mcpCallErrorOutcome(err error) string {
	text := err.Error()
	if strings.Contains(text, "not allowlisted") || strings.Contains(text, "requires confirmation") {
		return "denied"
	}
	return "failed"
}
