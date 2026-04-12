package api

import (
	"net/http/httptest"
	"testing"
)

func TestParseLogsLimitDefault(t *testing.T) {
	limit, err := parseLogsLimit("")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if limit != 120 {
		t.Fatalf("limit=%d want 120", limit)
	}
}

func TestParseLogsLimitRejectsOutOfRange(t *testing.T) {
	_, err := parseLogsLimit("0")
	if err == nil {
		t.Fatalf("expect out-of-range error")
	}
}

func TestDecodeLogsQueryReadsFilterFields(t *testing.T) {
	req := httptest.NewRequest("GET", "/api/v1/logs?session_id=s1&run_id=r1&limit=15", nil)
	query, err := decodeLogsQuery(req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if query.SessionID != "s1" || query.RunID != "r1" || query.Limit != 15 {
		t.Fatalf("unexpected query: %#v", query)
	}
}
