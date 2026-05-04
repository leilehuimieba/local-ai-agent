package mcp

import (
	"encoding/json"
	"fmt"
	"sync"

	"local-agent/gateway/internal/config"
)

// Manager manages multiple MCP clients.
type Manager struct {
	clients map[string]*Client
	servers []config.MCPServerConfig
	mu      sync.RWMutex
}

// ServerStatus holds the status of a single MCP server.
type ServerStatus struct {
	ID               string `json:"id"`
	Name             string `json:"name"`
	Type             string `json:"type"`
	URL              string `json:"url"`
	Enabled          bool   `json:"enabled"`
	Ready            bool   `json:"ready"`
	ToolCount        int    `json:"tool_count"`
	AllowedToolCount int    `json:"allowed_tool_count"`
	BlockedToolCount int    `json:"blocked_tool_count"`
	RequiresPolicy   bool   `json:"requires_policy"`
	Error            string `json:"error,omitempty"`
}

// NewManager creates a Manager from server configurations.
func NewManager(servers []config.MCPServerConfig) *Manager {
	m := &Manager{clients: configuredClients(servers), servers: copyServers(servers)}
	return m
}

func configuredClients(servers []config.MCPServerConfig) map[string]*Client {
	clients := make(map[string]*Client)
	for _, s := range servers {
		if !s.Enabled || s.URL == "" {
			continue
		}
		clients[s.ID] = NewClient(s.ID, s.Name, s.URL)
	}
	return clients
}

func copyServers(servers []config.MCPServerConfig) []config.MCPServerConfig {
	out := make([]config.MCPServerConfig, len(servers))
	copy(out, servers)
	return out
}

// ReplaceServers replaces the configured servers and reconnects ready clients.
func (m *Manager) ReplaceServers(servers []config.MCPServerConfig) {
	m.mu.Lock()
	m.servers = copyServers(servers)
	m.clients = configuredClients(servers)
	m.mu.Unlock()
	m.ConnectAll()
}

// ConnectAll attempts to connect all managed clients concurrently.
func (m *Manager) ConnectAll() {
	m.mu.RLock()
	clients := make([]*Client, 0, len(m.clients))
	for _, c := range m.clients {
		clients = append(clients, c)
	}
	m.mu.RUnlock()

	var wg sync.WaitGroup
	for _, c := range clients {
		wg.Add(1)
		go func(client *Client) {
			defer wg.Done()
			_ = client.Connect()
		}(c)
	}
	wg.Wait()
}

// Status returns the current status of all configured servers.
func (m *Manager) Status() []ServerStatus {
	m.mu.RLock()
	defer m.mu.RUnlock()

	out := make([]ServerStatus, 0, len(m.servers))
	for _, s := range m.servers {
		st := ServerStatus{
			ID:      s.ID,
			Name:    s.Name,
			Type:    s.Type,
			URL:     s.URL,
			Enabled: s.Enabled,
		}
		if c, ok := m.clients[s.ID]; ok {
			tools := m.toolsForClient(s, c)
			st.Ready = c.Ready()
			st.ToolCount = len(tools)
			st.AllowedToolCount = allowedToolCount(tools)
			st.BlockedToolCount = st.ToolCount - st.AllowedToolCount
			st.RequiresPolicy = st.BlockedToolCount > 0
		}
		out = append(out, st)
	}
	return out
}

// AllTools returns all tools from all ready clients, tagged with server_id.
func (m *Manager) AllTools() []Tool {
	m.mu.RLock()
	defer m.mu.RUnlock()

	var out []Tool
	for _, server := range m.servers {
		if c, ok := m.clients[server.ID]; ok {
			out = append(out, m.toolsForClient(server, c)...)
		}
	}
	return out
}

func (m *Manager) AllowedTools() []Tool {
	tools := m.AllTools()
	out := make([]Tool, 0, len(tools))
	for _, tool := range tools {
		if tool.Allowed {
			out = append(out, tool)
		}
	}
	return out
}

func (m *Manager) toolsForClient(server config.MCPServerConfig, c *Client) []Tool {
	raw := c.Tools()
	out := make([]Tool, 0, len(raw))
	for _, tool := range raw {
		out = append(out, withPolicy(tool, server, c.ID))
	}
	return out
}

func withPolicy(tool Tool, server config.MCPServerConfig, serverID string) Tool {
	policy := ResolveToolPolicy(server, tool.Name)
	tool.ServerID = serverID
	tool.Allowed = policy.Allowed
	tool.RiskLevel = policy.RiskLevel
	tool.RequiresConfirmation = policy.RequiresConfirmation
	tool.AuditEnabled = policy.AuditEnabled
	tool.PolicySource = policy.PolicySource
	return tool
}

func allowedToolCount(tools []Tool) int {
	count := 0
	for _, tool := range tools {
		if tool.Allowed {
			count++
		}
	}
	return count
}

func (m *Manager) ToolPolicy(serverID string, name string) (ToolPolicy, error) {
	m.mu.RLock()
	server, ok := m.serverConfig(serverID)
	m.mu.RUnlock()
	if !ok {
		return ToolPolicy{}, fmt.Errorf("mcp server %q config not found", serverID)
	}
	return ResolveToolPolicy(server, name), nil
}

// Call invokes a tool on the specified server.
func (m *Manager) Call(serverID string, name string, arguments map[string]any) (json.RawMessage, error) {
	m.mu.RLock()
	c, ok := m.clients[serverID]
	server, serverOK := m.serverConfig(serverID)
	m.mu.RUnlock()
	if !ok {
		return nil, fmt.Errorf("mcp server %q not found", serverID)
	}
	if !serverOK {
		return nil, fmt.Errorf("mcp server %q config not found", serverID)
	}
	policy := ResolveToolPolicy(server, name)
	if err := validateCallPolicy(name, policy); err != nil {
		return nil, err
	}
	return c.Call(name, arguments)
}

func (m *Manager) serverConfig(serverID string) (config.MCPServerConfig, bool) {
	for _, server := range m.servers {
		if server.ID == serverID {
			return server, true
		}
	}
	return config.MCPServerConfig{}, false
}

func validateCallPolicy(name string, policy ToolPolicy) error {
	if !policy.Allowed {
		return fmt.Errorf("mcp tool %q is not allowlisted", name)
	}
	if policy.RequiresConfirmation {
		return fmt.Errorf("mcp tool %q requires confirmation", name)
	}
	return nil
}

// GetClient returns a client by server ID.
func (m *Manager) GetClient(serverID string) *Client {
	m.mu.RLock()
	defer m.mu.RUnlock()
	return m.clients[serverID]
}
