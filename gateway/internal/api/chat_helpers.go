package api

import (
	"encoding/json"
	"fmt"
	"net/http"
	"sync/atomic"
	"time"

	"local-agent/gateway/internal/contracts"
)

var idCounter uint64

const (
	chatEntryID         = "gateway.chat.entry1"
	chatProtocolVersion = "v1"
)

func decodeRunPayload(r *http.Request) (ChatRunRequest, error) {
	var payload ChatRunRequest
	err := json.NewDecoder(r.Body).Decode(&payload)
	return payload, err
}

func copyContextHints(source map[string]string) map[string]string {
	if source == nil {
		return map[string]string{}
	}
	target := make(map[string]string, len(source))
	for key, value := range source {
		target[key] = value
	}
	return target
}

func newID(prefix string) string {
	counter := atomic.AddUint64(&idCounter, 1)
	return fmt.Sprintf("%s-%d-%d", prefix, time.Now().UnixMilli(), counter)
}

func timestampNow() string {
	return fmt.Sprintf("%d", time.Now().UnixMilli())
}

func newChatRunAccepted(runRequest contracts.RunRequest) ChatRunAccepted {
	return ChatRunAccepted{
		Accepted:        true,
		SessionID:       runRequest.SessionID,
		RunID:           runRequest.RunID,
		RequestID:       runRequest.RequestID,
		TraceID:         runRequest.TraceID,
		InitialStatus:   "accepted",
		EntryID:         chatEntryID,
		ProtocolVersion: chatProtocolVersion,
		StreamEndpoint:  "/api/v1/events/stream?session_id={session_id}",
		LogsEndpoint:    "/api/v1/logs?session_id={session_id}&run_id={run_id}",
		ConfirmEndpoint: "/api/v1/chat/confirm",
		RetryEndpoint:   "/api/v1/chat/retry",
		CancelEndpoint:  "/api/v1/chat/cancel",
	}
}
