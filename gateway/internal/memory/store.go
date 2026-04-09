package memory

import (
	"bufio"
	"database/sql"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"

	_ "modernc.org/sqlite"
)

type Entry struct {
	ID                 string `json:"id"`
	Kind               string `json:"kind"`
	Title              string `json:"title"`
	Summary            string `json:"summary"`
	Content            string `json:"content"`
	Reason             string `json:"reason"`
	Scope              string `json:"scope"`
	WorkspaceID        string `json:"workspace_id"`
	SessionID          string `json:"session_id"`
	SourceRunID        string `json:"source_run_id"`
	Source             string `json:"source"`
	SourceType         string `json:"source_type"`
	SourceTitle        string `json:"source_title"`
	SourceEventType    string `json:"source_event_type"`
	SourceArtifactPath string `json:"source_artifact_path"`
	GovernanceVersion  string `json:"governance_version"`
	GovernanceReason   string `json:"governance_reason"`
	GovernanceSource   string `json:"governance_source"`
	GovernanceAt       string `json:"governance_at"`
	ArchiveReason      string `json:"archive_reason"`
	Verified           bool   `json:"verified"`
	Priority           int    `json:"priority"`
	Archived           bool   `json:"archived"`
	ArchivedAt         string `json:"archived_at"`
	CreatedAt          string `json:"created_at"`
	UpdatedAt          string `json:"updated_at"`
	Timestamp          string `json:"timestamp"`
}

func (item Entry) MemoryKind() string {
	return item.Kind
}

func (item Entry) GovernanceStatus() string {
	if item.Archived {
		return "archived"
	}
	if item.Verified {
		return "verified"
	}
	return "active"
}

func (item Entry) MemoryAction() string {
	if item.Archived {
		return "archive"
	}
	return "write"
}

type Store struct {
	storageRoot string
}

type tombstone struct {
	MemoryID string `json:"memory_id"`
}

const MemoryGovernanceVersion = "memory_audit_v1"

func NewStore(repoRoot string) *Store {
	return &Store{storageRoot: filepath.Join(repoRoot, "data")}
}

func (s *Store) List(workspaceID string) ([]Entry, error) {
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer db.Close()
	rows, err := db.Query(listMemorySQL, workspaceID)
	if err != nil {
		if isMissingTable(err) {
			return []Entry{}, nil
		}
		return nil, err
	}
	defer rows.Close()
	deleted := s.deletedIDs(workspaceID)
	items := make([]Entry, 0, 16)
	for rows.Next() {
		item, scanErr := scanEntry(rows)
		if scanErr != nil {
			return nil, scanErr
		}
		if !deleted[item.ID] {
			items = append(items, item)
		}
	}
	return items, rows.Err()
}

func (s *Store) Delete(workspaceID string, memoryID string) error {
	if strings.TrimSpace(memoryID) == "" {
		return errors.New("memory id is required")
	}
	db, err := s.openDB()
	if err != nil {
		return err
	}
	defer db.Close()
	result, err := db.Exec(deleteMemorySQL, workspaceID, memoryID)
	if err != nil {
		if isMissingTable(err) {
			return sql.ErrNoRows
		}
		return err
	}
	rows, err := result.RowsAffected()
	if err != nil {
		return err
	}
	if rows == 0 {
		return sql.ErrNoRows
	}
	return s.appendTombstone(workspaceID, memoryID)
}

func (s *Store) Save(entry Entry) (bool, error) {
	db, err := s.openDB()
	if err != nil {
		return false, err
	}
	defer db.Close()
	item := normalizedEntry(entry)
	duplicate, err := hasDuplicateMemory(db, item)
	if err != nil || duplicate {
		return false, err
	}
	return true, insertMemoryEntry(db, item)
}

func (s *Store) openDB() (*sql.DB, error) {
	db, err := sql.Open("sqlite", filepath.Join(s.storageRoot, "storage", "main.db"))
	if err != nil {
		return nil, err
	}
	return db, ensureMemoryColumns(db)
}

func scanEntry(rows *sql.Rows) (Entry, error) {
	var item Entry
	var verified int
	var archived int
	err := rows.Scan(
		&item.ID,
		&item.Kind,
		&item.Title,
		&item.Summary,
		&item.Content,
		&item.Scope,
		&item.WorkspaceID,
		&item.SessionID,
		&item.SourceRunID,
		&item.Source,
		&item.SourceType,
		&item.SourceTitle,
		&item.SourceEventType,
		&item.SourceArtifactPath,
		&item.GovernanceVersion,
		&item.GovernanceReason,
		&item.GovernanceSource,
		&item.GovernanceAt,
		&item.ArchiveReason,
		&verified,
		&item.Priority,
		&archived,
		&item.ArchivedAt,
		&item.CreatedAt,
		&item.UpdatedAt,
		&item.Timestamp,
	)
	item.Verified = verified != 0
	item.Archived = archived != 0
	item = normalizedEntry(item)
	item.Reason = resolvedReason(item)
	return item, err
}

func (s *Store) deletedIDs(workspaceID string) map[string]bool {
	path := s.tombstonePath(workspaceID)
	file, err := os.Open(path)
	if err != nil {
		return map[string]bool{}
	}
	defer file.Close()
	items := map[string]bool{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var item tombstone
		if json.Unmarshal(scanner.Bytes(), &item) == nil && item.MemoryID != "" {
			items[item.MemoryID] = true
		}
	}
	return items
}

func (s *Store) appendTombstone(workspaceID string, memoryID string) error {
	path := s.tombstonePath(workspaceID)
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	file, err := os.OpenFile(path, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		return err
	}
	defer file.Close()
	payload, err := json.Marshal(tombstone{MemoryID: memoryID})
	if err != nil {
		return err
	}
	_, err = file.Write(append(payload, '\n'))
	return err
}

func (s *Store) tombstonePath(workspaceID string) string {
	return filepath.Join(s.storageRoot, "memory", "deletions", safeName(workspaceID)+".jsonl")
}

func safeName(input string) string {
	var builder strings.Builder
	for _, ch := range input {
		if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9') || ch == '-' || ch == '_' {
			builder.WriteRune(ch)
			continue
		}
		builder.WriteByte('_')
	}
	return builder.String()
}

func resolvedReason(item Entry) string {
	if strings.TrimSpace(item.GovernanceReason) != "" {
		return item.GovernanceReason
	}
	return memoryReason(item)
}

func normalizedEntry(item Entry) Entry {
	item.GovernanceVersion = firstText(item.GovernanceVersion, MemoryGovernanceVersion)
	item.GovernanceSource = firstText(item.GovernanceSource, governanceSource(item))
	item.GovernanceReason = firstText(item.GovernanceReason, memoryReason(item))
	item.GovernanceAt = firstText(item.GovernanceAt, item.UpdatedAt, item.CreatedAt, item.Timestamp)
	item.ArchiveReason = archiveReason(item)
	return item
}

func memoryReason(item Entry) string {
	if item.SourceType == "seed" {
		return "基线记忆已按当前治理版本固化。"
	}
	if item.SourceType == "runtime" && item.SourceEventType == "memory_written" {
		return "用户显式写入长期记忆。"
	}
	if item.SourceType == "runtime" && item.SourceEventType == "run_failed" {
		return "失败教训已纳入长期记忆治理。"
	}
	if item.SourceType == "runtime" && item.SourceEventType == "run_finished" {
		return "任务结果已按长期记忆治理规则沉淀。"
	}
	if item.SourceType == "runtime" && item.SourceEventType == "verification_completed" {
		return "验证通过后已沉淀长期记忆。"
	}
	if strings.Contains(item.Source, "README") || strings.Contains(item.Source, "docs/06-development") {
		return "高价值文档已按长期记忆治理规则保留。"
	}
	if item.SourceType == "runtime" {
		return "记忆记录已按当前治理版本写入。"
	}
	return "按当前工作区治理规则持续复用。"
}

func governanceSource(item Entry) string {
	if item.SourceType == "seed" {
		return "seed_baseline"
	}
	if item.SourceType == "runtime" && item.SourceEventType == "memory_written" {
		return "runtime_manual_write"
	}
	if item.SourceType == "runtime" && item.SourceEventType == "run_failed" {
		return "runtime_failure_lesson"
	}
	if item.SourceType == "runtime" && item.SourceEventType == "run_finished" {
		return "runtime_finish_memory"
	}
	if item.SourceType == "runtime" && item.SourceEventType == "verification_completed" {
		return "runtime_verified_memory"
	}
	if item.SourceType == "runtime" {
		return "runtime_memory"
	}
	return "memory_append"
}

func archiveReason(item Entry) string {
	if !item.Archived {
		return ""
	}
	return firstText(item.ArchiveReason, "当前记录已标记为归档。")
}

func firstText(values ...string) string {
	for _, value := range values {
		if strings.TrimSpace(value) != "" {
			return value
		}
	}
	return ""
}

func isMissingTable(err error) bool {
	return err != nil && strings.Contains(err.Error(), "no such table")
}

func ensureMemoryColumns(db *sql.DB) error {
	for _, statement := range memoryMigrationSQL {
		if _, err := db.Exec(statement); err != nil && !isDuplicateColumn(err) {
			return err
		}
	}
	return nil
}

func hasDuplicateMemory(db *sql.DB, item Entry) (bool, error) {
	var count int
	err := db.QueryRow(
		duplicateMemorySQL,
		item.WorkspaceID,
		item.Kind,
		item.Title,
		item.Summary,
	).Scan(&count)
	return count > 0, err
}

func insertMemoryEntry(db *sql.DB, item Entry) error {
	_, err := db.Exec(insertMemorySQL, memoryArgs(item)...)
	return err
}

func memoryArgs(item Entry) []any {
	return []any{
		item.ID, item.WorkspaceID, item.Kind, item.Title, item.Summary, item.Content,
		item.Source, item.SourceRunID, item.SourceType, item.SourceTitle,
		item.SourceEventType, item.SourceArtifactPath, item.GovernanceVersion,
		item.GovernanceReason, item.GovernanceSource, item.GovernanceAt,
		item.ArchiveReason, boolFlag(item.Verified), item.Priority, boolFlag(item.Archived),
		item.ArchivedAt, item.CreatedAt, item.UpdatedAt, item.Scope, item.SessionID,
		item.Timestamp,
	}
}

func boolFlag(value bool) int {
	if value {
		return 1
	}
	return 0
}

func isDuplicateColumn(err error) bool {
	return err != nil && strings.Contains(err.Error(), "duplicate column name")
}

const listMemorySQL = `
select id, memory_type, title, summary, content, scope, workspace_id, session_id,
source_run_id, source, source_type, source_title, source_event_type, source_artifact_path,
governance_version, governance_reason, governance_source, governance_at, archive_reason,
verified, priority, archived, archived_at, created_at, updated_at, timestamp
from long_term_memory
where workspace_id = ?
order by priority desc, updated_at desc
`

const deleteMemorySQL = `
delete from long_term_memory
where workspace_id = ? and id = ?
`

const duplicateMemorySQL = `
select count(1)
from long_term_memory
where workspace_id = ? and memory_type = ? and title = ? and summary = ?
`

const insertMemorySQL = `
insert into long_term_memory (
id, workspace_id, memory_type, title, summary, content, source, source_run_id, source_type,
source_title, source_event_type, source_artifact_path, governance_version, governance_reason,
governance_source, governance_at, archive_reason, verified, priority, archived, archived_at,
created_at, updated_at, scope, session_id, timestamp
) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
`

var memoryMigrationSQL = []string{
	"alter table long_term_memory add column source_title text not null default ''",
	"alter table long_term_memory add column source_event_type text not null default ''",
	"alter table long_term_memory add column source_artifact_path text not null default ''",
	"alter table long_term_memory add column governance_version text not null default ''",
	"alter table long_term_memory add column governance_reason text not null default ''",
	"alter table long_term_memory add column governance_source text not null default ''",
	"alter table long_term_memory add column governance_at text not null default ''",
	"alter table long_term_memory add column archive_reason text not null default ''",
	"alter table long_term_memory add column archived_at text not null default ''",
}
