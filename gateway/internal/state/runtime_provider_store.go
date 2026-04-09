package state

import (
	"encoding/json"
	"os"
	"path/filepath"
	"sort"
	"sync"
)

type RuntimeProviderRecord struct {
	ProviderID          string `json:"provider_id"`
	DisplayName         string `json:"display_name"`
	BaseURL             string `json:"base_url"`
	ChatCompletionsPath string `json:"chat_completions_path"`
	ModelsPath          string `json:"models_path"`
	APIKey              string `json:"api_key,omitempty"`
	AppliedAt           string `json:"applied_at,omitempty"`
	ConfigVersion       string `json:"config_version,omitempty"`
	Status              string `json:"status"`
	PendingReload       bool   `json:"pending_reload"`
	LastApplyMessage    string `json:"last_apply_message,omitempty"`
}

type runtimeProviderFile struct {
	ActiveProviderID string                  `json:"active_provider_id,omitempty"`
	Providers        []RuntimeProviderRecord `json:"providers"`
}

type RuntimeProviderStore struct {
	mu               sync.RWMutex
	path             string
	lockPath         string
	activeProviderID string
	providers        map[string]RuntimeProviderRecord
}

func NewRuntimeProviderStore(repoRoot string) *RuntimeProviderStore {
	store := &RuntimeProviderStore{
		path: filepath.Join(repoRoot, "data", "settings", "runtime-provider-state.json"),
		lockPath: filepath.Join(repoRoot, "data", "settings", "runtime-provider-state.lock"),
		providers: make(map[string]RuntimeProviderRecord),
	}
	store.load()
	return store
}

func (s *RuntimeProviderStore) Snapshot() (string, []RuntimeProviderRecord) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.syncLocked()
	return s.activeProviderID, s.listLocked()
}

func (s *RuntimeProviderStore) Get(providerID string) (RuntimeProviderRecord, bool) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.syncLocked()
	record, ok := s.providers[providerID]
	return record, ok
}

func (s *RuntimeProviderStore) IsActive(providerID string) bool {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.activeProviderID == providerID
}

func (s *RuntimeProviderStore) Apply(record RuntimeProviderRecord) (RuntimeProviderRecord, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	err := withFileLock(s.lockPath, func() error { return s.applyLocked(record) })
	return s.providers[record.ProviderID], err
}

func (s *RuntimeProviderStore) Save(record RuntimeProviderRecord) (RuntimeProviderRecord, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	err := withFileLock(s.lockPath, func() error { return s.saveRecordLocked(record) })
	return s.providers[record.ProviderID], err
}

func (s *RuntimeProviderStore) MarkPending(providerID string, configVersion string, message string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	return withFileLock(s.lockPath, func() error {
		s.syncLocked()
		record, ok := s.providers[providerID]
		if !ok {
			return nil
		}
		record.PendingReload = record.ConfigVersion != configVersion
		if record.PendingReload {
			record.LastApplyMessage = message
		}
		s.providers[providerID] = record
		return s.writeLocked()
	})
}

func (s *RuntimeProviderStore) Remove(providerID string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	return withFileLock(s.lockPath, func() error {
		s.syncLocked()
		delete(s.providers, providerID)
		if s.activeProviderID == providerID {
			s.activeProviderID = ""
		}
		return s.writeLocked()
	})
}

func (s *RuntimeProviderStore) load() {
	s.activeProviderID, s.providers = readRuntimeRecords(s.path)
}

func (s *RuntimeProviderStore) saveLocked() error {
	return withFileLock(s.lockPath, func() error { return s.writeLocked() })
}

func (s *RuntimeProviderStore) writeLocked() error {
	if err := os.MkdirAll(filepath.Dir(s.path), 0o755); err != nil {
		return err
	}
	payload := runtimeProviderFile{ActiveProviderID: s.activeProviderID, Providers: s.listLocked()}
	raw, err := json.MarshalIndent(payload, "", "  ")
	if err != nil {
		return err
	}
	return writeRuntimeFile(s.path, raw)
}

func (s *RuntimeProviderStore) listLocked() []RuntimeProviderRecord {
	items := make([]RuntimeProviderRecord, 0, len(s.providers))
	for _, item := range s.providers {
		items = append(items, item)
	}
	sort.Slice(items, func(i int, j int) bool { return items[i].ProviderID < items[j].ProviderID })
	return items
}

func (s *RuntimeProviderStore) syncLocked() {
	s.activeProviderID, s.providers = readRuntimeRecords(s.path)
}

func (s *RuntimeProviderStore) applyLocked(record RuntimeProviderRecord) error {
	s.syncLocked()
	record.Status = "applied"
	record.PendingReload = false
	s.providers[record.ProviderID] = record
	s.activeProviderID = record.ProviderID
	return s.writeLocked()
}

func (s *RuntimeProviderStore) saveRecordLocked(record RuntimeProviderRecord) error {
	s.syncLocked()
	s.providers[record.ProviderID] = record
	if record.Status == "applied" {
		s.activeProviderID = record.ProviderID
	}
	return s.writeLocked()
}

func writeRuntimeFile(path string, raw []byte) error {
	tempPath := path + ".tmp"
	if err := os.WriteFile(tempPath, raw, 0o600); err != nil {
		return err
	}
	if err := os.Rename(tempPath, path); err == nil {
		return nil
	}
	if err := os.Remove(path); err != nil && !os.IsNotExist(err) {
		return err
	}
	return os.Rename(tempPath, path)
}

func readRuntimeRecords(path string) (string, map[string]RuntimeProviderRecord) {
	raw, err := os.ReadFile(path)
	if err != nil {
		return "", make(map[string]RuntimeProviderRecord)
	}
	var payload runtimeProviderFile
	if json.Unmarshal(raw, &payload) != nil {
		return "", make(map[string]RuntimeProviderRecord)
	}
	items := make(map[string]RuntimeProviderRecord, len(payload.Providers))
	for _, item := range payload.Providers {
		items[item.ProviderID] = item
	}
	return payload.ActiveProviderID, items
}
