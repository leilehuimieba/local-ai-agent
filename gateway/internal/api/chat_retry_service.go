package api

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"

	"local-agent/gateway/internal/contracts"
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
	record, err := h.retryCheckpoint(payload)
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
	request.ContextHints = h.withKnowledgeHints(copyContextHints(request.ContextHints))
	applyRetryCheckpointResume(&request, record.CheckpointID)
	return request, nil
}

func (h *ChatHandler) retryCheckpoint(payload ChatRetryRequest) (state.RuntimeCheckpointRecord, error) {
	store := state.NewRuntimeCheckpointStore(h.repoRoot)
	record, err := store.FindRetryable(payload.RunID, payload.SessionID, payload.CheckpointID)
	if err != nil {
		return state.RuntimeCheckpointRecord{}, err
	}
	return validateRetryCheckpoint(payload, record)
}

func validateRetryCheckpoint(
	payload ChatRetryRequest,
	record state.RuntimeCheckpointRecord,
) (state.RuntimeCheckpointRecord, error) {
	if record.SessionID != payload.SessionID || record.RunID != payload.RunID {
		return state.RuntimeCheckpointRecord{}, fmt.Errorf("checkpoint 与当前会话或运行不匹配")
	}
	if !record.Resumable || record.ResumeReason != "retryable_failure" {
		return state.RuntimeCheckpointRecord{}, fmt.Errorf("当前 checkpoint 不支持失败重试")
	}
	return record, nil
}

func applyCheckpointResume(request *contracts.RunRequest, checkpointID string) {
	if checkpointID == "" {
		return
	}
	request.ResumeFromCheckpointID = checkpointID
	request.ResumeStrategy = "after_confirmation"
}

func applyRetryCheckpointResume(request *contracts.RunRequest, checkpointID string) {
	if checkpointID == "" {
		return
	}
	request.ResumeFromCheckpointID = checkpointID
	request.ResumeStrategy = "retry_failure"
}

func writeRetryError(w http.ResponseWriter, err error) {
	if errors.Is(err, state.ErrRuntimeCheckpointNotFound()) {
		http.Error(w, "未找到可重试 checkpoint", http.StatusNotFound)
		return
	}
	http.Error(w, err.Error(), http.StatusBadRequest)
}
