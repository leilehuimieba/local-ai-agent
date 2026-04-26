package service

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/require"

	"local-agent/gateway/internal/providers/bestblogs"
)

func TestBuildLearningRecommend(t *testing.T) {
	article := bestblogs.ArticleResponse{
		Provider: "bestblogs", Strategy: "public_api", ArticleID: "42acaf7d",
		Meta:    bestblogs.ArticleMeta{Title: "浏览器自动化：从 GUI 到 OpenCLI", Tags: []string{"浏览器自动化", "AI Agent", "OpenCLI"}},
		Summary: bestblogs.ArticleSummary{MainPoints: []bestblogs.MainPoint{{Point: "为什么我们需要浏览器自动化"}}},
		Content: bestblogs.ArticleContent{Markdown: strings.Repeat("浏览器自动化 OpenCLI Agent ", 180), Images: []string{"a"}},
	}
	result := BuildLearningRecommend(article)
	require.NotEmpty(t, result.Recommendation)
	require.NotEmpty(t, result.FocusTopics)
	require.NotEmpty(t, result.NextStep)
}
