package knowledge

import (
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	_ "modernc.org/sqlite"
)

type Store struct {
	storageRoot string
}

func NewStore(repoRoot string) *Store {
	s := &Store{storageRoot: filepath.Join(repoRoot, "data")}
	_ = s.migrateFromJSON(repoRoot)
	return s
}

func (s *Store) openDB() (*sql.DB, error) {
	dbPath := filepath.Join(s.storageRoot, "storage", "main.db")
	if err := os.MkdirAll(filepath.Dir(dbPath), 0o755); err != nil {
		return nil, err
	}
	db, err := sql.Open("sqlite", dbPath)
	if err != nil {
		return nil, err
	}
	return db, ensureKnowledgeTable(db)
}

func (s *Store) List(workspaceID string) ([]Item, error) {
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer db.Close()
	rows, err := db.Query(listKnowledgeSQL, workspaceID)
	if err != nil {
		if isMissingTable(err) {
			return []Item{}, nil
		}
		return nil, err
	}
	defer rows.Close()
	return scanItems(rows)
}

func (s *Store) Get(workspaceID string, id string) (*Item, error) {
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer db.Close()
	row := db.QueryRow(getKnowledgeSQL, workspaceID, id)
	item, err := scanItem(row)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, errors.New("not found")
		}
		return nil, err
	}
	return item, nil
}

func (s *Store) Create(workspaceID string, req CreateRequest) (*Item, error) {
	now := time.Now().Format(time.RFC3339)
	item := Item{
		ID:            generateID(),
		Title:         strings.TrimSpace(req.Title),
		Summary:       strings.TrimSpace(req.Summary),
		Content:       strings.TrimSpace(req.Content),
		Category:      req.Category,
		Tags:          req.Tags,
		Source:        strings.TrimSpace(req.Source),
		CitationCount: 0,
		CreatedAt:     now,
		UpdatedAt:     now,
	}
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer db.Close()
	if _, err := db.Exec(insertKnowledgeSQL, knowledgeArgs(workspaceID, item)...); err != nil {
		return nil, err
	}
	return &item, nil
}

func (s *Store) Update(workspaceID string, id string, req UpdateRequest) (*Item, error) {
	item, err := s.Get(workspaceID, id)
	if err != nil {
		return nil, err
	}
	if req.Title != "" {
		item.Title = strings.TrimSpace(req.Title)
	}
	if req.Summary != "" {
		item.Summary = strings.TrimSpace(req.Summary)
	}
	if req.Content != "" {
		item.Content = strings.TrimSpace(req.Content)
	}
	if req.Category != "" {
		item.Category = req.Category
	}
	if req.Tags != nil {
		item.Tags = req.Tags
	}
	if req.Source != "" {
		item.Source = strings.TrimSpace(req.Source)
	}
	if req.Embedding != nil {
		item.Embedding = req.Embedding
	}
	item.UpdatedAt = time.Now().Format(time.RFC3339)
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer db.Close()
	if _, err := db.Exec(updateKnowledgeSQL, updateArgs(workspaceID, id, *item)...); err != nil {
		return nil, err
	}
	return item, nil
}

func (s *Store) Delete(workspaceID string, id string) error {
	db, err := s.openDB()
	if err != nil {
		return err
	}
	defer db.Close()
	res, err := db.Exec(deleteKnowledgeSQL, workspaceID, id)
	if err != nil {
		return err
	}
	n, _ := res.RowsAffected()
	if n == 0 {
		return errors.New("not found")
	}
	return nil
}

func (s *Store) Search(workspaceID string, query string) ([]Item, error) {
	q := strings.TrimSpace(query)
	if q == "" {
		return s.List(workspaceID)
	}
	pattern := "%" + q + "%"
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer db.Close()
	rows, err := db.Query(searchKnowledgeSQL, workspaceID, pattern, pattern, pattern)
	if err != nil {
		if isMissingTable(err) {
			return []Item{}, nil
		}
		return nil, err
	}
	defer rows.Close()
	return scanItems(rows)
}

func (s *Store) Categories(workspaceID string) ([]string, error) {
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer db.Close()
	rows, err := db.Query(categoriesSQL, workspaceID)
	if err != nil {
		if isMissingTable(err) {
			return []string{}, nil
		}
		return nil, err
	}
	defer rows.Close()
	var result []string
	for rows.Next() {
		var c string
		if err := rows.Scan(&c); err == nil && c != "" {
			result = append(result, c)
		}
	}
	return result, rows.Err()
}

func (s *Store) Tags(workspaceID string) ([]string, error) {
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer db.Close()
	rows, err := db.Query(tagsSQL, workspaceID)
	if err != nil {
		if isMissingTable(err) {
			return []string{}, nil
		}
		return nil, err
	}
	defer rows.Close()
	set := make(map[string]bool)
	for rows.Next() {
		var raw string
		if err := rows.Scan(&raw); err != nil {
			continue
		}
		var tags []string
		_ = json.Unmarshal([]byte(raw), &tags)
		for _, t := range tags {
			if t != "" {
				set[t] = true
			}
		}
	}
	var result []string
	for t := range set {
		result = append(result, t)
	}
	return result, rows.Err()
}

func scanItems(rows *sql.Rows) ([]Item, error) {
	var items []Item
	for rows.Next() {
		item, err := scanItemRow(rows)
		if err != nil {
			return nil, err
		}
		items = append(items, item)
	}
	return items, rows.Err()
}

func scanItem(row *sql.Row) (*Item, error) {
	item, err := scanItemRow(row)
	return &item, err
}

func scanItemRow(scanner interface{ Scan(dest ...any) error }) (Item, error) {
	var item Item
	var tagsRaw string
	var embedRaw sql.NullString
	err := scanner.Scan(
		&item.ID,
		&item.Title,
		&item.Summary,
		&item.Content,
		&item.Category,
		&tagsRaw,
		&item.Source,
		&item.CitationCount,
		&embedRaw,
		&item.CreatedAt,
		&item.UpdatedAt,
	)
	if err != nil {
		return item, err
	}
	_ = json.Unmarshal([]byte(tagsRaw), &item.Tags)
	if embedRaw.Valid && embedRaw.String != "" {
		_ = json.Unmarshal([]byte(embedRaw.String), &item.Embedding)
	}
	return item, nil
}

func knowledgeArgs(workspaceID string, item Item) []any {
	tagsRaw, _ := json.Marshal(item.Tags)
	embedRaw, _ := json.Marshal(item.Embedding)
	return []any{
		item.ID, workspaceID, item.Title, item.Summary, item.Content,
		item.Category, string(tagsRaw), item.Source, item.CitationCount,
		string(embedRaw), item.CreatedAt, item.UpdatedAt,
	}
}

func updateArgs(workspaceID string, id string, item Item) []any {
	tagsRaw, _ := json.Marshal(item.Tags)
	embedRaw, _ := json.Marshal(item.Embedding)
	return []any{
		item.Title, item.Summary, item.Content, item.Category,
		string(tagsRaw), item.Source, string(embedRaw), item.UpdatedAt,
		workspaceID, id,
	}
}

func isMissingTable(err error) bool {
	return err != nil && strings.Contains(err.Error(), "no such table")
}

func ensureKnowledgeTable(db *sql.DB) error {
	for _, stmt := range createKnowledgeTableSQLs {
		if _, err := db.Exec(stmt); err != nil {
			return err
		}
	}
	for _, stmt := range knowledgeMigrationSQLs {
		if _, err := db.Exec(stmt); err != nil && !isDuplicateColumn(err) {
			return err
		}
	}
	return nil
}

func isDuplicateColumn(err error) bool {
	return err != nil && strings.Contains(err.Error(), "duplicate column name")
}

func generateID() string {
	return fmt.Sprintf("kb_%d_%s", time.Now().UnixNano(), randomSuffix())
}

func randomSuffix() string {
	const chars = "abcdefghijklmnopqrstuvwxyz0123456789"
	b := make([]byte, 6)
	for i := range b {
		b[i] = chars[time.Now().UnixNano()%int64(len(chars))]
	}
	return string(b)
}

func safeName(input string) string {
	var b strings.Builder
	for _, ch := range input {
		if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9') || ch == '-' || ch == '_' {
			b.WriteRune(ch)
			continue
		}
		b.WriteByte('_')
	}
	return b.String()
}

var createKnowledgeTableSQLs = []string{
	`create table if not exists knowledge_items (
		id text primary key,
		workspace_id text not null,
		title text not null,
		summary text not null default '',
		content text not null default '',
		category text not null default '',
		tags text not null default '[]',
		source text not null default '',
		citation_count integer not null default 0,
		embedding text,
		created_at text not null,
		updated_at text not null
	)`,
	`create index if not exists idx_knowledge_workspace on knowledge_items(workspace_id)`,
	`create index if not exists idx_knowledge_category on knowledge_items(category)`,
}

var knowledgeMigrationSQLs = []string{
	`alter table knowledge_items add column embedding text`,
}

const listKnowledgeSQL = `
select id, title, summary, content, category, tags, source, citation_count, embedding, created_at, updated_at
from knowledge_items
where workspace_id = ?
order by updated_at desc
`

const getKnowledgeSQL = `
select id, title, summary, content, category, tags, source, citation_count, embedding, created_at, updated_at
from knowledge_items
where workspace_id = ? and id = ?
`

const insertKnowledgeSQL = `
insert into knowledge_items (
	id, workspace_id, title, summary, content, category, tags, source, citation_count, embedding, created_at, updated_at
) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
`

const updateKnowledgeSQL = `
update knowledge_items
set title = ?, summary = ?, content = ?, category = ?, tags = ?, source = ?, embedding = ?, updated_at = ?
where workspace_id = ? and id = ?
`

const deleteKnowledgeSQL = `
delete from knowledge_items
where workspace_id = ? and id = ?
`

const searchKnowledgeSQL = `
select id, title, summary, content, category, tags, source, citation_count, embedding, created_at, updated_at
from knowledge_items
where workspace_id = ? and (title like ? or summary like ? or content like ?)
order by updated_at desc
`

const categoriesSQL = `
select distinct category from knowledge_items where workspace_id = ? order by category
`

const tagsSQL = `
select tags from knowledge_items where workspace_id = ?
`
