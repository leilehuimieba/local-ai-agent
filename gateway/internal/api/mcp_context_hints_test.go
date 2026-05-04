package api

import (
	"encoding/json"
	"strings"
	"testing"

	"local-agent/gateway/internal/mcp"

	"github.com/stretchr/testify/require"
)

func TestMCPToolPreviewIsStableAndScoped(t *testing.T) {
	tools := []mcp.Tool{
		{Name: "search", Description: "搜索网页", ServerID: "web", RiskLevel: "medium"},
		{Name: "read", Description: "读取仓库文件", ServerID: "code", RiskLevel: "low"},
	}
	preview := mcpToolPreview(tools, 8)
	require.Contains(t, preview, "MCP工具可自动执行或按策略拒绝")
	require.Contains(t, preview, "code/read[low] - 读取仓库文件")
	require.Contains(t, preview, "web/search[medium] - 搜索网页")
	require.Less(t, stringsIndex(t, preview, "code/read"), stringsIndex(t, preview, "web/search"))
}

func TestMCPToolServersAreSortedAndUnique(t *testing.T) {
	tools := []mcp.Tool{
		{Name: "b", ServerID: "web"},
		{Name: "a", ServerID: "code"},
		{Name: "c", ServerID: "web"},
	}
	require.Equal(t, []string{"code", "web"}, mcpToolServers(tools))
}

func TestMCPToolSpecsOnlyExposeExecutableTools(t *testing.T) {
	tools := []mcp.Tool{
		{Name: "search", ServerID: "docs", Allowed: true, RiskLevel: "low"},
		{Name: "write", ServerID: "docs", Allowed: true, RequiresConfirmation: true},
		{Name: "blocked", ServerID: "docs", Allowed: false},
	}
	specs := mcpToolSpecs(tools, 8)
	require.Len(t, specs, 1)
	require.Equal(t, "mcp__docs__search", specs[0].FunctionName)
	require.Equal(t, "low", specs[0].RiskLevel)
}

func TestMCPToolSpecsJSONIncludesPolicyFields(t *testing.T) {
	tools := []mcp.Tool{{
		Name: "search", ServerID: "docs", Description: "Search docs",
		Allowed: true, RiskLevel: "low", AuditEnabled: true,
		InputSchema: json.RawMessage(`{"type":"object"}`),
	}}
	raw := mcpToolSpecsJSON(tools, 8)
	require.Contains(t, raw, `"function_name":"mcp__docs__search"`)
	require.Contains(t, raw, `"audit_enabled":true`)
	require.Contains(t, raw, `"input_schema":{"type":"object"}`)
}

func TestMCPBridgeHintsDoNotRequireToolPreview(t *testing.T) {
	hints := withMCPBridgeHints(nil, 18080, "secret")
	require.Equal(t, "http://127.0.0.1:18080/api/v1/mcp/call", hints["mcp_gateway_url"])
	require.Equal(t, "secret", hints["mcp_gateway_token"])
}

func stringsIndex(t *testing.T, value string, token string) int {
	t.Helper()
	index := strings.Index(value, token)
	require.NotEqual(t, -1, index)
	return index
}
