package api

import (
	"testing"

	"local-agent/gateway/internal/config"

	"github.com/stretchr/testify/require"
)

func TestNextMCPServersAddsHTTPServer(t *testing.T) {
	servers, err := nextMCPServers(nil, settingsUpdatePayload{
		AddMCPName: "Local Tools",
		AddMCPURL:  "http://127.0.0.1:3333/mcp",
	})
	require.NoError(t, err)
	require.Len(t, servers, 1)
	require.Equal(t, "local-tools", servers[0].ID)
	require.Equal(t, "Local Tools", servers[0].Name)
	require.Equal(t, "http", servers[0].Type)
	require.True(t, servers[0].Enabled)
}

func TestNextMCPServersRejectsDuplicateURL(t *testing.T) {
	existing := []config.MCPServerConfig{{ID: "a", URL: "http://127.0.0.1:3333/mcp"}}
	_, err := nextMCPServers(existing, settingsUpdatePayload{
		AddMCPName: "A",
		AddMCPURL:  "http://127.0.0.1:3333/mcp",
	})
	require.EqualError(t, err, "mcp server url already exists")
}

func TestApplyMCPSettingsPersistsAddAndRemove(t *testing.T) {
	repoRoot := t.TempDir()
	require.NoError(t, config.SaveFile(repoRoot, config.AppConfig{}))
	require.NoError(t, applyMCPSettings(repoRoot, settingsUpdatePayload{
		AddMCPName: "Docs MCP",
		AddMCPURL:  "http://127.0.0.1:3456/mcp",
	}, nil))
	cfg, err := config.LoadFile(repoRoot)
	require.NoError(t, err)
	require.Len(t, cfg.MCP.Servers, 1)
	require.NoError(t, applyMCPSettings(repoRoot, settingsUpdatePayload{
		RemoveMCPID: cfg.MCP.Servers[0].ID,
	}, nil))
	cfg, err = config.LoadFile(repoRoot)
	require.NoError(t, err)
	require.Empty(t, cfg.MCP.Servers)
}

func TestNextMCPServersUpdatesToolPolicy(t *testing.T) {
	allowed := true
	confirm := false
	servers := []config.MCPServerConfig{{ID: "docs", URL: "http://127.0.0.1:3456/mcp"}}
	next, err := nextMCPServers(servers, settingsUpdatePayload{
		MCPPolicyServerID: "docs", MCPPolicyToolName: "search",
		MCPPolicyAllowed: &allowed, MCPPolicyRiskLevel: "low",
		MCPPolicyConfirm: &confirm,
	})
	require.NoError(t, err)
	require.Len(t, next[0].ToolPolicies, 1)
	require.Equal(t, "search", next[0].ToolPolicies[0].ToolName)
	require.True(t, next[0].ToolPolicies[0].Allowed)
	require.Equal(t, "low", next[0].ToolPolicies[0].RiskLevel)
	require.False(t, next[0].ToolPolicies[0].RequiresConfirmation)
	require.True(t, next[0].ToolPolicies[0].AuditEnabled)
}
