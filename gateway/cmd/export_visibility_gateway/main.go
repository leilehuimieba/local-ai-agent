package main

import (
	"bytes"
	"encoding/json"
	"os"
	"path/filepath"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/session"
)

type runtimePayload struct {
	Events []contracts.RunEvent `json:"events"`
}

type coverageReport struct {
	EventCount     int                `json:"event_count"`
	LogCount       int                `json:"log_count"`
	EventCoverage  map[string]int     `json:"event_coverage"`
	LogCoverage    map[string]int     `json:"log_coverage"`
	LatestEvent    contracts.RunEvent `json:"latest_event"`
	LatestLogEntry contracts.LogEntry `json:"latest_log_entry"`
}

func main() {
	wd, _ := os.Getwd()
	repoRoot := filepath.Dir(wd)
	payload := loadRuntimePayload(filepath.Join(repoRoot, "tmp", "stage-h-visibility", "runtime.json"))
	report := buildCoverageReport(repoRoot, payload)
	out, err := json.MarshalIndent(report, "", "  ")
	if err != nil {
		return
	}
	_, _ = os.Stdout.Write(out)
}

func loadRuntimePayload(path string) runtimePayload {
	blob, err := os.ReadFile(path)
	if err != nil {
		return runtimePayload{}
	}
	blob = bytes.TrimPrefix(blob, []byte{0xEF, 0xBB, 0xBF})
	var payload runtimePayload
	_ = json.Unmarshal(blob, &payload)
	return payload
}

func buildCoverageReport(repoRoot string, payload runtimePayload) coverageReport {
	bus := session.NewEventBus(repoRoot)
	for _, event := range payload.Events {
		bus.Publish(event)
	}
	sessionID, runID := readIdentity(payload.Events)
	events := bus.Snapshot(sessionID)
	logs := bus.RecentBy(len(events)+10, sessionID, runID)
	return coverageReport{
		EventCount:     len(events),
		LogCount:       len(logs),
		EventCoverage:  eventCoverage(events),
		LogCoverage:    logCoverage(logs),
		LatestEvent:    pickLatestEvent(events),
		LatestLogEntry: pickLatestLog(logs),
	}
}

func readIdentity(events []contracts.RunEvent) (string, string) {
	if len(events) == 0 {
		return "", ""
	}
	return events[0].SessionID, events[0].RunID
}

func eventCoverage(events []contracts.RunEvent) map[string]int {
	return map[string]int{
		"activity_state":   countRunEvent(events, func(item contracts.RunEvent) string { return item.ActivityState }),
		"heartbeat_at":     countRunEvent(events, func(item contracts.RunEvent) string { return item.HeartbeatAt }),
		"stall_seconds":    countRunEvent(events, func(item contracts.RunEvent) string { return item.StallSeconds }),
		"waiting_reason":   countRunEvent(events, func(item contracts.RunEvent) string { return item.WaitingReason }),
		"next_action_hint": countRunEvent(events, func(item contracts.RunEvent) string { return item.NextActionHint }),
		"failure_route":    countRunEvent(events, func(item contracts.RunEvent) string { return item.FailureRoute }),
		"updated_at":       countRunEvent(events, func(item contracts.RunEvent) string { return item.UpdatedAt }),
		"task_title":       countRunEvent(events, func(item contracts.RunEvent) string { return item.TaskTitle }),
		"active_tool":      countRunEvent(events, func(item contracts.RunEvent) string { return item.ActiveTool }),
		"evidence_ref":     countRunEvent(events, func(item contracts.RunEvent) string { return item.EvidenceRef }),
	}
}

func logCoverage(items []contracts.LogEntry) map[string]int {
	return map[string]int{
		"activity_state":   countLogEntry(items, func(item contracts.LogEntry) string { return item.ActivityState }),
		"heartbeat_at":     countLogEntry(items, func(item contracts.LogEntry) string { return item.HeartbeatAt }),
		"stall_seconds":    countLogEntry(items, func(item contracts.LogEntry) string { return item.StallSeconds }),
		"waiting_reason":   countLogEntry(items, func(item contracts.LogEntry) string { return item.WaitingReason }),
		"next_action_hint": countLogEntry(items, func(item contracts.LogEntry) string { return item.NextActionHint }),
		"failure_route":    countLogEntry(items, func(item contracts.LogEntry) string { return item.FailureRoute }),
		"updated_at":       countLogEntry(items, func(item contracts.LogEntry) string { return item.UpdatedAt }),
		"task_title":       countLogEntry(items, func(item contracts.LogEntry) string { return item.TaskTitle }),
		"active_tool":      countLogEntry(items, func(item contracts.LogEntry) string { return item.ActiveTool }),
		"evidence_ref":     countLogEntry(items, func(item contracts.LogEntry) string { return item.EvidenceRef }),
	}
}

func countRunEvent(items []contracts.RunEvent, read func(contracts.RunEvent) string) int {
	count := 0
	for _, item := range items {
		if read(item) != "" {
			count++
		}
	}
	return count
}

func countLogEntry(items []contracts.LogEntry, read func(contracts.LogEntry) string) int {
	count := 0
	for _, item := range items {
		if read(item) != "" {
			count++
		}
	}
	return count
}

func pickLatestEvent(items []contracts.RunEvent) contracts.RunEvent {
	if len(items) == 0 {
		return contracts.RunEvent{}
	}
	return items[len(items)-1]
}

func pickLatestLog(items []contracts.LogEntry) contracts.LogEntry {
	if len(items) == 0 {
		return contracts.LogEntry{}
	}
	return items[len(items)-1]
}
