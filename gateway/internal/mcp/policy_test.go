package mcp

import (
	"testing"

	"local-agent/gateway/internal/config"

	"github.com/stretchr/testify/require"
)

func TestResolveToolPolicyDefaultsToDenyAndAudit(t *testing.T) {
	policy := ResolveToolPolicy(config.MCPServerConfig{}, "search")
	require.False(t, policy.Allowed)
	require.Equal(t, "medium", policy.RiskLevel)
	require.True(t, policy.RequiresConfirmation)
	require.True(t, policy.AuditEnabled)
	require.Equal(t, "default_deny", policy.PolicySource)
}

func TestResolveToolPolicyUsesConfiguredAllowlist(t *testing.T) {
	server := config.MCPServerConfig{ToolPolicies: []config.MCPToolPolicy{{
		ToolName: "search", Allowed: true, RiskLevel: "low",
		RequiresConfirmation: false, AuditEnabled: true,
	}}}
	policy := ResolveToolPolicy(server, "search")
	require.True(t, policy.Allowed)
	require.Equal(t, "low", policy.RiskLevel)
	require.False(t, policy.RequiresConfirmation)
	require.Equal(t, "config", policy.PolicySource)
}
