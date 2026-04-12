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
}

func newRunEvent(sessionID string, runID string, sequence string) contracts.RunEvent {
	return contracts.RunEvent{
		EventID: "event-" + sequence, EventType: "run_started", SessionID: sessionID,
		RunID: runID, Sequence: 1, Timestamp: sequence, Stage: "Analyze",
		Summary: "test", Metadata: map[string]string{"task_title": "test"},
	}
}
