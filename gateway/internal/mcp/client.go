package mcp

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"sync"
	"time"
)

// MCPClient is the interface for MCP transport clients (HTTP or stdio).
type MCPClient interface {
	Connect() error
	Call(name string, arguments map[string]any) (json.RawMessage, error)
	Ready() bool
	Tools() []Tool
}

// Client is an MCP HTTP client.
type Client struct {
	ID         string
	Name       string
	baseURL    string
	httpClient *http.Client
	mu         sync.RWMutex
	tools      []Tool
	ready      bool
}

// Tool represents an MCP tool.
type Tool struct {
	Name                 string          `json:"name"`
	Description          string          `json:"description"`
	InputSchema          json.RawMessage `json:"inputSchema"`
	ServerID             string          `json:"server_id,omitempty"`
	Allowed              bool            `json:"allowed"`
	RiskLevel            string          `json:"risk_level"`
	RequiresConfirmation bool            `json:"requires_confirmation"`
	AuditEnabled         bool            `json:"audit_enabled"`
	PolicySource         string          `json:"policy_source"`
}

// NewClient creates a new MCP client.
func NewClient(id, name, addr string) *Client {
	if addr == "" {
		addr = "http://127.0.0.1:3344/mcp"
	}
	return &Client{
		ID:         id,
		Name:       name,
		baseURL:    addr,
		httpClient: &http.Client{Timeout: 30 * time.Second},
	}
}

// Connect initializes the MCP session and fetches the tool list.
func (c *Client) Connect() error {
	if err := c.initialize(); err != nil {
		return fmt.Errorf("mcp initialize: %w", err)
	}
	tools, err := c.listTools()
	if err != nil {
		return fmt.Errorf("mcp list tools: %w", err)
	}
	c.mu.Lock()
	c.tools = tools
	c.ready = true
	c.mu.Unlock()
	return nil
}

// Ready reports whether the client has successfully connected.
func (c *Client) Ready() bool {
	c.mu.RLock()
	defer c.mu.RUnlock()
	return c.ready
}

// Tools returns the cached tool list.
func (c *Client) Tools() []Tool {
	c.mu.RLock()
	defer c.mu.RUnlock()
	out := make([]Tool, len(c.tools))
	copy(out, c.tools)
	return out
}

// Call invokes an MCP tool by name with the given arguments.
func (c *Client) Call(name string, arguments map[string]any) (json.RawMessage, error) {
	reqBody := rpcRequest{
		JSONRPC: "2.0",
		ID:      time.Now().UnixNano(),
		Method:  "tools/call",
		Params: map[string]any{
			"name":      name,
			"arguments": arguments,
		},
	}
	resp, err := c.post(reqBody)
	if err != nil {
		return nil, err
	}
	if resp.Error != nil {
		return nil, fmt.Errorf("mcp tool error %d: %s", resp.Error.Code, resp.Error.Message)
	}
	return json.Marshal(resp.Result)
}

func (c *Client) initialize() error {
	reqBody := rpcRequest{
		JSONRPC: "2.0",
		ID:      1,
		Method:  "initialize",
		Params: map[string]any{
			"protocolVersion": "2024-11-05",
			"capabilities":    map[string]any{},
			"clientInfo": map[string]string{
				"name":    "local-agent",
				"version": "1.0.0",
			},
		},
	}
	_, err := c.post(reqBody)
	return err
}

func (c *Client) listTools() ([]Tool, error) {
	reqBody := rpcRequest{
		JSONRPC: "2.0",
		ID:      2,
		Method:  "tools/list",
	}
	resp, err := c.post(reqBody)
	if err != nil {
		return nil, err
	}
	var result struct {
		Tools []Tool `json:"tools"`
	}
	if err := json.Unmarshal(resp.Result, &result); err != nil {
		return nil, err
	}
	return result.Tools, nil
}

func (c *Client) post(reqBody rpcRequest) (*rpcResponse, error) {
	data, err := json.Marshal(reqBody)
	if err != nil {
		return nil, err
	}
	req, err := http.NewRequest("POST", c.baseURL, bytes.NewReader(data))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Accept", "application/json, text/event-stream")

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, err
	}
	defer func() { _ = resp.Body.Close() }()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("mcp http %d", resp.StatusCode)
	}

	var rpcResp rpcResponse
	if err := json.NewDecoder(resp.Body).Decode(&rpcResp); err != nil {
		return nil, err
	}
	return &rpcResp, nil
}

type rpcRequest struct {
	JSONRPC string `json:"jsonrpc"`
	ID      int64  `json:"id"`
	Method  string `json:"method"`
	Params  any    `json:"params,omitempty"`
}

type rpcResponse struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      int64           `json:"id"`
	Result  json.RawMessage `json:"result,omitempty"`
	Error   *rpcError       `json:"error,omitempty"`
}

type rpcError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}
