package api

import (
	"encoding/json"
	"fmt"
	"net/http"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/service"
	"local-agent/gateway/internal/state"
)

func decodeConfirmationDecision(r *http.Request) (contracts.ConfirmationDecision, error) {
	var decision contracts.ConfirmationDecision
	if err := json.NewDecoder(r.Body).Decode(&decision); err != nil {
		return contracts.ConfirmationDecision{}, fmt.Errorf("invalid json body")
	}
	if decision.ConfirmationID == "" || decision.RunID == "" {
		return contracts.ConfirmationDecision{}, fmt.Errorf("confirmation_id and run_id are required")
	}
	switch decision.Decision {
	case "approve", "reject", "cancel":
		return decision, nil
	default:
		return contracts.ConfirmationDecision{}, fmt.Errorf("invalid decision")
	}
}

func (h *ChatHandler) pendingConfirmation(
	decision contracts.ConfirmationDecision,
) (state.PendingConfirmation, int, error) {
	pending, err := service.PendingConfirmation(decision.ConfirmationID, decision.RunID, h.confirmationStore)
	if err != nil {
		if err.Error() == "confirmation not found" {
			return state.PendingConfirmation{}, http.StatusNotFound, err
		}
		return state.PendingConfirmation{}, http.StatusBadRequest, err
	}
	return pending, http.StatusOK, nil
}

func (h *ChatHandler) approveConfirmation(
	w http.ResponseWriter,
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) {
	taken, ok := h.confirmationStore.Take(decision.ConfirmationID)
	if !ok {
		http.Error(w, "confirmation already handled", http.StatusConflict)
		return
	}
	pending = taken
	h.publishConfirmationApproved(decision, pending)
	if pending.Confirmation.Kind == "workspace_access" && decision.Remember {
		h.settingsStore.ApproveWorkspace(pending.Request.WorkspaceRef.WorkspaceID)
	}
	if pending.Request.ContextHints == nil {
		pending.Request.ContextHints = make(map[string]string)
	}
	pending.Request.ContextHints["workspace_first_seen"] = "false"
	pending.Request.ConfirmationDecision = &decision
	service.ApplyCheckpointResume(&pending.Request, pending.CheckpointID)
	go service.Execute(pending.Request, h.runtimeClient, h.eventBus, h.confirmationStore, h.executionRegistry)
	writeConfirmationResponse(w, http.StatusAccepted, decision)
}

func (h *ChatHandler) closeConfirmation(
	w http.ResponseWriter,
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) {
	if taken, ok := h.confirmationStore.Take(decision.ConfirmationID); ok {
		pending = taken
	}
	h.publishConfirmationClosure(decision, pending)
	writeConfirmationResponse(w, http.StatusOK, decision)
}

func writeConfirmationResponse(
	w http.ResponseWriter,
	status int,
	decision contracts.ConfirmationDecision,
) {
	writeJSON(w, status, map[string]any{
		"accepted":        true,
		"confirmation_id": decision.ConfirmationID,
		"decision":        decision.Decision,
	})
}
