package service

import (
	"fmt"
	"sync/atomic"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/state"
)

func ResolveRunContext(
	sessionID, mode string,
	model config.ModelRef,
	workspaceInput config.WorkspaceRef,
	settingsStore *state.SettingsStore,
) (string, string, config.ModelRef, config.WorkspaceRef, bool, error) {
	if sessionID == "" {
		sessionID = NewID("session")
	}
	currentMode, currentModel, _, currentWorkspace, _, directoryPromptEnabled, _, _ := settingsStore.Snapshot()
	if mode == "" {
		mode = currentMode
	}
	if model.ModelID == "" {
		model = currentModel
	}
	workspace, err := ResolveWorkspace(workspaceInput, currentWorkspace, settingsStore)
	if err != nil {
		return "", "", config.ModelRef{}, config.WorkspaceRef{}, false, err
	}
	firstSeen := directoryPromptEnabled && !settingsStore.IsWorkspaceApproved(workspace.WorkspaceID)
	return sessionID, mode, model, workspace, firstSeen, nil
}

func ResolveWorkspace(
	input config.WorkspaceRef,
	fallback config.WorkspaceRef,
	settingsStore *state.SettingsStore,
) (config.WorkspaceRef, error) {
	if input.WorkspaceID == "" {
		return fallback, nil
	}
	workspace, ok := settingsStore.WorkspaceByID(input.WorkspaceID)
	if !ok {
		return config.WorkspaceRef{}, fmt.Errorf("workspace not found")
	}
	return workspace, nil
}

func RunContextHints(source map[string]string, repoRoot string, firstSeen bool) map[string]string {
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

func WithKnowledgeHints(hints map[string]string, cfg config.SiyuanConfig) map[string]string {
	hints["siyuan_root"] = cfg.RootDir
	hints["siyuan_export_dir"] = cfg.ExportDir
	hints["siyuan_auto_write_enabled"] = fmt.Sprintf("%t", cfg.AutoWriteEnabled)
	hints["siyuan_sync_enabled"] = fmt.Sprintf("%t", cfg.SyncEnabled)
	return hints
}

func PickRunIdentity(source string, prefix string) string {
	if source != "" {
		return source
	}
	return NewID(prefix)
}

var idCounter uint64

func NewID(prefix string) string {
	counter := atomic.AddUint64(&idCounter, 1)
	return fmt.Sprintf("%s-%d-%d", prefix, time.Now().UnixMilli(), counter)
}
