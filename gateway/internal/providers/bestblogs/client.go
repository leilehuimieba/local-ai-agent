package bestblogs

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
	"time"
)

const defaultBaseURL = "https://www.bestblogs.dev"

type Client struct {
	baseURL    string
	httpClient *http.Client
}

func NewClient(httpClient *http.Client) *Client {
	if httpClient == nil {
		httpClient = &http.Client{Timeout: 15 * time.Second}
	}
	return &Client{baseURL: defaultBaseURL, httpClient: httpClient}
}

func (c *Client) ReadArticle(ctx context.Context, req ReadArticleRequest) (ArticleResponse, error) {
	articleID, err := extractArticleID(req.ArticleURL)
	if err != nil {
		return ArticleResponse{}, err
	}
	envelope, err := c.fetchArticle(ctx, articleID, normalizeLanguage(req.Language))
	if err != nil {
		return ArticleResponse{}, err
	}
	if !envelope.Success {
		return ArticleResponse{}, newUpstreamNotSuccessError()
	}
	if strings.TrimSpace(envelope.Data.ContentData.DisplayDocument) == "" {
		return ArticleResponse{}, newEmptyContentError()
	}
	return normalizeArticle(articleID, envelope, req), nil
}

func extractArticleID(articleURL string) (string, error) {
	parsed, err := url.Parse(strings.TrimSpace(articleURL))
	if err != nil || parsed.Host == "" {
		return "", newInvalidInputError("article_url 无效")
	}
	if !strings.Contains(parsed.Host, "bestblogs.dev") {
		return "", newInvalidInputError("article_url 不是 BestBlogs 文章地址")
	}
	parts := strings.Split(strings.Trim(parsed.Path, "/"), "/")
	if len(parts) != 2 || parts[0] != "article" || strings.TrimSpace(parts[1]) == "" {
		return "", newInvalidInputError("article_url 缺少 article_id")
	}
	return parts[1], nil
}

func normalizeLanguage(language string) string {
	if strings.TrimSpace(language) == "" {
		return "zh"
	}
	return language
}

func (c *Client) fetchArticle(
	ctx context.Context,
	articleID string,
	language string,
) (upstreamEnvelope, error) {
	target := fmt.Sprintf("%s/api/proxy/resources/%s?language=%s", c.baseURL, articleID, url.QueryEscape(language))
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, target, nil)
	if err != nil {
		return upstreamEnvelope{}, newInvalidInputError("BestBlogs 请求构造失败")
	}
	resp, err := c.httpClient.Do(req)
	if err != nil {
		return upstreamEnvelope{}, newUpstreamHTTPError(http.StatusBadGateway, err)
	}
	defer resp.Body.Close()
	return decodeEnvelope(resp)
}

func decodeEnvelope(resp *http.Response) (upstreamEnvelope, error) {
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return upstreamEnvelope{}, newDecodeError(err)
	}
	if resp.StatusCode < http.StatusOK || resp.StatusCode >= http.StatusMultipleChoices {
		return upstreamEnvelope{}, newUpstreamHTTPError(resp.StatusCode, nil)
	}
	clean := bytes.ToValidUTF8(body, []byte{})
	var envelope upstreamEnvelope
	if err := json.Unmarshal(clean, &envelope); err != nil {
		return upstreamEnvelope{}, newDecodeError(err)
	}
	return envelope, nil
}
