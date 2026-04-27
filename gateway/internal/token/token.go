package token

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

const tokenFileName = ".gateway_token"
const headerName = "X-Local-Agent-Token"

// Manager holds the gateway authentication token.
type Manager struct {
	value string
}

// LoadOrCreate reads an existing token from disk or generates a new one.
func LoadOrCreate(repoRoot string) (*Manager, error) {
	dataDir := filepath.Join(repoRoot, "data")
	if err := os.MkdirAll(dataDir, 0o755); err != nil {
		return nil, fmt.Errorf("create data dir: %w", err)
	}
	path := filepath.Join(dataDir, tokenFileName)

	if raw, err := os.ReadFile(path); err == nil && len(raw) > 0 {
		return &Manager{value: strings.TrimSpace(string(raw))}, nil
	}

	b := make([]byte, 32)
	if _, err := rand.Read(b); err != nil {
		return nil, fmt.Errorf("generate token: %w", err)
	}
	tok := hex.EncodeToString(b)
	if err := os.WriteFile(path, []byte(tok+"\n"), 0o600); err != nil {
		return nil, fmt.Errorf("write token: %w", err)
	}
	return &Manager{value: tok}, nil
}

// Value returns the token string.
func (m *Manager) Value() string {
	return m.value
}

// Middleware returns an HTTP middleware that validates the token header.
// The health endpoint is always exempt.
func (m *Manager) Middleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/health" {
			next.ServeHTTP(w, r)
			return
		}
		if strings.HasPrefix(r.URL.Path, "/api/") {
			if r.Header.Get(headerName) != m.value {
				w.WriteHeader(http.StatusUnauthorized)
				_, _ = w.Write([]byte(`{"error":"unauthorized"}`))
				return
			}
		}
		next.ServeHTTP(w, r)
	})
}
