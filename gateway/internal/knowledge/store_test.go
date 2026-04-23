package knowledge

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestStore_CRUD(t *testing.T) {
	tmp := t.TempDir()
	store := NewStore(tmp)

	// Create
	item, err := store.Create("ws1", CreateRequest{Title: "t1", Summary: "s1", Content: "c1", Category: "cat1", Tags: []string{"a", "b"}})
	require.NoError(t, err)
	require.NotEmpty(t, item.ID)
	assert.Equal(t, "t1", item.Title)

	// Get
	got, err := store.Get("ws1", item.ID)
	require.NoError(t, err)
	assert.Equal(t, item.ID, got.ID)

	// List
	items, err := store.List("ws1")
	require.NoError(t, err)
	assert.Len(t, items, 1)

	// Update
	updated, err := store.Update("ws1", item.ID, UpdateRequest{Title: "t2"})
	require.NoError(t, err)
	assert.Equal(t, "t2", updated.Title)

	// Search
	found, err := store.Search("ws1", "t2")
	require.NoError(t, err)
	assert.Len(t, found, 1)

	// Categories & Tags
	cats, err := store.Categories("ws1")
	require.NoError(t, err)
	assert.Contains(t, cats, "cat1")
	tags, err := store.Tags("ws1")
	require.NoError(t, err)
	assert.Contains(t, tags, "a")
	assert.Contains(t, tags, "b")

	// Delete
	err = store.Delete("ws1", item.ID)
	require.NoError(t, err)
	_, err = store.Get("ws1", item.ID)
	assert.Error(t, err)
}

func TestStore_MigrateFromJSON(t *testing.T) {
	tmp := t.TempDir()
	oldDir := filepath.Join(tmp, "data", "knowledge_base")
	require.NoError(t, os.MkdirAll(oldDir, 0o755))

	payload := []byte(`[{"id":"kb_old_1","title":"legacy","summary":"s","content":"c","category":"doc","tags":["x"],"source":"src","citation_count":3,"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]`)
	require.NoError(t, os.WriteFile(filepath.Join(oldDir, "ws_legacy.json"), payload, 0o644))

	store := NewStore(tmp)
	items, err := store.List("ws_legacy")
	require.NoError(t, err)
	require.Len(t, items, 1)
	assert.Equal(t, "legacy", items[0].Title)
	assert.Equal(t, []string{"x"}, items[0].Tags)
	assert.Equal(t, 3, items[0].CitationCount)

	_, err = os.Stat(filepath.Join(oldDir, "ws_legacy.json"))
	assert.True(t, os.IsNotExist(err))
}
