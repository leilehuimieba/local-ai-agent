package api

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestReleaseRunHandlerRejectsGet(t *testing.T) {
	handler := releaseRunHandler("D:/repo")
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodGet, "/api/v1/release/run", nil)
	handler(recorder, req)
	require.Equal(t, http.StatusMethodNotAllowed, recorder.Code)
}

func TestReleaseRunHandlerRejectsUnsupportedStep(t *testing.T) {
	handler := releaseRunHandler("D:/repo")
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/release/run", strings.NewReader(`{"step":"bad"}`))
	handler(recorder, req)
	require.Equal(t, http.StatusBadRequest, recorder.Code)
}
