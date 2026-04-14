package api

import (
	"fmt"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/memory"
	"local-agent/gateway/internal/state"
)

func (h *ChatHandler) publishConfirmationClosure(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) {
	h.eventBus.Publish(h.confirmationMemoryEvent(decision, pending))
	h.eventBus.Publish(rejectedConfirmationEvent(decision, pending))
}

func (h *ChatHandler) confirmationMemoryEvent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) contracts.RunEvent {
	entry, ok, reason := confirmationMemoryEntry(decision, pending)
	if !ok {
		return skippedConfirmationMemoryEvent(decision, pending, reason)
	}
	written, err := h.memoryStore.Save(entry)
	if err != nil {
		return skippedConfirmationMemoryEvent(decision, pending, err.Error())
	}
	if !written {
		return skippedConfirmationMemoryEvent(decision, pending, "命中重复风险确认治理记录，跳过写入。")
	}
	return writtenConfirmationMemoryEvent(decision, pending, entry)
}

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

func rejectionSummary(decision string) string {
	if decision == "reject" {
		return "用户拒绝了本次高风险动作，任务按确认结果结束。"
	}
	return "用户取消了本次高风险动作确认，任务按确认结果结束。"
}

func rejectionMetadata(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
	summary string,
) map[string]string {
	metadata := map[string]string{
		"confirmation_id":   decision.ConfirmationID,
		"decision":          decision.Decision,
		"decision_note":     decision.Note,
		"completion_status": rejectionStatus(decision.Decision),
		"completion_reason": rejectionReason(decision.Decision),
		"result_summary":    summary,
		"final_answer":      summary,
		"kind":              pending.Confirmation.Kind,
		"risk_level":        pending.Confirmation.RiskLevel,
		"task_title":        pending.Confirmation.ActionSummary,
		"record_type":       "confirmation_result",
		"source_type":       "gateway",
		"next_step":         "任务已结束",
	}
	appendPermissionDecisionMetadata(metadata, decision.Decision, pending, "closed")
	for key, value := range confirmationAuditMetadata(pending) {
		metadata[key] = value
	}
	return metadata
}

func rejectionStatus(decision string) string {
	if decision == "reject" {
		return "rejected"
	}
	return "cancelled"
}

func rejectionReason(decision string) string {
	if decision == "reject" {
		return "用户明确拒绝了当前高风险动作。"
	}
	return "用户取消了当前高风险动作确认。"
}

func confirmationMemoryEntry(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) (memory.Entry, bool, string) {
	if pending.Confirmation.Kind != "high_risk_action" {
		return memory.Entry{}, false, "当前确认类型仅记录日志，不沉淀长期记忆。"
	}
	now := timestampNow()
	return memory.Entry{
		ID: confirmationMemoryID(decision.Decision), Kind: confirmationMemoryKind(decision.Decision),
		Title:   confirmationMemoryTitle(decision.Decision),
		Summary: confirmationMemoryTitle(decision.Decision),
		Content: confirmationMemoryContent(decision, pending), Scope: pending.Request.WorkspaceRef.Name,
		WorkspaceID: pending.Request.WorkspaceRef.WorkspaceID, SessionID: pending.Request.SessionID,
		SourceRunID: pending.Request.RunID, Source: "gateway_confirmation", SourceType: "gateway",
		SourceTitle:        pending.Confirmation.ActionSummary,
		SourceEventType:    confirmationEventType(decision.Decision),
		SourceArtifactPath: firstTargetPath(pending), GovernanceVersion: memory.MemoryGovernanceVersion,
		GovernanceReason: confirmationGovernanceReason(decision.Decision),
		GovernanceSource: confirmationGovernanceSource(decision.Decision), GovernanceAt: now,
		Verified: true, Priority: confirmationPriority(decision.Decision), Archived: false,
		ArchivedAt: "", CreatedAt: now, UpdatedAt: now, Timestamp: now,
	}, true, ""
}

func confirmationMemoryID(decision string) string {
	return fmt.Sprintf("memory-confirmation-%s-%s", decision, timestampNow())
}

func confirmationMemoryKind(decision string) string {
	if decision == "reject" {
		return "lesson_learned"
	}
	return "workflow_pattern"
}

func confirmationMemoryTitle(decision string) string {
	if decision == "reject" {
		return "失败教训：高风险动作被用户拒绝时应先缩小范围并提供更安全替代。"
	}
	return "流程模式：高风险动作在信息不足时应先取消，并补充范围说明后再继续。"
}

func confirmationMemoryContent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) string {
	return fmt.Sprintf(
		"decision=%s; note=%s; action=%s; risk_level=%s; reason=%s; impact_scope=%s; target_paths=%s; alternatives=%s",
		decision.Decision,
		decision.Note,
		pending.Confirmation.ActionSummary,
		pending.Confirmation.RiskLevel,
		pending.Confirmation.Reason,
		pending.Confirmation.ImpactScope,
		joinValues(pending.Confirmation.TargetPaths),
		joinValues(pending.Confirmation.Alternatives),
	)
}

func firstTargetPath(pending state.PendingConfirmation) string {
	if len(pending.Confirmation.TargetPaths) == 0 {
		return ""
	}
	return pending.Confirmation.TargetPaths[0]
}

func joinValues(values []string) string {
	if len(values) == 0 {
		return ""
	}
	return fmt.Sprintf("%v", values)
}

func confirmationGovernanceReason(decision string) string {
	if decision == "reject" {
		return "用户拒绝高风险动作后，已沉淀为正式失败教训。"
	}
	return "用户取消高风险动作确认后，已沉淀为可复用流程模式。"
}

func confirmationGovernanceSource(decision string) string {
	if decision == "reject" {
		return "gateway_confirmation_reject"
	}
	return "gateway_confirmation_cancel"
}

func confirmationEventType(decision string) string {
	if decision == "reject" {
		return "confirmation_rejected"
	}
	return "confirmation_cancelled"
}

func confirmationPriority(decision string) int {
	if decision == "reject" {
		return 65
	}
	return 55
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

func confirmationMemoryMetadata(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
	entry memory.Entry,
	layer string,
) map[string]string {
	metadata := map[string]string{
		"layer": layer, "record_type": entry.Kind, "source_type": "gateway",
		"memory_kind": entry.Kind, "reason": entry.GovernanceReason,
		"decision": decision.Decision, "decision_note": decision.Note,
		"confirmation_id": decision.ConfirmationID, "kind": pending.Confirmation.Kind,
		"risk_level": pending.Confirmation.RiskLevel, "task_title": pending.Confirmation.ActionSummary,
		"artifact_path": firstTargetPath(pending), "next_step": "任务已结束",
		"governance_status":    confirmationGovernanceStatus(entry),
		"memory_action":        confirmationMemoryAction(entry),
		"governance_version":   entry.GovernanceVersion,
		"governance_reason":    entry.GovernanceReason,
		"governance_source":    entry.GovernanceSource,
		"governance_at":        entry.GovernanceAt,
		"source_event_type":    entry.SourceEventType,
		"source_artifact_path": entry.SourceArtifactPath,
		"archive_reason":       entry.ArchiveReason,
	}
	appendPermissionDecisionMetadata(metadata, decision.Decision, pending, "closed")
	for key, value := range confirmationAuditMetadata(pending) {
		metadata[key] = value
	}
	return metadata
}

func confirmationAuditMetadata(pending state.PendingConfirmation) map[string]string {
	return map[string]string{
		"checkpoint_id":                pending.CheckpointID,
		"confirmation_resume_strategy": "after_confirmation",
		"confirmation_chain_step":      "closed",
		"confirmation_decision_source": "user_confirm_api",
	}
}

func confirmationGovernanceStatus(entry memory.Entry) string {
	if entry.Archived {
		return "archived"
	}
	if entry.SourceEventType == "memory_write_skipped" {
		return "skipped"
	}
	return "written"
}

func confirmationMemoryAction(entry memory.Entry) string {
	if entry.Archived {
		return "archive"
	}
	if entry.SourceEventType == "memory_write_skipped" {
		return "skip"
	}
	return "write"
}
