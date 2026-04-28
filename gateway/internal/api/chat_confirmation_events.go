package api

import (
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/memory"
	"local-agent/gateway/internal/state"
)

func rejectedConfirmationEvent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) contracts.RunEvent {
	summary := rejectionSummary(decision.Decision)
	return contracts.RunEvent{
		EventID:          newID("event"),
		Kind:             "run_event",
		Source:           "gateway",
		AgentID:          "primary",
		AgentLabel:       "主智能体",
		EventType:        "run_finished",
		TraceID:          pending.Request.TraceID,
		SessionID:        pending.Request.SessionID,
		RunID:            pending.Request.RunID,
		Sequence:         99,
		Timestamp:        timestampNow(),
		Stage:            "Finish",
		Summary:          summary,
		Detail:           summary,
		RiskLevel:        pending.Confirmation.RiskLevel,
		ConfirmationID:   decision.ConfirmationID,
		CompletionStatus: rejectionStatus(decision.Decision),
		CompletionReason: rejectionReason(decision.Decision),
		Metadata:         rejectionMetadata(decision, pending, summary),
	}
}

func skippedConfirmationMemoryEvent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
	reason string,
) contracts.RunEvent {
	return contracts.RunEvent{
		EventID: newID("event"), Kind: "run_event", Source: "gateway",
		RecordType: "confirmation_result", SourceType: "gateway",
		AgentID: "primary", AgentLabel: "主智能体", EventType: "memory_write_skipped",
		TraceID: pending.Request.TraceID, SessionID: pending.Request.SessionID,
		RunID: pending.Request.RunID, Sequence: 98, Timestamp: timestampNow(), Stage: "Finish",
		Summary: "跳过写入", Detail: reason, RiskLevel: pending.Confirmation.RiskLevel,
		ConfirmationID: decision.ConfirmationID, ArtifactPath: firstTargetPath(pending),
		Metadata: confirmationMemoryMetadata(decision, pending, memory.Entry{
			Kind: "confirmation_result", SourceType: "gateway", GovernanceVersion: memory.MemoryGovernanceVersion,
			GovernanceReason: reason, GovernanceSource: "gateway_confirmation_skip", GovernanceAt: timestampNow(),
			SourceEventType: "memory_write_skipped", SourceArtifactPath: firstTargetPath(pending),
		}, "long_term_memory"),
	}
}

func writtenConfirmationMemoryEvent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
	entry memory.Entry,
) contracts.RunEvent {
	return contracts.RunEvent{
		EventID: newID("event"), Kind: "run_event", Source: "gateway",
		RecordType: entry.Kind, SourceType: entry.SourceType, AgentID: "primary",
		AgentLabel: "主智能体", EventType: "memory_written", TraceID: pending.Request.TraceID,
		SessionID: pending.Request.SessionID, RunID: pending.Request.RunID, Sequence: 98,
		Timestamp: timestampNow(), Stage: "Finish", Summary: entry.Title,
		Detail: "风险确认治理结果已写入长期记忆。", ResultSummary: entry.Summary,
		ArtifactPath: entry.SourceArtifactPath, RiskLevel: pending.Confirmation.RiskLevel,
		ConfirmationID: decision.ConfirmationID,
		Metadata:       confirmationMemoryMetadata(decision, pending, entry, "long_term_memory"),
	}
}
