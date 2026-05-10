package api

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestQueryMCPAuditsFiltersLatestFirst(t *testing.T) {
	repoRoot := t.TempDir()
	writeMCPAuditFixture(t, repoRoot, mcpAuditRecord{AuditID: "1", ServerID: "docs", ToolName: "search"})
	writeMCPAuditFixture(t, repoRoot, mcpAuditRecord{AuditID: "2", ServerID: "docs", ToolName: "read"})
	writeMCPAuditFixture(t, repoRoot, mcpAuditRecord{AuditID: "3", ServerID: "browser", ToolName: "open"})

	items := queryMCPAudits(repoRoot, mcpAuditQuery{Limit: 2, ServerID: "docs"})

	require.Len(t, items, 2)
	require.Equal(t, "2", items[0].AuditID)
	require.Equal(t, "1", items[1].AuditID)
}

func TestParseMCPAuditLimit(t *testing.T) {
	value, err := parseMCPAuditLimit("101")
	require.EqualError(t, err, "limit must be in [1,100]")
	require.Zero(t, value)
}

func writeMCPAuditFixture(t *testing.T, repoRoot string, record mcpAuditRecord) {
	t.Helper()
	path := filepath.Join(repoRoot, "logs", "mcp-audit.jsonl")
	require.NoError(t, os.MkdirAll(filepath.Dir(path), 0o755))
	raw, err := json.Marshal(record)
	require.NoError(t, err)
	file, err := os.OpenFile(path, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	require.NoError(t, err)
	defer file.Close()
	_, err = file.WriteString(string(raw) + "\n")
	require.NoError(t, err)
}

func TestScanMCPAuditRecordsSkipsInvalidLines(t *testing.T) {
	file := writeTempAuditFile(t, `{"audit_id":"1"}`+"\n"+"bad-json\n")
	defer file.Close()
	items := scanMCPAuditRecords(file)
	require.Len(t, items, 1)
	require.Equal(t, "1", items[0].AuditID)
}

func writeTempAuditFile(t *testing.T, body string) *os.File {
	t.Helper()
	file, err := os.CreateTemp(t.TempDir(), "audit-*.jsonl")
	require.NoError(t, err)
	_, err = file.WriteString(strings.TrimSpace(body))
	require.NoError(t, err)
	_, err = file.Seek(0, 0)
	require.NoError(t, err)
	return file
}
