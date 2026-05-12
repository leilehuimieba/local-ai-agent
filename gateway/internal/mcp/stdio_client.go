package mcp

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"sync"
	"time"
)

// StdioClient is an MCP client that communicates via stdin/stdout with a child process.
type StdioClient struct {
	ID      string
	Name    string
	Command string
	Args    []string
	URL     string // optional health-check endpoint exposed by the child process

	cmd    *exec.Cmd
	stdin  io.WriteCloser
	stdout io.ReadCloser
	mu     sync.Mutex // serializes writes to stdin and reads from stdout
	tools  []Tool
	ready  bool
	cancel context.CancelFunc
}

// NewStdioClient creates a new StdioClient. Connect() must be called to spawn the process.
func NewStdioClient(id, name, command string, args []string, url string) *StdioClient {
	return &StdioClient{
		ID:      id,
		Name:    name,
		Command: command,
		Args:    args,
		URL:     url,
	}
}

// Connect spawns the child process, initializes the MCP session, and fetches tools.
func (c *StdioClient) Connect() error {
	ctx, cancel := context.WithCancel(context.Background())
	c.cancel = cancel

	c.cmd = exec.CommandContext(ctx, c.Command, c.Args...)
	c.cmd.Stderr = os.Stderr

	var err error
	c.stdin, err = c.cmd.StdinPipe()
	if err != nil {
		cancel()
		return fmt.Errorf("stdio mcp stdin pipe: %w", err)
	}
	c.stdout, err = c.cmd.StdoutPipe()
	if err != nil {
		cancel()
		return fmt.Errorf("stdio mcp stdout pipe: %w", err)
	}

	if err := c.cmd.Start(); err != nil {
		cancel()
		return fmt.Errorf("stdio mcp start %s: %w", c.Command, err)
	}

	if err := c.initialize(); err != nil {
		_ = c.Shutdown()
		return fmt.Errorf("stdio mcp initialize: %w", err)
	}

	tools, err := c.listTools()
	if err != nil {
		_ = c.Shutdown()
		return fmt.Errorf("stdio mcp list tools: %w", err)
	}

	c.tools = tools
	c.ready = true
	return nil
}

// Call invokes an MCP tool by name with the given arguments over stdin/stdout.
func (c *StdioClient) Call(name string, arguments map[string]any) (json.RawMessage, error) {
	c.mu.Lock()
	defer c.mu.Unlock()

	if !c.Ready() {
		return nil, fmt.Errorf("stdio mcp %q: process not running", c.ID)
	}

	reqBody := rpcRequest{
		JSONRPC: "2.0",
		ID:      time.Now().UnixNano(),
		Method:  "tools/call",
		Params: map[string]any{
			"name":      name,
			"arguments": arguments,
		},
	}
	if err := c.writeRequest(&reqBody); err != nil {
		return nil, fmt.Errorf("stdio mcp call %q: %w", name, err)
	}

	resp, err := c.readResponse()
	if err != nil {
		return nil, fmt.Errorf("stdio mcp call %q: %w", name, err)
	}
	if resp.Error != nil {
		return nil, fmt.Errorf("mcp tool error %d: %s", resp.Error.Code, resp.Error.Message)
	}
	return json.Marshal(resp.Result)
}

// Ready reports whether the client has successfully connected and the process is alive.
func (c *StdioClient) Ready() bool {
	if !c.ready {
		return false
	}
	if c.cmd == nil || c.cmd.Process == nil {
		return false
	}
	if c.cmd.ProcessState != nil && c.cmd.ProcessState.Exited() {
		c.ready = false
		return false
	}
	return true
}

// Tools returns the cached tool list.
func (c *StdioClient) Tools() []Tool {
	out := make([]Tool, len(c.tools))
	copy(out, c.tools)
	return out
}

// Shutdown terminates the child process gracefully, then forcefully after a timeout.
func (c *StdioClient) Shutdown() error {
	if c.cancel != nil {
		c.cancel()
	}
	if c.stdin != nil {
		_ = c.stdin.Close()
	}
	if c.cmd == nil || c.cmd.Process == nil {
		c.ready = false
		return nil
	}

	done := make(chan error, 1)
	go func() {
		done <- c.cmd.Wait()
	}()

	select {
	case <-done:
	case <-time.After(5 * time.Second):
		_ = c.cmd.Process.Kill()
		<-done
	}

	c.ready = false
	return nil
}

func (c *StdioClient) initialize() error {
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
	if err := c.writeRequest(&reqBody); err != nil {
		return err
	}
	_, err := c.readResponse()
	return err
}

func (c *StdioClient) listTools() ([]Tool, error) {
	reqBody := rpcRequest{
		JSONRPC: "2.0",
		ID:      2,
		Method:  "tools/list",
	}
	if err := c.writeRequest(&reqBody); err != nil {
		return nil, err
	}

	resp, err := c.readResponse()
	if err != nil {
		return nil, err
	}
	var result struct {
		Tools []Tool `json:"tools"`
	}
	if err := json.Unmarshal(resp.Result, &result); err != nil {
		return nil, err
	}

	var filtered []Tool
	for _, tool := range result.Tools {
		if tool.Name != "" {
			filtered = append(filtered, tool)
		}
	}
	return filtered, nil
}

func (c *StdioClient) writeRequest(req *rpcRequest) error {
	data, err := json.Marshal(req)
	if err != nil {
		return err
	}
	data = append(data, '\n')
	_, err = c.stdin.Write(data)
	return err
}

func (c *StdioClient) readResponse() (*rpcResponse, error) {
	scanner := bufio.NewScanner(c.stdout)
	scanner.Buffer(make([]byte, 64*1024), 1024*1024) // 1MB max buffer

	for scanner.Scan() {
		line := scanner.Bytes()
		if len(line) == 0 {
			continue
		}
		var resp rpcResponse
		if err := json.Unmarshal(line, &resp); err != nil {
			continue
		}
		if resp.ID == 0 && resp.Result == nil && resp.Error == nil {
			continue
		}
		return &resp, nil
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return nil, fmt.Errorf("stdio mcp: no valid response received (process may have exited)")
}
