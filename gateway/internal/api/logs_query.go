package api

import (
	"fmt"
	"net/http"
	"strconv"
)

type logsQuery struct {
	Limit     int
	View      string
	SessionID string
	RunID     string
}

func decodeLogsQuery(r *http.Request) (logsQuery, error) {
	limit, err := parseLogsLimit(r.URL.Query().Get("limit"))
	if err != nil {
		return logsQuery{}, err
	}
	view, err := parseLogsView(r.URL.Query().Get("view"))
	if err != nil {
		return logsQuery{}, err
	}
	return logsQuery{
		Limit:     limit,
		View:      view,
		SessionID: r.URL.Query().Get("session_id"),
		RunID:     r.URL.Query().Get("run_id"),
	}, nil
}

func parseLogsLimit(raw string) (int, error) {
	if raw == "" {
		return 120, nil
	}
	value, err := strconv.Atoi(raw)
	if err != nil {
		return 0, fmt.Errorf("limit must be integer")
	}
	if value < 1 || value > 500 {
		return 0, fmt.Errorf("limit must be in [1,500]")
	}
	return value, nil
}

func parseLogsView(raw string) (string, error) {
	if raw == "" {
		return "events", nil
	}
	if raw == "events" || raw == "runs" {
		return raw, nil
	}
	return "", fmt.Errorf("view must be one of events,runs")
}
