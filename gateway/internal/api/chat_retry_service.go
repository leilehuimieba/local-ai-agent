package api

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/service"
	"local-agent/gateway/internal/state"
)

func decodeRetryPayload(r *http.Request) (ChatRetryRequest, error) {
	var payload ChatRetryRequest
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		return ChatRetryRequest{}, fmt.Errorf("invalid json body")
	}
	if payload.SessionID == "" || payload.RunID == "" {
		return ChatRetryRequest{}, fmt.Errorf("session_id and run_id are required")
	}
	return payload, nil
}

func (h *ChatHandler) buildRetryRunRequest(payload ChatRetryRequest) (contracts.RunRequest, error) {
	record, err := service.RetryCheckpoint(h.repoRoot, payload.RunID, payload.SessionID, payload.CheckpointID)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	request := record.Request
	providerRef, err := h.resolveProviderRef(request.ModelRef.ProviderID)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	request.RequestID = newID("request")
	request.TraceID = newID("trace")
	request.SessionID = payload.SessionID
	request.ProviderRef = providerRef
	request.ConfirmationDecision = nil
	request.ContextHints = service.WithKnowledgeHints(copyContextHints(request.ContextHints), h.appConfig.Siyuan)
	service.EnsureContextBudgetHints(request.ContextHints)
	service.ApplyRetryCheckpointResume(&request, record.CheckpointID)
	return request, nil
}

func writeRetryError(w http.ResponseWriter, err error) {
	if errors.Is(err, state.ErrRuntimeCheckpointNotFound()) {
		http.Error(w, "未找到可重试 checkpoint", http.StatusNotFound)
		return
	}
	http.Error(w, err.Error(), http.StatusBadRequest)
}
