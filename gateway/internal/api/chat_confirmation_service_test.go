package api

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/state"

	"github.com/stretchr/testify/require"
)

func TestApproveConfirmationReturnsConflictWhenAlreadyHandled(t *testing.T) {
	repoRoot := t.TempDir()
	handler := newRetryTestHandler(repoRoot, sampleAppConfig(), state.NewProviderCredentialStore(repoRoot))
	decision := contracts.ConfirmationDecision{
		ConfirmationID: "confirm-missing",
		RunID:          "run-1",
		Decision:       "approve",
	}
	recorder := httptest.NewRecorder()
	handler.approveConfirmation(recorder, decision, confirmationPendingFixture())
	require.Equal(t, http.StatusConflict, recorder.Code)
	require.Contains(t, recorder.Body.String(), "confirmation already handled")
}

func TestPublishConfirmationApprovedWritesPermissionMetadata(t *testing.T) {
	repoRoot := t.TempDir()
	handler := newRetryTestHandler(repoRoot, sampleAppConfig(), state.NewProviderCredentialStore(repoRoot))
	pending := confirmationPendingFixture()
	decision := contracts.ConfirmationDecision{
		ConfirmationID: pending.Confirmation.ConfirmationID,
		RunID:          pending.Request.RunID,
		Decision:       "approve",
		Note:           "looks safe now",
	}
	handler.publishConfirmationApproved(decision, pending)
	events := handler.eventBus.Snapshot("session-1")
	require.Len(t, events, 1)
	require.Equal(t, "confirmation_approved", events[0].EventType)
	require.Equal(t, "proceed", events[0].Metadata["permission_decision"])
	require.Equal(t, "ask_approved", events[0].Metadata["permission_flow_step"])
	require.Equal(t, "high_risk_guard", events[0].Metadata["permission_rule_layer"])
	require.Equal(t, "approved", events[0].Metadata["confirmation_chain_step"])
}
