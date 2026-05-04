package mcp

import "local-agent/gateway/internal/config"

const DefaultRiskLevel = "medium"

type ToolPolicy struct {
	Allowed              bool   `json:"allowed"`
	RiskLevel            string `json:"risk_level"`
	RequiresConfirmation bool   `json:"requires_confirmation"`
	AuditEnabled         bool   `json:"audit_enabled"`
	PolicySource         string `json:"policy_source"`
}

func ResolveToolPolicy(server config.MCPServerConfig, toolName string) ToolPolicy {
	for _, item := range server.ToolPolicies {
		if item.ToolName == toolName {
			return policyFromConfig(item)
		}
	}
	return defaultToolPolicy()
}

func policyFromConfig(item config.MCPToolPolicy) ToolPolicy {
	risk := NormalizeRiskLevel(item.RiskLevel)
	return ToolPolicy{
		Allowed:              item.Allowed,
		RiskLevel:            risk,
		RequiresConfirmation: item.RequiresConfirmation,
		AuditEnabled:         item.AuditEnabled,
		PolicySource:         "config",
	}
}

func defaultToolPolicy() ToolPolicy {
	return ToolPolicy{
		Allowed:              false,
		RiskLevel:            DefaultRiskLevel,
		RequiresConfirmation: true,
		AuditEnabled:         true,
		PolicySource:         "default_deny",
	}
}

func NormalizeRiskLevel(value string) string {
	switch value {
	case "low", "medium", "high", "critical":
		return value
	default:
		return DefaultRiskLevel
	}
}
