package session

import (
	"encoding/json"
	"os"
	"path/filepath"
	"sync"

	"local-agent/gateway/internal/contracts"
)

type EventBus struct {
	mu           sync.RWMutex
	subscribers  map[string]map[chan contracts.RunEvent]struct{}
	history      map[string][]contracts.RunEvent
	eventLogPath string
	logPath      string
}

func NewEventBus(repoRoot string) *EventBus {
	logDir := filepath.Join(repoRoot, "logs")
	_ = os.MkdirAll(logDir, 0o755)

	return &EventBus{
		subscribers:  make(map[string]map[chan contracts.RunEvent]struct{}),
		history:      make(map[string][]contracts.RunEvent),
		eventLogPath: filepath.Join(logDir, "run-events.jsonl"),
		logPath:      filepath.Join(logDir, "run-logs.jsonl"),
	}
}

func (b *EventBus) Publish(event contracts.RunEvent) {
	b.mu.Lock()
	current := b.normalizeEventLocked(event)
	b.history[current.SessionID] = append(b.history[current.SessionID], current)
	subscribers := b.subscribersForSessionLocked(current.SessionID)
	b.mu.Unlock()

	b.appendEventLog(current)
	b.appendLog(current)
	for _, ch := range subscribers {
		select {
		case ch <- current:
		default:
		}
	}
}

func (b *EventBus) subscribersForSessionLocked(sessionID string) []chan contracts.RunEvent {
	var subscribers []chan contracts.RunEvent
	for ch := range b.subscribers[sessionID] {
		subscribers = append(subscribers, ch)
	}
	return subscribers
}

func (b *EventBus) normalizeEventLocked(event contracts.RunEvent) contracts.RunEvent {
	event = normalizeEventFields(event, b.latestRunEventLocked(event.SessionID, event.RunID))
	if event.EventType == "confirmation_required" {
		event.RiskLevel = pickFirst(event.RiskLevel, event.Metadata["risk_level"])
		event.ConfirmationID = pickFirst(event.ConfirmationID, event.Metadata["confirmation_id"])
	}
	if event.EventType != "run_failed" {
		return event
	}
	previous := b.latestRunEventLocked(event.SessionID, event.RunID)
	return normalizeRunFailedEvent(event, previous)
}

func (b *EventBus) latestRunEventLocked(sessionID string, runID string) contracts.RunEvent {
	items := b.history[sessionID]
	for index := len(items) - 1; index >= 0; index-- {
		if items[index].RunID == runID {
			return items[index]
		}
	}
	return contracts.RunEvent{}
}

func (b *EventBus) Snapshot(sessionID string) []contracts.RunEvent {
	b.mu.RLock()
	defer b.mu.RUnlock()

	items := b.history[sessionID]
	cloned := make([]contracts.RunEvent, len(items))
	copy(cloned, items)
	return cloned
}

func (b *EventBus) Subscribe(sessionID string) (<-chan contracts.RunEvent, func()) {
	ch := make(chan contracts.RunEvent, 16)

	b.mu.Lock()
	if _, ok := b.subscribers[sessionID]; !ok {
		b.subscribers[sessionID] = make(map[chan contracts.RunEvent]struct{})
	}
	b.subscribers[sessionID][ch] = struct{}{}
	b.mu.Unlock()

	cancel := func() {
		b.mu.Lock()
		defer b.mu.Unlock()
		if subscribers, ok := b.subscribers[sessionID]; ok {
			if _, exists := subscribers[ch]; exists {
				delete(subscribers, ch)
				close(ch)
			}
			if len(subscribers) == 0 {
				delete(b.subscribers, sessionID)
			}
		}
	}

	return ch, cancel
}

func (b *EventBus) Recent(limit int) []contracts.LogEntry {
	return tailLogEntries(b.readLogsFromFile(), limit)
}

func (b *EventBus) RecentBy(limit int, sessionID string, runID string) []contracts.LogEntry {
	items := b.readLogsFromFile()
	filtered := filterLogEntries(items, sessionID, runID)
	return tailLogEntries(filtered, limit)
}

func (b *EventBus) RecentRuns(limit int, sessionID string) []contracts.LogEntry {
	items := b.readLogsFromFile()
	filtered := filterLogEntries(items, sessionID, "")
	return tailUniqueRunEntries(filtered, limit)
}

func (b *EventBus) appendEventLog(event contracts.RunEvent) {
	file, err := os.OpenFile(b.eventLogPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		return
	}
	defer file.Close()

	payload, err := json.Marshal(event)
	if err != nil {
		return
	}
	_, _ = file.Write(append(payload, '\n'))
}

func (b *EventBus) appendLog(event contracts.RunEvent) {
	file, err := os.OpenFile(b.logPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		return
	}
	defer file.Close()

	entry := logEntryFromEvent(event)
	payload, err := json.Marshal(entry)
	if err != nil {
		return
	}
	_, _ = file.Write(append(payload, '\n'))
}

func logEntryFromEvent(event contracts.RunEvent) contracts.LogEntry {
	level, category := classifyLogLevelAndCategory(event)
	entry := baseLogEntry(event, level, category)
	fillLogEntryOutcome(&entry, event)
	fillLogEntrySnapshots(&entry, event, buildErrorInfo(event))
	return entry
}

func classifyLogLevelAndCategory(event contracts.RunEvent) (string, string) {
	level := "info"
	category := "runtime"
	if event.ToolName != "" {
		category = "tool"
	}
	switch event.EventType {
	case "confirmation_required":
		level, category = "warn", "risk"
	case "memory_written", "memory_recalled":
		category = "memory"
	case "knowledge_written":
		category = "knowledge"
	case "knowledge_write_skipped":
		level, category = "warn", "knowledge"
	case "memory_write_skipped":
		level, category = "warn", "memory"
	case "run_failed":
		level = "error"
	}
	return level, category
}

func buildErrorInfo(event contracts.RunEvent) *contracts.ErrorInfo {
	if event.EventType != "run_failed" && event.Metadata["error_code"] == "" {
		return nil
	}
	return &contracts.ErrorInfo{
		ErrorCode: event.Metadata["error_code"],
		Message:   pickFirst(event.Metadata["error_message"], event.Detail),
		Summary:   event.Summary,
		Retryable: event.Metadata["retryable"] != "false",
		Source:    pickFirst(event.Metadata["error_source"], event.Source),
		Stage:     event.Stage,
		Metadata:  event.Metadata,
	}
}

func baseLogEntry(event contracts.RunEvent, level string, category string) contracts.LogEntry {
	return contracts.LogEntry{
		LogID:       event.EventID,
		SessionID:   event.SessionID,
		RunID:       event.RunID,
		Timestamp:   event.Timestamp,
		Level:       level,
		Category:    category,
		Source:      event.Source,
		RecordType:  pickFirst(event.RecordType, event.Metadata["record_type"]),
		SourceType:  pickFirst(event.SourceType, event.Metadata["source_type"]),
		AgentID:     event.AgentID,
		AgentLabel:  event.AgentLabel,
		EventType:   event.EventType,
		TraceID:     pickFirst(event.TraceID, event.Metadata["trace_id"]),
		Stage:       event.Stage,
		Summary:     event.Summary,
		Detail:      event.Detail,
		Metadata:    event.Metadata,
		FinalAnswer: pickFirst(event.FinalAnswer, event.Metadata["final_answer"]),
	}
}

func fillLogEntryOutcome(entry *contracts.LogEntry, event contracts.RunEvent) {
	entry.ToolName = pickFirst(event.ToolName, event.Metadata["tool_name"])
	entry.ToolDisplayName = pickFirst(event.ToolDisplayName, event.Metadata["tool_display_name"])
	entry.ToolCategory = pickFirst(event.ToolCategory, event.Metadata["tool_category"])
	entry.OutputKind = pickFirst(event.OutputKind, event.Metadata["output_kind"])
	entry.ResultSummary = pickFirst(event.ResultSummary, event.Metadata["result_summary"])
	entry.ArtifactPath = pickFirst(event.ArtifactPath, event.Metadata["artifact_path"])
	entry.RiskLevel = pickFirst(event.RiskLevel, event.Metadata["risk_level"])
	entry.ConfirmationID = pickFirst(event.ConfirmationID, event.Metadata["confirmation_id"])
	entry.CompletionStatus = pickFirst(event.CompletionStatus, event.Metadata["completion_status"])
	entry.CompletionReason = pickFirst(event.CompletionReason, event.Metadata["completion_reason"])
	entry.VerificationSummary = pickFirst(event.VerificationSummary, event.Metadata["verification_summary"], verificationSummaryFromSnapshot(event.VerificationSnapshot))
}

func fillLogEntrySnapshots(entry *contracts.LogEntry, event contracts.RunEvent, errorInfo *contracts.ErrorInfo) {
	entry.ContextSnapshot = event.ContextSnapshot
	entry.ToolCallSnapshot = event.ToolCallSnapshot
	entry.VerificationSnapshot = event.VerificationSnapshot
	entry.Error = errorInfo
}

func normalizeEventFields(event contracts.RunEvent, previous contracts.RunEvent) contracts.RunEvent {
	event.RecordType = pickFirst(event.RecordType, event.Metadata["record_type"], previous.RecordType)
	event.SourceType = pickFirst(event.SourceType, event.Metadata["source_type"], previous.SourceType)
	event.ArtifactPath = pickFirst(event.ArtifactPath, event.Metadata["artifact_path"], previous.ArtifactPath)
	event.CompletionReason = pickFirst(event.CompletionReason, event.Metadata["completion_reason"], previous.CompletionReason)
	event.VerificationSummary = pickFirst(event.VerificationSummary, event.Metadata["verification_summary"], verificationSummaryFromSnapshot(event.VerificationSnapshot), previous.VerificationSummary)
	return event
}

func normalizeRunFailedEvent(event contracts.RunEvent, previous contracts.RunEvent) contracts.RunEvent {
	event.ToolName = pickFirst(event.ToolName, event.Metadata["tool_name"], previous.ToolName)
	event.ToolDisplayName = pickFirst(event.ToolDisplayName, event.Metadata["tool_display_name"], previous.ToolDisplayName)
	event.ToolCategory = pickFirst(event.ToolCategory, event.Metadata["tool_category"], previous.ToolCategory)
	event.OutputKind = pickFirst(event.OutputKind, event.Metadata["output_kind"], previous.OutputKind)
	event.ResultSummary = pickFirst(event.ResultSummary, event.Metadata["result_summary"], previous.ResultSummary, event.Summary)
	event.RiskLevel = pickFirst(event.RiskLevel, event.Metadata["risk_level"], previous.RiskLevel)
	event.ConfirmationID = pickFirst(event.ConfirmationID, event.Metadata["confirmation_id"], previous.ConfirmationID)
	return event
}

func normalizeLogEntry(item contracts.LogEntry) contracts.LogEntry {
	item.RecordType = pickFirst(item.RecordType, item.Metadata["record_type"])
	item.SourceType = pickFirst(item.SourceType, item.Metadata["source_type"])
	item.TraceID = pickFirst(item.TraceID, item.Metadata["trace_id"])
	item.ArtifactPath = pickFirst(item.ArtifactPath, item.Metadata["artifact_path"])
	item.CompletionReason = pickFirst(item.CompletionReason, item.Metadata["completion_reason"])
	item.VerificationSummary = pickFirst(item.VerificationSummary, item.Metadata["verification_summary"], verificationSummaryFromSnapshot(item.VerificationSnapshot))
	return item
}

func verificationSummaryFromSnapshot(snapshot *contracts.VerificationSnapshot) string {
	if snapshot == nil {
		return ""
	}
	return snapshot.Summary
}

func pickFirst(values ...string) string {
	for _, value := range values {
		if value != "" {
			return value
		}
	}
	return ""
}

func (b *EventBus) readLogsFromFile() []contracts.LogEntry {
	file, err := os.Open(b.logPath)
	if err != nil {
		return nil
	}
	defer file.Close()
	var items []contracts.LogEntry
	decoder := json.NewDecoder(file)
	for {
		var item contracts.LogEntry
		if err := decoder.Decode(&item); err != nil {
			break
		}
		items = append(items, normalizeLogEntry(item))
	}
	return items
}

func filterLogEntries(items []contracts.LogEntry, sessionID string, runID string) []contracts.LogEntry {
	if sessionID == "" && runID == "" {
		return items
	}
	filtered := make([]contracts.LogEntry, 0, len(items))
	for _, item := range items {
		if !matchesLogEntry(item, sessionID, runID) {
			continue
		}
		filtered = append(filtered, item)
	}
	return filtered
}

func matchesLogEntry(item contracts.LogEntry, sessionID string, runID string) bool {
	if sessionID != "" && item.SessionID != sessionID {
		return false
	}
	if runID != "" && item.RunID != runID {
		return false
	}
	return true
}

func tailLogEntries(items []contracts.LogEntry, limit int) []contracts.LogEntry {
	if limit <= 0 {
		limit = 50
	}
	if len(items) <= limit {
		return items
	}
	return items[len(items)-limit:]
}

func tailUniqueRunEntries(items []contracts.LogEntry, limit int) []contracts.LogEntry {
	if limit <= 0 {
		limit = 50
	}
	seen := make(map[string]struct{})
	result := make([]contracts.LogEntry, 0, limit)
	for index := len(items) - 1; index >= 0 && len(result) < limit; index-- {
		key := runIdentityKey(items[index])
		if _, ok := seen[key]; ok {
			continue
		}
		seen[key] = struct{}{}
		result = append(result, items[index])
	}
	reverseLogEntries(result)
	return result
}

func runIdentityKey(item contracts.LogEntry) string {
	return item.SessionID + "|" + item.RunID
}

func reverseLogEntries(items []contracts.LogEntry) {
	for left, right := 0, len(items)-1; left < right; left, right = left+1, right-1 {
		items[left], items[right] = items[right], items[left]
	}
}
