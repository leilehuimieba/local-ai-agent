package api

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/session"
)

func TestLogsHandlerRunsViewReturnsDistinctRuns(t *testing.T) {
	bus := session.NewEventBus(t.TempDir())
	bus.Publish(testRunEvent("s1", "r1", "1", "run_started"))
	bus.Publish(testRunEvent("s1", "r1", "2", "run_finished"))
	bus.Publish(testRunEvent("s1", "r2", "3", "run_started"))
	req := httptest.NewRequest(http.MethodGet, "/api/v1/logs?view=runs&session_id=s1&limit=10", nil)
	rec := httptest.NewRecorder()
	logsHandler(bus).ServeHTTP(rec, req)
	if rec.Code != http.StatusOK {
		t.Fatalf("code=%d want 200", rec.Code)
	}
	items := decodeLogsResponse(t, rec.Body.Bytes())
	if len(items) != 2 || items[0].RunID != "r1" || items[1].RunID != "r2" {
		t.Fatalf("unexpected items: %#v", items)
	}
	if items[0].TraceID != "trace-r1" || items[1].TraceID != "trace-r2" {
		t.Fatalf("unexpected trace ids: %#v", items)
	}
}

func decodeLogsResponse(t *testing.T, body []byte) []contracts.LogEntry {
	t.Helper()
	var payload LogsResponse
	if err := json.Unmarshal(body, &payload); err != nil {
		t.Fatalf("decode response: %v", err)
	}
	return payload.Items
}

func testRunEvent(sessionID string, runID string, stamp string, eventType string) contracts.RunEvent {
	return contracts.RunEvent{
		EventID: "event-" + stamp, EventType: eventType, SessionID: sessionID,
		RunID: runID, TraceID: "trace-" + runID, Sequence: 1, Timestamp: stamp, Stage: "Analyze", Summary: "test",
		Metadata: map[string]string{"task_title": "test"},
	}
}
