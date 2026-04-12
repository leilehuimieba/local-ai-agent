package api

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type ChatCancelRequest struct {
	SessionID string `json:"session_id"`
	RunID     string `json:"run_id"`
}

func (h *ChatHandler) Cancel(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	payload, err := decodeCancelPayload(r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	if !h.cancelExecution(payload.SessionID, payload.RunID) {
		http.Error(w, "run not running", http.StatusNotFound)
		return
	}
	writeJSON(w, http.StatusAccepted, map[string]any{
		"accepted": true, "session_id": payload.SessionID, "run_id": payload.RunID, "status": "cancelling",
	})
}

func decodeCancelPayload(r *http.Request) (ChatCancelRequest, error) {
	var payload ChatCancelRequest
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		return ChatCancelRequest{}, fmt.Errorf("invalid json body")
	}
	if payload.SessionID == "" || payload.RunID == "" {
		return ChatCancelRequest{}, fmt.Errorf("session_id and run_id are required")
	}
	return payload, nil
}
