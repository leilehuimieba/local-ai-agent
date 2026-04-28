package api

import (
	"database/sql"
	"encoding/json"
	"net/http"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/memory"
	"local-agent/gateway/internal/state"
)

func registerMemoryRoutes(mux *http.ServeMux, deps memoryRouteDeps) {
	mux.HandleFunc("/api/v1/memories", deps.handleMemories)
	mux.HandleFunc("/api/v1/memories/delete", deps.handleMemoryDelete)
}

func (deps memoryRouteDeps) handleMemories(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	workspaceID, ok := currentWorkspaceID(deps.state)
	if !ok {
		http.Error(w, "workspace not found", http.StatusNotFound)
		return
	}
	writeMemoryList(w, deps, workspaceID)
}

func (deps memoryRouteDeps) handleMemoryDelete(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	payload, err := decodeMemoryDeletePayload(r)
	if err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return
	}
	workspaceID, ok := currentWorkspaceID(deps.state)
	if !ok {
		http.Error(w, "workspace not found", http.StatusNotFound)
		return
	}
	if err := deps.store.Delete(workspaceID, payload.MemoryID); err != nil {
		writeMemoryDeleteError(w, err)
		return
	}
	writeMemoryList(w, deps, workspaceID)
}

func decodeMemoryDeletePayload(r *http.Request) (struct {
	MemoryID string `json:"memory_id"`
}, error) {
	var payload struct {
		MemoryID string `json:"memory_id"`
	}
	err := json.NewDecoder(r.Body).Decode(&payload)
	return payload, err
}

func writeMemoryDeleteError(w http.ResponseWriter, err error) {
	if err == sql.ErrNoRows {
		http.Error(w, "memory not found", http.StatusNotFound)
		return
	}
	http.Error(w, err.Error(), http.StatusInternalServerError)
}

func writeMemoryList(w http.ResponseWriter, deps memoryRouteDeps, workspaceID string) {
	items, err := deps.store.List(workspaceID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	writeJSON(w, http.StatusOK, contracts.MemoryListResponse{Items: toContractMemories(items)})
}

func currentWorkspaceID(store *state.SettingsStore) (string, bool) {
	_, _, _, workspace, _, _, _, _ := store.Snapshot()
	if workspace.WorkspaceID == "" {
		return "", false
	}
	return workspace.WorkspaceID, true
}

func toContractMemories(items []memory.Entry) []contracts.MemoryEntry {
	result := make([]contracts.MemoryEntry, 0, len(items))
	for _, item := range items {
		result = append(result, contracts.MemoryEntry{
			ID:                 item.ID,
			Kind:               item.Kind,
			Title:              item.Title,
			Summary:            item.Summary,
			Content:            item.Content,
			Reason:             item.Reason,
			Scope:              item.Scope,
			WorkspaceID:        item.WorkspaceID,
			SessionID:          item.SessionID,
			SourceRunID:        item.SourceRunID,
			Source:             item.Source,
			SourceType:         item.SourceType,
			SourceTitle:        item.SourceTitle,
			SourceEventType:    item.SourceEventType,
			SourceArtifactPath: item.SourceArtifactPath,
			Verified:           item.Verified,
			Priority:           item.Priority,
			Archived:           item.Archived,
			ArchivedAt:         item.ArchivedAt,
			CreatedAt:          item.CreatedAt,
			UpdatedAt:          item.UpdatedAt,
			Timestamp:          item.Timestamp,
		})
	}
	return result
}
