package service

import (
	"strings"

	"local-agent/gateway/internal/providers/bestblogs"
)

type LearningExplainResponse struct {
	OK         bool                  `json:"ok"`
	Provider   string                `json:"provider"`
	Strategy   string                `json:"strategy"`
	ArticleID  string                `json:"article_id"`
	Title      string                `json:"title"`
	Explain    string                `json:"explain"`
	MainPoints []string              `json:"main_points"`
	KeyTerms   []LearningExplainTerm `json:"key_terms"`
}

type LearningExplainTerm struct {
	Term        string `json:"term"`
	Explanation string `json:"explanation"`
}

func BuildLearningExplain(article bestblogs.ArticleResponse) LearningExplainResponse {
	return LearningExplainResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		Title: article.Meta.Title, Explain: ExplainText(article),
		MainPoints: ExplainMainPoints(article), KeyTerms: ExplainKeyTerms(article),
	}
}

func ExplainText(article bestblogs.ArticleResponse) string {
	parts := []string{PickExplainLead(article), ExplainStructure(article)}
	if len(article.Meta.Tags) > 0 {
		parts = append(parts, "如果你只看一个入口，先看摘要，再根据标签定位自己关心的部分。")
	}
	return strings.Join(parts, " ")
}

func PickExplainLead(article bestblogs.ArticleResponse) string {
	if strings.TrimSpace(article.Summary.OneSentence) != "" {
		return "这篇文章想解决的问题是：" + strings.TrimSpace(article.Summary.OneSentence)
	}
	return "这篇文章围绕「" + article.Meta.Title + "」展开，适合先抓主题再读细节。"
}

func ExplainStructure(article bestblogs.ArticleResponse) string {
	if len(article.Summary.MainPoints) >= 3 {
		return "它已经给出了结构化要点，你可以把它当成一篇可快速扫读、再按需深挖的学习材料。"
	}
	return "正文信息量较集中，建议先看标题、摘要和前几个段落，再决定是否完整阅读。"
}

func ExplainMainPoints(article bestblogs.ArticleResponse) []string {
	points := make([]string, 0, len(article.Summary.MainPoints))
	for _, item := range article.Summary.MainPoints {
		points = append(points, strings.TrimSpace(item.Point))
	}
	return points
}

func ExplainKeyTerms(article bestblogs.ArticleResponse) []LearningExplainTerm {
	items := make([]LearningExplainTerm, 0, 4)
	for _, tag := range FirstExplainTerms(article.Meta.Tags) {
		items = append(items, LearningExplainTerm{Term: tag, Explanation: ExplainTerm(tag)})
	}
	return items
}

func FirstExplainTerms(tags []string) []string {
	if len(tags) <= 4 {
		return tags
	}
	return tags[:4]
}

func ExplainTerm(tag string) string {
	text := NormalizeBridgeText(tag)
	if strings.Contains(text, "Agent") {
		return "强调让模型以可执行步骤完成任务，而不是只生成答案。"
	}
	if strings.Contains(text, "OpenCLI") || strings.Contains(text, "API") {
		return "强调直接调用能力接口，而不是依赖页面点击流程。"
	}
	return "这是文章主题里的核心线索，可用来判断它是否和你的学习方向相关。"
}
