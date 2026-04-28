package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"local-agent/gateway/internal/contracts"
)

func validateStreamRequest(w http.ResponseWriter, r *http.Request) (string, http.Flusher, bool) {
	sessionID := r.URL.Query().Get("session_id")
	if sessionID == "" {
		http.Error(w, "session_id is required", http.StatusBadRequest)
		return "", nil, false
	}
	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "streaming unsupported", http.StatusInternalServerError)
		return "", nil, false
	}
	return sessionID, flusher, true
}

func setStreamHeaders(w http.ResponseWriter) {
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")
}

func (h *ChatHandler) flushSnapshot(sessionID string, w http.ResponseWriter, flusher http.Flusher) {
	for _, item := range h.eventBus.Snapshot(sessionID) {
		writeSSE(w, item)
	}
	flusher.Flush()
}

func streamSessionEvents(ctx context.Context, w http.ResponseWriter, flusher http.Flusher, stream <-chan contracts.RunEvent) {
	heartbeat := time.NewTicker(15 * time.Second)
	defer heartbeat.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case item, ok := <-stream:
			if !ok {
				return
			}
			writeSSE(w, item)
			flusher.Flush()
		case <-heartbeat.C:
			_, _ = fmt.Fprint(w, ": keep-alive\n\n")
			flusher.Flush()
		}
	}
}

func writeSSE(w http.ResponseWriter, item contracts.RunEvent) {
	payload, _ := json.Marshal(item)
	_, _ = fmt.Fprintf(w, "event: run_event\n")
	_, _ = fmt.Fprintf(w, "data: %s\n\n", payload)
}
