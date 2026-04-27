package token

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestLoadOrCreate_GeneratesToken(t *testing.T) {
	dir := t.TempDir()
	m, err := LoadOrCreate(dir)
	if err != nil {
		t.Fatalf("LoadOrCreate failed: %v", err)
	}
	if m.Value() == "" {
		t.Fatal("expected non-empty token")
	}
	if len(m.Value()) != 64 {
		t.Fatalf("expected 64 hex chars, got %d", len(m.Value()))
	}
}

func TestLoadOrCreate_ReusesExisting(t *testing.T) {
	dir := t.TempDir()
	m1, err := LoadOrCreate(dir)
	if err != nil {
		t.Fatalf("LoadOrCreate failed: %v", err)
	}
	m2, err := LoadOrCreate(dir)
	if err != nil {
		t.Fatalf("LoadOrCreate failed: %v", err)
	}
	if m1.Value() != m2.Value() {
		t.Fatal("expected same token on second load")
	}
}

func TestMiddleware_HealthExempt(t *testing.T) {
	dir := t.TempDir()
	m, _ := LoadOrCreate(dir)
	handler := m.Middleware(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))

	req := httptest.NewRequest("GET", "/health", nil)
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200 for /health, got %d", rr.Code)
	}
}

func TestMiddleware_APIBlocksWithoutToken(t *testing.T) {
	dir := t.TempDir()
	m, _ := LoadOrCreate(dir)
	handler := m.Middleware(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))

	req := httptest.NewRequest("GET", "/api/v1/settings", nil)
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusUnauthorized {
		t.Fatalf("expected 401 without token, got %d", rr.Code)
	}
}

func TestMiddleware_APIAllowsWithToken(t *testing.T) {
	dir := t.TempDir()
	m, _ := LoadOrCreate(dir)
	handler := m.Middleware(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))

	req := httptest.NewRequest("GET", "/api/v1/settings", nil)
	req.Header.Set("X-Local-Agent-Token", m.Value())
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200 with token, got %d", rr.Code)
	}
}
