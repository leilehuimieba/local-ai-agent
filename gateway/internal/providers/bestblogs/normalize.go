package bestblogs

import (
	"html"
	"regexp"
	"strings"
)

var (
	imageTagPattern = regexp.MustCompile(`(?is)<img[^>]+src=["']([^"']+)["'][^>]*>`)
	brTagPattern    = regexp.MustCompile(`(?i)<br\s*/?>`)
	blockTagPattern = regexp.MustCompile(`(?is)</?(?:p|div|h[1-6]|ul|ol|table|tbody|tr|pre|blockquote)[^>]*>`)
	liTagPattern    = regexp.MustCompile(`(?is)<li[^>]*>`)
	anyTagPattern   = regexp.MustCompile(`(?is)<[^>]+>`)
)

func normalizeArticle(articleID string, envelope upstreamEnvelope, req ReadArticleRequest) ArticleResponse {
	htmlText := includeHTML(req.IncludeHTML, envelope.Data.ContentData.DisplayDocument)
	markdown := renderMarkdown(envelope.Data.ContentData.DisplayDocument, req.IncludeMarkdown)
	images := collectImages(envelope.Data.MetaData.Cover, envelope.Data.ContentData.DisplayDocument, req.IncludeImages)
	return ArticleResponse{
		OK: true, Provider: "bestblogs", Strategy: "public_api", ArticleID: articleID,
		Meta:    normalizeMeta(envelope.Data.MetaData, envelope.Data.ContentData),
		Summary: normalizeSummary(envelope.Data.MetaData),
		Content: ArticleContent{HTML: htmlText, Markdown: markdown, Images: images},
	}
}

func normalizeMeta(meta upstreamMetaData, content upstreamContentData) ArticleMeta {
	return ArticleMeta{
		Title: meta.Title, Author: firstNonEmpty(meta.Authors...), PublishTime: firstNonEmpty(meta.PublishDateTimeStr, content.UpdateTime),
		Tags: meta.Tags, SourceURL: meta.URL, SourceName: meta.SourceName,
	}
}

func normalizeSummary(meta upstreamMetaData) ArticleSummary {
	return ArticleSummary{
		OneSentence: meta.OneSentenceSummary,
		Full:        meta.Summary,
		MainPoints:  normalizeMainPoints(meta.MainPoints),
		KeyQuotes:   meta.KeyQuotes,
	}
}

func normalizeMainPoints(items []upstreamMainPoint) []MainPoint {
	points := make([]MainPoint, 0, len(items))
	for _, item := range items {
		points = append(points, MainPoint{Point: item.Point, Explanation: item.Explanation})
	}
	return points
}

func includeHTML(enabled bool, content string) string {
	if !enabled {
		return ""
	}
	return strings.TrimSpace(content)
}

func renderMarkdown(content string, enabled bool) string {
	if !enabled {
		return ""
	}
	text := imageTagPattern.ReplaceAllString(content, "\n![]($1)\n")
	text = brTagPattern.ReplaceAllString(text, "\n")
	text = liTagPattern.ReplaceAllString(text, "\n- ")
	text = blockTagPattern.ReplaceAllString(text, "\n")
	text = anyTagPattern.ReplaceAllString(text, "")
	return cleanMarkdown(html.UnescapeString(text))
}

func cleanMarkdown(text string) string {
	lines := strings.Split(strings.ReplaceAll(text, "\r", ""), "\n")
	result := make([]string, 0, len(lines))
	lastBlank := true
	for _, line := range lines {
		trimmed := strings.TrimSpace(strings.ReplaceAll(line, "\u00a0", " "))
		if trimmed == "" {
			if !lastBlank {
				result = append(result, "")
			}
			lastBlank = true
			continue
		}
		result = append(result, trimmed)
		lastBlank = false
	}
	return strings.TrimSpace(strings.Join(result, "\n"))
}

func collectImages(cover string, content string, enabled bool) []string {
	if !enabled {
		return nil
	}
	seen := map[string]struct{}{}
	images := make([]string, 0)
	addImage(&images, seen, cover)
	for _, match := range imageTagPattern.FindAllStringSubmatch(content, -1) {
		if len(match) > 1 {
			addImage(&images, seen, match[1])
		}
	}
	return images
}

func addImage(images *[]string, seen map[string]struct{}, value string) {
	item := strings.TrimSpace(html.UnescapeString(value))
	if item == "" {
		return
	}
	if _, ok := seen[item]; ok {
		return
	}
	seen[item] = struct{}{}
	*images = append(*images, item)
}

func firstNonEmpty(values ...string) string {
	for _, value := range values {
		if strings.TrimSpace(value) != "" {
			return value
		}
	}
	return ""
}
