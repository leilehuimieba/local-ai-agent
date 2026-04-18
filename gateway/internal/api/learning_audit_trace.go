package api

import (
	"net/http"
	"strconv"

	"local-agent/gateway/internal/memory"
	"local-agent/gateway/internal/providers/bestblogs"
)

type learningAuditTraceRequest struct {
	ArticleURL   string `json:"article_url"`
	ProviderHint string `json:"provider_hint,omitempty"`
	Language     string `json:"language,omitempty"`
	TraceID      string `json:"trace_id,omitempty"`
}

type learningAuditTraceResponse struct {
	OK           bool                     `json:"ok"`
	Provider     string                   `json:"provider"`
	Strategy     string                   `json:"strategy"`
	ArticleID    string                   `json:"article_id"`
	Title        string                   `json:"title"`
	TraceID      string                   `json:"trace_id"`
	ReplayReady  bool                     `json:"replay_ready"`
	Steps        []learningAuditTraceStep `json:"steps"`
	EvidenceRefs []string                 `json:"evidence_refs"`
}

type learningAuditTraceStep struct {
	Sequence    int    `json:"sequence"`
	TraceID     string `json:"trace_id"`
	Stage       string `json:"stage"`
	EventType   string `json:"event_type"`
	Status      string `json:"status"`
	Summary     string `json:"summary"`
	ArtifactRef string `json:"artifact_ref"`
}

func learningAuditTraceHandler(deps memoryRouteDeps) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeLearningAuditTraceRequest(w, r)
		if !ok {
			return
		}
		article, ok := readLearningArticle(w, r, learningAuditPayload(payload))
		if !ok {
			return
		}
		workspaceID, ok := currentWorkspaceID(deps.state)
		if !ok {
			http.Error(w, "workspace not found", http.StatusNotFound)
			return
		}
		response, err := buildLearningAuditTraceResponse(deps.store, workspaceID, article, payload)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		writeJSON(w, http.StatusOK, response)
	}
}

func decodeLearningAuditTraceRequest(w http.ResponseWriter, r *http.Request) (learningAuditTraceRequest, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return learningAuditTraceRequest{}, false
	}
	var payload learningAuditTraceRequest
	if !decodeJSONBody(w, r, &payload) || payload.ArticleURL == "" {
		http.Error(w, "article_url is required", http.StatusBadRequest)
		return learningAuditTraceRequest{}, false
	}
	return payload, true
}

func learningAuditPayload(payload learningAuditTraceRequest) learningExtractRequest {
	return learningExtractRequest{
		ArticleURL: payload.ArticleURL, ProviderHint: payload.ProviderHint, Language: payload.Language,
	}
}

func buildLearningAuditTraceResponse(store *memory.Store, workspaceID string, article bestblogs.ArticleResponse, payload learningAuditTraceRequest) (learningAuditTraceResponse, error) {
	traceID := learningTraceID(payload.TraceID)
	score := scoreLearningArticle(article)
	explain := buildLearningExplain(article)
	translate := buildLearningTranslate(article, learningTranslateRequest{Language: payload.Language, TargetLanguage: "en"})
	recommend := buildLearningRecommend(article)
	memoryResp, err := buildLearningMemoryResponse(store, workspaceID, article)
	if err != nil {
		return learningAuditTraceResponse{}, err
	}
	steps := buildLearningAuditSteps(traceID, article, score, explain, translate, recommend, memoryResp)
	return learningAuditTraceResponse{
		OK: true, Provider: article.Provider, Strategy: article.Strategy, ArticleID: article.ArticleID,
		Title: article.Meta.Title, TraceID: traceID, ReplayReady: len(steps) > 0,
		Steps: steps, EvidenceRefs: learningAuditEvidenceRefs(),
	}, nil
}

func learningTraceID(traceID string) string {
	if traceID != "" {
		return traceID
	}
	return newID("trace")
}

func buildLearningAuditSteps(traceID string, article bestblogs.ArticleResponse, score learningValueScoreResponse, explain learningExplainResponse, translate learningTranslateResponse, recommend learningRecommendResponse, memoryResp learningMemoryWriteResponse) []learningAuditTraceStep {
	return []learningAuditTraceStep{
		learningExtractAuditStep(traceID, article),
		learningExplainAuditStep(traceID, article.ArticleID, explain),
		learningTranslateAuditStep(traceID, article.ArticleID, translate),
		learningScoreAuditStep(traceID, article.ArticleID, score),
		learningRecommendAuditStep(traceID, article.ArticleID, recommend),
		learningMemoryAuditStep(traceID, article.ArticleID, memoryResp),
	}
}

func learningExtractAuditStep(traceID string, article bestblogs.ArticleResponse) learningAuditTraceStep {
	return learningAuditTraceStep{
		Sequence: 1, TraceID: traceID, Stage: "extract", EventType: "extract_completed",
		Status: "ok", Summary: article.Meta.Title + "；main_points=" + strconv.Itoa(len(article.Summary.MainPoints)),
		ArtifactRef: learningAuditArtifact("extract", article.ArticleID),
	}
}

func learningExplainAuditStep(traceID string, articleID string, explain learningExplainResponse) learningAuditTraceStep {
	return learningAuditTraceStep{
		Sequence: 2, TraceID: traceID, Stage: "explain", EventType: "explain_completed",
		Status: "ok", Summary: "key_terms=" + strconv.Itoa(len(explain.KeyTerms)),
		ArtifactRef: learningAuditArtifact("explain", articleID),
	}
}

func learningTranslateAuditStep(traceID string, articleID string, translate learningTranslateResponse) learningAuditTraceStep {
	return learningAuditTraceStep{
		Sequence: 3, TraceID: traceID, Stage: "translate", EventType: "translate_completed",
		Status: "ok", Summary: "type=" + translate.TranslationType + "；main_points=" + strconv.Itoa(len(translate.MainPoints)),
		ArtifactRef: learningAuditArtifact("translate", articleID),
	}
}

func learningScoreAuditStep(traceID string, articleID string, score learningValueScoreResponse) learningAuditTraceStep {
	return learningAuditTraceStep{
		Sequence: 4, TraceID: traceID, Stage: "value_score", EventType: "value_score_completed",
		Status: "ok", Summary: "score=" + strconv.Itoa(score.Score) + "；level=" + score.Level,
		ArtifactRef: learningAuditArtifact("value-score", articleID),
	}
}

func learningRecommendAuditStep(traceID string, articleID string, recommend learningRecommendResponse) learningAuditTraceStep {
	return learningAuditTraceStep{
		Sequence: 5, TraceID: traceID, Stage: "recommend", EventType: "recommend_completed",
		Status: "ok", Summary: recommend.Recommendation,
		ArtifactRef: learningAuditArtifact("recommend", articleID),
	}
}

func learningMemoryAuditStep(traceID string, articleID string, memoryResp learningMemoryWriteResponse) learningAuditTraceStep {
	return learningAuditTraceStep{
		Sequence: 6, TraceID: traceID, Stage: "memory_write", EventType: "memory_write_completed",
		Status: "ok", Summary: "route=" + memoryResp.Route + "；write_status=" + memoryResp.WriteStatus + "；recall=" + strconv.Itoa(memoryResp.RecallCount),
		ArtifactRef: learningAuditArtifact("memory", articleID),
	}
}

func learningAuditArtifact(stage string, articleID string) string {
	return "learning:" + stage + ":" + articleID
}

func learningAuditEvidenceRefs() []string {
	return []string{
		"tmp/stage-h-learning/extract.json",
		"tmp/stage-h-learning/explain-translate.json",
		"tmp/stage-h-learning/value-score.json",
		"tmp/stage-h-learning/recommend.json",
		"tmp/stage-h-learning/memory-routing.json",
		"tmp/stage-h-learning/audit-trace.json",
	}
}
