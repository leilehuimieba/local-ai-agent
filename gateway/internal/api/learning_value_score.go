package api

import (
	"context"
	"net/http"
	"time"

	"local-agent/gateway/internal/providers/bestblogs"
	"local-agent/gateway/internal/service"
)

type learningValueScoreResponse = service.LearningValueScoreResponse
type learningValueSignals = service.LearningValueSignals

func learningValueScoreHandler() http.HandlerFunc {
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
		article, err := bestblogs.NewClient(nil).ReadArticle(ctx, learningProviderRequest(payload))
		if err != nil {
			writeBestblogsError(w, err)
			return
		}
		writeJSON(w, http.StatusOK, scoreLearningArticle(article))
	}
}

func scoreLearningArticle(article bestblogs.ArticleResponse) learningValueScoreResponse {
	return service.ScoreLearningArticle(article)
}

func buildLearningValueSignals(article bestblogs.ArticleResponse) learningValueSignals {
	return service.BuildLearningValueSignals(article)
}
