package api

import (
	"fmt"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
)

func (h *ChatHandler) buildRunRequest(payload ChatRunRequest) (contracts.RunRequest, error) {
	sessionID, mode, model, workspace, firstSeen, err := h.resolveRunContext(payload)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	providerRef, err := h.resolveProviderRef(model.ProviderID)
	if err != nil {
		return contracts.RunRequest{}, err
	}
	return contracts.RunRequest{
		RequestID:              pickRunIdentity(payload.RequestID, "request"),
		RunID:                  pickRunIdentity(payload.RunID, "run"),
		SessionID:              sessionID,
		TraceID:                pickRunIdentity(payload.TraceID, "trace"),
		UserInput:              payload.UserInput,
		Mode:                   mode,
		ModelRef:               model,
		ProviderRef:            providerRef,
		WorkspaceRef:           workspace,
		ContextHints:           h.withKnowledgeHints(runContextHints(payload.ContextHints, h.repoRoot, firstSeen)),
		ResumeFromCheckpointID: "",
		ResumeStrategy:         "",
	}, nil
}

func pickRunIdentity(source string, prefix string) string {
	if source != "" {
		return source
	}
	return newID(prefix)
}

func (h *ChatHandler) resolveRunContext(payload ChatRunRequest) (string, string, config.ModelRef, config.WorkspaceRef, bool, error) {
	sessionID := payload.SessionID
	if sessionID == "" {
		sessionID = newID("session")
	}
	currentMode, currentModel, _, currentWorkspace, _, directoryPromptEnabled, _, _ := h.settingsStore.Snapshot()
	mode := payload.Mode
	if mode == "" {
		mode = currentMode
	}
	model := payload.Model
	if model.ModelID == "" {
		model = currentModel
	}
	workspace, err := h.resolveWorkspace(payload.Workspace, currentWorkspace)
	if err != nil {
		return "", "", config.ModelRef{}, config.WorkspaceRef{}, false, err
	}
	firstSeen := directoryPromptEnabled && !h.settingsStore.IsWorkspaceApproved(workspace.WorkspaceID)
	return sessionID, mode, model, workspace, firstSeen, nil
}

func (h *ChatHandler) resolveWorkspace(input config.WorkspaceRef, fallback config.WorkspaceRef) (config.WorkspaceRef, error) {
	if input.WorkspaceID == "" {
		return fallback, nil
	}
	workspace, ok := h.settingsStore.WorkspaceByID(input.WorkspaceID)
	if !ok {
		return config.WorkspaceRef{}, fmt.Errorf("workspace not found")
	}
	return workspace, nil
}

func runContextHints(source map[string]string, repoRoot string, firstSeen bool) map[string]string {
	hints := make(map[string]string)
	for key, value := range source {
		hints[key] = value
	}
	hints["repo_root"] = repoRoot
	hints["workspace_first_seen"] = fmt.Sprintf("%t", firstSeen)
	if _, ok := hints["context_budget_tokens"]; !ok {
		hints["context_budget_tokens"] = "512000"
	}
	if _, ok := hints["codex_context_tokens"]; !ok {
		hints["codex_context_tokens"] = hints["context_budget_tokens"]
	}
	return hints
}

func (h *ChatHandler) withKnowledgeHints(hints map[string]string) map[string]string {
	hints["siyuan_root"] = h.appConfig.Siyuan.RootDir
	hints["siyuan_export_dir"] = h.appConfig.Siyuan.ExportDir
	hints["siyuan_auto_write_enabled"] = fmt.Sprintf("%t", h.appConfig.Siyuan.AutoWriteEnabled)
	hints["siyuan_sync_enabled"] = fmt.Sprintf("%t", h.appConfig.Siyuan.SyncEnabled)
	return hints
}
