package api

import (
	"context"
	"net/url"
	"os"
	"strings"
	"testing"

	"local-agent/gateway/internal/providers/bestblogs"
)

func TestMain(m *testing.M) {
	restore := swapBestblogsArticleReader(fakeBestblogsArticleReader)
	code := m.Run()
	restore()
	os.Exit(code)
}

func fakeBestblogsArticleReader(_ context.Context, req bestblogs.ReadArticleRequest) (bestblogs.ArticleResponse, error) {
	id := fakeArticleID(req.ArticleURL)
	return bestblogs.ArticleResponse{
		OK: true, Provider: "bestblogs", Strategy: "public_api", ArticleID: id,
		Meta: fakeArticleMeta(id, req.ArticleURL),
		Summary: bestblogs.ArticleSummary{
			OneSentence: "这是一篇关于浏览器自动化、OpenCLI 与 AI Agent 执行能力的学习文章。",
			Full:        "文章围绕本地智能体、工具调用、软件工程和上下文工程展开。",
			MainPoints:  fakeMainPoints(),
		},
		Content: bestblogs.ArticleContent{
			HTML:     "<article><p>为什么我们需要浏览器自动化</p></article>",
			Markdown: fakeArticleMarkdown(),
			Images:   []string{"image-1.png", "image-2.png", "image-3.png"},
		},
	}, nil
}

func fakeArticleID(rawURL string) string {
	parsed, err := url.Parse(rawURL)
	if err != nil {
		return "fixture"
	}
	parts := strings.Split(strings.Trim(parsed.Path, "/"), "/")
	if len(parts) == 0 || parts[len(parts)-1] == "" {
		return "fixture"
	}
	return parts[len(parts)-1]
}

func fakeArticleMeta(id string, rawURL string) bestblogs.ArticleMeta {
	title := "测试文章：" + id
	if id == "42acaf7d" {
		title = "浏览器自动化：从 GUI 到 OpenCLI"
	}
	return bestblogs.ArticleMeta{
		Title: title, Tags: []string{"浏览器自动化", "AI Agent", "OpenCLI", "软件工程"},
		SourceURL: rawURL,
	}
}

func fakeMainPoints() []bestblogs.MainPoint {
	return []bestblogs.MainPoint{
		{Point: "为什么我们需要浏览器自动化"},
		{Point: "OpenCLI 让 Agent 更稳定地调用工具"},
		{Point: "上下文工程决定复杂任务的可靠性"},
		{Point: "未来软件竞争维度会转向可执行能力"},
	}
}

func fakeArticleMarkdown() string {
	base := "为什么我们需要浏览器自动化。OpenCLI 和 AI Agent 可以把 GUI 流程转为稳定工具调用。未来软件竞争维度会围绕上下文工程、RAG、代码和软件工程展开。"
	return strings.Repeat(base, 60)
}
