package memory

import (
	"os"
	"path/filepath"
	"testing"
)

func TestSafeName(t *testing.T) {
	if got := safeName("ws/测试:1"); got != "ws____1" {
		t.Fatalf("unexpected safe name: %s", got)
	}
}

func TestAppendTombstone(t *testing.T) {
	root := t.TempDir()
	store := &Store{storageRoot: root}
	if err := store.appendTombstone("demo", "memory-1"); err != nil {
		t.Fatalf("append tombstone: %v", err)
	}
	path := filepath.Join(root, "memory", "deletions", "demo.jsonl")
	if _, err := os.Stat(path); err != nil {
		t.Fatalf("expected tombstone file: %v", err)
	}
	deleted := store.deletedIDs("demo")
	if !deleted["memory-1"] {
		t.Fatalf("expected deleted id to be recorded")
	}
}
