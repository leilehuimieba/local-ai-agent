package api

import (
	"net/http"
	"strconv"
	"strings"

	"local-agent/gateway/internal/memory"
	"local-agent/gateway/internal/providers/bestblogs"
	"local-agent/gateway/internal/service"
)

type learningMemoryWriteResponse struct {
	OK               bool   `json:"ok"`
	Provider         string `json:"provider"`
	Strategy         string `json:"strategy"`
	ArticleID        string `json:"article_id"`
	Title            string `json:"title"`
	Route            string `json:"route"`
	WriteStatus      string `json:"write_status"`
	Reason           string `json:"reason"`
	MemoryID         string `json:"memory_id"`
	MemoryTitle      string `json:"memory_title"`
	Score            int    `json:"score"`
	Level            string `json:"level"`
	RecallCount      int    `json:"recall_count"`
	MemoryDigest     string `json:"memory_digest"`
	InjectionPreview string `json:"injection_preview"`
}

type learningMemoryDecision struct {
	Route            string
	WriteStatus      string
	Reason           string
	MemoryID         string
	MemoryTitle      string
	RecallCount      int
	MemoryDigest     string
	InjectionPreview string
}

func learningMemoryWriteHandler(deps memoryRouteDeps) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeLearningExtractRequest(w, r)
		if !ok {
			return
		}
		article, ok := readLearningArticle(w, r, payload)
		if !ok {
			return
		}
		workspaceID, ok := currentWorkspaceID(deps.state)
		if !ok {
			http.Error(w, "workspace not found", http.StatusNotFound)
			return
		}
		response, err := buildLearningMemoryResponse(deps.store, workspaceID, article)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		writeJSON(w, http.StatusOK, response)
	}
}

func buildLearningMemoryResponse(store *memory.Store, workspaceID string, article bestblogs.ArticleResponse) (learningMemoryWriteResponse, error) {
	signals := buildLearningValueSignals(article)
	score := service.CalculateLearningValueScore(signals)
	decision, err := routeLearningMemory(store, workspaceID, article, score)
	if err != nil {
		return learningMemoryWriteResponse{}, err
	}
	return learningMemoryWriteResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		Title: article.Meta.Title, Route: decision.Route, WriteStatus: decision.WriteStatus,
		Reason: decision.Reason, MemoryID: decision.MemoryID, MemoryTitle: decision.MemoryTitle,
		Score: score, Level: service.LearningValueLevel(score), RecallCount: decision.RecallCount,
		MemoryDigest: decision.MemoryDigest, InjectionPreview: decision.InjectionPreview,
	}, nil
}

func routeLearningMemory(store *memory.Store, workspaceID string, article bestblogs.ArticleResponse, score int) (learningMemoryDecision, error) {
	if score < 70 {
		return skippedLearningMemoryDecision(score), nil
	}
	entry := learningMemoryEntry(workspaceID, article, score)
	written, err := store.Save(entry)
	if err != nil {
		return learningMemoryDecision{}, err
	}
	items, err := store.List(workspaceID)
	if err != nil {
		return learningMemoryDecision{}, err
	}
	recall := selectLearningRecall(items, article)
	return learningMemoryDecision{
		Route: "long_term_memory", WriteStatus: learningWriteStatus(written),
		Reason: learningMemoryReason(score, written), MemoryID: entry.ID, MemoryTitle: entry.Title,
		RecallCount: len(recall), MemoryDigest: learningMemoryDigest(recall),
		InjectionPreview: learningInjectionPreview(recall),
	}, nil
}

func skippedLearningMemoryDecision(score int) learningMemoryDecision {
	return learningMemoryDecision{
		Route: "skip", WriteStatus: "skipped_low_score",
		Reason:       "文章当前分值不足长期记忆写入阈值，暂不写入，只保留即时阅读结果。",
		MemoryDigest: "当前没有命中相关长期记忆。", InjectionPreview: learningInjectionPreview(nil),
	}
}

func learningMemoryEntry(workspaceID string, article bestblogs.ArticleResponse, score int) memory.Entry {
	now := timestampNow()
	return memory.Entry{
		ID: learningMemoryID(article), Kind: "learning_article", Title: "学习文章：" + article.Meta.Title,
		Summary: learningMemorySummary(article), Content: learningMemoryContent(article, score),
		Scope: "learning_mode", WorkspaceID: workspaceID, Source: article.Meta.SourceURL,
		SourceType: "runtime", SourceTitle: article.Meta.Title, SourceEventType: "learning_memory_written",
		SourceArtifactPath: "", GovernanceVersion: memory.MemoryGovernanceVersion,
		GovernanceReason: "高价值学习文章已按学习模式最小规则写入长期记忆。",
		GovernanceSource: "learning_mode_article", GovernanceAt: now, Verified: true,
		Priority: learningMemoryPriority(score), Archived: false, CreatedAt: now, UpdatedAt: now, Timestamp: now,
	}
}

func learningMemoryID(article bestblogs.ArticleResponse) string {
	return "learning-article-" + article.ArticleID
}

func learningMemorySummary(article bestblogs.ArticleResponse) string {
	if strings.TrimSpace(article.Summary.OneSentence) != "" {
		return strings.TrimSpace(article.Summary.OneSentence)
	}
	return firstLearningPoint(article)
}

func learningMemoryContent(article bestblogs.ArticleResponse, score int) string {
	return strings.Join([]string{
		"title=" + article.Meta.Title,
		"article_id=" + article.ArticleID,
		"score=" + service.LearningValueLevel(score),
		"tags=" + strings.Join(article.Meta.Tags, "|"),
		"summary=" + learningMemorySummary(article),
	}, "; ")
}

func learningMemoryPriority(score int) int {
	if score >= 85 {
		return 80
	}
	return 65
}

func learningWriteStatus(written bool) string {
	if written {
		return "written"
	}
	return "duplicate"
}

func learningMemoryReason(score int, written bool) string {
	if written {
		return "文章分值达到学习模式写入阈值，已写入长期记忆并可参与后续注入。"
	}
	if score >= 70 {
		return "文章已命中历史重复项，本次跳过重复写入，但可直接参与后续注入。"
	}
	return "文章当前仅保留即时结果，不进入长期记忆。"
}

func selectLearningRecall(items []memory.Entry, article bestblogs.ArticleResponse) []memory.Entry {
	selected := make([]memory.Entry, 0, 3)
	for _, item := range items {
		if learningRecallMatch(item, article) {
			selected = append(selected, item)
		}
	}
	if len(selected) > 3 {
		return selected[:3]
	}
	return selected
}

func learningRecallMatch(item memory.Entry, article bestblogs.ArticleResponse) bool {
	return strings.Contains(item.Source, article.ArticleID) || item.SourceTitle == article.Meta.Title
}

func learningMemoryDigest(items []memory.Entry) string {
	if len(items) == 0 {
		return "当前没有命中相关长期记忆。"
	}
	lines := make([]string, 0, len(items))
	for _, item := range items {
		lines = append(lines, learningDigestLine(item))
	}
	return strings.Join(lines, " || ")
}

func learningDigestLine(item memory.Entry) string {
	return "[" + item.Kind + "] " + item.Summary + " | 来源=" + item.Source + " | 类型=" + item.SourceType +
		" | 理由=" + item.Reason + " | 优先级=" + strconv.Itoa(item.Priority) +
		" | 更新时间=" + learningUpdatedAt(item)
}

func learningUpdatedAt(item memory.Entry) string {
	if strings.TrimSpace(item.UpdatedAt) != "" {
		return item.UpdatedAt
	}
	return item.Timestamp
}

func learningInjectionPreview(items []memory.Entry) string {
	if len(items) == 0 {
		return "当前没有可注入的学习记忆。"
	}
	return "学习模式注入预览：" + learningMemoryDigest(items)
}
