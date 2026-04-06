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
	Verified           bool   `json:"verified"`
	Priority           int    `json:"priority"`
	Archived           bool   `json:"archived"`
	ArchivedAt         string `json:"archived_at"`
	CreatedAt          string `json:"created_at"`
	UpdatedAt          string `json:"updated_at"`
	Timestamp          string `json:"timestamp"`
}

type Store struct {
	storageRoot string
}

type tombstone struct {
	MemoryID string `json:"memory_id"`
}

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
	item.Reason = memoryReason(item)
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

func memoryReason(item Entry) string {
	if item.SourceType == "seed" {
		return "基线记忆优先"
	}
	if strings.Contains(item.Source, "README") || strings.Contains(item.Source, "docs/06-development") {
		return "高价值文档命中"
	}
	if item.SourceType == "runtime" {
		return "运行时沉淀后持续复用"
	}
	return "按当前工作区持续复用"
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

func isDuplicateColumn(err error) bool {
	return err != nil && strings.Contains(err.Error(), "duplicate column name")
}

const listMemorySQL = `
select id, memory_type, title, summary, content, scope, workspace_id, session_id,
source_run_id, source, source_type, source_title, source_event_type, source_artifact_path,
verified, priority, archived, archived_at, created_at, updated_at, timestamp
from long_term_memory
where workspace_id = ?
order by priority desc, updated_at desc
`

const deleteMemorySQL = `
delete from long_term_memory
where workspace_id = ? and id = ?
`

var memoryMigrationSQL = []string{
	"alter table long_term_memory add column source_title text not null default ''",
	"alter table long_term_memory add column source_event_type text not null default ''",
	"alter table long_term_memory add column source_artifact_path text not null default ''",
	"alter table long_term_memory add column archived_at text not null default ''",
}
