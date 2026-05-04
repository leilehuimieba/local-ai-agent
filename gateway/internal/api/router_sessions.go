package api

import (
	"encoding/json"
	"net/http"
	"strings"
	"time"

	"local-agent/gateway/internal/session"
)

type SessionHandler struct {
	store *session.SessionStore
}

func NewSessionHandler(repoRoot string) *SessionHandler {
	return &SessionHandler{store: session.NewSessionStore(repoRoot)}
}

func registerSessionRoutes(mux *http.ServeMux, h *SessionHandler) {
	mux.HandleFunc("/api/v1/sessions", h.handleSessions)
	mux.HandleFunc("/api/v1/sessions/", h.handleSessionDetail)
}

func (h *SessionHandler) handleSessions(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		h.listSessions(w, r)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (h *SessionHandler) handleSessionDetail(w http.ResponseWriter, r *http.Request) {
	path := strings.TrimPrefix(r.URL.Path, "/api/v1/sessions/")
	parts := strings.SplitN(path, "/", 2)
	sessionID := parts[0]
	if sessionID == "" {
		http.Error(w, "session id required", http.StatusBadRequest)
		return
	}

	if len(parts) == 1 {
		switch r.Method {
		case http.MethodGet:
			h.getSession(w, r, sessionID)
		case http.MethodDelete:
			h.deleteSession(w, r, sessionID)
		default:
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		}
		return
	}

	switch parts[1] {
	case "messages":
		switch r.Method {
		case http.MethodGet:
			h.getMessages(w, r, sessionID)
		case http.MethodPost:
			h.addMessage(w, r, sessionID)
		default:
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		}
	default:
		http.Error(w, "not found", http.StatusNotFound)
	}
}

func (h *SessionHandler) listSessions(w http.ResponseWriter, r *http.Request, ) {
	sessions, err := h.store.ListSessions()
	if err != nil {
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
		return
	}
	writeJSON(w, http.StatusOK, map[string]any{"items": sessions})
}

func (h *SessionHandler) getSession(w http.ResponseWriter, r *http.Request, sessionID string) {
	sess, err := h.store.GetSession(sessionID)
	if err != nil {
		writeJSON(w, http.StatusNotFound, map[string]string{"error": err.Error()})
		return
	}
	writeJSON(w, http.StatusOK, sess)
}

func (h *SessionHandler) deleteSession(w http.ResponseWriter, r *http.Request, sessionID string) {
	if err := h.store.DeleteSession(sessionID); err != nil {
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
		return
	}
	writeJSON(w, http.StatusOK, map[string]bool{"ok": true})
}

func (h *SessionHandler) getMessages(w http.ResponseWriter, r *http.Request, sessionID string) {
	messages, err := h.store.GetMessages(sessionID)
	if err != nil {
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
		return
	}
	writeJSON(w, http.StatusOK, map[string]any{"items": messages})
}

type addMessageRequest struct {
	Role      string `json:"role"`
	Content   string `json:"content"`
	Timestamp string `json:"timestamp,omitempty"`
	Blocks    []session.MessageBlock `json:"blocks,omitempty"`
}

func (h *SessionHandler) addMessage(w http.ResponseWriter, r *http.Request, sessionID string) {
	var req addMessageRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return
	}
	if req.Role == "" || req.Content == "" {
		http.Error(w, "role and content are required", http.StatusBadRequest)
		return
	}
	if req.Timestamp == "" {
		req.Timestamp = time.Now().Format(time.RFC3339)
	}

	msg, err := h.store.AddMessage(sessionID, req.Role, req.Content, req.Blocks, req.Timestamp)
	if err != nil {
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
		return
	}
	writeJSON(w, http.StatusCreated, msg)
}
