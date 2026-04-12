package api

import "local-agent/gateway/internal/contracts"

func (h *ChatHandler) registerExecution(request contracts.RunRequest, cancelFn func()) {
	h.executionMu.Lock()
	defer h.executionMu.Unlock()
	if h.executions == nil {
		h.executions = make(map[string]*runExecution)
	}
	h.executions[request.RunID] = &runExecution{
		sessionID: request.SessionID,
		cancel:    cancelFn,
	}
}

func (h *ChatHandler) finishExecution(runID string) {
	h.executionMu.Lock()
	defer h.executionMu.Unlock()
	delete(h.executions, runID)
}

func (h *ChatHandler) wasCancelled(runID string) bool {
	h.executionMu.Lock()
	defer h.executionMu.Unlock()
	item, ok := h.executions[runID]
	return ok && item.cancelled
}

func (h *ChatHandler) cancelExecution(sessionID string, runID string) bool {
	h.executionMu.Lock()
	item, ok := h.executions[runID]
	if !ok || item.sessionID != sessionID {
		h.executionMu.Unlock()
		return false
	}
	item.cancelled = true
	cancelFn := item.cancel
	h.executionMu.Unlock()
	cancelFn()
	return true
}
