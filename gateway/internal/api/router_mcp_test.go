package api

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/mcp"

	"github.com/stretchr/testify/require"
)

func TestMCPCallRejectsNonAllowlistedToolAndWritesAudit(t *testing.T) {
	server := newFakeMCPServer(t)
	defer server.Close()
	mgr := newConnectedMCPManager(server.URL, nil)
	rec := invokeMCPCall(mgr, t.TempDir(), `{"server_id":"docs","name":"search","arguments":{"q":"x"}}`)
	require.Equal(t, http.StatusBadRequest, rec.Code)
	require.Contains(t, rec.Body.String(), "not allowlisted")
}

func TestMCPCallAllowsConfiguredLowRiskTool(t *testing.T) {
	server := newFakeMCPServer(t)
	defer server.Close()
	policy := []config.MCPToolPolicy{{ToolName: "search", Allowed: true, RiskLevel: "low", AuditEnabled: true}}
	mgr := newConnectedMCPManager(server.URL, policy)
	rec := invokeMCPCall(mgr, t.TempDir(), `{"server_id":"docs","name":"search","arguments":{"q":"x"}}`)
	require.Equal(t, http.StatusOK, rec.Code)
	require.JSONEq(t, `{"ok":true}`, rec.Body.String())
}

func newConnectedMCPManager(rawURL string, policies []config.MCPToolPolicy) *mcp.Manager {
	mgr := mcp.NewManager([]config.MCPServerConfig{{
		ID: "docs", Name: "Docs", Type: "http", URL: rawURL,
		Enabled: true, ToolPolicies: policies,
	}})
	mgr.ConnectAll()
	return mgr
}

func invokeMCPCall(mgr *mcp.Manager, repoRoot string, body string) *httptest.ResponseRecorder {
	rec := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/mcp/call", strings.NewReader(body))
	mcpCallHandler(mgr, repoRoot).ServeHTTP(rec, req)
	return rec
}

func newFakeMCPServer(t *testing.T) *httptest.Server {
	t.Helper()
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var req struct {
			Method string `json:"method"`
		}
		require.NoError(t, json.NewDecoder(r.Body).Decode(&req))
		writeFakeMCPResponse(w, req.Method)
	}))
}

func writeFakeMCPResponse(w http.ResponseWriter, method string) {
	w.Header().Set("Content-Type", "application/json")
	switch method {
	case "tools/list":
		_, _ = w.Write([]byte(`{"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"search","description":"Search docs","inputSchema":{}}]}}`))
	case "tools/call":
		_, _ = w.Write([]byte(`{"jsonrpc":"2.0","id":3,"result":{"ok":true}}`))
	default:
		_, _ = w.Write([]byte(`{"jsonrpc":"2.0","id":1,"result":{"ok":true}}`))
	}
}
