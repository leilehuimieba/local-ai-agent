package api

import (
	"net/http"

	"local-agent/gateway/internal/providers/bestblogs"
)

type learningRollbackRequest struct {
	ArticleURL          string `json:"article_url"`
	ProviderHint        string `json:"provider_hint,omitempty"`
	Language            string `json:"language,omitempty"`
	LearningModeEnabled bool   `json:"learning_mode_enabled"`
}

type learningRollbackResponse struct {
	OK               bool                      `json:"ok"`
	RollbackApplied  bool                      `json:"rollback_applied"`
	LearningModeOn   bool                      `json:"learning_mode_enabled"`
	FallbackMode     string                    `json:"fallback_mode"`
	Provider         string                    `json:"provider"`
	Strategy         string                    `json:"strategy"`
	ArticleID        string                    `json:"article_id"`
	Title            string                    `json:"title"`
	AllowedActions   []string                  `json:"allowed_actions"`
	DisabledActions  []string                  `json:"disabled_actions"`
	ExplainPreview   string                    `json:"explain_preview"`
	TranslatePreview string                    `json:"translate_preview"`
	Explain          learningExplainResponse   `json:"explain"`
	Translate        learningTranslateResponse `json:"translate"`
}

func learningRollbackHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeLearningRollbackRequest(w, r)
		if !ok {
			return
		}
		article, ok := readLearningArticle(w, r, learningRollbackPayload(payload))
		if !ok {
			return
		}
		writeJSON(w, http.StatusOK, buildLearningRollbackResponse(article, payload))
	}
}

func decodeLearningRollbackRequest(w http.ResponseWriter, r *http.Request) (learningRollbackRequest, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return learningRollbackRequest{}, false
	}
	var payload learningRollbackRequest
	if !decodeJSONBody(w, r, &payload) || payload.ArticleURL == "" {
		http.Error(w, "article_url is required", http.StatusBadRequest)
		return learningRollbackRequest{}, false
	}
	return payload, true
}

func learningRollbackPayload(payload learningRollbackRequest) learningExtractRequest {
	return learningExtractRequest{
		ArticleURL: payload.ArticleURL, ProviderHint: payload.ProviderHint, Language: payload.Language,
	}
}

func buildLearningRollbackResponse(article bestblogs.ArticleResponse, payload learningRollbackRequest) learningRollbackResponse {
	explain := buildLearningExplain(article)
	translate := buildLearningTranslate(article, learningTranslateRequest{Language: payload.Language, TargetLanguage: "en"})
	return learningRollbackResponse{
		OK: true, RollbackApplied: !payload.LearningModeEnabled, LearningModeOn: payload.LearningModeEnabled,
		FallbackMode: learningFallbackMode(payload.LearningModeEnabled), Provider: article.Provider,
		Strategy: article.Strategy, ArticleID: article.ArticleID, Title: article.Meta.Title,
		AllowedActions: learningAllowedActions(payload.LearningModeEnabled), DisabledActions: learningDisabledActions(payload.LearningModeEnabled),
		ExplainPreview: previewText(explain.Explain), TranslatePreview: previewText(translate.Summary),
		Explain: explain, Translate: translate,
	}
}

func learningFallbackMode(enabled bool) string {
	if enabled {
		return "learning_mode"
	}
	return "explain_translate_only"
}

func learningAllowedActions(enabled bool) []string {
	if enabled {
		return []string{"extract", "explain", "translate", "value_score", "recommend", "memory_write", "audit_trace"}
	}
	return []string{"extract", "explain", "translate"}
}

func learningDisabledActions(enabled bool) []string {
	if enabled {
		return nil
	}
	return []string{"value_score", "recommend", "memory_write", "audit_trace"}
}
