package api

import (
	"testing"

	"local-agent/gateway/internal/contracts"
)

func TestNewChatRunAcceptedContainsProtocolFields(t *testing.T) {
	request := contracts.RunRequest{
		RequestID: "req-1",
		RunID:     "run-1",
		SessionID: "session-1",
		TraceID:   "trace-1",
	}
	accepted := newChatRunAccepted(request)
	if !accepted.Accepted || accepted.InitialStatus != "accepted" {
		t.Fatalf("unexpected accepted payload: %#v", accepted)
	}
	if accepted.EntryID != chatEntryID || accepted.ProtocolVersion != chatProtocolVersion {
		t.Fatalf("protocol field mismatch: %#v", accepted)
	}
	if accepted.LogsEndpoint == "" || accepted.StreamEndpoint == "" {
		t.Fatalf("missing endpoint hints: %#v", accepted)
	}
}
