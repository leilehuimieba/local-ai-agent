package service

import (
	"strings"

	"local-agent/gateway/internal/providers/bestblogs"
)

type LearningValueScoreResponse struct {
	OK         bool                  `json:"ok"`
	Provider   string                `json:"provider"`
	Strategy   string                `json:"strategy"`
	ArticleID  string                `json:"article_id"`
	Score      int                   `json:"score"`
	Level      string                `json:"level"`
	Reason     string                `json:"reason"`
	NextAction string                `json:"next_action"`
	Signals    LearningValueSignals  `json:"signals"`
	Meta       bestblogs.ArticleMeta `json:"meta"`
}

type LearningValueSignals struct {
	MainPoints    int `json:"main_points"`
	Tags          int `json:"tags"`
	Images        int `json:"images"`
	MarkdownChars int `json:"markdown_chars"`
	KeywordHits   int `json:"keyword_hits"`
}

func ScoreLearningArticle(article bestblogs.ArticleResponse) LearningValueScoreResponse {
	signals := BuildLearningValueSignals(article)
	score := CalculateLearningValueScore(signals)
	return LearningValueScoreResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		Score: score, Level: LearningValueLevel(score), Reason: LearningValueReason(signals),
		NextAction: LearningValueNextAction(score), Signals: signals, Meta: article.Meta,
	}
}

func BuildLearningValueSignals(article bestblogs.ArticleResponse) LearningValueSignals {
	return LearningValueSignals{
		MainPoints:    len(article.Summary.MainPoints),
		Tags:          len(article.Meta.Tags),
		Images:        len(article.Content.Images),
		MarkdownChars: len([]rune(strings.TrimSpace(article.Content.Markdown))),
		KeywordHits:   LearningKeywordHits(article),
	}
}

func CalculateLearningValueScore(signals LearningValueSignals) int {
	score := 25 + ScoreMainPoints(signals.MainPoints) + ScoreMarkdown(signals.MarkdownChars)
	score += ScoreImages(signals.Images) + ScoreTags(signals.Tags) + ScoreKeywords(signals.KeywordHits)
	if score > 100 {
		return 100
	}
	return score
}

func ScoreMainPoints(count int) int {
	if count >= 4 {
		return 20
	}
	if count >= 2 {
		return 12
	}
	return 6
}

func ScoreMarkdown(chars int) int {
	if chars >= 6000 {
		return 20
	}
	if chars >= 2500 {
		return 14
	}
	return 8
}

func ScoreImages(count int) int {
	if count >= 3 {
		return 10
	}
	if count > 0 {
		return 6
	}
	return 0
}

func ScoreTags(count int) int {
	if count >= 4 {
		return 10
	}
	if count >= 2 {
		return 6
	}
	return 2
}

func ScoreKeywords(hits int) int {
	if hits >= 4 {
		return 15
	}
	if hits >= 2 {
		return 10
	}
	if hits == 1 {
		return 5
	}
	return 0
}

func LearningKeywordHits(article bestblogs.ArticleResponse) int {
	text := strings.ToLower(strings.Join([]string{
		article.Meta.Title, article.Summary.OneSentence, article.Summary.Full,
		strings.Join(article.Meta.Tags, " "), article.Content.Markdown,
	}, " "))
	return CountContains(text, []string{"agent", "浏览器自动化", "opencli", "prompt", "上下文工程", "rag", "代码", "软件工程"})
}

func CountContains(text string, keywords []string) int {
	count := 0
	for _, keyword := range keywords {
		if strings.Contains(text, strings.ToLower(keyword)) {
			count++
		}
	}
	return count
}

func LearningValueLevel(score int) string {
	if score >= 85 {
		return "high"
	}
	if score >= 70 {
		return "medium"
	}
	return "low"
}

func LearningValueReason(signals LearningValueSignals) string {
	parts := []string{
		MainPointReason(signals.MainPoints),
		MarkdownReason(signals.MarkdownChars),
		TagKeywordReason(signals.Tags, signals.KeywordHits),
	}
	if signals.Images > 0 {
		parts = append(parts, "正文包含配图或示意内容，适合继续深读。")
	}
	return strings.Join(parts, " ")
}

func MainPointReason(count int) string {
	if count >= 4 {
		return "结构化要点完整，主题脉络清晰。"
	}
	return "存在可用摘要，但结构化要点仍偏少。"
}

func MarkdownReason(chars int) string {
	if chars >= 2500 {
		return "正文篇幅充足，具备继续拆解的内容密度。"
	}
	return "正文长度中等，适合先做快速判断。"
}

func TagKeywordReason(tags int, hits int) string {
	if tags >= 3 && hits >= 2 {
		return "标签与正文关键词聚焦明确，和学习主题相关度较高。"
	}
	return "主题有一定相关性，建议结合摘要再决定是否深挖。"
}

func LearningValueNextAction(score int) string {
	if score >= 85 {
		return "先阅读 main_points，再进入 explain/translate；若与你当前方向一致，可进入记忆候选。"
	}
	if score >= 70 {
		return "先看摘要和关键段落，再决定是否进入 explain 或收藏。"
	}
	return "先保留摘要结论，暂不进入记忆写入。"
}
