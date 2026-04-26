package service

import (
	"strings"

	"local-agent/gateway/internal/providers/bestblogs"
)

type LearningRecommendResponse struct {
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

func BuildLearningRecommend(article bestblogs.ArticleResponse) LearningRecommendResponse {
	signals := BuildLearningValueSignals(article)
	score := CalculateLearningValueScore(signals)
	return LearningRecommendResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		Score: score, Level: LearningValueLevel(score), Recommendation: LearningRecommendation(score),
		FocusTopics: LearningFocusTopics(article), Why: LearningRecommendWhy(article, signals),
		NextStep: LearningRecommendNextStep(article, score), Meta: article.Meta,
	}
}

func LearningRecommendation(score int) string {
	if score >= 85 {
		return "建议把这篇文章作为当前主题的深读样本，先扫摘要，再回到原文拆关键观点。"
	}
	if score >= 70 {
		return "建议把它当成主题观察样本，先看摘要和 main_points，再决定是否深读。"
	}
	return "建议先做轻量浏览，只保留结论和关键词，不进入重度投入。"
}

func LearningFocusTopics(article bestblogs.ArticleResponse) []string {
	topics := PickFocusTopics(article.Meta.Tags)
	if len(topics) > 0 {
		return topics
	}
	return []string{strings.TrimSpace(article.Meta.Title)}
}

func PickFocusTopics(tags []string) []string {
	seen := map[string]struct{}{}
	items := make([]string, 0, 3)
	for _, tag := range tags {
		items = AppendFocusTopic(items, seen, tag)
	}
	return items
}

func AppendFocusTopic(items []string, seen map[string]struct{}, tag string) []string {
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

func LearningRecommendWhy(article bestblogs.ArticleResponse, signals LearningValueSignals) string {
	topics := strings.Join(LearningFocusTopics(article), "、")
	return "优先关注 " + topics + "。 " + LearningValueReason(signals)
}

func LearningRecommendNextStep(article bestblogs.ArticleResponse, score int) string {
	point := FirstLearningPoint(article)
	if point == "" {
		return LearningValueNextAction(score)
	}
	if score >= 85 {
		return "先看要点「" + point + "」，再回到原文对应段落做深读。"
	}
	return "先看要点「" + point + "」，确认是否值得进入完整阅读。"
}

func FirstLearningPoint(article bestblogs.ArticleResponse) string {
	if len(article.Summary.MainPoints) == 0 {
		return ""
	}
	return strings.TrimSpace(article.Summary.MainPoints[0].Point)
}
