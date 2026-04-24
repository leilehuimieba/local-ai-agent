package service

import (
	"fmt"

	"local-agent/gateway/internal/state"
)

func PendingConfirmation(
	confirmationID, runID string,
	store *state.ConfirmationStore,
) (state.PendingConfirmation, error) {
	pending, ok := store.Get(confirmationID)
	if !ok {
		return state.PendingConfirmation{}, fmt.Errorf("confirmation not found")
	}
	if pending.Request.RunID != runID {
		return state.PendingConfirmation{}, fmt.Errorf("run_id does not match confirmation")
	}
	return pending, nil
}
