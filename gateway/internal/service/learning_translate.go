package service

import (
	"strings"

	"local-agent/gateway/internal/providers/bestblogs"
)

type LearningTranslateResponse struct {
	OK              bool     `json:"ok"`
	Provider        string   `json:"provider"`
	Strategy        string   `json:"strategy"`
	ArticleID       string   `json:"article_id"`
	SourceLanguage  string   `json:"source_language"`
	TargetLanguage  string   `json:"target_language"`
	TranslationType string   `json:"translation_type"`
	Title           string   `json:"title"`
	Summary         string   `json:"summary"`
	MainPoints      []string `json:"main_points"`
	Notes           string   `json:"notes"`
}

type PhrasePair struct {
	Source string
	Target string
}

func BuildLearningTranslate(article bestblogs.ArticleResponse, sourceLanguage, targetLanguage string) LearningTranslateResponse {
	return LearningTranslateResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		SourceLanguage: NormalizeSourceLanguage(sourceLanguage), TargetLanguage: NormalizeTargetLanguage(targetLanguage),
		TranslationType: "reader_bridge", Title: BridgeTitle(article.Meta.Title),
		Summary: BridgeSummary(article), MainPoints: BridgeMainPoints(article),
		Notes: "这是面向学习模式的英文桥接卡，优先帮助你快速理解主题，再决定是否回到中文原文深读。",
	}
}

func NormalizeSourceLanguage(language string) string {
	if strings.TrimSpace(language) == "" {
		return "zh"
	}
	return language
}

func NormalizeTargetLanguage(language string) string {
	if strings.TrimSpace(language) == "" {
		return "en"
	}
	return language
}

func BridgeTitle(title string) string {
	return CleanBridgeSpaces(ReplaceBridgePhrases(title, BridgeTitlePairs()))
}

func BridgeSummary(article bestblogs.ArticleResponse) string {
	return "This Chinese article explains " + BridgeTopic(article) + ". It provides " + BridgePointCount(article) + " and is suitable for summary-first reading."
}

func BridgeTopic(article bestblogs.ArticleResponse) string {
	if len(article.Meta.Tags) > 0 {
		return strings.Join(BridgeTagLabels(article.Meta.Tags), ", ")
	}
	return "the main idea in the original article"
}

func BridgePointCount(article bestblogs.ArticleResponse) string {
	if len(article.Summary.MainPoints) == 0 {
		return "a compact overview"
	}
	return "structured takeaways across " + strings.TrimSpace(BridgeNumber(len(article.Summary.MainPoints))) + " key points"
}

func BridgeMainPoints(article bestblogs.ArticleResponse) []string {
	points := make([]string, 0, len(article.Summary.MainPoints))
	for _, item := range article.Summary.MainPoints {
		points = append(points, BridgePointLine(item.Point, article.Meta.Tags))
	}
	if len(points) == 0 {
		return []string{"Bridge point: Start from the summary and tags before reading the original Chinese text."}
	}
	return points
}

func BridgePointLine(point string, tags []string) string {
	labels := BridgePointLabels(point, tags)
	return "Bridge point: " + strings.Join(labels, ", ") + "."
}

func BridgePointLabels(point string, tags []string) []string {
	labels := ExtractBridgeLabels(point)
	if len(labels) > 0 {
		return labels
	}
	labels = BridgeTagLabels(tags)
	if len(labels) > 0 {
		return labels
	}
	return []string{"See the original Chinese takeaway"}
}

func ExtractBridgeLabels(text string) []string {
	seen := map[string]struct{}{}
	labels := make([]string, 0, 4)
	for _, item := range BridgeCommonPairs() {
		if strings.Contains(text, item.Source) {
			labels = AppendBridgeLabel(labels, seen, CleanBridgeSpaces(item.Target))
		}
	}
	return labels
}

func AppendBridgeLabel(items []string, seen map[string]struct{}, label string) []string {
	if _, ok := seen[label]; ok || label == "" {
		return items
	}
	seen[label] = struct{}{}
	return append(items, label)
}

func BridgeTagLabels(tags []string) []string {
	labels := make([]string, 0, len(tags))
	for _, tag := range FirstExplainTerms(tags) {
		labels = append(labels, CleanBridgeSpaces(NormalizeBridgeText(tag)))
	}
	return labels
}

func BridgeNumber(value int) string {
	if value == 1 {
		return "one"
	}
	if value == 2 {
		return "two"
	}
	if value == 3 {
		return "three"
	}
	if value == 4 {
		return "four"
	}
	return "multiple"
}

func NormalizeBridgeText(text string) string {
	return ReplaceBridgePhrases(text, BridgeCommonPairs())
}

func ReplaceBridgePhrases(text string, pairs []PhrasePair) string {
	result := text
	for _, item := range pairs {
		result = strings.ReplaceAll(result, item.Source, item.Target)
	}
	return result
}

func CleanBridgeSpaces(text string) string {
	return strings.Join(strings.Fields(strings.ReplaceAll(text, "：", " - ")), " ")
}

func BridgeTitlePairs() []PhrasePair {
	return append([]PhrasePair{{Source: "从", Target: "From "}, {Source: "到", Target: " to "}}, BridgeCommonPairs()...)
}

func BridgeCommonPairs() []PhrasePair {
	return []PhrasePair{
		{Source: "人工智能", Target: "AI"},
		{Source: "商业科技", Target: "Business Tech"},
		{Source: "浏览器自动化", Target: "Browser Automation"},
		{Source: "软件编程", Target: "Software Development"},
		{Source: "检索增强生成", Target: "Retrieval-Augmented Generation"},
		{Source: "多模态 RAG", Target: "Multimodal RAG"},
		{Source: "上下文工程", Target: "Context Engineering"},
		{Source: "提示工程", Target: "Prompt Engineering"},
		{Source: "AI编程助手", Target: "AI Coding Assistant"},
		{Source: "软件工程", Target: "Software Engineering"},
		{Source: "开源模型", Target: "Open Models"},
		{Source: "代码生成", Target: "Code Generation"},
		{Source: "智能编程助手", Target: "Coding Assistant"},
		{Source: "深度研究", Target: "Deep Research"},
		{Source: "Agent模型", Target: "Agent Models"},
		{Source: "推理模型", Target: "Reasoning Models"},
		{Source: "图像生成", Target: "Image Generation"},
		{Source: "产品管理", Target: "Product Management"},
		{Source: "增长策略", Target: "Growth Strategy"},
		{Source: "AI创业", Target: "AI Startup"},
		{Source: "价值判断", Target: "Value Assessment"},
		{Source: "深度解读", Target: "Deep Dive"},
		{Source: "重磅开源", Target: "Open-Sourced Release"},
		{Source: "AI智能体", Target: "AI Agents"},
		{Source: "AI 智能体", Target: "AI Agents"},
		{Source: "AI Agent", Target: "AI Agent"},
		{Source: "大语言模型", Target: "Large Language Models"},
	}
}
