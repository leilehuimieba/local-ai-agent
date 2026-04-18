package api

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/require"

	"local-agent/gateway/internal/providers/bestblogs"
)

func TestBestblogsArticleReadHandler(t *testing.T) {
	restore := swapBestblogsArticleReader(func(ctx context.Context, req bestblogs.ReadArticleRequest) (bestblogs.ArticleResponse, error) {
		return bestblogs.ArticleResponse{
			OK: true, Provider: "bestblogs", Strategy: "public_api", ArticleID: "42acaf7d",
			Meta:    bestblogs.ArticleMeta{Title: "浏览器自动化：从 GUI 到 OpenCLI"},
			Summary: bestblogs.ArticleSummary{MainPoints: []bestblogs.MainPoint{{Point: "a"}}},
			Content: bestblogs.ArticleContent{HTML: "<p>x</p>", Markdown: "为什么我们需要浏览器自动化\n未来软件竞争维度", Images: []string{"img"}},
		}, nil
	})
	defer restore()
	body, err := json.Marshal(bestblogs.ReadArticleRequest{ArticleURL: "https://www.bestblogs.dev/article/42acaf7d", Language: "zh"})
	require.NoError(t, err)
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/providers/bestblogs/article/read", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	bestblogsArticleReadHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusOK, recorder.Code)
	var resp bestblogs.ArticleResponse
	require.NoError(t, json.Unmarshal(recorder.Body.Bytes(), &resp))
	require.Equal(t, "42acaf7d", resp.ArticleID)
	require.Equal(t, "浏览器自动化：从 GUI 到 OpenCLI", resp.Meta.Title)
	require.NotEmpty(t, resp.Summary.MainPoints)
	require.NotEmpty(t, resp.Content.HTML)
	require.NotEmpty(t, resp.Content.Images)
}

func TestBestblogsArticleReadHandlerReturnsProviderError(t *testing.T) {
	restore := swapBestblogsArticleReader(func(ctx context.Context, req bestblogs.ReadArticleRequest) (bestblogs.ArticleResponse, error) {
		return bestblogs.ArticleResponse{}, bestblogs.Error{Code: bestblogs.ErrInvalidInput, Message: "article_url 无效", Status: http.StatusBadRequest}
	})
	defer restore()
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/providers/bestblogs/article/read", bytes.NewBufferString(`{"article_url":"bad"}`))
	req.Header.Set("Content-Type", "application/json")
	bestblogsArticleReadHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusBadRequest, recorder.Code)
	require.Contains(t, recorder.Body.String(), "\"error_code\":\"BESTBLOGS_INVALID_INPUT\"")
	require.Contains(t, recorder.Body.String(), "\"message\":\"article_url 无效\"")
}

func TestBestblogsArticleReadHandlerRejectsMethod(t *testing.T) {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodGet, "/api/v1/providers/bestblogs/article/read", nil)
	bestblogsArticleReadHandler().ServeHTTP(recorder, req)
	require.Equal(t, http.StatusMethodNotAllowed, recorder.Code)
	require.Contains(t, recorder.Body.String(), "method not allowed")
}
