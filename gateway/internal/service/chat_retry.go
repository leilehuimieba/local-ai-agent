package service

import (
	"fmt"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/state"
)

func RetryCheckpoint(repoRoot string, runID, sessionID, checkpointID string) (state.RuntimeCheckpointRecord, error) {
	store := state.NewRuntimeCheckpointStore(repoRoot)
	record, err := store.FindRetryable(runID, sessionID, checkpointID)
	if err != nil {
		return state.RuntimeCheckpointRecord{}, err
	}
	return ValidateRetryCheckpoint(sessionID, runID, record)
}

func ValidateRetryCheckpoint(
	sessionID, runID string,
	record state.RuntimeCheckpointRecord,
) (state.RuntimeCheckpointRecord, error) {
	if record.SessionID != sessionID || record.RunID != runID {
		return state.RuntimeCheckpointRecord{}, fmt.Errorf("checkpoint 与当前会话或运行不匹配")
	}
	if !record.Resumable || record.ResumeReason != "retryable_failure" {
		return state.RuntimeCheckpointRecord{}, fmt.Errorf("当前 checkpoint 不支持失败重试")
	}
	return record, nil
}

func ApplyCheckpointResume(request *contracts.RunRequest, checkpointID string) {
	if checkpointID == "" {
		return
	}
	request.ResumeFromCheckpointID = checkpointID
	request.ResumeStrategy = "after_confirmation"
}

func ApplyRetryCheckpointResume(request *contracts.RunRequest, checkpointID string) {
	if checkpointID == "" {
		return
	}
	request.ResumeFromCheckpointID = checkpointID
	request.ResumeStrategy = "retry_failure"
}

func EnsureContextBudgetHints(hints map[string]string) {
	if hints == nil {
		return
	}
	if _, ok := hints["context_budget_tokens"]; !ok {
		hints["context_budget_tokens"] = "512000"
	}
	if _, ok := hints["codex_context_tokens"]; !ok {
		hints["codex_context_tokens"] = hints["context_budget_tokens"]
	}
}
