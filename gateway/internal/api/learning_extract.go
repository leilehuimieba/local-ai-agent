package api

import (
	"context"
	"net/http"
	"net/url"
	"strings"
	"time"

	"local-agent/gateway/internal/providers/bestblogs"
)

type learningExtractRequest struct {
	ArticleURL      string `json:"article_url"`
	ProviderHint    string `json:"provider_hint,omitempty"`
	Language        string `json:"language,omitempty"`
	IncludeHTML     *bool  `json:"include_html,omitempty"`
	IncludeMarkdown *bool  `json:"include_markdown,omitempty"`
	IncludeImages   *bool  `json:"include_images,omitempty"`
}

func registerLearningRoutes(mux *http.ServeMux, deps memoryRouteDeps) {
	mux.HandleFunc("/api/v1/learning/extract", learningExtractHandler())
	mux.HandleFunc("/api/v1/learning/explain", learningExplainHandler())
	mux.HandleFunc("/api/v1/learning/translate", learningTranslateHandler())
	mux.HandleFunc("/api/v1/learning/value-score", learningValueScoreHandler())
	mux.HandleFunc("/api/v1/learning/recommend", learningRecommendHandler())
	mux.HandleFunc("/api/v1/learning/memory/write", learningMemoryWriteHandler(deps))
	mux.HandleFunc("/api/v1/learning/audit-trace", learningAuditTraceHandler(deps))
	mux.HandleFunc("/api/v1/learning/rollback-check", learningRollbackHandler())
}

func learningExtractHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeLearningExtractRequest(w, r)
		if !ok {
			return
		}
		if resolveLearningProvider(payload.ArticleURL, payload.ProviderHint) != "bestblogs" {
			http.Error(w, "unsupported learning provider", http.StatusBadRequest)
			return
		}
		ctx, cancel := context.WithTimeout(r.Context(), 20*time.Second)
		defer cancel()
		result, err := bestblogsArticleReader(ctx, learningProviderRequest(payload))
		if err != nil {
			writeBestblogsError(w, err)
			return
		}
		writeJSON(w, http.StatusOK, result)
	}
}

func decodeLearningExtractRequest(w http.ResponseWriter, r *http.Request) (learningExtractRequest, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return learningExtractRequest{}, false
	}
	var payload learningExtractRequest
	if !decodeJSONBody(w, r, &payload) {
		return learningExtractRequest{}, false
	}
	if strings.TrimSpace(payload.ArticleURL) == "" {
		http.Error(w, "article_url is required", http.StatusBadRequest)
		return learningExtractRequest{}, false
	}
	return payload, true
}

func resolveLearningProvider(articleURL string, hint string) string {
	trimmed := strings.TrimSpace(strings.ToLower(hint))
	if trimmed == "bestblogs" {
		return "bestblogs"
	}
	if trimmed != "" {
		return ""
	}
	parsed, err := url.Parse(strings.TrimSpace(articleURL))
	if err != nil || parsed.Host == "" {
		return ""
	}
	if strings.Contains(parsed.Host, "bestblogs.dev") {
		return "bestblogs"
	}
	return ""
}

func learningProviderRequest(payload learningExtractRequest) bestblogs.ReadArticleRequest {
	return bestblogs.ReadArticleRequest{
		ArticleURL:      payload.ArticleURL,
		Language:        payload.Language,
		IncludeHTML:     boolOrDefault(payload.IncludeHTML),
		IncludeMarkdown: boolOrDefault(payload.IncludeMarkdown),
		IncludeImages:   boolOrDefault(payload.IncludeImages),
	}
}

func boolOrDefault(value *bool) bool {
	if value == nil {
		return true
	}
	return *value
}
