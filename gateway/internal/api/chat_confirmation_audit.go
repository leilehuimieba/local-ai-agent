package api

import (
	"fmt"

	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/state"
)

func (h *ChatHandler) publishConfirmationApproved(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) {
	h.eventBus.Publish(approvedConfirmationEvent(decision, pending))
}

func approvedConfirmationEvent(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
) contracts.RunEvent {
	summary := "用户已批准高风险动作，任务准备恢复执行。"
	return contracts.RunEvent{
		EventID: newID("event"), Kind: "run_event", Source: "gateway",
		RecordType: "confirmation_result", SourceType: "gateway",
		AgentID: "primary", AgentLabel: "主智能体", EventType: "confirmation_approved",
		TraceID: pending.Request.TraceID, SessionID: pending.Request.SessionID,
		RunID: pending.Request.RunID, Sequence: 97, Timestamp: timestampNow(),
		Stage: "PausedForConfirmation", Summary: summary, Detail: decisionDetail(decision),
		RiskLevel: pending.Confirmation.RiskLevel, ConfirmationID: decision.ConfirmationID,
		ResultSummary: summary, Metadata: approvedMetadata(decision, pending, summary),
	}
}

func approvedMetadata(
	decision contracts.ConfirmationDecision,
	pending state.PendingConfirmation,
	summary string,
) map[string]string {
	metadata := map[string]string{
		"confirmation_id": decision.ConfirmationID,
		"decision":        decision.Decision,
		"decision_note":   decision.Note,
		"result_summary":  summary,
		"kind":            pending.Confirmation.Kind,
		"risk_level":      pending.Confirmation.RiskLevel,
		"task_title":      pending.Confirmation.ActionSummary,
		"record_type":     "confirmation_result",
		"source_type":     "gateway",
		"next_step":       "恢复执行 checkpoint 链路",
	}
	appendPermissionDecisionMetadata(metadata, decision.Decision, pending, "approved")
	return metadata
}

func appendPermissionDecisionMetadata(
	metadata map[string]string,
	decision string,
	pending state.PendingConfirmation,
	chainStep string,
) {
	metadata["checkpoint_id"] = pending.CheckpointID
	metadata["confirmation_resume_strategy"] = "after_confirmation"
	metadata["confirmation_chain_step"] = chainStep
	metadata["confirmation_decision_source"] = "user_confirm_api"
	metadata["permission_decision"] = permissionDecision(decision)
	metadata["permission_flow_step"] = permissionFlowStep(decision)
	metadata["permission_rule_layer"] = permissionRuleLayer(pending.Confirmation.Kind)
}

func permissionDecision(decision string) string {
	if decision == "approve" {
		return "proceed"
	}
	return "blocked"
}

func permissionFlowStep(decision string) string {
	if decision == "approve" {
		return "ask_approved"
	}
	return fmt.Sprintf("ask_%s", decision)
}

func permissionRuleLayer(kind string) string {
	switch kind {
	case "workspace_access":
		return "workspace_guard"
	case "high_risk_action":
		return "high_risk_guard"
	default:
		return "risk_guard"
	}
}

func decisionDetail(decision contracts.ConfirmationDecision) string {
	if decision.Note != "" {
		return fmt.Sprintf("确认已通过，备注：%s", decision.Note)
	}
	return "确认已通过，继续恢复执行。"
}
