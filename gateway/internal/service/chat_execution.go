package service

import (
	"context"
	"sync"

	"local-agent/gateway/internal/contracts"
)

type RunExecution struct {
	SessionID string
	Cancel    context.CancelFunc
	Cancelled bool
}

type ExecutionRegistry struct {
	mu         sync.Mutex
	executions map[string]*RunExecution
}

func NewExecutionRegistry() *ExecutionRegistry {
	return &ExecutionRegistry{
		executions: make(map[string]*RunExecution),
	}
}

func (r *ExecutionRegistry) Register(request contracts.RunRequest, cancelFn context.CancelFunc) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.executions[request.RunID] = &RunExecution{
		SessionID: request.SessionID,
		Cancel:    cancelFn,
	}
}

func (r *ExecutionRegistry) Finish(runID string) {
	r.mu.Lock()
	defer r.mu.Unlock()
	delete(r.executions, runID)
}

func (r *ExecutionRegistry) WasCancelled(runID string) bool {
	r.mu.Lock()
	defer r.mu.Unlock()
	item, ok := r.executions[runID]
	return ok && item.Cancelled
}

func (r *ExecutionRegistry) Cancel(sessionID string, runID string) bool {
	r.mu.Lock()
	item, ok := r.executions[runID]
	if !ok || item.SessionID != sessionID {
		r.mu.Unlock()
		return false
	}
	item.Cancelled = true
	cancelFn := item.Cancel
	r.mu.Unlock()
	cancelFn()
	return true
}
