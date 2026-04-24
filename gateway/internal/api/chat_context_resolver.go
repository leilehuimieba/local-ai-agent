package api

import (
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/service"
)

func (h *ChatHandler) buildRunRequest(payload ChatRunRequest) (contracts.RunRequest, error) {
	sessionID, mode, model, workspace, firstSeen, err := service.ResolveRunContext(payload.SessionID, payload.Mode, payload.Model, payload.Workspace, h.settingsStore)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	providerRef, err := h.resolveProviderRef(model.ProviderID)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	hints := service.WithKnowledgeHints(service.RunContextHints(payload.ContextHints, h.repoRoot, firstSeen), h.appConfig.Siyuan)
	if payload.KnowledgeBaseID != "" && payload.KnowledgeBaseID != "_none_" {
		hints["knowledge_base_id"] = payload.KnowledgeBaseID
	}
	return contracts.RunRequest{
		RequestID:              service.PickRunIdentity(payload.RequestID, "request"),
		RunID:                  service.PickRunIdentity(payload.RunID, "run"),
		SessionID:              sessionID,
		TraceID:                service.PickRunIdentity(payload.TraceID, "trace"),
		UserInput:              payload.UserInput,
		Mode:                   mode,
		ModelRef:               model,
		ProviderRef:            providerRef,
		WorkspaceRef:           workspace,
		ContextHints:           hints,
		ResumeFromCheckpointID: "",
		ResumeStrategy:         "",
	}, nil
}

func pickRunIdentity(source string, prefix string) string {
	return service.PickRunIdentity(source, prefix)
}

func runContextHints(source map[string]string, repoRoot string, firstSeen bool) map[string]string {
	return service.RunContextHints(source, repoRoot, firstSeen)
}
