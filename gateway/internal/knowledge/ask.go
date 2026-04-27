package knowledge

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/state"
)

type AskRequest struct {
	Question string `json:"question"`
}

type AskResponse struct {
	Answer  string `json:"answer"`
	Sources []Item `json:"sources"`
}

func (h *Handler) handleAsk(w http.ResponseWriter, r *http.Request, workspaceID string, cfg config.AppConfig, settingsStore *state.SettingsStore) {
	var req AskRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid json", http.StatusBadRequest)
		return
	}
	if strings.TrimSpace(req.Question) == "" {
		http.Error(w, "question is required", http.StatusBadRequest)
		return
	}

	items, err := h.store.List(workspaceID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	sources := rankItemsWithVector(items, req.Question, cfg, settingsStore)
	if len(sources) > 3 {
		sources = sources[:3]
	}

	answer, err := callLLM(cfg, settingsStore, req.Question, sources)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	if warning := verifyAnswer(answer, sources); warning != "" {
		answer = warning + "\n\n" + answer
	}

	writeJSON(w, http.StatusOK, AskResponse{Answer: answer, Sources: sources})
}

func rankItemsWithVector(items []Item, query string, cfg config.AppConfig, settingsStore *state.SettingsStore) []Item {
	_, currentModel, _, _, _, _, _, _ := settingsStore.Snapshot()
	provider := findProvider(cfg, currentModel.ProviderID)
	queryEmbed, err := GetEmbedding(query, provider, currentModel.ModelID)
	if err != nil || len(queryEmbed) == 0 {
		return rankItemsKeyword(items, query)
	}

	hasAnyEmbedding := false
	for _, it := range items {
		if len(it.Embedding) > 0 {
			hasAnyEmbedding = true
			break
		}
	}
	if !hasAnyEmbedding {
		return rankItemsKeyword(items, query)
	}

	scored := make([]struct {
		item  Item
		score float64
	}, 0, len(items))
	for _, item := range items {
		if len(item.Embedding) == 0 {
			continue
		}
		sim := CosineSimilarity(queryEmbed, item.Embedding)
		if sim > 0 {
			scored = append(scored, struct {
				item  Item
				score float64
			}{item: item, score: sim})
		}
	}

	for i := 0; i < len(scored); i++ {
		for j := i + 1; j < len(scored); j++ {
			if scored[j].score > scored[i].score {
				scored[i], scored[j] = scored[j], scored[i]
			}
		}
	}

	result := make([]Item, 0, len(scored))
	for _, s := range scored {
		result = append(result, s.item)
	}
	return result
}

func rankItemsKeyword(items []Item, query string) []Item {
	q := strings.ToLower(query)
	keywords := tokenizeQuery(q)
	if len(keywords) == 0 {
		return nil
	}

	scored := make([]struct {
		item  Item
		score int
	}, 0, len(items))

	for _, item := range items {
		score := 0
		text := strings.ToLower(item.Title + " " + item.Summary + " " + item.Content + " " + strings.Join(item.Tags, " "))
		for _, kw := range keywords {
			if strings.Contains(text, kw) {
				score++
			}
		}
		if score > 0 {
			scored = append(scored, struct {
				item  Item
				score int
			}{item: item, score: score})
		}
	}

	for i := 0; i < len(scored); i++ {
		for j := i + 1; j < len(scored); j++ {
			if scored[j].score > scored[i].score {
				scored[i], scored[j] = scored[j], scored[i]
			}
		}
	}

	result := make([]Item, 0, len(scored))
	for _, s := range scored {
		result = append(result, s.item)
	}
	return result
}

func tokenizeQuery(q string) []string {
	hasCJK := false
	for _, r := range q {
		if (r >= 0x4E00 && r <= 0x9FFF) || (r >= 0x3400 && r <= 0x4DBF) || (r >= 0xF900 && r <= 0xFAFF) {
			hasCJK = true
			break
		}
	}
	if !hasCJK {
		return strings.Fields(q)
	}

	// 中文查询：全词匹配 + 二元组分词，同时保留空格分词的结果
	seen := make(map[string]bool)
	var tokens []string

	// 全词保留（去除空格后的完整查询）
	full := strings.ReplaceAll(q, " ", "")
	if len(full) > 0 {
		tokens = append(tokens, full)
		seen[full] = true
	}

	// 空格分词
	for _, w := range strings.Fields(q) {
		if !seen[w] {
			tokens = append(tokens, w)
			seen[w] = true
		}
	}

	// 对纯中文部分做二元组分词
	runes := []rune(full)
	for i := 0; i < len(runes)-1; i++ {
		bigram := string(runes[i : i+2])
		if !seen[bigram] {
			tokens = append(tokens, bigram)
			seen[bigram] = true
		}
	}

	return tokens
}

func callLLM(cfg config.AppConfig, settingsStore *state.SettingsStore, question string, sources []Item) (string, error) {
	_, currentModel, _, _, _, _, _, _ := settingsStore.Snapshot()
	if currentModel.ProviderID == "" || currentModel.ModelID == "" {
		return "", fmt.Errorf("模型未配置")
	}

	provider := findProvider(cfg, currentModel.ProviderID)
	if provider.ProviderID == "" {
		return "", fmt.Errorf("provider 未找到: %s", currentModel.ProviderID)
	}
	if provider.APIKey == "" {
		return "", fmt.Errorf("provider %s 缺少 API Key", provider.ProviderID)
	}

	prompt := buildRAGPrompt(question, sources)
	return sendChatCompletion(provider, currentModel.ModelID, prompt)
}

func findProvider(cfg config.AppConfig, providerID string) config.ProviderConfig {
	for _, p := range cfg.Providers {
		if p.ProviderID == providerID {
			return p
		}
	}
	return config.ProviderConfig{}
}

const (
	maxCharsPerSource    = 2500
	maxTotalContentChars = 7000
)

func buildRAGPrompt(question string, sources []Item) string {
	var b strings.Builder
	b.WriteString("你是一位知识库助手。请根据以下资料回答问题。\n")
	b.WriteString("如果资料不足以回答问题，请明确说明。\n")
	b.WriteString("回答时请引用资料来源编号，例如 [1]、[2]。\n\n")
	b.WriteString("--- 资料 ---\n")

	total := 0
	for i, src := range sources {
		content := truncateText(src.Content, maxCharsPerSource)
		// 确保总内容不超限
		remaining := maxTotalContentChars - total
		if remaining <= 0 {
			break
		}
		if len(content) > remaining {
			content = truncateText(content, remaining)
		}
		total += len(content)
		b.WriteString(fmt.Sprintf("[%d] %s\n%s\n\n", i+1, src.Title, content))
	}
	b.WriteString("--- 资料结束 ---\n\n")
	b.WriteString(fmt.Sprintf("问题：%s\n\n请用中文回答。", question))
	return b.String()
}

func truncateText(s string, maxChars int) string {
	if len(s) <= maxChars {
		return s
	}
	// 在 maxChars 附近找最近的换行符截断，保证语义完整
	cut := maxChars
	for i := maxChars; i > maxChars-200 && i > 0; i-- {
		if s[i] == '\n' {
			cut = i
			break
		}
	}
	return s[:cut] + "\n...(内容已截断)"
}

func verifyAnswer(answer string, sources []Item) string {
	if len(sources) == 0 {
		return ""
	}
	answerLower := strings.ToLower(answer)
	overlapCount := 0
	for _, src := range sources {
		// 检查答案是否引用了来源编号
		for i := 1; i <= len(sources); i++ {
			if strings.Contains(answerLower, fmt.Sprintf("[%d]", i)) {
				overlapCount++
				break
			}
		}
		// 检查答案是否包含资料中的关键短语（取前 30 个字符作为检查点）
		keyPhrase := src.Title
		if len(keyPhrase) > 30 {
			keyPhrase = keyPhrase[:30]
		}
		if strings.Contains(answerLower, strings.ToLower(keyPhrase)) {
			overlapCount++
		}
	}
	if overlapCount == 0 {
		return "[!] 注意：回答未引用所提供的资料，可能存在幻觉"
	}
	return ""
}

func sendChatCompletion(provider config.ProviderConfig, modelID string, prompt string) (string, error) {
	url := provider.BaseURL + provider.ChatCompletionsPath
	payload := map[string]any{
		"model": modelID,
		"messages": []map[string]string{
			{"role": "user", "content": prompt},
		},
		"temperature": 0.3,
		"max_tokens": 2048,
	}
	body, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}

	req, err := http.NewRequest("POST", url, bytes.NewReader(body))
	if err != nil {
		return "", err
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+provider.APIKey)

	client := &http.Client{Timeout: 60 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		text, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("LLM API %d: %s", resp.StatusCode, string(text))
	}

	var result struct {
		Choices []struct {
			Message struct {
				Content string `json:"content"`
			} `json:"message"`
		} `json:"choices"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return "", err
	}
	if len(result.Choices) == 0 {
		return "", fmt.Errorf("LLM 无返回内容")
	}
	return strings.TrimSpace(result.Choices[0].Message.Content), nil
}
