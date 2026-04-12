package api

import (
	"fmt"
	"net/http"
	"strconv"
)

type logsQuery struct {
	Limit     int
	SessionID string
	RunID     string
}

func decodeLogsQuery(r *http.Request) (logsQuery, error) {
	limit, err := parseLogsLimit(r.URL.Query().Get("limit"))
	if err != nil {
		return logsQuery{}, err
	}
	return logsQuery{
		Limit:     limit,
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
