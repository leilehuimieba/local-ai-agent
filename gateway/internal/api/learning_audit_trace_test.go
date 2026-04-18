package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"local-agent/gateway/internal/providers/bestblogs"
)

type auditTraceEvidenceCase struct {
	ArticleURL  string          `json:"article_url"`
	ArticleID   string          `json:"article_id"`
	Title       string          `json:"title"`
	TraceID     string          `json:"trace_id"`
	OK          bool            `json:"ok"`
	Checks      map[string]bool `json:"checks"`
	StageCount  int             `json:"stage_count"`
	ReplayReady bool            `json:"replay_ready"`
	FirstStage  string          `json:"first_stage"`
	LastStage   string          `json:"last_stage"`
}

type auditTraceEvidenceReport struct {
	GeneratedAt    string                   `json:"generated_at"`
	TotalSamples   int                      `json:"total_samples"`
	SuccessCount   int                      `json:"success_count"`
	TraceLinkCount int                      `json:"trace_link_count"`
	TraceLinkRate  float64                  `json:"trace_link_rate"`
	Threshold      float64                  `json:"threshold"`
	Passed         bool                     `json:"passed"`
	Samples        []auditTraceEvidenceCase `json:"samples"`
}

func TestBuildLearningAuditTraceResponse(t *testing.T) {
	deps := newLearningMemoryDeps(t)
	article := bestblogs.ArticleResponse{
		Provider: "bestblogs", Strategy: "public_api", ArticleID: "42acaf7d",
		Meta:    bestblogs.ArticleMeta{Title: "浏览器自动化：从 GUI 到 OpenCLI", Tags: []string{"浏览器自动化", "AI Agent", "OpenCLI", "RPA"}, SourceURL: "https://www.bestblogs.dev/article/42acaf7d"},
		Summary: bestblogs.ArticleSummary{OneSentence: "从 GUI 自动化转向 API 驱动。", MainPoints: []bestblogs.MainPoint{{Point: "为什么我们需要浏览器自动化"}, {Point: "API 路径更稳定"}, {Point: "OpenCLI 更适合 Agent"}, {Point: "未来软件竞争维度发生变化"}}},
		Content: bestblogs.ArticleContent{Markdown: stringsRepeat("浏览器自动化 OpenCLI Agent 软件工程 ", 220), Images: []string{"a", "b", "c"}},
	}
	resp, err := buildLearningAuditTraceResponse(deps.store, "main", article, learningAuditTraceRequest{TraceID: "trace-test", Language: "zh"})
	require.NoError(t, err)
	require.Equal(t, "trace-test", resp.TraceID)
	require.Len(t, resp.Steps, 6)
	require.True(t, allAuditTraceLinked(resp.Steps, resp.TraceID))
}

func TestGenerateLearningAuditTraceEvidence(t *testing.T) {
	deps := newLearningMemoryDeps(t)
	report := buildLearningAuditTraceReport(t, deps)
	writeLearningAuditTraceEvidence(t, report)
	require.GreaterOrEqual(t, report.TraceLinkRate, 1.0)
}

func buildLearningAuditTraceReport(t *testing.T, deps memoryRouteDeps) auditTraceEvidenceReport {
	samples := make([]auditTraceEvidenceCase, 0, len(bestblogsExtractEvidenceInputs))
	for _, item := range bestblogsExtractEvidenceInputs {
		samples = append(samples, runLearningAuditTraceSample(t, deps, item))
	}
	success := countAuditTraceSuccess(samples)
	linked := countAuditTraceLinked(samples)
	rate := successRate(linked, len(samples))
	return auditTraceEvidenceReport{
		GeneratedAt: time.Now().Format(time.RFC3339), TotalSamples: len(samples),
		SuccessCount: success, TraceLinkCount: linked, TraceLinkRate: rate,
		Threshold: 1, Passed: rate >= 1, Samples: samples,
	}
}

func runLearningAuditTraceSample(t *testing.T, deps memoryRouteDeps, sample extractEvidenceInput) auditTraceEvidenceCase {
	resp := requestLearningAuditTraceResponse(t, deps, sample)
	checks := auditTraceChecks(resp)
	return auditTraceEvidenceCase{
		ArticleURL: sample.ArticleURL, ArticleID: resp.ArticleID, Title: resp.Title,
		TraceID: resp.TraceID, OK: allChecksPass(checks), Checks: checks,
		StageCount: len(resp.Steps), ReplayReady: resp.ReplayReady,
		FirstStage: resp.Steps[0].Stage, LastStage: resp.Steps[len(resp.Steps)-1].Stage,
	}
}

func requestLearningAuditTraceResponse(t *testing.T, deps memoryRouteDeps, sample extractEvidenceInput) learningAuditTraceResponse {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/learning/audit-trace", bytes.NewReader(learningAuditTraceBody(t, sample)))
	req.Header.Set("Content-Type", "application/json")
	learningAuditTraceHandler(deps).ServeHTTP(recorder, req)
	require.Equal(t, http.StatusOK, recorder.Code)
	return decodeLearningAuditTraceResponse(t, recorder.Body.Bytes())
}

func learningAuditTraceBody(t *testing.T, sample extractEvidenceInput) []byte {
	body, err := json.Marshal(learningAuditTraceRequest{
		ArticleURL: sample.ArticleURL, ProviderHint: "bestblogs", Language: "zh",
	})
	require.NoError(t, err)
	return body
}

func decodeLearningAuditTraceResponse(t *testing.T, body []byte) learningAuditTraceResponse {
	var resp learningAuditTraceResponse
	require.NoError(t, json.Unmarshal(body, &resp))
	return resp
}

func auditTraceChecks(resp learningAuditTraceResponse) map[string]bool {
	return map[string]bool{
		"ok":            resp.OK,
		"trace_id":      resp.TraceID != "",
		"replay_ready":  resp.ReplayReady,
		"stage_count":   len(resp.Steps) == 6,
		"trace_linked":  allAuditTraceLinked(resp.Steps, resp.TraceID),
		"evidence_refs": len(resp.EvidenceRefs) >= 6,
		"memory_stage":  resp.Steps[len(resp.Steps)-1].Stage == "memory_write",
	}
}

func allAuditTraceLinked(steps []learningAuditTraceStep, traceID string) bool {
	for _, step := range steps {
		if step.TraceID != traceID {
			return false
		}
	}
	return true
}

func countAuditTraceSuccess(samples []auditTraceEvidenceCase) int {
	count := 0
	for _, sample := range samples {
		if sample.OK {
			count++
		}
	}
	return count
}

func countAuditTraceLinked(samples []auditTraceEvidenceCase) int {
	count := 0
	for _, sample := range samples {
		if sample.TraceID != "" && sample.OK {
			count++
		}
	}
	return count
}

func writeLearningAuditTraceEvidence(t *testing.T, report auditTraceEvidenceReport) {
	data, err := json.MarshalIndent(report, "", "  ")
	require.NoError(t, err)
	path := filepath.Join("..", "..", "..", "tmp", "stage-h-learning", "audit-trace.json")
	require.NoError(t, os.MkdirAll(filepath.Dir(path), 0o755))
	require.NoError(t, os.WriteFile(path, data, 0o644))
}
