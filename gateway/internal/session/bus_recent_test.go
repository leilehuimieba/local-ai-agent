package session

import (
	"testing"

	"local-agent/gateway/internal/contracts"
)

func TestRecentByFiltersSessionAndRun(t *testing.T) {
	repoRoot := t.TempDir()
	bus := NewEventBus(repoRoot)
	bus.Publish(newRunEvent("s1", "r1", "1"))
	bus.Publish(newRunEvent("s1", "r2", "2"))
	bus.Publish(newRunEvent("s2", "r3", "3"))
	items := bus.RecentBy(20, "s1", "r2")
	if len(items) != 1 {
		t.Fatalf("len=%d want 1", len(items))
	}
	if items[0].SessionID != "s1" || items[0].RunID != "r2" {
		t.Fatalf("unexpected log: %#v", items[0])
	}
	if items[0].TraceID != "trace-r2" {
		t.Fatalf("trace_id=%q want trace-r2", items[0].TraceID)
	}
}

func TestRecentRunsKeepsLatestPerRun(t *testing.T) {
	repoRoot := t.TempDir()
	bus := NewEventBus(repoRoot)
	bus.Publish(newRunEventWithType("s1", "r1", "1", "run_started"))
	bus.Publish(newRunEventWithType("s1", "r1", "2", "run_finished"))
	bus.Publish(newRunEventWithType("s1", "r2", "3", "run_started"))
	items := bus.RecentRuns(10, "s1")
	if len(items) != 2 {
		t.Fatalf("len=%d want 2", len(items))
	}
	if items[0].RunID != "r1" || items[0].EventType != "run_finished" {
		t.Fatalf("unexpected first item: %#v", items[0])
	}
	if items[1].RunID != "r2" || items[1].EventType != "run_started" {
		t.Fatalf("unexpected second item: %#v", items[1])
	}
}

func newRunEvent(sessionID string, runID string, sequence string) contracts.RunEvent {
	return newRunEventWithType(sessionID, runID, sequence, "run_started")
}

func newRunEventWithType(sessionID string, runID string, sequence string, eventType string) contracts.RunEvent {
	return contracts.RunEvent{
		EventID: "event-" + sequence, EventType: eventType, SessionID: sessionID,
		RunID: runID, TraceID: "trace-" + runID, Sequence: 1, Timestamp: sequence, Stage: "Analyze",
		Summary: "test", Metadata: map[string]string{"task_title": "test"},
	}
}
