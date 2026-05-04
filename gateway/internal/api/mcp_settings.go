package api

import (
	"errors"
	"fmt"
	"hash/fnv"
	"net/url"
	"strings"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/mcp"
)

func applyMCPSettings(repoRoot string, payload settingsUpdatePayload, mgr *mcp.Manager) error {
	if !hasMCPSettingsPatch(payload) {
		return nil
	}
	cfg, err := config.LoadFile(repoRoot)
	if err != nil {
		return err
	}
	servers, err := nextMCPServers(cfg.MCP.Servers, payload)
	if err != nil {
		return err
	}
	cfg.MCP.Servers = servers
	if err := config.SaveFile(repoRoot, cfg); err != nil {
		return err
	}
	if mgr != nil {
		mgr.ReplaceServers(servers)
	}
	return nil
}

func nextMCPServers(servers []config.MCPServerConfig, payload settingsUpdatePayload) ([]config.MCPServerConfig, error) {
	if payload.RemoveMCPID != "" {
		return removeMCPServer(servers, payload.RemoveMCPID)
	}
	if payload.MCPPolicyServerID != "" || payload.MCPPolicyToolName != "" {
		return updateMCPToolPolicy(servers, payload)
	}
	return addMCPServer(servers, payload.AddMCPName, payload.AddMCPURL)
}

func hasMCPSettingsPatch(payload settingsUpdatePayload) bool {
	return payload.AddMCPURL != "" || payload.RemoveMCPID != "" || payload.MCPPolicyServerID != "" || payload.MCPPolicyToolName != ""
}

func addMCPServer(servers []config.MCPServerConfig, name string, rawURL string) ([]config.MCPServerConfig, error) {
	name = strings.TrimSpace(name)
	rawURL = strings.TrimSpace(rawURL)
	if err := validateMCPURL(rawURL); err != nil {
		return nil, err
	}
	if hasMCPURL(servers, rawURL) {
		return nil, errors.New("mcp server url already exists")
	}
	if name == "" {
		name = rawURL
	}
	next := append(copyMCPServers(servers), config.MCPServerConfig{
		ID: mcpServerID(name, rawURL, servers), Name: name,
		Type: "http", URL: rawURL, Enabled: true,
	})
	return next, nil
}

func updateMCPToolPolicy(servers []config.MCPServerConfig, payload settingsUpdatePayload) ([]config.MCPServerConfig, error) {
	serverID := strings.TrimSpace(payload.MCPPolicyServerID)
	toolName := strings.TrimSpace(payload.MCPPolicyToolName)
	if serverID == "" || toolName == "" {
		return nil, errors.New("mcp policy server_id and tool_name are required")
	}
	next := copyMCPServers(servers)
	for i := range next {
		if next[i].ID == serverID {
			next[i].ToolPolicies = upsertMCPToolPolicy(next[i].ToolPolicies, toolName, payload)
			return next, nil
		}
	}
	return nil, fmt.Errorf("mcp server %q not found", serverID)
}

func upsertMCPToolPolicy(items []config.MCPToolPolicy, toolName string, payload settingsUpdatePayload) []config.MCPToolPolicy {
	next := append([]config.MCPToolPolicy(nil), items...)
	for i := range next {
		if next[i].ToolName == toolName {
			next[i] = nextMCPToolPolicy(next[i], payload)
			return next
		}
	}
	return append(next, nextMCPToolPolicy(config.MCPToolPolicy{ToolName: toolName}, payload))
}

func nextMCPToolPolicy(current config.MCPToolPolicy, payload settingsUpdatePayload) config.MCPToolPolicy {
	if payload.MCPPolicyAllowed != nil {
		current.Allowed = *payload.MCPPolicyAllowed
	}
	if payload.MCPPolicyConfirm != nil {
		current.RequiresConfirmation = *payload.MCPPolicyConfirm
	}
	if strings.TrimSpace(payload.MCPPolicyRiskLevel) != "" {
		current.RiskLevel = mcp.NormalizeRiskLevel(payload.MCPPolicyRiskLevel)
	}
	if current.RiskLevel == "" {
		current.RiskLevel = mcp.DefaultRiskLevel
	}
	current.AuditEnabled = true
	return current
}

func removeMCPServer(servers []config.MCPServerConfig, id string) ([]config.MCPServerConfig, error) {
	id = strings.TrimSpace(id)
	next := make([]config.MCPServerConfig, 0, len(servers))
	for _, server := range servers {
		if server.ID != id {
			next = append(next, server)
		}
	}
	if len(next) == len(servers) {
		return nil, fmt.Errorf("mcp server %q not found", id)
	}
	return next, nil
}

func validateMCPURL(rawURL string) error {
	parsed, err := url.Parse(rawURL)
	if err != nil || parsed.Host == "" {
		return errors.New("mcp url is invalid")
	}
	if parsed.Scheme != "http" && parsed.Scheme != "https" {
		return errors.New("mcp url must use http or https")
	}
	return nil
}

func hasMCPURL(servers []config.MCPServerConfig, rawURL string) bool {
	for _, server := range servers {
		if strings.EqualFold(server.URL, rawURL) {
			return true
		}
	}
	return false
}

func mcpServerID(name string, rawURL string, servers []config.MCPServerConfig) string {
	base := sanitizeMCPID(name)
	if base == "" {
		base = fmt.Sprintf("mcp-%x", hashString(rawURL))
	}
	id := base
	for i := 2; hasMCPID(servers, id); i++ {
		id = fmt.Sprintf("%s-%d", base, i)
	}
	return id
}

func sanitizeMCPID(value string) string {
	var b strings.Builder
	for _, r := range strings.ToLower(value) {
		if r >= 'a' && r <= 'z' || r >= '0' && r <= '9' {
			b.WriteRune(r)
			continue
		}
		if b.Len() > 0 && b.String()[b.Len()-1] != '-' {
			b.WriteByte('-')
		}
	}
	return strings.Trim(b.String(), "-")
}

func hasMCPID(servers []config.MCPServerConfig, id string) bool {
	for _, server := range servers {
		if server.ID == id {
			return true
		}
	}
	return false
}

func hashString(value string) uint32 {
	h := fnv.New32a()
	_, _ = h.Write([]byte(value))
	return h.Sum32()
}

func copyMCPServers(servers []config.MCPServerConfig) []config.MCPServerConfig {
	out := make([]config.MCPServerConfig, len(servers))
	copy(out, servers)
	return out
}
