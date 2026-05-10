package api

import (
	"bytes"
	"encoding/json"
	"net"
	"net/http"
	"testing"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/mcp"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/session"
	"local-agent/gateway/internal/state"
	"local-agent/gateway/internal/token"

	"github.com/stretchr/testify/require"
)

func TestMCPRuntimeBridgeE2EUsesGatewayPolicyAndAudit(t *testing.T) {
	repoRoot := t.TempDir()
	mcpServer := newFakeMCPServer(t)
	defer mcpServer.Close()
	mgr := newConnectedMCPManager(mcpServer.URL, e2eMCPPolicy())
	rt := newBridgeRuntime(t)
	cfg := sampleAppConfig()
	cfg.Providers[0].APIKey = "test-key"
	gateway, gatewayToken := startBridgeGateway(t, repoRoot, cfg, rt.port, mgr)
	defer gateway.Close()

	resp := postBridgeChatRun(t, gateway, gatewayToken)
	require.Equal(t, http.StatusAccepted, resp.StatusCode)
	capture := rt.wait(t)
	require.Contains(t, capture.request.ContextHints["mcp_tool_specs_json"], "mcp__docs__search")
	require.Equal(t, http.StatusOK, capture.bridgeStatus)
	require.JSONEq(t, `{"ok":true}`, capture.bridgeBody)
	require.FileExists(t, repoRoot+"/logs/mcp-audit.jsonl")
}

func TestCapabilitiesAPIIncludesRequestScopedMCPCapability(t *testing.T) {
	repoRoot := t.TempDir()
	mcpServer := newFakeMCPServer(t)
	defer mcpServer.Close()
	mgr := newConnectedMCPManager(mcpServer.URL, e2eMCPPolicy())
	rt := newCatalogRuntime(t)
	cfg := sampleAppConfig()
	gateway, gatewayToken := startBridgeGateway(t, repoRoot, cfg, rt.port, mgr)
	defer gateway.Close()

	resp := getCapabilities(t, gateway, gatewayToken)
	require.Equal(t, http.StatusOK, resp.StatusCode)
	capture := rt.wait(t)
	require.Contains(t, capture.request.ContextHints["mcp_tool_specs_json"], "mcp__docs__search")
	require.Contains(t, capture.capabilityBody, `"capability_id":"mcp__docs__search"`)
}

func e2eMCPPolicy() []config.MCPToolPolicy {
	return []config.MCPToolPolicy{{
		ToolName: "search", Allowed: true, RiskLevel: "low",
		RequiresConfirmation: false, AuditEnabled: true,
	}}
}

func startBridgeGateway(
	t *testing.T,
	repoRoot string,
	cfg config.AppConfig,
	runtimePort int,
	mgr *mcp.Manager,
) (*http.Server, string) {
	t.Helper()
	listener := localListener(t)
	cfg.GatewayPort = listener.Addr().(*net.TCPAddr).Port
	cfg.RuntimePort = runtimePort
	tok, err := token.LoadOrCreate(repoRoot)
	require.NoError(t, err)
	router := bridgeRouter(repoRoot, cfg, tok, mgr)
	return serveOnListener(t, listener, router), tok.Value()
}

func bridgeRouter(repoRoot string, cfg config.AppConfig, tok *token.Manager, mgr *mcp.Manager) http.Handler {
	return NewRouter(
		repoRoot, cfg, runtimeclient.NewClient(cfg.RuntimePort), session.NewEventBus(repoRoot),
		state.NewSettingsStore(repoRoot, cfg), state.NewConfirmationStore(),
		state.NewProviderCredentialStore(repoRoot), state.NewRuntimeProviderStore(repoRoot), tok, mgr,
	)
}

func postBridgeChatRun(t *testing.T, server *http.Server, tok string) *http.Response {
	t.Helper()
	body := bytes.NewBufferString(`{"session_id":"s1","user_input":"用 MCP 搜索 rust","mode":"standard"}`)
	req, err := http.NewRequest(http.MethodPost, "http://"+server.Addr+"/api/v1/chat/run", body)
	require.NoError(t, err)
	req.Header.Set("X-Local-Agent-Token", tok)
	resp, err := http.DefaultClient.Do(req)
	require.NoError(t, err)
	t.Cleanup(func() { _ = resp.Body.Close() })
	return resp
}

func getCapabilities(t *testing.T, server *http.Server, tok string) *http.Response {
	t.Helper()
	req, err := http.NewRequest(http.MethodGet, "http://"+server.Addr+"/api/v1/capabilities?mode=standard", nil)
	require.NoError(t, err)
	req.Header.Set("X-Local-Agent-Token", tok)
	resp, err := http.DefaultClient.Do(req)
	require.NoError(t, err)
	t.Cleanup(func() { _ = resp.Body.Close() })
	return resp
}

type bridgeRuntime struct {
	port int
	ch   chan bridgeCapture
}

type bridgeCapture struct {
	request        contracts.RunRequest
	bridgeStatus   int
	bridgeBody     string
	capabilityBody string
}

func newBridgeRuntime(t *testing.T) *bridgeRuntime {
	t.Helper()
	listener := localListener(t)
	rt := &bridgeRuntime{port: listener.Addr().(*net.TCPAddr).Port, ch: make(chan bridgeCapture, 1)}
	server := serveOnListener(t, listener, http.HandlerFunc(rt.handleRun))
	t.Cleanup(func() { _ = server.Close() })
	return rt
}

func newCatalogRuntime(t *testing.T) *bridgeRuntime {
	t.Helper()
	listener := localListener(t)
	rt := &bridgeRuntime{port: listener.Addr().(*net.TCPAddr).Port, ch: make(chan bridgeCapture, 1)}
	server := serveOnListener(t, listener, http.HandlerFunc(rt.handleCapabilities))
	t.Cleanup(func() { _ = server.Close() })
	return rt
}

func (rt *bridgeRuntime) handleRun(w http.ResponseWriter, r *http.Request) {
	var request contracts.RunRequest
	_ = json.NewDecoder(r.Body).Decode(&request)
	status, body := callBridgeFromRuntime(request)
	rt.ch <- bridgeCapture{request: request, bridgeStatus: status, bridgeBody: body}
	writeJSON(w, http.StatusOK, contracts.RuntimeRunResponse{
		Result: contracts.RunResult{RunID: request.RunID, Status: "completed", FinalStage: "Finish"},
	})
}

func (rt *bridgeRuntime) handleCapabilities(w http.ResponseWriter, r *http.Request) {
	if r.URL.Path != "/v1/runtime/capabilities/query" {
		http.NotFound(w, r)
		return
	}
	var request contracts.RunRequest
	_ = json.NewDecoder(r.Body).Decode(&request)
	body := `{"items":[{"capability_id":"mcp__docs__search","display_name":"MCP: docs/search","domain":"mcp","risk_level":"low","input_schema":"{}","output_kind":"json_preview","side_effect_level":"local_side_effect","supports_modes":["observe","standard","full_access"],"verification_policy":"check_result_summary","connector_slot":"mcp_gateway","source_kind":"connector_backed","requires_confirmation":false}]}`
	rt.ch <- bridgeCapture{request: request, capabilityBody: body}
	w.Header().Set("Content-Type", "application/json")
	_, _ = w.Write([]byte(body))
}

func callBridgeFromRuntime(request contracts.RunRequest) (int, string) {
	payload := bytes.NewBufferString(`{"server_id":"docs","name":"search","arguments":{"q":"rust"}}`)
	httpReq, _ := http.NewRequest(http.MethodPost, request.ContextHints["mcp_gateway_url"], payload)
	httpReq.Header.Set("Content-Type", "application/json")
	httpReq.Header.Set("X-Local-Agent-Token", request.ContextHints["mcp_gateway_token"])
	resp, err := http.DefaultClient.Do(httpReq)
	if err != nil {
		return 0, err.Error()
	}
	defer resp.Body.Close()
	var body bytes.Buffer
	_, _ = body.ReadFrom(resp.Body)
	return resp.StatusCode, body.String()
}

func (rt *bridgeRuntime) wait(t *testing.T) bridgeCapture {
	t.Helper()
	select {
	case capture := <-rt.ch:
		return capture
	case <-time.After(5 * time.Second):
		t.Fatal("runtime bridge was not called")
		return bridgeCapture{}
	}
}

func localListener(t *testing.T) net.Listener {
	t.Helper()
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	require.NoError(t, err)
	return listener
}

func serveOnListener(t *testing.T, listener net.Listener, handler http.Handler) *http.Server {
	t.Helper()
	server := &http.Server{Addr: listener.Addr().String(), Handler: handler}
	go func() { _ = server.Serve(listener) }()
	return server
}
