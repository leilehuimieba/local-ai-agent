package api

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestReleaseSpecAllowsOnlyKnownSteps(t *testing.T) {
	_, err := releaseSpec("D:/repo", "unknown")
	require.Error(t, err)
}

func TestReleaseSpecUsesWhitelistedScript(t *testing.T) {
	spec, err := releaseSpec("D:/repo", "doctor")
	require.NoError(t, err)
	require.Equal(t, "doctor.ps1", spec.Script)
	require.Contains(t, spec.Args, "-OutFile")
}

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
