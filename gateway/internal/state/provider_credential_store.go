package state

import (
	"encoding/json"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"
)

type ProviderCredentialRecord struct {
	ProviderID          string `json:"provider_id"`
	DisplayName         string `json:"display_name,omitempty"`
	BaseURL             string `json:"base_url,omitempty"`
	ChatCompletionsPath string `json:"chat_completions_path,omitempty"`
	ModelsPath          string `json:"models_path,omitempty"`
	CredentialKind      string `json:"credential_kind"`
	APIKey              string `json:"api_key,omitempty"`
	APIKeyMasked        string `json:"api_key_masked,omitempty"`
	HasCredential       bool   `json:"has_credential"`
	UpdatedAt           string `json:"updated_at,omitempty"`
	LastTestStatus      string `json:"last_test_status,omitempty"`
	LastTestMessage     string `json:"last_test_message,omitempty"`
	LastTestAt          string `json:"last_test_at,omitempty"`
}

type providerCredentialFile struct {
	Providers []ProviderCredentialRecord `json:"providers"`
}

type ProviderCredentialStore struct {
	mu        sync.RWMutex
	path      string
	lockPath  string
	providers map[string]ProviderCredentialRecord
}

func NewProviderCredentialStore(repoRoot string) *ProviderCredentialStore {
	store := &ProviderCredentialStore{
		path: filepath.Join(repoRoot, "data", "settings", "provider-credentials.json"),
		lockPath: filepath.Join(repoRoot, "data", "settings", "provider-credentials.lock"),
		providers: make(map[string]ProviderCredentialRecord),
	}
	store.load()
	return store
}

func (s *ProviderCredentialStore) Snapshot() []ProviderCredentialRecord {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.syncLocked()
	return s.listLocked()
}

func (s *ProviderCredentialStore) Get(providerID string) (ProviderCredentialRecord, bool) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.syncLocked()
	record, ok := s.providers[providerID]
	return record, ok
}

func (s *ProviderCredentialStore) Save(record ProviderCredentialRecord) (ProviderCredentialRecord, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	err := withFileLock(s.lockPath, func() error { return s.saveRecordLocked(record) })
	return s.providers[record.ProviderID], err
}

func (s *ProviderCredentialStore) UpdateTestResult(providerID string, status string, message string, checkedAt string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	return withFileLock(s.lockPath, func() error {
		s.syncLocked()
		record := s.providers[providerID]
		record.ProviderID = providerID
		record.CredentialKind = "api_key"
		record.LastTestStatus = status
		record.LastTestMessage = message
		record.LastTestAt = checkedAt
		s.providers[providerID] = record
		return s.writeLocked()
	})
}

func (s *ProviderCredentialStore) Remove(providerID string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	return withFileLock(s.lockPath, func() error {
		s.syncLocked()
		delete(s.providers, providerID)
		return s.writeLocked()
	})
}

func (s *ProviderCredentialStore) load() {
	s.providers = readCredentialRecords(s.path)
}

func (s *ProviderCredentialStore) saveLocked() error {
	return withFileLock(s.lockPath, func() error { return s.writeLocked() })
}

func (s *ProviderCredentialStore) writeLocked() error {
	if err := os.MkdirAll(filepath.Dir(s.path), 0o755); err != nil {
		return err
	}
	payload := providerCredentialFile{Providers: s.listLocked()}
	raw, err := json.MarshalIndent(payload, "", "  ")
	if err != nil {
		return err
	}
	return writeCredentialFile(s.path, raw)
}

func (s *ProviderCredentialStore) listLocked() []ProviderCredentialRecord {
	items := make([]ProviderCredentialRecord, 0, len(s.providers))
	for _, item := range s.providers {
		items = append(items, item)
	}
	sort.Slice(items, func(i int, j int) bool { return items[i].ProviderID < items[j].ProviderID })
	return items
}

func (s *ProviderCredentialStore) syncLocked() {
	s.providers = readCredentialRecords(s.path)
}

func (s *ProviderCredentialStore) saveRecordLocked(record ProviderCredentialRecord) error {
	s.syncLocked()
	current := s.providers[record.ProviderID]
	record.LastTestStatus = current.LastTestStatus
	record.LastTestMessage = current.LastTestMessage
	record.LastTestAt = current.LastTestAt
	record.CredentialKind = "api_key"
	record.APIKeyMasked = maskAPIKey(record.APIKey)
	record.HasCredential = true
	record.UpdatedAt = nowMillis()
	s.providers[record.ProviderID] = record
	return s.writeLocked()
}

func writeCredentialFile(path string, raw []byte) error {
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

func readCredentialRecords(path string) map[string]ProviderCredentialRecord {
	raw, err := os.ReadFile(path)
	if err != nil {
		return make(map[string]ProviderCredentialRecord)
	}
	var payload providerCredentialFile
	if json.Unmarshal(raw, &payload) != nil {
		return make(map[string]ProviderCredentialRecord)
	}
	items := make(map[string]ProviderCredentialRecord, len(payload.Providers))
	for _, item := range payload.Providers {
		items[item.ProviderID] = item
	}
	return items
}

func maskAPIKey(apiKey string) string {
	if apiKey == "" {
		return ""
	}
	trimmed := strings.TrimSpace(apiKey)
	if len(trimmed) <= 8 {
		return trimmed
	}
	return trimmed[:4] + "..." + trimmed[len(trimmed)-4:]
}

func nowMillis() string {
	return strconv.FormatInt(time.Now().UnixMilli(), 10)
}
