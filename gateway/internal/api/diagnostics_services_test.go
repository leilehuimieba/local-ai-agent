package api

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestBuildServiceStatusesCoversDoctorSurface(t *testing.T) {
	root := t.TempDir()
	require.NoError(t, os.MkdirAll(filepath.Join(root, "logs"), 0o755))
	require.NoError(t, os.MkdirAll(filepath.Join(root, "data", "knowledge_base"), 0o755))
	settings := SettingsResponse{
		Ports:         map[string]int{"gateway": 38603},
		RuntimeStatus: RuntimeStatus{OK: true, Name: "runtime-host", Version: "test"},
		Diagnostics:   DiagnosticsStatus{ProviderCount: 1, ModelCount: 1, KnowledgeBasePathExists: true},
		MCP:           MCPInfo{Servers: []MCPServerStatus{{ID: "s1", Enabled: true, Ready: true}}},
	}

	statuses := buildServiceStatuses(root, settings)

	require.Len(t, statuses, 8)
	require.Equal(t, "gateway", statuses[0].ID)
	require.Equal(t, "ok", findServiceStatus(statuses, "runtime").Status)
	require.Equal(t, "ok", findServiceStatus(statuses, "sessions").Status)
	require.Equal(t, "ok", findServiceStatus(statuses, "knowledge").Status)
}

func findServiceStatus(items []ServiceStatus, id string) ServiceStatus {
	for _, item := range items {
		if item.ID == id {
			return item
		}
	}
	return ServiceStatus{}
}
