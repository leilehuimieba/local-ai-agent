package api

import (
	"testing"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/state"

	"github.com/stretchr/testify/require"
)

func TestPublishConfirmationClosureRejectWritesRunFinishedEvent(t *testing.T) {
	repoRoot := t.TempDir()
	handler := newRetryTestHandler(repoRoot, sampleAppConfig(), state.NewProviderCredentialStore(repoRoot))
	decision := contracts.ConfirmationDecision{
		ConfirmationID: "confirm-1",
		RunID:          "run-1",
		Decision:       "reject",
		Note:           "too risky",
	}
	handler.publishConfirmationClosure(decision, confirmationPendingFixture())
	events := handler.eventBus.Snapshot("session-1")
	require.Len(t, events, 2)
	require.Equal(t, "run_finished", events[1].EventType)
	require.Equal(t, "rejected", events[1].CompletionStatus)
	require.Equal(t, "confirm-1", events[1].ConfirmationID)
	require.Equal(t, "closed", events[1].Metadata["confirmation_chain_step"])
	require.Equal(t, "after_confirmation", events[1].Metadata["confirmation_resume_strategy"])
	require.Equal(t, "blocked", events[1].Metadata["permission_decision"])
	require.Equal(t, "ask_reject", events[1].Metadata["permission_flow_step"])
	require.Equal(t, "high_risk_guard", events[1].Metadata["permission_rule_layer"])
	require.Equal(t, "user_confirm_api", events[1].Metadata["confirmation_decision_source"])
	require.Equal(t, "cp-1", events[1].Metadata["checkpoint_id"])
}

func TestPublishConfirmationClosureCancelWritesRunFinishedEvent(t *testing.T) {
	repoRoot := t.TempDir()
	handler := newRetryTestHandler(repoRoot, sampleAppConfig(), state.NewProviderCredentialStore(repoRoot))
	decision := contracts.ConfirmationDecision{
		ConfirmationID: "confirm-2",
		RunID:          "run-1",
		Decision:       "cancel",
		Note:           "need more info",
	}
	handler.publishConfirmationClosure(decision, confirmationPendingFixture())
	events := handler.eventBus.Snapshot("session-1")
	require.Len(t, events, 2)
	require.Equal(t, "run_finished", events[1].EventType)
	require.Equal(t, "cancelled", events[1].CompletionStatus)
	require.Equal(t, "confirm-2", events[1].ConfirmationID)
	require.Equal(t, "closed", events[1].Metadata["confirmation_chain_step"])
	require.Equal(t, "after_confirmation", events[1].Metadata["confirmation_resume_strategy"])
	require.Equal(t, "blocked", events[1].Metadata["permission_decision"])
	require.Equal(t, "ask_cancel", events[1].Metadata["permission_flow_step"])
	require.Equal(t, "high_risk_guard", events[1].Metadata["permission_rule_layer"])
	require.Equal(t, "user_confirm_api", events[1].Metadata["confirmation_decision_source"])
	require.Equal(t, "cp-1", events[1].Metadata["checkpoint_id"])
}

func confirmationPendingFixture() state.PendingConfirmation {
	return state.PendingConfirmation{
		Request: contracts.RunRequest{
			RunID:        "run-1",
			SessionID:    "session-1",
			TraceID:      "trace-1",
			WorkspaceRef: sampleWorkspace(),
		},
		Confirmation: contracts.ConfirmationRequest{
			ConfirmationID: "confirm-1",
			RunID:          "run-1",
			RiskLevel:      "high",
			ActionSummary:  "删除历史数据",
			Reason:         "高风险变更",
			ImpactScope:    "workspace",
			TargetPaths:    []string{"/workspace/data"},
			Alternatives:   []string{"先备份再处理"},
			Kind:           "high_risk_action",
		},
		CheckpointID: "cp-1",
	}
}
