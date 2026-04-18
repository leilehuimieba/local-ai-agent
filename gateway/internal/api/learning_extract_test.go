package api

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"local-agent/gateway/internal/providers/bestblogs"
)

type extractEvidenceInput struct {
	ArticleURL      string
	ExpectArticleID string
	ExpectTitle     string
	ExpectPhrases   []string
}

type extractEvidenceCase struct {
	ArticleURL string          `json:"article_url"`
	ArticleID  string          `json:"article_id,omitempty"`
	Title      string          `json:"title,omitempty"`
	OK         bool            `json:"ok"`
	StatusCode int             `json:"status_code"`
	Error      string          `json:"error,omitempty"`
	Checks     map[string]bool `json:"checks"`
}

type extractEvidenceReport struct {
	GeneratedAt  string                `json:"generated_at"`
	Endpoint     string                `json:"endpoint"`
	Provider     string                `json:"provider"`
	Strategy     string                `json:"strategy"`
	TotalSamples int                   `json:"total_samples"`
	SuccessCount int                   `json:"success_count"`
	FailureCount int                   `json:"failure_count"`
	SuccessRate  float64               `json:"success_rate"`
	Threshold    float64               `json:"threshold"`
	Passed       bool                  `json:"passed"`
	Samples      []extractEvidenceCase `json:"samples"`
}

var bestblogsExtractEvidenceInputs = []extractEvidenceInput{
	{
		ArticleURL:      "https://www.bestblogs.dev/article/42acaf7d?entry=resource_card&from=%2Fexplore%2Fbrief",
		ExpectArticleID: "42acaf7d",
		ExpectTitle:     "浏览器自动化：从 GUI 到 OpenCLI",
		ExpectPhrases:   []string{"为什么我们需要浏览器自动化", "未来软件竞争维度"},
	},
	{ArticleURL: "https://www.bestblogs.dev/article/4e45fa?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/aaa15c?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/2de06daf?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/c5df0b?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/539020?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/908c12?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/33a85144?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/c3b1e5?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/baab8fef?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/cf9e79?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle"},
	{ArticleURL: "https://www.bestblogs.dev/article/2601db?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
	{ArticleURL: "https://www.bestblogs.dev/article/fda3d8?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
	{ArticleURL: "https://www.bestblogs.dev/article/657929?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
	{ArticleURL: "https://www.bestblogs.dev/article/6048ef?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
	{ArticleURL: "https://www.bestblogs.dev/article/f24b52?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
	{ArticleURL: "https://www.bestblogs.dev/article/c5766e97?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
	{ArticleURL: "https://www.bestblogs.dev/article/9e0fdd94?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
	{ArticleURL: "https://www.bestblogs.dev/article/72bf05?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
	{ArticleURL: "https://www.bestblogs.dev/article/5c79977a?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
	{ArticleURL: "https://www.bestblogs.dev/article/c7c9ec14?entry=resource_card&from=%2Fexplore%3Ftime%3D7d%26qualified%3Dtrue%26type%3Darticle%26page%3D2"},
}

func TestResolveLearningProvider(t *testing.T) {
	provider := resolveLearningProvider("https://www.bestblogs.dev/article/42acaf7d", "")
	require.Equal(t, "bestblogs", provider)
	require.Equal(t, "bestblogs", resolveLearningProvider("", "bestblogs"))
	require.Empty(t, resolveLearningProvider("https://example.com/article/1", ""))
}

func TestLearningProviderRequestDefaults(t *testing.T) {
	req := learningProviderRequest(learningExtractRequest{ArticleURL: "u", Language: "zh"})
	require.True(t, req.IncludeHTML)
	require.True(t, req.IncludeMarkdown)
	require.True(t, req.IncludeImages)
}

func TestLearningProviderRequestAllowsDisable(t *testing.T) {
	disabled := false
	req := learningProviderRequest(learningExtractRequest{
		ArticleURL: "u", IncludeHTML: &disabled, IncludeImages: &disabled,
	})
	require.False(t, req.IncludeHTML)
	require.True(t, req.IncludeMarkdown)
	require.False(t, req.IncludeImages)
}

func TestLearningExtractHandler(t *testing.T) {
	restore := swapBestblogsArticleReader(func(ctx context.Context, req bestblogs.ReadArticleRequest) (bestblogs.ArticleResponse, error) {
		return bestblogs.ArticleResponse{
			OK: true, Provider: "bestblogs", Strategy: "public_api", ArticleID: "42acaf7d",
			Meta:    bestblogs.ArticleMeta{Title: "浏览器自动化：从 GUI 到 OpenCLI"},
			Summary: bestblogs.ArticleSummary{MainPoints: []bestblogs.MainPoint{{Point: "a"}}},
			Content: bestblogs.ArticleContent{HTML: "<p>x</p>", Markdown: "为什么我们需要浏览器自动化\n未来软件竞争维度", Images: []string{"img"}},
		}, nil
	})
	defer restore()
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/learning/extract", bytes.NewBufferString(`{"article_url":"https://www.bestblogs.dev/article/42acaf7d","language":"zh"}`))
	req.Header.Set("Content-Type", "application/json")
	learningExtractHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusOK, recorder.Code)
	var resp bestblogs.ArticleResponse
	require.NoError(t, json.Unmarshal(recorder.Body.Bytes(), &resp))
	require.Equal(t, "42acaf7d", resp.ArticleID)
	require.Equal(t, "浏览器自动化：从 GUI 到 OpenCLI", resp.Meta.Title)
}

func TestLearningExtractHandlerRejectsUnsupportedProvider(t *testing.T) {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/learning/extract", bytes.NewBufferString(`{"article_url":"https://example.com/article/1"}`))
	req.Header.Set("Content-Type", "application/json")
	learningExtractHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusBadRequest, recorder.Code)
	require.Contains(t, recorder.Body.String(), "unsupported learning provider")
}

func TestLearningExtractHandlerRejectsMethod(t *testing.T) {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodGet, "/api/v1/learning/extract", nil)
	learningExtractHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusMethodNotAllowed, recorder.Code)
	require.Contains(t, recorder.Body.String(), "method not allowed")
}

func TestLearningExtractHandlerRequiresArticleURL(t *testing.T) {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/learning/extract", bytes.NewBufferString(`{"provider_hint":"bestblogs"}`))
	req.Header.Set("Content-Type", "application/json")
	learningExtractHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusBadRequest, recorder.Code)
	require.Contains(t, recorder.Body.String(), "article_url is required")
}

func TestLearningExtractHandlerReturnsProviderError(t *testing.T) {
	restore := swapBestblogsArticleReader(func(ctx context.Context, req bestblogs.ReadArticleRequest) (bestblogs.ArticleResponse, error) {
		return bestblogs.ArticleResponse{}, bestblogs.Error{Code: bestblogs.ErrInvalidInput, Message: "article_url 无效", Status: http.StatusBadRequest}
	})
	defer restore()
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/learning/extract", bytes.NewBufferString(`{"article_url":"bad","provider_hint":"bestblogs"}`))
	req.Header.Set("Content-Type", "application/json")
	learningExtractHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusBadRequest, recorder.Code)
	require.Contains(t, recorder.Body.String(), "\"error_code\":\"BESTBLOGS_INVALID_INPUT\"")
	require.Contains(t, recorder.Body.String(), "\"message\":\"article_url 无效\"")
}

func TestGenerateLearningExtractEvidence(t *testing.T) {
	report := buildExtractEvidence(t)
	writeExtractEvidence(t, report)
	require.GreaterOrEqual(t, report.SuccessRate, 0.95)
}

func buildExtractEvidence(t *testing.T) extractEvidenceReport {
	handler := learningExtractHandler()
	results := make([]extractEvidenceCase, 0, len(bestblogsExtractEvidenceInputs))
	for _, sample := range bestblogsExtractEvidenceInputs {
		results = append(results, runExtractSample(t, handler, sample))
	}
	return summarizeExtractEvidence(results)
}

func runExtractSample(t *testing.T, handler http.Handler, sample extractEvidenceInput) extractEvidenceCase {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/learning/extract", bytes.NewReader(extractRequestBody(t, sample)))
	req.Header.Set("Content-Type", "application/json")
	handler.ServeHTTP(recorder, req)
	if recorder.Code != http.StatusOK {
		return extractEvidenceCase{ArticleURL: sample.ArticleURL, OK: false, StatusCode: recorder.Code, Error: strings.TrimSpace(recorder.Body.String())}
	}
	response, err := decodeExtractResponse(recorder.Body.Bytes())
	if err != nil {
		return extractEvidenceCase{ArticleURL: sample.ArticleURL, OK: false, StatusCode: recorder.Code, Error: err.Error()}
	}
	checks := extractChecks(sample, response)
	return extractEvidenceCase{
		ArticleURL: sample.ArticleURL, ArticleID: response.ArticleID, Title: response.Meta.Title,
		OK: allChecksPass(checks), StatusCode: recorder.Code, Checks: checks,
	}
}

func extractRequestBody(t *testing.T, sample extractEvidenceInput) []byte {
	body, err := json.Marshal(learningExtractRequest{
		ArticleURL: sample.ArticleURL, ProviderHint: "bestblogs", Language: "zh",
	})
	require.NoError(t, err)
	return body
}

func decodeExtractResponse(body []byte) (bestblogs.ArticleResponse, error) {
	var response bestblogs.ArticleResponse
	err := json.Unmarshal(body, &response)
	return response, err
}

func extractChecks(sample extractEvidenceInput, response bestblogs.ArticleResponse) map[string]bool {
	checks := map[string]bool{
		"ok":                  response.OK,
		"provider":            response.Provider == "bestblogs",
		"strategy":            response.Strategy == "public_api",
		"article_id":          matchesExpected(sample.ExpectArticleID, response.ArticleID),
		"title":               strings.TrimSpace(response.Meta.Title) != "",
		"summary_main_points": len(response.Summary.MainPoints) > 0,
		"content_html":        strings.TrimSpace(response.Content.HTML) != "",
		"content_markdown":    strings.TrimSpace(response.Content.Markdown) != "",
		"content_images":      len(response.Content.Images) > 0,
		"utf8_clean":          !strings.Contains(response.Meta.Title+response.Content.Markdown, "�"),
	}
	if sample.ExpectTitle != "" {
		checks["expected_title"] = response.Meta.Title == sample.ExpectTitle
	}
	for i, phrase := range sample.ExpectPhrases {
		checks[fmt.Sprintf("phrase_%d", i+1)] = strings.Contains(response.Content.Markdown, phrase)
	}
	return checks
}

func matchesExpected(expected string, value string) bool {
	if strings.TrimSpace(expected) == "" {
		return strings.TrimSpace(value) != ""
	}
	return value == expected
}

func allChecksPass(checks map[string]bool) bool {
	for _, ok := range checks {
		if !ok {
			return false
		}
	}
	return true
}

func summarizeExtractEvidence(results []extractEvidenceCase) extractEvidenceReport {
	successCount := 0
	for _, result := range results {
		if result.OK {
			successCount++
		}
	}
	total := len(results)
	rate := 0.0
	if total > 0 {
		rate = float64(successCount) / float64(total)
	}
	return extractEvidenceReport{
		GeneratedAt: time.Now().Format(time.RFC3339), Endpoint: "/api/v1/learning/extract",
		Provider: "bestblogs", Strategy: "public_api", TotalSamples: total,
		SuccessCount: successCount, FailureCount: total - successCount, SuccessRate: rate,
		Threshold: 0.95, Passed: rate >= 0.95, Samples: results,
	}
}

func writeExtractEvidence(t *testing.T, report extractEvidenceReport) {
	data, err := json.MarshalIndent(report, "", "  ")
	require.NoError(t, err)
	path := filepath.Join("..", "..", "..", "tmp", "stage-h-learning", "extract.json")
	require.NoError(t, os.MkdirAll(filepath.Dir(path), 0o755))
	require.NoError(t, os.WriteFile(path, data, 0o644))
}
