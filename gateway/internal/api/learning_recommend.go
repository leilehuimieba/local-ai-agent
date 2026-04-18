package api

import (
	"net/http"
	"strings"

	"local-agent/gateway/internal/providers/bestblogs"
)

type learningRecommendResponse struct {
	OK             bool                  `json:"ok"`
	Provider       string                `json:"provider"`
	Strategy       string                `json:"strategy"`
	ArticleID      string                `json:"article_id"`
	Score          int                   `json:"score"`
	Level          string                `json:"level"`
	Recommendation string                `json:"recommendation"`
	FocusTopics    []string              `json:"focus_topics"`
	Why            string                `json:"why"`
	NextStep       string                `json:"next_step"`
	Meta           bestblogs.ArticleMeta `json:"meta"`
}

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
	signals := buildLearningValueSignals(article)
	score := calculateLearningValueScore(signals)
	return learningRecommendResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		Score: score, Level: learningValueLevel(score), Recommendation: learningRecommendation(score),
		FocusTopics: learningFocusTopics(article), Why: learningRecommendWhy(article, signals),
		NextStep: learningRecommendNextStep(article, score), Meta: article.Meta,
	}
}

func learningRecommendation(score int) string {
	if score >= 85 {
		return "建议把这篇文章作为当前主题的深读样本，先扫摘要，再回到原文拆关键观点。"
	}
	if score >= 70 {
		return "建议把它当成主题观察样本，先看摘要和 main_points，再决定是否深读。"
	}
	return "建议先做轻量浏览，只保留结论和关键词，不进入重度投入。"
}

func learningFocusTopics(article bestblogs.ArticleResponse) []string {
	topics := pickFocusTopics(article.Meta.Tags)
	if len(topics) > 0 {
		return topics
	}
	return []string{strings.TrimSpace(article.Meta.Title)}
}

func pickFocusTopics(tags []string) []string {
	seen := map[string]struct{}{}
	items := make([]string, 0, 3)
	for _, tag := range tags {
		items = appendFocusTopic(items, seen, tag)
	}
	return items
}

func appendFocusTopic(items []string, seen map[string]struct{}, tag string) []string {
	text := strings.TrimSpace(tag)
	if text == "" || len(items) == 3 {
		return items
	}
	if _, ok := seen[text]; ok {
		return items
	}
	seen[text] = struct{}{}
	return append(items, text)
}

func learningRecommendWhy(article bestblogs.ArticleResponse, signals learningValueSignals) string {
	topics := strings.Join(learningFocusTopics(article), "、")
	return "优先关注 " + topics + "。 " + learningValueReason(signals)
}

func learningRecommendNextStep(article bestblogs.ArticleResponse, score int) string {
	point := firstLearningPoint(article)
	if point == "" {
		return learningValueNextAction(score)
	}
	if score >= 85 {
		return "先看要点「" + point + "」，再回到原文对应段落做深读。"
	}
	return "先看要点「" + point + "」，确认是否值得进入完整阅读。"
}

func firstLearningPoint(article bestblogs.ArticleResponse) string {
	if len(article.Summary.MainPoints) == 0 {
		return ""
	}
	return strings.TrimSpace(article.Summary.MainPoints[0].Point)
}
