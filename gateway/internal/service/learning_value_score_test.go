package service

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/require"

	"local-agent/gateway/internal/providers/bestblogs"
)

func TestScoreLearningArticle(t *testing.T) {
	article := bestblogs.ArticleResponse{
		Provider: "bestblogs", Strategy: "public_api", ArticleID: "42acaf7d",
		Meta:    bestblogs.ArticleMeta{Title: "浏览器自动化：从 GUI 到 OpenCLI", Tags: []string{"浏览器自动化", "AI Agent", "OpenCLI", "RPA"}},
		Summary: bestblogs.ArticleSummary{MainPoints: []bestblogs.MainPoint{{}, {}, {}, {}}, Full: "面向 Agent 的自动化与软件工程。"},
		Content: bestblogs.ArticleContent{Markdown: strings.Repeat("浏览器自动化 OpenCLI Agent 软件工程 ", 220), Images: []string{"a", "b", "c"}},
	}
	result := ScoreLearningArticle(article)
	require.Equal(t, "high", result.Level)
	require.GreaterOrEqual(t, result.Score, 85)
	require.NotEmpty(t, result.Reason)
	require.NotEmpty(t, result.NextAction)
}
