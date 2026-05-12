package api

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"time"

	"local-agent/gateway/internal/mcp"
)

type mcpAuditRecord struct {
	AuditID              string `json:"audit_id"`
	Timestamp            string `json:"timestamp"`
	ServerID             string `json:"server_id"`
	ToolName             string `json:"tool_name"`
	SessionID            string `json:"session_id,omitempty"`
	RunID                string `json:"run_id,omitempty"`
	TraceID              string `json:"trace_id,omitempty"`
	Allowed              bool   `json:"allowed"`
	RiskLevel            string `json:"risk_level"`
	RequiresConfirmation bool   `json:"requires_confirmation"`
	AuditEnabled         bool   `json:"audit_enabled"`
	PolicySource         string `json:"policy_source"`
	ArgumentsHash        string `json:"arguments_hash"`
	Outcome              string `json:"outcome"`
	ErrorCode            string `json:"error_code,omitempty"`
	ErrorMessage         string `json:"error_message,omitempty"`
	ElapsedMS            int64  `json:"elapsed_ms"`
}

func newMCPAuditRecord(payload mcpCallPayload, policy mcp.ToolPolicy, start time.Time, outcome string, err error) mcpAuditRecord {
	record := mcpAuditRecord{
		AuditID:   "mcp-" + time.Now().Format("20060102150405.000000000"),
		Timestamp: time.Now().Format(time.RFC3339Nano),
		ServerID:  payload.ServerID, ToolName: payload.Name,
		SessionID: payload.SessionID, RunID: payload.RunID, TraceID: payload.TraceID,
		Allowed: policy.Allowed, RiskLevel: policy.RiskLevel,
		RequiresConfirmation: policy.RequiresConfirmation, AuditEnabled: policy.AuditEnabled,
		PolicySource:  policy.PolicySource,
		ArgumentsHash: hashMCPArguments(payload.Arguments), Outcome: outcome,
		ElapsedMS: time.Since(start).Milliseconds(),
	}
	if err != nil {
		record.ErrorCode = mcpAuditErrorCode(err)
		record.ErrorMessage = err.Error()
	}
	return record
}

func writeMCPAudit(repoRoot string, record mcpAuditRecord) {
	raw, err := json.Marshal(record)
	if err != nil {
		return
	}
	path := filepath.Join(repoRoot, "logs", "mcp-audit.jsonl")
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return
	}
	file, err := os.OpenFile(path, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		return
	}
	defer func() { _ = file.Close() }()
	_, _ = file.Write(append(raw, '\n'))
}

func hashMCPArguments(arguments map[string]any) string {
	raw, _ := json.Marshal(arguments)
	sum := sha256.Sum256(raw)
	return hex.EncodeToString(sum[:])
}

func mcpAuditErrorCode(err error) string {
	text := err.Error()
	if strings.Contains(text, "not allowlisted") {
		return "mcp_tool_not_allowlisted"
	}
	if strings.Contains(text, "requires confirmation") {
		return "mcp_confirmation_required"
	}
	return "mcp_call_failed"
}
