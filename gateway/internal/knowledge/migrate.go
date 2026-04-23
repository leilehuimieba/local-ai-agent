package knowledge

import (
	"database/sql"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
)

func (s *Store) migrateFromJSON(repoRoot string) error {
	oldDir := filepath.Join(repoRoot, "data", "knowledge_base")
	entries, err := os.ReadDir(oldDir)
	if err != nil {
		return nil
	}
	db, err := s.openDB()
	if err != nil {
		return err
	}
	defer db.Close()
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".json") {
			continue
		}
		path := filepath.Join(oldDir, entry.Name())
		data, err := os.ReadFile(path)
		if err != nil {
			continue
		}
		var items []Item
		if err := json.Unmarshal(data, &items); err != nil {
			continue
		}
		workspaceID := strings.TrimSuffix(entry.Name(), ".json")
		for _, item := range items {
			if item.ID == "" {
				continue
			}
			_ = s.upsertItem(db, workspaceID, item)
		}
		_ = os.Rename(path, path+".migrated")
	}
	return nil
}

func (s *Store) upsertItem(db *sql.DB, workspaceID string, item Item) error {
	tagsRaw, _ := json.Marshal(item.Tags)
	_, err := db.Exec(
		`insert into knowledge_items (id, workspace_id, title, summary, content, category, tags, source, citation_count, created_at, updated_at)
		values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
		on conflict(id) do update set
			title=excluded.title, summary=excluded.summary, content=excluded.content,
			category=excluded.category, tags=excluded.tags, source=excluded.source,
			citation_count=excluded.citation_count, updated_at=excluded.updated_at`,
		item.ID, workspaceID, item.Title, item.Summary, item.Content,
		item.Category, string(tagsRaw), item.Source, item.CitationCount,
		item.CreatedAt, item.UpdatedAt,
	)
	return err
}
