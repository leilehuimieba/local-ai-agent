package api

import (
	"local-agent/gateway/internal/contracts"
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
