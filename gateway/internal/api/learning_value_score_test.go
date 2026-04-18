package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/require"

	"local-agent/gateway/internal/providers/bestblogs"
)

type valueScoreEvidence struct {
	ArticleURL string                     `json:"article_url"`
	Checks     map[string]bool            `json:"checks"`
	Response   learningValueScoreResponse `json:"response"`
}

func TestScoreLearningArticle(t *testing.T) {
	article := bestblogs.ArticleResponse{
		Provider: "bestblogs", Strategy: "public_api", ArticleID: "42acaf7d",
		Meta:    bestblogs.ArticleMeta{Title: "浏览器自动化：从 GUI 到 OpenCLI", Tags: []string{"浏览器自动化", "AI Agent", "OpenCLI", "RPA"}},
		Summary: bestblogs.ArticleSummary{MainPoints: []bestblogs.MainPoint{{}, {}, {}, {}}, Full: "面向 Agent 的自动化与软件工程。"},
		Content: bestblogs.ArticleContent{Markdown: stringsRepeat("浏览器自动化 OpenCLI Agent 软件工程 ", 220), Images: []string{"a", "b", "c"}},
	}
	result := scoreLearningArticle(article)
	require.Equal(t, "high", result.Level)
	require.GreaterOrEqual(t, result.Score, 85)
	require.NotEmpty(t, result.Reason)
	require.NotEmpty(t, result.NextAction)
}

func TestGenerateLearningValueScoreEvidence(t *testing.T) {
	evidence := buildLearningValueEvidence(t)
	writeLearningValueEvidence(t, evidence)
	require.True(t, evidence.Checks["article_id"])
	require.True(t, evidence.Checks["reason"])
	require.True(t, evidence.Checks["next_action"])
}

func buildLearningValueEvidence(t *testing.T) valueScoreEvidence {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/learning/value-score", bytes.NewReader(valueScoreRequestBody(t)))
	req.Header.Set("Content-Type", "application/json")
	learningValueScoreHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusOK, recorder.Code)
	response := decodeLearningValueResponse(t, recorder.Body.Bytes())
	return valueScoreEvidence{ArticleURL: bestblogsExtractEvidenceInputs[0].ArticleURL, Checks: valueScoreChecks(response), Response: response}
}

func valueScoreRequestBody(t *testing.T) []byte {
	body, err := json.Marshal(learningExtractRequest{
		ArticleURL: bestblogsExtractEvidenceInputs[0].ArticleURL, ProviderHint: "bestblogs", Language: "zh",
	})
	require.NoError(t, err)
	return body
}

func decodeLearningValueResponse(t *testing.T, body []byte) learningValueScoreResponse {
	var response learningValueScoreResponse
	require.NoError(t, json.Unmarshal(body, &response))
	return response
}

func valueScoreChecks(response learningValueScoreResponse) map[string]bool {
	return map[string]bool{
		"ok":          response.OK,
		"provider":    response.Provider == "bestblogs",
		"article_id":  response.ArticleID == "42acaf7d",
		"score":       response.Score >= 70,
		"level":       response.Level != "",
		"reason":      response.Reason != "",
		"next_action": response.NextAction != "",
	}
}

func writeLearningValueEvidence(t *testing.T, evidence valueScoreEvidence) {
	data, err := json.MarshalIndent(evidence, "", "  ")
	require.NoError(t, err)
	path := filepath.Join("..", "..", "..", "tmp", "stage-h-learning", "value-score.json")
	require.NoError(t, os.MkdirAll(filepath.Dir(path), 0o755))
	require.NoError(t, os.WriteFile(path, data, 0o644))
}

func stringsRepeat(value string, count int) string {
	result := ""
	for i := 0; i < count; i++ {
		result += value
	}
	return result
}
