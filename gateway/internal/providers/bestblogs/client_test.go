package bestblogs

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestExtractArticleID(t *testing.T) {
	id, err := extractArticleID("https://www.bestblogs.dev/article/42acaf7d?entry=resource_card")
	require.NoError(t, err)
	require.Equal(t, "42acaf7d", id)
}

func TestRenderMarkdownAndImages(t *testing.T) {
	input := `<html><body><div><p>为什么我们需要浏览器自动化</p><p>未来软件竞争维度</p><img src="https://img.test/a.png"></div></body></html>`
	markdown := renderMarkdown(input, true)
	images := collectImages("", input, true)
	require.Contains(t, markdown, "为什么我们需要浏览器自动化")
	require.Contains(t, markdown, "未来软件竞争维度")
	require.Equal(t, []string{"https://img.test/a.png"}, images)
}
