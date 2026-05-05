package api

import (
	"encoding/json"
	"net"
	"net/http"
	"net/http/httptest"
	"testing"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/state"

	"github.com/stretchr/testify/require"
)

func TestHandleCapabilitiesUsesRequestScopedRuntimeCatalog(t *testing.T) {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodGet, "/api/v1/capabilities?mode=full_access", nil)
	deps := catalogRouteDeps{runtimeClient: runtimeCatalogClient(t), state: runtimeCatalogState(t)}
	deps.handleCapabilities(recorder, req)
	require.Equal(t, http.StatusOK, recorder.Code)
	require.Contains(t, recorder.Body.String(), `"capability_id":"mcp__docs__search"`)
}

func runtimeCatalogClient(t *testing.T) *runtimeclient.Client {
	t.Helper()
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	require.NoError(t, err)
	server := &http.Server{Handler: http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		require.Equal(t, http.MethodPost, r.Method)
		require.Equal(t, "/v1/runtime/capabilities/query", r.URL.Path)
		var request contracts.RunRequest
		require.NoError(t, json.NewDecoder(r.Body).Decode(&request))
		require.Equal(t, "full_access", request.Mode)
		writeJSON(w, http.StatusOK, contracts.CapabilityListResponse{
			Items: []contracts.CapabilitySpec{{CapabilityID: "mcp__docs__search"}},
		})
	})}
	go func() { _ = server.Serve(listener) }()
	t.Cleanup(func() { _ = server.Close() })
	port := listener.Addr().(*net.TCPAddr).Port
	return runtimeclient.NewClient(port)
}

func runtimeCatalogState(t *testing.T) *state.SettingsStore {
	t.Helper()
	cfg := config.AppConfig{
		DefaultMode: "standard",
		DefaultModel: contracts.ModelRef{
			ProviderID: "p1", ModelID: "m1", DisplayName: "Model", Enabled: true, Available: true,
		},
		DefaultWorkspace: contracts.WorkspaceRef{
			WorkspaceID: "w1", Name: "repo", RootPath: "D:/repo", IsActive: true,
		},
		AvailableModels: []contracts.ModelRef{{
			ProviderID: "p1", ModelID: "m1", DisplayName: "Model", Enabled: true, Available: true,
		}},
		Workspaces: []contracts.WorkspaceRef{{
			WorkspaceID: "w1", Name: "repo", RootPath: "D:/repo", IsActive: true,
		}},
	}
	return state.NewSettingsStore(t.TempDir(), cfg)
}
