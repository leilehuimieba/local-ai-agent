package api

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"net/http/httptest"
	"os"
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
	require.Contains(t, capture.request.ContextHints["mcp_tool_specs_json"], "mcp__browser__click")
	require.Contains(t, capture.request.ContextHints["mcp_tool_specs_json"], "mcp__browser__type")
	require.Contains(t, capture.request.ContextHints["mcp_tool_specs_json"], "mcp__browser__select")
	require.Contains(t, capture.request.ContextHints["mcp_tool_specs_json"], "mcp__browser__submit")
	require.Contains(t, capture.request.ContextHints["mcp_tool_specs_json"], "mcp__browser__upload")
	require.Equal(t, http.StatusOK, capture.openStatus)
	require.Contains(t, capture.openBody, `"page_id":"page_`)
	require.Equal(t, http.StatusOK, capture.readStatus)
	require.Contains(t, capture.readBody, "AE Browser Bridge")
	require.Contains(t, capture.readBody, "Bridge Ready")
}

func TestBrowserMCPRuntimeBridgeE2ERiskyInteractions(t *testing.T) {
	repoRoot := t.TempDir()
	pageServer := newRiskyBrowserFixtureServer()
	defer pageServer.Close()
	mcpURL := startBrowserMCPProcess(t)
	mgr := newBrowserConnectedMCPManager(mcpURL)
	uploadFile := filepath.Join(t.TempDir(), "upload-e2e.txt")
	require.NoError(t, os.WriteFile(uploadFile, []byte("upload-content"), 0o644))
	rt := newBrowserRiskyBridgeRuntime(t, pageServer.URL, uploadFile)
	cfg := sampleAppConfig()
	cfg.Providers[0].APIKey = "test-key"
	gateway, gatewayToken := startBridgeGateway(t, repoRoot, cfg, rt.port, mgr)
	defer gateway.Close()
	resp := postBridgeChatRun(t, gateway, gatewayToken)
	require.Equal(t, http.StatusAccepted, resp.StatusCode)
	capture := rt.wait(t)
	require.Equal(t, http.StatusOK, capture.openStatus)
	require.Equal(t, http.StatusOK, capture.selectStatus)
	require.Equal(t, http.StatusOK, capture.uploadStatus)
	require.Equal(t, http.StatusOK, capture.submitStatus)
	require.Equal(t, http.StatusOK, capture.readStatus)
	require.Contains(t, capture.readBody, "advanced")
	require.Contains(t, capture.readBody, filepath.Base(uploadFile))
	require.Contains(t, capture.readBody, "submitted")
}

func TestBrowserMCPGatewayAllowsApprovedInteractionTools(t *testing.T) {
	pageServer := newInteractiveBrowserFixtureServer()
	defer pageServer.Close()
	mcpURL := startBrowserMCPProcess(t)
	mgr := newBrowserConnectedMCPManager(mcpURL)
	rt := newCatalogRuntime(t)
	cfg := sampleAppConfig()
	cfg.Providers[0].APIKey = "test-key"
	gateway, gatewayToken := startBridgeGateway(t, t.TempDir(), cfg, rt.port, mgr)
	defer gateway.Close()
	assertApprovedInteractionFlow(t, gateway, gatewayToken, pageServer.URL)
}

func TestBrowserMCPGatewayAllowsApprovedRiskyInteractionTools(t *testing.T) {
	pageServer := newRiskyBrowserFixtureServer()
	defer pageServer.Close()
	mcpURL := startBrowserMCPProcess(t)
	mgr := newBrowserConnectedMCPManager(mcpURL)
	rt := newCatalogRuntime(t)
	cfg := sampleAppConfig()
	cfg.Providers[0].APIKey = "test-key"
	gateway, gatewayToken := startBridgeGateway(t, t.TempDir(), cfg, rt.port, mgr)
	defer gateway.Close()
	uploadFile := filepath.Join(t.TempDir(), "upload.txt")
	require.NoError(t, os.WriteFile(uploadFile, []byte("upload-content"), 0o644))
	assertApprovedRiskyInteractionFlow(t, gateway, gatewayToken, pageServer.URL, uploadFile)
}

func assertApprovedInteractionFlow(t *testing.T, gateway *http.Server, gatewayToken string, pageURL string) {
	openStatus, openBody := callBrowserToolDirect(gateway, gatewayToken, "open_page", map[string]any{"url": pageURL}, "", "")
	require.Equal(t, http.StatusOK, openStatus)
	pageID := readPageID(openBody)
	clickStatus, clickBody := callBrowserToolDirect(gateway, gatewayToken, "click", map[string]any{"page_id": pageID, "selector": "#advance"}, "", "")
	require.Equal(t, http.StatusBadRequest, clickStatus)
	require.Contains(t, clickBody, "requires confirmation")
	clickStatus, _ = callBrowserToolDirect(gateway, gatewayToken, "click", map[string]any{"page_id": pageID, "selector": "#advance"}, "confirm-click-1", "approve")
	require.Equal(t, http.StatusOK, clickStatus)
	typeStatus, _ := callBrowserToolDirect(gateway, gatewayToken, "type", map[string]any{"page_id": pageID, "selector": "#editor", "text": "approved text"}, "confirm-type-1", "approve")
	require.Equal(t, http.StatusOK, typeStatus)
	readStatus, readBody := callBrowserToolDirect(gateway, gatewayToken, "read_page", map[string]any{"page_id": pageID}, "", "")
	require.Equal(t, http.StatusOK, readStatus)
	require.Contains(t, readBody, "Clicked once")
	require.Contains(t, readBody, "approved text")
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
		{ToolName: "click", Allowed: true, RiskLevel: "medium", RequiresConfirmation: true, AuditEnabled: true},
		{ToolName: "type", Allowed: true, RiskLevel: "high", RequiresConfirmation: true, AuditEnabled: true},
		{ToolName: "select", Allowed: true, RiskLevel: "medium", RequiresConfirmation: true, AuditEnabled: true},
		{ToolName: "submit", Allowed: true, RiskLevel: "high", RequiresConfirmation: true, AuditEnabled: true},
		{ToolName: "upload", Allowed: true, RiskLevel: "high", RequiresConfirmation: true, AuditEnabled: true},
	}
}

func newBrowserFixtureServer() *httptest.Server {
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		_, _ = w.Write([]byte(`<html><head><title>AE Browser Bridge</title></head><body><h1>Bridge Ready</h1><p>Hello browser mcp.</p></body></html>`))
	}))
}

func newInteractiveBrowserFixtureServer() *httptest.Server {
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		_, _ = w.Write([]byte(`<html><head><title>AF Browser Interaction</title></head><body><button id="advance" onclick="document.getElementById('state').textContent='Clicked once'">Advance</button><p id="state">Idle</p><input id="editor" oninput="document.getElementById('mirror').textContent=this.value" /><p id="mirror">Empty</p></body></html>`))
	}))
}

func newRiskyBrowserFixtureServer() *httptest.Server {
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		_, _ = w.Write([]byte(`<html><head><title>AM Browser Risky Interaction</title></head><body><select id="tier" onchange="document.getElementById('tier-state').textContent=this.value"><option value="basic">basic</option><option value="advanced">advanced</option></select><p id="tier-state">basic</p><input id="upload" type="file" onchange="document.getElementById('upload-state').textContent=this.files[0] ? this.files[0].name : 'empty'" /><p id="upload-state">empty</p><button id="submit" onclick="document.getElementById('submit-state').textContent='submitted'">Submit</button><p id="submit-state">pending</p></body></html>`))
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

type browserRiskyBridgeRuntime struct {
	port       int
	targetURL  string
	uploadFile string
	ch         chan browserRiskyBridgeCapture
}

type browserRiskyBridgeCapture struct {
	openStatus   int
	selectStatus int
	uploadStatus int
	submitStatus int
	readStatus   int
	readBody     string
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

func newBrowserRiskyBridgeRuntime(t *testing.T, targetURL string, uploadFile string) *browserRiskyBridgeRuntime {
	listener := localListener(t)
	rt := &browserRiskyBridgeRuntime{
		port: listener.Addr().(*net.TCPAddr).Port, targetURL: targetURL,
		uploadFile: uploadFile, ch: make(chan browserRiskyBridgeCapture, 1),
	}
	server := serveOnListener(t, listener, http.HandlerFunc(rt.handleRun))
	t.Cleanup(func() { _ = server.Close() })
	return rt
}

func (rt *browserRiskyBridgeRuntime) handleRun(w http.ResponseWriter, r *http.Request) {
	var request contracts.RunRequest
	_ = json.NewDecoder(r.Body).Decode(&request)
	rt.ch <- browserRiskyBridgeFlow(request, rt.targetURL, rt.uploadFile)
	writeJSON(w, http.StatusOK, contracts.RuntimeRunResponse{
		Result: contracts.RunResult{RunID: request.RunID, Status: "completed", FinalStage: "Finish"},
	})
}

func (rt *browserRiskyBridgeRuntime) wait(t *testing.T) browserRiskyBridgeCapture {
	t.Helper()
	select {
	case capture := <-rt.ch:
		return capture
	case <-time.After(5 * time.Second):
		t.Fatal("browser risky bridge runtime was not called")
		return browserRiskyBridgeCapture{}
	}
}

func browserBridgeFlow(request contracts.RunRequest, targetURL string) (int, string, int, string) {
	openStatus, openBody := callBrowserTool(request, "open_page", map[string]any{"url": targetURL})
	pageID := readPageID(openBody)
	readStatus, readBody := callBrowserTool(request, "read_page", map[string]any{"page_id": pageID})
	return openStatus, openBody, readStatus, readBody
}

func callBrowserTool(request contracts.RunRequest, name string, arguments map[string]any) (int, string) {
	return callBrowserToolWithDecision(request, name, arguments, "", "")
}

func callBrowserToolWithDecision(
	request contracts.RunRequest,
	name string,
	arguments map[string]any,
	confirmationID string,
	decision string,
) (int, string) {
	body, _ := json.Marshal(map[string]any{"server_id": "browser", "name": name, "arguments": arguments})
	if confirmationID != "" {
		body, _ = json.Marshal(map[string]any{
			"server_id": "browser", "name": name, "arguments": arguments,
			"confirmation_id": confirmationID, "confirmation_decision": decision,
		})
	}
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

func callBrowserToolApproved(request contracts.RunRequest, name string, arguments map[string]any, confirmationID string) (int, string) {
	return callBrowserToolWithDecision(request, name, arguments, confirmationID, "approve")
}

func browserRiskyBridgeFlow(request contracts.RunRequest, targetURL string, uploadFile string) browserRiskyBridgeCapture {
	pageID, openStatus := openBrowserPage(request, targetURL)
	selectStatus, _ := callBrowserToolApproved(request, "select", riskySelectArgs(pageID), "bridge-select-1")
	uploadStatus, _ := callBrowserToolApproved(request, "upload", riskyUploadArgs(pageID, uploadFile), "bridge-upload-1")
	submitStatus, _ := callBrowserToolApproved(request, "submit", riskySubmitArgs(pageID), "bridge-submit-1")
	readStatus, readBody := callBrowserTool(request, "read_page", map[string]any{"page_id": pageID})
	return browserRiskyBridgeCapture{
		openStatus: openStatus, selectStatus: selectStatus, uploadStatus: uploadStatus,
		submitStatus: submitStatus, readStatus: readStatus, readBody: readBody,
	}
}

func openBrowserPage(request contracts.RunRequest, targetURL string) (string, int) {
	openStatus, openBody := callBrowserTool(request, "open_page", map[string]any{"url": targetURL})
	return readPageID(openBody), openStatus
}

func riskySelectArgs(pageID string) map[string]any {
	return map[string]any{"page_id": pageID, "selector": "#tier", "value": "advanced"}
}

func riskyUploadArgs(pageID string, uploadFile string) map[string]any {
	return map[string]any{"page_id": pageID, "selector": "#upload", "file_paths": []string{uploadFile}}
}

func riskySubmitArgs(pageID string) map[string]any {
	return map[string]any{"page_id": pageID, "selector": "#submit"}
}

func callBrowserToolDirect(
	gateway *http.Server,
	token string,
	name string,
	arguments map[string]any,
	confirmationID string,
	decision string,
) (int, string) {
	body, _ := json.Marshal(map[string]any{
		"server_id": "browser", "name": name, "arguments": arguments,
		"confirmation_id": confirmationID, "confirmation_decision": decision,
	})
	req, _ := http.NewRequest(http.MethodPost, "http://"+gateway.Addr+"/api/v1/mcp/call", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-Local-Agent-Token", token)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return 0, err.Error()
	}
	defer resp.Body.Close()
	var raw bytes.Buffer
	_, _ = raw.ReadFrom(resp.Body)
	return resp.StatusCode, raw.String()
}

func assertApprovedRiskyInteractionFlow(
	t *testing.T,
	gateway *http.Server,
	token string,
	pageURL string,
	uploadFile string,
) {
	openStatus, openBody := callBrowserToolDirect(gateway, token, "open_page", map[string]any{"url": pageURL}, "", "")
	require.Equal(t, http.StatusOK, openStatus)
	pageID := readPageID(openBody)
	selectStatus, selectBody := callBrowserToolDirect(gateway, token, "select", map[string]any{"page_id": pageID, "selector": "#tier", "value": "advanced"}, "", "")
	require.Equal(t, http.StatusBadRequest, selectStatus)
	require.Contains(t, selectBody, "requires confirmation")
	selectStatus, _ = callBrowserToolDirect(gateway, token, "select", map[string]any{"page_id": pageID, "selector": "#tier", "value": "advanced"}, "confirm-select-1", "approve")
	require.Equal(t, http.StatusOK, selectStatus)
	uploadStatus, uploadBody := callBrowserToolDirect(gateway, token, "upload", map[string]any{"page_id": pageID, "selector": "#upload", "file_paths": []string{uploadFile}}, "", "")
	require.Equal(t, http.StatusBadRequest, uploadStatus)
	require.Contains(t, uploadBody, "requires confirmation")
	uploadStatus, _ = callBrowserToolDirect(gateway, token, "upload", map[string]any{"page_id": pageID, "selector": "#upload", "file_paths": []string{uploadFile}}, "confirm-upload-1", "approve")
	require.Equal(t, http.StatusOK, uploadStatus)
	submitStatus, submitBody := callBrowserToolDirect(gateway, token, "submit", map[string]any{"page_id": pageID, "selector": "#submit"}, "", "")
	require.Equal(t, http.StatusBadRequest, submitStatus)
	require.Contains(t, submitBody, "requires confirmation")
	submitStatus, _ = callBrowserToolDirect(gateway, token, "submit", map[string]any{"page_id": pageID, "selector": "#submit"}, "confirm-submit-1", "approve")
	require.Equal(t, http.StatusOK, submitStatus)
	readStatus, readBody := callBrowserToolDirect(gateway, token, "read_page", map[string]any{"page_id": pageID}, "", "")
	require.Equal(t, http.StatusOK, readStatus)
	require.Contains(t, readBody, "advanced")
	require.Contains(t, readBody, filepath.Base(uploadFile))
	require.Contains(t, readBody, "submitted")
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
