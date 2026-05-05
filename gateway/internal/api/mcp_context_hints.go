package api

import (
	"encoding/json"
	"fmt"
	"sort"
	"strconv"
	"strings"

	"local-agent/gateway/internal/mcp"
)

const mcpToolPreviewLimit = 12
const mcpToolFunctionPrefix = "mcp__"

func withMCPToolHints(hints map[string]string, mgr *mcp.Manager) map[string]string {
	if mgr == nil {
		return hints
	}
	tools := mgr.AllowedTools()
	if len(tools) == 0 {
		return hints
	}
	if hints == nil {
		hints = map[string]string{}
	}
	hints["mcp_tool_count"] = strconv.Itoa(len(tools))
	hints["mcp_servers"] = strings.Join(mcpToolServers(tools), ",")
	hints["mcp_tool_preview"] = mcpToolPreview(tools, mcpToolPreviewLimit)
	hints["mcp_tool_specs_json"] = mcpToolSpecsJSON(tools, mcpToolPreviewLimit)
	return hints
}

func withMCPBridgeHints(hints map[string]string, port int, token string) map[string]string {
	if port == 0 || token == "" {
		return hints
	}
	if hints == nil {
		hints = map[string]string{}
	}
	hints["mcp_gateway_url"] = fmt.Sprintf("http://127.0.0.1:%d/api/v1/mcp/call", port)
	hints["mcp_gateway_token"] = token
	return hints
}

func mcpToolServers(tools []mcp.Tool) []string {
	seen := map[string]bool{}
	for _, tool := range tools {
		if tool.ServerID != "" {
			seen[tool.ServerID] = true
		}
	}
	servers := make([]string, 0, len(seen))
	for serverID := range seen {
		servers = append(servers, serverID)
	}
	sort.Strings(servers)
	return servers
}

func mcpToolPreview(tools []mcp.Tool, limit int) string {
	items := sortedMCPTools(tools)
	if limit > 0 && len(items) > limit {
		items = items[:limit]
	}
	parts := make([]string, 0, len(items))
	for _, tool := range items {
		parts = append(parts, mcpToolPreviewItem(tool))
	}
	return "MCP工具可按策略直连、确认或拒绝：" + strings.Join(parts, "；")
}

func sortedMCPTools(tools []mcp.Tool) []mcp.Tool {
	items := append([]mcp.Tool(nil), tools...)
	sort.Slice(items, func(i int, j int) bool {
		left := items[i].ServerID + "/" + items[i].Name
		right := items[j].ServerID + "/" + items[j].Name
		return left < right
	})
	return items
}

func mcpToolPreviewItem(tool mcp.Tool) string {
	name := fmt.Sprintf("%s/%s[%s]", tool.ServerID, tool.Name, tool.RiskLevel)
	desc := truncateMCPText(strings.TrimSpace(tool.Description), 96)
	if desc == "" {
		return name
	}
	return fmt.Sprintf("%s - %s", name, desc)
}

func truncateMCPText(value string, limit int) string {
	runes := []rune(value)
	if len(runes) <= limit {
		return value
	}
	return string(runes[:limit]) + "..."
}

type mcpRuntimeToolSpec struct {
	ServerID             string          `json:"server_id"`
	Name                 string          `json:"name"`
	FunctionName         string          `json:"function_name"`
	Description          string          `json:"description"`
	RiskLevel            string          `json:"risk_level"`
	RequiresConfirmation bool            `json:"requires_confirmation"`
	AuditEnabled         bool            `json:"audit_enabled"`
	InputSchema          json.RawMessage `json:"input_schema,omitempty"`
}

func mcpToolSpecsJSON(tools []mcp.Tool, limit int) string {
	specs := mcpToolSpecs(tools, limit)
	raw, err := json.Marshal(specs)
	if err != nil {
		return "[]"
	}
	return string(raw)
}

func mcpToolSpecs(tools []mcp.Tool, limit int) []mcpRuntimeToolSpec {
	items := sortedMCPTools(mcpVisibleTools(tools))
	if limit > 0 && len(items) > limit {
		items = items[:limit]
	}
	specs := make([]mcpRuntimeToolSpec, 0, len(items))
	for _, tool := range items {
		specs = append(specs, mcpToolSpec(tool))
	}
	return specs
}

func mcpVisibleTools(tools []mcp.Tool) []mcp.Tool {
	out := make([]mcp.Tool, 0, len(tools))
	for _, tool := range tools {
		if tool.Allowed && mcpToolFunctionValid(tool) {
			out = append(out, tool)
		}
	}
	return out
}

func mcpToolSpec(tool mcp.Tool) mcpRuntimeToolSpec {
	return mcpRuntimeToolSpec{
		ServerID: tool.ServerID, Name: tool.Name,
		FunctionName:         mcpToolFunctionName(tool.ServerID, tool.Name),
		Description:          tool.Description,
		RiskLevel:            tool.RiskLevel,
		RequiresConfirmation: tool.RequiresConfirmation,
		AuditEnabled:         tool.AuditEnabled,
		InputSchema:          tool.InputSchema,
	}
}

func mcpToolFunctionName(serverID string, name string) string {
	return mcpToolFunctionPrefix + serverID + "__" + name
}

func mcpToolFunctionValid(tool mcp.Tool) bool {
	name := mcpToolFunctionName(tool.ServerID, tool.Name)
	return len(name) <= 64 && mcpToolNamePartValid(tool.ServerID) && mcpToolNamePartValid(tool.Name)
}

func mcpToolNamePartValid(value string) bool {
	if value == "" {
		return false
	}
	if strings.Contains(value, "__") {
		return false
	}
	for _, r := range value {
		if !isMCPToolNameRune(r) {
			return false
		}
	}
	return true
}

func isMCPToolNameRune(r rune) bool {
	return r == '_' || r == '-' || r >= '0' && r <= '9' ||
		r >= 'A' && r <= 'Z' || r >= 'a' && r <= 'z'
}
