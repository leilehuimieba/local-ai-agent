package api

import (
	"bufio"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
)

type mcpAuditQuery struct {
	Limit    int
	ServerID string
	ToolName string
}

func mcpAuditsHandler(repoRoot string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		query, err := decodeMCPAuditQuery(r)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		writeJSON(w, http.StatusOK, map[string]any{
			"items": queryMCPAudits(repoRoot, query),
		})
	}
}

func decodeMCPAuditQuery(r *http.Request) (mcpAuditQuery, error) {
	limit, err := parseMCPAuditLimit(r.URL.Query().Get("limit"))
	if err != nil {
		return mcpAuditQuery{}, err
	}
	return mcpAuditQuery{
		Limit:    limit,
		ServerID: r.URL.Query().Get("server_id"),
		ToolName: r.URL.Query().Get("tool_name"),
	}, nil
}

func parseMCPAuditLimit(raw string) (int, error) {
	if raw == "" {
		return 20, nil
	}
	value, err := strconv.Atoi(raw)
	if err != nil {
		return 0, fmt.Errorf("limit must be integer")
	}
	if value < 1 || value > 100 {
		return 0, fmt.Errorf("limit must be in [1,100]")
	}
	return value, nil
}

func queryMCPAudits(repoRoot string, query mcpAuditQuery) []mcpAuditRecord {
	items := readMCPAuditRecords(filepath.Join(repoRoot, "logs", "mcp-audit.jsonl"))
	out := make([]mcpAuditRecord, 0, query.Limit)
	for i := len(items) - 1; i >= 0; i-- {
		if !matchMCPAudit(items[i], query) {
			continue
		}
		out = append(out, items[i])
		if len(out) >= query.Limit {
			break
		}
	}
	return out
}

func readMCPAuditRecords(path string) []mcpAuditRecord {
	file, err := os.Open(path)
	if err != nil {
		return nil
	}
	defer file.Close()
	return scanMCPAuditRecords(file)
}

func scanMCPAuditRecords(file *os.File) []mcpAuditRecord {
	records := make([]mcpAuditRecord, 0, 32)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var record mcpAuditRecord
		if json.Unmarshal(scanner.Bytes(), &record) == nil {
			records = append(records, record)
		}
	}
	return records
}

func matchMCPAudit(record mcpAuditRecord, query mcpAuditQuery) bool {
	if query.ServerID != "" && record.ServerID != query.ServerID {
		return false
	}
	if query.ToolName != "" && record.ToolName != query.ToolName {
		return false
	}
	return true
}
