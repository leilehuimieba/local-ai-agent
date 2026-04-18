package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"local-agent/gateway/internal/providers/bestblogs"
)

type rollbackEvidenceCase struct {
	ArticleURL      string          `json:"article_url"`
	ArticleID       string          `json:"article_id"`
	Title           string          `json:"title"`
	OK              bool            `json:"ok"`
	Checks          map[string]bool `json:"checks"`
	FallbackMode    string          `json:"fallback_mode"`
	AllowedActions  []string        `json:"allowed_actions"`
	DisabledActions []string        `json:"disabled_actions"`
}

type rollbackEvidenceReport struct {
	GeneratedAt      string                 `json:"generated_at"`
	TotalSamples     int                    `json:"total_samples"`
	SuccessCount     int                    `json:"success_count"`
	RollbackPassRate float64                `json:"rollback_pass_rate"`
	Threshold        float64                `json:"threshold"`
	Passed           bool                   `json:"passed"`
	Samples          []rollbackEvidenceCase `json:"samples"`
}

func TestBuildLearningRollbackResponse(t *testing.T) {
	article := bestblogs.ArticleResponse{
		Provider: "bestblogs", Strategy: "public_api", ArticleID: "42acaf7d",
		Meta:    bestblogs.ArticleMeta{Title: "浏览器自动化：从 GUI 到 OpenCLI", Tags: []string{"浏览器自动化", "AI Agent", "OpenCLI"}},
		Summary: bestblogs.ArticleSummary{MainPoints: []bestblogs.MainPoint{{Point: "为什么我们需要浏览器自动化"}}},
		Content: bestblogs.ArticleContent{Markdown: stringsRepeat("浏览器自动化 OpenCLI Agent ", 180), Images: []string{"a"}},
	}
	resp := buildLearningRollbackResponse(article, learningRollbackRequest{Language: "zh", LearningModeEnabled: false})
	require.True(t, resp.RollbackApplied)
	require.Equal(t, "explain_translate_only", resp.FallbackMode)
	require.Contains(t, resp.DisabledActions, "memory_write")
}

func TestGenerateLearningRollbackEvidence(t *testing.T) {
	report := buildLearningRollbackReport(t)
	writeLearningRollbackEvidence(t, report)
	require.GreaterOrEqual(t, report.RollbackPassRate, 1.0)
}

func buildLearningRollbackReport(t *testing.T) rollbackEvidenceReport {
	samples := make([]rollbackEvidenceCase, 0, len(bestblogsExtractEvidenceInputs))
	for _, item := range bestblogsExtractEvidenceInputs {
		samples = append(samples, runLearningRollbackSample(t, item))
	}
	success := countRollbackSuccess(samples)
	rate := successRate(success, len(samples))
	return rollbackEvidenceReport{
		GeneratedAt: time.Now().Format(time.RFC3339), TotalSamples: len(samples),
		SuccessCount: success, RollbackPassRate: rate, Threshold: 1, Passed: rate >= 1, Samples: samples,
	}
}

func runLearningRollbackSample(t *testing.T, sample extractEvidenceInput) rollbackEvidenceCase {
	resp := requestLearningRollbackResponse(t, sample)
	checks := rollbackChecks(resp)
	return rollbackEvidenceCase{
		ArticleURL: sample.ArticleURL, ArticleID: resp.ArticleID, Title: resp.Title,
		OK: allChecksPass(checks), Checks: checks, FallbackMode: resp.FallbackMode,
		AllowedActions: resp.AllowedActions, DisabledActions: resp.DisabledActions,
	}
}

func requestLearningRollbackResponse(t *testing.T, sample extractEvidenceInput) learningRollbackResponse {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/learning/rollback-check", bytes.NewReader(learningRollbackBody(t, sample)))
	req.Header.Set("Content-Type", "application/json")
	learningRollbackHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusOK, recorder.Code)
	return decodeLearningRollbackResponse(t, recorder.Body.Bytes())
}

func learningRollbackBody(t *testing.T, sample extractEvidenceInput) []byte {
	body, err := json.Marshal(learningRollbackRequest{
		ArticleURL: sample.ArticleURL, ProviderHint: "bestblogs", Language: "zh", LearningModeEnabled: false,
	})
	require.NoError(t, err)
	return body
}

func decodeLearningRollbackResponse(t *testing.T, body []byte) learningRollbackResponse {
	var resp learningRollbackResponse
	require.NoError(t, json.Unmarshal(body, &resp))
	return resp
}

func rollbackChecks(resp learningRollbackResponse) map[string]bool {
	return map[string]bool{
		"ok":               resp.OK,
		"rollback_applied": resp.RollbackApplied,
		"learning_off":     !resp.LearningModeOn,
		"fallback_mode":    resp.FallbackMode == "explain_translate_only",
		"explain":          resp.Explain.Explain != "",
		"translate":        resp.Translate.Summary != "",
		"no_value_score":   !containsAction(resp.AllowedActions, "value_score"),
		"no_memory_write":  containsAction(resp.DisabledActions, "memory_write"),
	}
}

func containsAction(items []string, target string) bool {
	for _, item := range items {
		if item == target {
			return true
		}
	}
	return false
}

func countRollbackSuccess(samples []rollbackEvidenceCase) int {
	count := 0
	for _, sample := range samples {
		if sample.OK {
			count++
		}
	}
	return count
}

func writeLearningRollbackEvidence(t *testing.T, report rollbackEvidenceReport) {
	data, err := json.MarshalIndent(report, "", "  ")
	require.NoError(t, err)
	path := filepath.Join("..", "..", "..", "tmp", "stage-h-learning", "rollback.json")
	require.NoError(t, os.MkdirAll(filepath.Dir(path), 0o755))
	require.NoError(t, os.WriteFile(path, data, 0o644))
}
