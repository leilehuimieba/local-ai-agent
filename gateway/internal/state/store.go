package state

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"sync"

	"local-agent/gateway/internal/config"
)

type persistedSettings struct {
	Mode                   string   `json:"mode"`
	ModelProviderID        string   `json:"model_provider_id"`
	ModelID                string   `json:"model_id"`
	WorkspaceID            string   `json:"workspace_id"`
	DirectoryPromptEnabled *bool    `json:"directory_prompt_enabled,omitempty"`
	ShowRiskLevel          *bool    `json:"show_risk_level,omitempty"`
	ApprovedWorkspaceIDs   []string `json:"approved_workspace_ids"`
	ApprovedDirectories    []ApprovedDirectoryRecord `json:"approved_directories,omitempty"`
}

type ApprovedDirectoryRecord struct {
	ApprovalID  string `json:"approval_id"`
	WorkspaceID string `json:"workspace_id"`
	Name        string `json:"name"`
	RootPath    string `json:"root_path"`
	CreatedAt   string `json:"created_at,omitempty"`
}

type SettingsStore struct {
	mu                     sync.RWMutex
	path                   string
	mode                   string
	model                  config.ModelRef
	models                 []config.ModelRef
	workspace              config.WorkspaceRef
	workspaces             []config.WorkspaceRef
	directoryPromptEnabled bool
	showRiskLevel          bool
	approvedDirectories    map[string]ApprovedDirectoryRecord
}

func NewSettingsStore(repoRoot string, cfg config.AppConfig) *SettingsStore {
	workspaces := make([]config.WorkspaceRef, len(cfg.Workspaces))
	copy(workspaces, cfg.Workspaces)
	models := make([]config.ModelRef, len(cfg.AvailableModels))
	copy(models, cfg.AvailableModels)

	store := &SettingsStore{
		path:                   filepath.Join(repoRoot, "data", "settings", "ui-state.json"),
		mode:                   cfg.DefaultMode,
		model:                  cfg.DefaultModel,
		models:                 models,
		workspace:              cfg.DefaultWorkspace,
		workspaces:             workspaces,
		directoryPromptEnabled: true,
		showRiskLevel:          true,
		approvedDirectories:    defaultApprovedDirectories(cfg.DefaultWorkspace),
	}
	store.loadPersisted()
	return store
}

func (s *SettingsStore) Snapshot() (
	string,
	config.ModelRef,
	[]config.ModelRef,
	config.WorkspaceRef,
	[]config.WorkspaceRef,
	bool,
	bool,
	[]ApprovedDirectoryRecord,
) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	models := make([]config.ModelRef, len(s.models))
	copy(models, s.models)
	workspaces := make([]config.WorkspaceRef, len(s.workspaces))
	copy(workspaces, s.workspaces)
	approved := s.approvedDirectoryListLocked()
	return s.mode, s.model, models, s.workspace, workspaces, s.directoryPromptEnabled, s.showRiskLevel, approved
}

func (s *SettingsStore) Update(
	mode string,
	model config.ModelRef,
	workspaceID string,
	directoryPromptEnabled *bool,
	showRiskLevel *bool,
) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	if mode != "" {
		switch mode {
		case "observe", "standard", "full_access":
		default:
			return errors.New("invalid mode")
		}
		s.mode = mode
	}
	if model.ModelID != "" {
		for _, item := range s.models {
			if item.ModelID == model.ModelID && item.ProviderID == model.ProviderID {
				if !item.Enabled || !item.Available {
					return errors.New("model not available")
				}
				s.model = item
				goto workspaceUpdate
			}
		}
		return errors.New("model not found")
	}

workspaceUpdate:
	if workspaceID != "" {
		for _, workspace := range s.workspaces {
			if workspace.WorkspaceID == workspaceID {
				s.workspace = workspace
				goto preferenceUpdate
			}
		}
		return errors.New("workspace not found")
	}

preferenceUpdate:
	if directoryPromptEnabled != nil {
		s.directoryPromptEnabled = *directoryPromptEnabled
	}
	if showRiskLevel != nil {
		s.showRiskLevel = *showRiskLevel
	}

	s.saveLocked()
	return nil
}

func (s *SettingsStore) WorkspaceByID(workspaceID string) (config.WorkspaceRef, bool) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	for _, workspace := range s.workspaces {
		if workspace.WorkspaceID == workspaceID {
			return workspace, true
		}
	}

	return config.WorkspaceRef{}, false
}

func (s *SettingsStore) IsWorkspaceApproved(workspaceID string) bool {
	workspace, ok := s.WorkspaceByID(workspaceID)
	if !ok {
		return false
	}
	return s.IsDirectoryApproved(workspace.RootPath)
}

func (s *SettingsStore) IsDirectoryApproved(rootPath string) bool {
	s.mu.RLock()
	defer s.mu.RUnlock()
	_, ok := s.approvedDirectories[rootPath]
	return ok
}

func (s *SettingsStore) IsDirectoryPromptEnabled() bool {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.directoryPromptEnabled
}

func (s *SettingsStore) ApproveWorkspace(workspaceID string) {
	workspace, ok := s.WorkspaceByID(workspaceID)
	if !ok {
		return
	}
	s.ApproveDirectory(workspace)
}

func (s *SettingsStore) ApproveDirectory(workspace config.WorkspaceRef) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.approvedDirectories[workspace.RootPath] = approvalRecord(workspace)
	s.saveLocked()
}

func (s *SettingsStore) RevokeDirectoryApproval(rootPath string) {
	s.mu.Lock()
	defer s.mu.Unlock()
	delete(s.approvedDirectories, rootPath)
	s.saveLocked()
}

func (s *SettingsStore) loadPersisted() {
	persisted, ok := readPersistedSettings(s.path)
	if !ok {
		return
	}
	s.applyPersistedCore(persisted)
	s.loadApprovedDirectories(persisted)
}

func readPersistedSettings(path string) (persistedSettings, bool) {
	raw, err := os.ReadFile(path)
	if err != nil {
		return persistedSettings{}, false
	}
	var persisted persistedSettings
	if err := json.Unmarshal(raw, &persisted); err != nil {
		return persistedSettings{}, false
	}
	return persisted, true
}

func (s *SettingsStore) applyPersistedCore(persisted persistedSettings) {
	if persisted.Mode != "" {
		s.mode = persisted.Mode
	}
	if persisted.ModelID != "" {
		s.model = resolvePersistedModel(s.models, persisted, s.model)
	}
	if workspace, ok := workspaceFromID(s.workspaces, persisted.WorkspaceID); ok {
		s.workspace = workspace
	}
	if persisted.DirectoryPromptEnabled != nil {
		s.directoryPromptEnabled = *persisted.DirectoryPromptEnabled
	}
	if persisted.ShowRiskLevel != nil {
		s.showRiskLevel = *persisted.ShowRiskLevel
	}
}

func (s *SettingsStore) loadApprovedDirectories(persisted persistedSettings) {
	if len(persisted.ApprovedDirectories) > 0 {
		s.approvedDirectories = mapApprovedDirectories(persisted.ApprovedDirectories)
	}
	if len(s.approvedDirectories) == 0 && len(persisted.ApprovedWorkspaceIDs) > 0 {
		s.approvedDirectories = approvedDirectoriesFromIDs(s.workspaces, persisted.ApprovedWorkspaceIDs)
	}
	if len(s.approvedDirectories) == 0 {
		s.approvedDirectories = defaultApprovedDirectories(s.workspace)
	}
}

func resolvePersistedModel(models []config.ModelRef, persisted persistedSettings, fallback config.ModelRef) config.ModelRef {
	for _, item := range models {
		if item.ModelID == persisted.ModelID && item.ProviderID == persisted.ModelProviderID {
			return item
		}
	}
	return fallback
}

func (s *SettingsStore) saveLocked() {
	if err := os.MkdirAll(filepath.Dir(s.path), 0o755); err != nil {
		return
	}

	payload := persistedSettings{
		Mode:                   s.mode,
		ModelProviderID:        s.model.ProviderID,
		ModelID:                s.model.ModelID,
		WorkspaceID:            s.workspace.WorkspaceID,
		DirectoryPromptEnabled: boolPtr(s.directoryPromptEnabled),
		ShowRiskLevel:          boolPtr(s.showRiskLevel),
		ApprovedDirectories:    s.approvedDirectoryListLocked(),
	}
	payload.ApprovedWorkspaceIDs = approvedWorkspaceIDs(payload.ApprovedDirectories)

	raw, err := json.MarshalIndent(payload, "", "  ")
	if err != nil {
		return
	}
	_ = os.WriteFile(s.path, raw, 0o644)
}

func boolPtr(value bool) *bool {
	return &value
}

func approvedWorkspaceIDs(items []ApprovedDirectoryRecord) []string {
	ids := make([]string, 0, len(items))
	for _, item := range items {
		if item.WorkspaceID != "" {
			ids = append(ids, item.WorkspaceID)
		}
	}
	return ids
}

func (s *SettingsStore) approvedDirectoryListLocked() []ApprovedDirectoryRecord {
	items := make([]ApprovedDirectoryRecord, 0, len(s.approvedDirectories))
	for _, workspace := range s.workspaces {
		record, ok := s.approvedDirectories[workspace.RootPath]
		if ok {
			items = append(items, normalizeApprovalRecord(record))
		}
	}
	for rootPath, record := range s.approvedDirectories {
		if !containsApproval(items, rootPath) {
			items = append(items, normalizeApprovalRecord(record))
		}
	}
	return items
}

func containsApproval(items []ApprovedDirectoryRecord, rootPath string) bool {
	for _, item := range items {
		if item.RootPath == rootPath {
			return true
		}
	}
	return false
}

func defaultApprovedDirectories(workspace config.WorkspaceRef) map[string]ApprovedDirectoryRecord {
	return map[string]ApprovedDirectoryRecord{
		workspace.RootPath: approvalRecord(workspace),
	}
}

func approvedDirectoriesFromIDs(workspaces []config.WorkspaceRef, ids []string) map[string]ApprovedDirectoryRecord {
	items := make(map[string]ApprovedDirectoryRecord, len(ids))
	for _, workspaceID := range ids {
		if workspace, ok := workspaceFromID(workspaces, workspaceID); ok {
			items[workspace.RootPath] = approvalRecord(workspace)
		}
	}
	return items
}

func mapApprovedDirectories(items []ApprovedDirectoryRecord) map[string]ApprovedDirectoryRecord {
	mapped := make(map[string]ApprovedDirectoryRecord, len(items))
	for _, item := range items {
		if item.RootPath != "" {
			mapped[item.RootPath] = normalizeApprovalRecord(item)
		}
	}
	return mapped
}

func workspaceFromID(workspaces []config.WorkspaceRef, workspaceID string) (config.WorkspaceRef, bool) {
	for _, workspace := range workspaces {
		if workspace.WorkspaceID == workspaceID {
			return workspace, true
		}
	}
	return config.WorkspaceRef{}, false
}

func approvalRecord(workspace config.WorkspaceRef) ApprovedDirectoryRecord {
	return ApprovedDirectoryRecord{
		ApprovalID:  workspace.WorkspaceID,
		WorkspaceID: workspace.WorkspaceID,
		Name:        workspace.Name,
		RootPath:    workspace.RootPath,
	}
}

func normalizeApprovalRecord(item ApprovedDirectoryRecord) ApprovedDirectoryRecord {
	if item.ApprovalID == "" {
		item.ApprovalID = item.WorkspaceID
	}
	if item.Name == "" {
		item.Name = item.RootPath
	}
	return item
}
