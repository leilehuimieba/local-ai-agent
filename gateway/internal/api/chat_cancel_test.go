package api

import (
	"encoding/json"
	"net"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"local-agent/gateway/internal/contracts"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/session"
	"local-agent/gateway/internal/state"
)

func TestCancelStopsRunningRun(t *testing.T) {
	started := make(chan struct{}, 1)
	server := blockingRuntimeServer(started)
	defer server.Close()
	cfg := sampleAppConfig()
	cfg.Providers[0].APIKey = "key-for-cancel-test"
	repoRoot := t.TempDir()
	handler := NewChatHandler(repoRoot, cfg, runtimeclient.NewClient(runtimePort(server.URL)), session.NewEventBus(repoRoot), state.NewSettingsStore(repoRoot, cfg), state.NewConfirmationStore(), state.NewProviderCredentialStore(repoRoot), state.NewRuntimeProviderStore(repoRoot))
	accepted := invokeRunAndDecode(t, handler, "session-cancel-1")
	<-started
	invokeCancel(t, handler, accepted.SessionID, accepted.RunID)
	terminal := waitRunTerminal(t, handler, accepted.SessionID, accepted.RunID)
	if terminal.CompletionStatus != "cancelled" || terminal.Metadata["error_code"] != "run_cancelled" {
		t.Fatalf("unexpected terminal: %#v", terminal)
	}
}

func blockingRuntimeServer(started chan struct{}) *httptest.Server {
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/runtime/run" {
			http.NotFound(w, r)
			return
		}
		select {
		case started <- struct{}{}:
		default:
		}
		select {
		case <-r.Context().Done():
			return
		case <-time.After(2 * time.Second):
			w.Header().Set("Content-Type", "application/json")
			_, _ = w.Write([]byte(`{"events":[],"result":{"run_id":"x","session_id":"x","status":"completed","final_answer":"","summary":"","final_stage":"Finish"}}`))
		}
	}))
}

func invokeRunAndDecode(t *testing.T, handler *ChatHandler, sessionID string) ChatRunAccepted {
	t.Helper()
	body := `{"session_id":"` + sessionID + `","user_input":"cmd: timeout /t 5"}`
	rec := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/chat/run", strings.NewReader(body))
	handler.Run(rec, req)
	if rec.Code != http.StatusAccepted {
		t.Fatalf("run status=%d body=%s", rec.Code, rec.Body.String())
	}
	var accepted ChatRunAccepted
	if err := json.Unmarshal(rec.Body.Bytes(), &accepted); err != nil {
		t.Fatalf("decode accepted: %v", err)
	}
	return accepted
}

func invokeCancel(t *testing.T, handler *ChatHandler, sessionID string, runID string) {
	t.Helper()
	body := `{"session_id":"` + sessionID + `","run_id":"` + runID + `"}`
	rec := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/chat/cancel", strings.NewReader(body))
	handler.Cancel(rec, req)
	if rec.Code != http.StatusAccepted {
		t.Fatalf("cancel status=%d body=%s", rec.Code, rec.Body.String())
	}
}

func waitRunTerminal(t *testing.T, handler *ChatHandler, sessionID string, runID string) contracts.LogEntry {
	t.Helper()
	for i := 0; i < 60; i++ {
		items := handler.eventBus.RecentBy(30, sessionID, runID)
		for _, item := range items {
			if item.EventType == "run_finished" {
				return item
			}
		}
		time.Sleep(50 * time.Millisecond)
	}
	t.Fatalf("terminal event timeout")
	return contracts.LogEntry{}
}

func runtimePort(rawURL string) int {
	hostPort := strings.TrimPrefix(rawURL, "http://")
	_, portRaw, _ := net.SplitHostPort(hostPort)
	port, _ := net.LookupPort("tcp", portRaw)
	return port
}
