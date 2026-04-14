package api

import (
	"path/filepath"
	"testing"
)

func TestResolveArtifactPathAcceptsPathUnderArtifactRoot(t *testing.T) {
	repoRoot := t.TempDir()
	raw := filepath.Join(repoRoot, "data", "artifacts", "s1", "r1", "out.txt")
	path, err := resolveArtifactPath(repoRoot, raw)
	if err != nil {
		t.Fatalf("resolveArtifactPath() error = %v", err)
	}
	if path != raw {
		t.Fatalf("resolveArtifactPath() = %q, want %q", path, raw)
	}
}

func TestResolveArtifactPathRejectsPathOutOfArtifactRoot(t *testing.T) {
	repoRoot := t.TempDir()
	raw := filepath.Join(repoRoot, "data", "other", "out.txt")
	_, err := resolveArtifactPath(repoRoot, raw)
	if err == nil {
		t.Fatal("resolveArtifactPath() expected error for out-of-scope path")
	}
}

