package api

import (
	"context"
	"net/http"
	"strings"
	"time"

	"local-agent/gateway/internal/providers/bestblogs"
	"local-agent/gateway/internal/service"
)

type learningTranslateRequest struct {
	ArticleURL     string `json:"article_url"`
	ProviderHint   string `json:"provider_hint,omitempty"`
	Language       string `json:"language,omitempty"`
	TargetLanguage string `json:"target_language,omitempty"`
}

type learningExplainResponse = service.LearningExplainResponse
type learningExplainTerm = service.LearningExplainTerm
type learningTranslateResponse = service.LearningTranslateResponse

func learningExplainHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeLearningExtractRequest(w, r)
		if !ok {
			return
		}
		article, ok := readLearningArticle(w, r, payload)
		if !ok {
			return
		}
		writeJSON(w, http.StatusOK, buildLearningExplain(article))
	}
}

func learningTranslateHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeLearningTranslateRequest(w, r)
		if !ok {
			return
		}
		article, ok := readLearningArticle(w, r, learningTranslatePayload(payload))
		if !ok {
			return
		}
		writeJSON(w, http.StatusOK, buildLearningTranslate(article, payload))
	}
}

func learningTranslatePayload(payload learningTranslateRequest) learningExtractRequest {
	return learningExtractRequest{
		ArticleURL: payload.ArticleURL, ProviderHint: payload.ProviderHint, Language: payload.Language,
	}
}

func decodeLearningTranslateRequest(w http.ResponseWriter, r *http.Request) (learningTranslateRequest, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return learningTranslateRequest{}, false
	}
	var payload learningTranslateRequest
	if !decodeJSONBody(w, r, &payload) {
		return learningTranslateRequest{}, false
	}
	if strings.TrimSpace(payload.ArticleURL) == "" {
		http.Error(w, "article_url is required", http.StatusBadRequest)
		return learningTranslateRequest{}, false
	}
	return payload, true
}

func readLearningArticle(w http.ResponseWriter, r *http.Request, payload learningExtractRequest) (bestblogs.ArticleResponse, bool) {
	if resolveLearningProvider(payload.ArticleURL, payload.ProviderHint) != "bestblogs" {
		http.Error(w, "unsupported learning provider", http.StatusBadRequest)
		return bestblogs.ArticleResponse{}, false
	}
	ctx, cancel := context.WithTimeout(r.Context(), 20*time.Second)
	defer cancel()
	article, err := bestblogs.NewClient(nil).ReadArticle(ctx, learningProviderRequest(payload))
	if err != nil {
		writeBestblogsError(w, err)
		return bestblogs.ArticleResponse{}, false
	}
	return article, true
}

func buildLearningExplain(article bestblogs.ArticleResponse) learningExplainResponse {
	return service.BuildLearningExplain(article)
}

func buildLearningTranslate(article bestblogs.ArticleResponse, payload learningTranslateRequest) learningTranslateResponse {
	return service.BuildLearningTranslate(article, payload.Language, payload.TargetLanguage)
}
