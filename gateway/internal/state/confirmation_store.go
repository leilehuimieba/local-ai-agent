package state

import (
	"sync"

	"local-agent/gateway/internal/contracts"
)

type PendingConfirmation struct {
	Request      contracts.RunRequest
	Confirmation contracts.ConfirmationRequest
	CheckpointID string
}

type ConfirmationStore struct {
	mu      sync.RWMutex
	pending map[string]PendingConfirmation
}

func NewConfirmationStore() *ConfirmationStore {
	return &ConfirmationStore{
		pending: make(map[string]PendingConfirmation),
	}
}

func (s *ConfirmationStore) Save(item PendingConfirmation) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.pending[item.Confirmation.ConfirmationID] = item
}

func (s *ConfirmationStore) Get(confirmationID string) (PendingConfirmation, bool) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	item, ok := s.pending[confirmationID]
	return item, ok
}

func (s *ConfirmationStore) Delete(confirmationID string) {
	s.mu.Lock()
	defer s.mu.Unlock()
	delete(s.pending, confirmationID)
}

func (s *ConfirmationStore) Take(confirmationID string) (PendingConfirmation, bool) {
	s.mu.Lock()
	defer s.mu.Unlock()
	item, ok := s.pending[confirmationID]
	delete(s.pending, confirmationID)
	return item, ok
}
