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
	req := httptest.NewRequest("GET", "/api/v1/logs?session_id=s1&run_id=r1&limit=15&view=runs", nil)
	query, err := decodeLogsQuery(req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if query.SessionID != "s1" || query.RunID != "r1" || query.Limit != 15 || query.View != "runs" {
		t.Fatalf("unexpected query: %#v", query)
	}
}

func TestParseLogsViewDefaultAndReject(t *testing.T) {
	view, err := parseLogsView("")
	if err != nil || view != "events" {
		t.Fatalf("default view=%q err=%v", view, err)
	}
	_, err = parseLogsView("invalid")
	if err == nil {
		t.Fatalf("expect invalid view error")
	}
}
