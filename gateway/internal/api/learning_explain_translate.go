package api

import (
	"context"
	"net/http"
	"strings"
	"time"

	"local-agent/gateway/internal/providers/bestblogs"
)

type learningTranslateRequest struct {
	ArticleURL     string `json:"article_url"`
	ProviderHint   string `json:"provider_hint,omitempty"`
	Language       string `json:"language,omitempty"`
	TargetLanguage string `json:"target_language,omitempty"`
}

type learningExplainResponse struct {
	OK         bool                  `json:"ok"`
	Provider   string                `json:"provider"`
	Strategy   string                `json:"strategy"`
	ArticleID  string                `json:"article_id"`
	Title      string                `json:"title"`
	Explain    string                `json:"explain"`
	MainPoints []string              `json:"main_points"`
	KeyTerms   []learningExplainTerm `json:"key_terms"`
}

type learningExplainTerm struct {
	Term        string `json:"term"`
	Explanation string `json:"explanation"`
}

type learningTranslateResponse struct {
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

type phrasePair struct {
	Source string
	Target string
}

func learningExplainHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeLearningExtractRequest(w, r)
		if !ok {
			return
		}
		article, ok := readLearningArticle(w, r, payload)
		if !ok {
			return
		}
		writeJSON(w, http.StatusOK, buildLearningExplain(article))
	}
}

func learningTranslateHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeLearningTranslateRequest(w, r)
		if !ok {
			return
		}
		article, ok := readLearningArticle(w, r, learningTranslatePayload(payload))
		if !ok {
			return
		}
		writeJSON(w, http.StatusOK, buildLearningTranslate(article, payload))
	}
}

func learningTranslatePayload(payload learningTranslateRequest) learningExtractRequest {
	return learningExtractRequest{
		ArticleURL: payload.ArticleURL, ProviderHint: payload.ProviderHint, Language: payload.Language,
	}
}

func decodeLearningTranslateRequest(w http.ResponseWriter, r *http.Request) (learningTranslateRequest, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return learningTranslateRequest{}, false
	}
	var payload learningTranslateRequest
	if !decodeJSONBody(w, r, &payload) {
		return learningTranslateRequest{}, false
	}
	if strings.TrimSpace(payload.ArticleURL) == "" {
		http.Error(w, "article_url is required", http.StatusBadRequest)
		return learningTranslateRequest{}, false
	}
	return payload, true
}

func readLearningArticle(w http.ResponseWriter, r *http.Request, payload learningExtractRequest) (bestblogs.ArticleResponse, bool) {
	if resolveLearningProvider(payload.ArticleURL, payload.ProviderHint) != "bestblogs" {
		http.Error(w, "unsupported learning provider", http.StatusBadRequest)
		return bestblogs.ArticleResponse{}, false
	}
	ctx, cancel := context.WithTimeout(r.Context(), 20*time.Second)
	defer cancel()
	article, err := bestblogs.NewClient(nil).ReadArticle(ctx, learningProviderRequest(payload))
	if err != nil {
		writeBestblogsError(w, err)
		return bestblogs.ArticleResponse{}, false
	}
	return article, true
}

func buildLearningExplain(article bestblogs.ArticleResponse) learningExplainResponse {
	return learningExplainResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		Title: article.Meta.Title, Explain: explainText(article),
		MainPoints: explainMainPoints(article), KeyTerms: explainKeyTerms(article),
	}
}

func explainText(article bestblogs.ArticleResponse) string {
	parts := []string{pickExplainLead(article), explainStructure(article)}
	if len(article.Meta.Tags) > 0 {
		parts = append(parts, "如果你只看一个入口，先看摘要，再根据标签定位自己关心的部分。")
	}
	return strings.Join(parts, " ")
}

func pickExplainLead(article bestblogs.ArticleResponse) string {
	if strings.TrimSpace(article.Summary.OneSentence) != "" {
		return "这篇文章想解决的问题是：" + strings.TrimSpace(article.Summary.OneSentence)
	}
	return "这篇文章围绕「" + article.Meta.Title + "」展开，适合先抓主题再读细节。"
}

func explainStructure(article bestblogs.ArticleResponse) string {
	if len(article.Summary.MainPoints) >= 3 {
		return "它已经给出了结构化要点，你可以把它当成一篇可快速扫读、再按需深挖的学习材料。"
	}
	return "正文信息量较集中，建议先看标题、摘要和前几个段落，再决定是否完整阅读。"
}

func explainMainPoints(article bestblogs.ArticleResponse) []string {
	points := make([]string, 0, len(article.Summary.MainPoints))
	for _, item := range article.Summary.MainPoints {
		points = append(points, strings.TrimSpace(item.Point))
	}
	return points
}

func explainKeyTerms(article bestblogs.ArticleResponse) []learningExplainTerm {
	items := make([]learningExplainTerm, 0, 4)
	for _, tag := range firstExplainTerms(article.Meta.Tags) {
		items = append(items, learningExplainTerm{Term: tag, Explanation: explainTerm(tag)})
	}
	return items
}

func firstExplainTerms(tags []string) []string {
	if len(tags) <= 4 {
		return tags
	}
	return tags[:4]
}

func explainTerm(tag string) string {
	text := normalizeBridgeText(tag)
	if strings.Contains(text, "Agent") {
		return "强调让模型以可执行步骤完成任务，而不是只生成答案。"
	}
	if strings.Contains(text, "OpenCLI") || strings.Contains(text, "API") {
		return "强调直接调用能力接口，而不是依赖页面点击流程。"
	}
	return "这是文章主题里的核心线索，可用来判断它是否和你的学习方向相关。"
}

func buildLearningTranslate(article bestblogs.ArticleResponse, payload learningTranslateRequest) learningTranslateResponse {
	return learningTranslateResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		SourceLanguage: normalizeSourceLanguage(payload.Language), TargetLanguage: normalizeTargetLanguage(payload.TargetLanguage),
		TranslationType: "reader_bridge", Title: bridgeTitle(article.Meta.Title),
		Summary: bridgeSummary(article), MainPoints: bridgeMainPoints(article),
		Notes: "这是面向学习模式的英文桥接卡，优先帮助你快速理解主题，再决定是否回到中文原文深读。",
	}
}

func normalizeSourceLanguage(language string) string {
	if strings.TrimSpace(language) == "" {
		return "zh"
	}
	return language
}

func normalizeTargetLanguage(language string) string {
	if strings.TrimSpace(language) == "" {
		return "en"
	}
	return language
}

func bridgeTitle(title string) string {
	return cleanBridgeSpaces(replaceBridgePhrases(title, bridgeTitlePairs()))
}

func bridgeSummary(article bestblogs.ArticleResponse) string {
	return "This Chinese article explains " + bridgeTopic(article) + ". It provides " + bridgePointCount(article) + " and is suitable for summary-first reading."
}

func bridgeTopic(article bestblogs.ArticleResponse) string {
	if len(article.Meta.Tags) > 0 {
		return strings.Join(bridgeTagLabels(article.Meta.Tags), ", ")
	}
	return "the main idea in the original article"
}

func bridgePointCount(article bestblogs.ArticleResponse) string {
	if len(article.Summary.MainPoints) == 0 {
		return "a compact overview"
	}
	return "structured takeaways across " + strings.TrimSpace(bridgeNumber(len(article.Summary.MainPoints))) + " key points"
}

func bridgeMainPoints(article bestblogs.ArticleResponse) []string {
	points := make([]string, 0, len(article.Summary.MainPoints))
	for _, item := range article.Summary.MainPoints {
		points = append(points, bridgePointLine(item.Point, article.Meta.Tags))
	}
	if len(points) == 0 {
		return []string{"Bridge point: Start from the summary and tags before reading the original Chinese text."}
	}
	return points
}

func bridgePointLine(point string, tags []string) string {
	labels := bridgePointLabels(point, tags)
	return "Bridge point: " + strings.Join(labels, ", ") + "."
}

func bridgePointLabels(point string, tags []string) []string {
	labels := extractBridgeLabels(point)
	if len(labels) > 0 {
		return labels
	}
	labels = bridgeTagLabels(tags)
	if len(labels) > 0 {
		return labels
	}
	return []string{"See the original Chinese takeaway"}
}

func extractBridgeLabels(text string) []string {
	seen := map[string]struct{}{}
	labels := make([]string, 0, 4)
	for _, item := range bridgeCommonPairs() {
		if strings.Contains(text, item.Source) {
			labels = appendBridgeLabel(labels, seen, cleanBridgeSpaces(item.Target))
		}
	}
	return labels
}

func appendBridgeLabel(items []string, seen map[string]struct{}, label string) []string {
	if _, ok := seen[label]; ok || label == "" {
		return items
	}
	seen[label] = struct{}{}
	return append(items, label)
}

func bridgeTagLabels(tags []string) []string {
	labels := make([]string, 0, len(tags))
	for _, tag := range firstExplainTerms(tags) {
		labels = append(labels, cleanBridgeSpaces(normalizeBridgeText(tag)))
	}
	return labels
}

func bridgeNumber(value int) string {
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

func normalizeBridgeText(text string) string {
	return replaceBridgePhrases(text, bridgeCommonPairs())
}

func replaceBridgePhrases(text string, pairs []phrasePair) string {
	result := text
	for _, item := range pairs {
		result = strings.ReplaceAll(result, item.Source, item.Target)
	}
	return result
}

func cleanBridgeSpaces(text string) string {
	return strings.Join(strings.Fields(strings.ReplaceAll(text, "：", " - ")), " ")
}

func bridgeTitlePairs() []phrasePair {
	return append([]phrasePair{{Source: "从", Target: "From "}, {Source: "到", Target: " to "}}, bridgeCommonPairs()...)
}

func bridgeCommonPairs() []phrasePair {
	return []phrasePair{
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
