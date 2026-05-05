package api

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"net/http/httptest"
	"os/exec"
	"path/filepath"
	"runtime"
	"testing"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/mcp"

	"github.com/stretchr/testify/require"
)

func TestBrowserMCPRuntimeBridgeE2EOpenAndReadPage(t *testing.T) {
	repoRoot := t.TempDir()
	pageServer := newBrowserFixtureServer()
	defer pageServer.Close()
	mcpURL := startBrowserMCPProcess(t)
	mgr := newBrowserConnectedMCPManager(mcpURL)
	rt := newBrowserBridgeRuntime(t, pageServer.URL)
	cfg := sampleAppConfig()
	cfg.Providers[0].APIKey = "test-key"
	gateway, gatewayToken := startBridgeGateway(t, repoRoot, cfg, rt.port, mgr)
	defer gateway.Close()
	resp := postBridgeChatRun(t, gateway, gatewayToken)
	require.Equal(t, http.StatusAccepted, resp.StatusCode)
	capture := rt.wait(t)
	require.Contains(t, capture.request.ContextHints["mcp_tool_specs_json"], "mcp__browser__open_page")
	require.Contains(t, capture.request.ContextHints["mcp_tool_specs_json"], "mcp__browser__read_page")
	require.Equal(t, http.StatusOK, capture.openStatus)
	require.Contains(t, capture.openBody, `"page_id":"page_`)
	require.Equal(t, http.StatusOK, capture.readStatus)
	require.Contains(t, capture.readBody, "AE Browser Bridge")
	require.Contains(t, capture.readBody, "Bridge Ready")
}

func newBrowserConnectedMCPManager(rawURL string) *mcp.Manager {
	mgr := mcp.NewManager([]config.MCPServerConfig{{
		ID: "browser", Name: "Browser", Type: "http", URL: rawURL, Enabled: true,
		ToolPolicies: browserMCPPolicies(),
	}})
	mgr.ConnectAll()
	return mgr
}

func browserMCPPolicies() []config.MCPToolPolicy {
	return []config.MCPToolPolicy{
		{ToolName: "open_page", Allowed: true, RiskLevel: "medium", AuditEnabled: true},
		{ToolName: "read_page", Allowed: true, RiskLevel: "low", AuditEnabled: true},
	}
}

func newBrowserFixtureServer() *httptest.Server {
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		_, _ = w.Write([]byte(`<html><head><title>AE Browser Bridge</title></head><body><h1>Bridge Ready</h1><p>Hello browser mcp.</p></body></html>`))
	}))
}

type browserBridgeRuntime struct {
	port      int
	targetURL string
	ch        chan browserBridgeCapture
}

type browserBridgeCapture struct {
	request    contracts.RunRequest
	openStatus int
	openBody   string
	readStatus int
	readBody   string
}

func newBrowserBridgeRuntime(t *testing.T, targetURL string) *browserBridgeRuntime {
	listener := localListener(t)
	rt := &browserBridgeRuntime{
		port: listener.Addr().(*net.TCPAddr).Port, targetURL: targetURL,
		ch: make(chan browserBridgeCapture, 1),
	}
	server := serveOnListener(t, listener, http.HandlerFunc(rt.handleRun))
	t.Cleanup(func() { _ = server.Close() })
	return rt
}

func (rt *browserBridgeRuntime) handleRun(w http.ResponseWriter, r *http.Request) {
	var request contracts.RunRequest
	_ = json.NewDecoder(r.Body).Decode(&request)
	openStatus, openBody, readStatus, readBody := browserBridgeFlow(request, rt.targetURL)
	rt.ch <- browserBridgeCapture{
		request: request, openStatus: openStatus, openBody: openBody,
		readStatus: readStatus, readBody: readBody,
	}
	writeJSON(w, http.StatusOK, contracts.RuntimeRunResponse{
		Result: contracts.RunResult{RunID: request.RunID, Status: "completed", FinalStage: "Finish"},
	})
}

func (rt *browserBridgeRuntime) wait(t *testing.T) browserBridgeCapture {
	t.Helper()
	select {
	case capture := <-rt.ch:
		return capture
	case <-time.After(5 * time.Second):
		t.Fatal("browser bridge runtime was not called")
		return browserBridgeCapture{}
	}
}

func browserBridgeFlow(request contracts.RunRequest, targetURL string) (int, string, int, string) {
	openStatus, openBody := callBrowserTool(request, "open_page", map[string]any{"url": targetURL})
	pageID := readPageID(openBody)
	readStatus, readBody := callBrowserTool(request, "read_page", map[string]any{"page_id": pageID})
	return openStatus, openBody, readStatus, readBody
}

func callBrowserTool(request contracts.RunRequest, name string, arguments map[string]any) (int, string) {
	body, _ := json.Marshal(map[string]any{"server_id": "browser", "name": name, "arguments": arguments})
	httpReq, _ := http.NewRequest(http.MethodPost, request.ContextHints["mcp_gateway_url"], bytes.NewReader(body))
	httpReq.Header.Set("Content-Type", "application/json")
	httpReq.Header.Set("X-Local-Agent-Token", request.ContextHints["mcp_gateway_token"])
	resp, err := http.DefaultClient.Do(httpReq)
	if err != nil {
		return 0, err.Error()
	}
	defer resp.Body.Close()
	var raw bytes.Buffer
	_, _ = raw.ReadFrom(resp.Body)
	return resp.StatusCode, raw.String()
}

func readPageID(raw string) string {
	var payload struct {
		PageID string `json:"page_id"`
	}
	_ = json.Unmarshal([]byte(raw), &payload)
	return payload.PageID
}

func startBrowserMCPProcess(t *testing.T) string {
	t.Helper()
	if _, err := exec.LookPath("node"); err != nil {
		t.Skip("node not found in PATH")
	}
	port := freePort(t)
	script := filepath.Join(repoRootFromTestFile(t), "frontend", "scripts", "browser-mcp-server.mjs")
	cmd := exec.Command("node", script)
	cmd.Dir = filepath.Join(repoRootFromTestFile(t), "frontend")
	cmd.Env = append(cmd.Environ(), "LOCAL_AGENT_BROWSER_MCP_PORT="+port)
	require.NoError(t, cmd.Start())
	t.Cleanup(func() { _ = cmd.Process.Kill() })
	healthURL := "http://127.0.0.1:" + port + "/health"
	if !waitForHTTP200(healthURL, 20*time.Second) {
		t.Skipf("browser mcp did not become ready: %s", healthURL)
	}
	return "http://127.0.0.1:" + port + "/mcp"
}

func freePort(t *testing.T) string {
	t.Helper()
	listener := localListener(t)
	defer listener.Close()
	return strconvPort(listener.Addr().(*net.TCPAddr).Port)
}

func strconvPort(port int) string {
	return fmt.Sprintf("%d", port)
}

func waitForHTTP200(target string, timeout time.Duration) bool {
	deadline := time.Now().Add(timeout)
	for time.Now().Before(deadline) {
		if httpStatusOK(target) {
			return true
		}
		time.Sleep(300 * time.Millisecond)
	}
	return false
}

func httpStatusOK(target string) bool {
	client := http.Client{Timeout: time.Second}
	resp, err := client.Get(target)
	if err != nil {
		return false
	}
	defer resp.Body.Close()
	return resp.StatusCode == http.StatusOK
}

func repoRootFromTestFile(t *testing.T) string {
	t.Helper()
	return filepath.Clean(filepath.Join(filepath.Dir(currentTestFile(t)), "..", "..", ".."))
}

func currentTestFile(t *testing.T) string {
	t.Helper()
	_, file, _, ok := runtime.Caller(0)
	require.True(t, ok)
	return file
}
