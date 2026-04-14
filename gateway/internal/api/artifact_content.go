package api

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

type artifactContentResponse struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

func artifactContentHandler(repoRoot string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		payload, status, err := buildArtifactContentResponse(repoRoot, r.URL.Query().Get("path"))
		if err != nil {
			http.Error(w, err.Error(), status)
			return
		}
		writeJSON(w, http.StatusOK, payload)
	}
}

func buildArtifactContentResponse(
	repoRoot string,
	rawPath string,
) (artifactContentResponse, int, error) {
	if strings.TrimSpace(rawPath) == "" {
		return artifactContentResponse{}, http.StatusBadRequest, fmt.Errorf("path is required")
	}
	path, err := resolveArtifactPath(repoRoot, rawPath)
	if err != nil {
		return artifactContentResponse{}, http.StatusBadRequest, err
	}
	content, err := os.ReadFile(path)
	if err != nil {
		return artifactContentResponse{}, http.StatusNotFound, fmt.Errorf("artifact not found")
	}
	return artifactContentResponse{Path: path, Content: string(content)}, http.StatusOK, nil
}

func resolveArtifactPath(repoRoot string, rawPath string) (string, error) {
	artifactRoot := filepath.Join(repoRoot, "data", "artifacts")
	path := rawPath
	if !filepath.IsAbs(path) {
		path = filepath.Join(repoRoot, path)
	}
	clean := filepath.Clean(path)
	rel, err := filepath.Rel(artifactRoot, clean)
	if err != nil {
		return "", fmt.Errorf("invalid artifact path")
	}
	if rel == "." {
		return "", fmt.Errorf("artifact path must be file")
	}
	if strings.HasPrefix(rel, "..") || filepath.IsAbs(rel) {
		return "", fmt.Errorf("artifact path out of scope")
	}
	return clean, nil
}

