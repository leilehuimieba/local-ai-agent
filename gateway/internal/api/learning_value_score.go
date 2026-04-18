package api

import (
	"context"
	"net/http"
	"strings"
	"time"

	"local-agent/gateway/internal/providers/bestblogs"
)

type learningValueScoreResponse struct {
	OK         bool                  `json:"ok"`
	Provider   string                `json:"provider"`
	Strategy   string                `json:"strategy"`
	ArticleID  string                `json:"article_id"`
	Score      int                   `json:"score"`
	Level      string                `json:"level"`
	Reason     string                `json:"reason"`
	NextAction string                `json:"next_action"`
	Signals    learningValueSignals  `json:"signals"`
	Meta       bestblogs.ArticleMeta `json:"meta"`
}

type learningValueSignals struct {
	MainPoints    int `json:"main_points"`
	Tags          int `json:"tags"`
	Images        int `json:"images"`
	MarkdownChars int `json:"markdown_chars"`
	KeywordHits   int `json:"keyword_hits"`
}

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
	signals := buildLearningValueSignals(article)
	score := calculateLearningValueScore(signals)
	return learningValueScoreResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		Score: score, Level: learningValueLevel(score), Reason: learningValueReason(signals),
		NextAction: learningValueNextAction(score), Signals: signals, Meta: article.Meta,
	}
}

func buildLearningValueSignals(article bestblogs.ArticleResponse) learningValueSignals {
	return learningValueSignals{
		MainPoints:    len(article.Summary.MainPoints),
		Tags:          len(article.Meta.Tags),
		Images:        len(article.Content.Images),
		MarkdownChars: len([]rune(strings.TrimSpace(article.Content.Markdown))),
		KeywordHits:   learningKeywordHits(article),
	}
}

func calculateLearningValueScore(signals learningValueSignals) int {
	score := 25 + scoreMainPoints(signals.MainPoints) + scoreMarkdown(signals.MarkdownChars)
	score += scoreImages(signals.Images) + scoreTags(signals.Tags) + scoreKeywords(signals.KeywordHits)
	if score > 100 {
		return 100
	}
	return score
}

func scoreMainPoints(count int) int {
	if count >= 4 {
		return 20
	}
	if count >= 2 {
		return 12
	}
	return 6
}

func scoreMarkdown(chars int) int {
	if chars >= 6000 {
		return 20
	}
	if chars >= 2500 {
		return 14
	}
	return 8
}

func scoreImages(count int) int {
	if count >= 3 {
		return 10
	}
	if count > 0 {
		return 6
	}
	return 0
}

func scoreTags(count int) int {
	if count >= 4 {
		return 10
	}
	if count >= 2 {
		return 6
	}
	return 2
}

func scoreKeywords(hits int) int {
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

func learningKeywordHits(article bestblogs.ArticleResponse) int {
	text := strings.ToLower(strings.Join([]string{
		article.Meta.Title, article.Summary.OneSentence, article.Summary.Full,
		strings.Join(article.Meta.Tags, " "), article.Content.Markdown,
	}, " "))
	return countContains(text, []string{"agent", "浏览器自动化", "opencli", "prompt", "上下文工程", "rag", "代码", "软件工程"})
}

func countContains(text string, keywords []string) int {
	count := 0
	for _, keyword := range keywords {
		if strings.Contains(text, strings.ToLower(keyword)) {
			count++
		}
	}
	return count
}

func learningValueLevel(score int) string {
	if score >= 85 {
		return "high"
	}
	if score >= 70 {
		return "medium"
	}
	return "low"
}

func learningValueReason(signals learningValueSignals) string {
	parts := []string{
		mainPointReason(signals.MainPoints),
		markdownReason(signals.MarkdownChars),
		tagKeywordReason(signals.Tags, signals.KeywordHits),
	}
	if signals.Images > 0 {
		parts = append(parts, "正文包含配图或示意内容，适合继续深读。")
	}
	return strings.Join(parts, " ")
}

func mainPointReason(count int) string {
	if count >= 4 {
		return "结构化要点完整，主题脉络清晰。"
	}
	return "存在可用摘要，但结构化要点仍偏少。"
}

func markdownReason(chars int) string {
	if chars >= 2500 {
		return "正文篇幅充足，具备继续拆解的内容密度。"
	}
	return "正文长度中等，适合先做快速判断。"
}

func tagKeywordReason(tags int, hits int) string {
	if tags >= 3 && hits >= 2 {
		return "标签与正文关键词聚焦明确，和学习主题相关度较高。"
	}
	return "主题有一定相关性，建议结合摘要再决定是否深挖。"
}

func learningValueNextAction(score int) string {
	if score >= 85 {
		return "先阅读 main_points，再进入 explain/translate；若与你当前方向一致，可进入记忆候选。"
	}
	if score >= 70 {
		return "先看摘要和关键段落，再决定是否进入 explain 或收藏。"
	}
	return "先保留摘要结论，暂不进入记忆写入。"
}
