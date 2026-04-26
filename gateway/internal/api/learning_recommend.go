package api

import (
	"net/http"

	"local-agent/gateway/internal/providers/bestblogs"
	"local-agent/gateway/internal/service"
)

type learningRecommendResponse = service.LearningRecommendResponse

func learningRecommendHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeLearningExtractRequest(w, r)
		if !ok {
			return
		}
		article, ok := readLearningArticle(w, r, payload)
		if !ok {
			return
		}
		writeJSON(w, http.StatusOK, buildLearningRecommend(article))
	}
}

func buildLearningRecommend(article bestblogs.ArticleResponse) learningRecommendResponse {
	return service.BuildLearningRecommend(article)
}
