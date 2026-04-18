package api

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/memory"
	"local-agent/gateway/internal/providers/bestblogs"
	"local-agent/gateway/internal/state"
)

type memoryRoutingEvidenceCase struct {
	ArticleURL    string          `json:"article_url"`
	ArticleID     string          `json:"article_id"`
	Title         string          `json:"title"`
	OK            bool            `json:"ok"`
	Checks        map[string]bool `json:"checks"`
	Route         string          `json:"route"`
	WriteStatus   string          `json:"write_status"`
	RecallCount   int             `json:"recall_count"`
	DigestPreview string          `json:"digest_preview"`
	InjectPreview string          `json:"inject_preview"`
}

type memoryRoutingEvidenceReport struct {
	GeneratedAt      string                      `json:"generated_at"`
	TotalSamples     int                         `json:"total_samples"`
	SuccessCount     int                         `json:"success_count"`
	RecallReadyCount int                         `json:"recall_ready_count"`
	InjectReadyCount int                         `json:"inject_ready_count"`
	EffectiveHitRate float64                     `json:"effective_hit_rate"`
	Threshold        float64                     `json:"threshold"`
	Passed           bool                        `json:"passed"`
	Samples          []memoryRoutingEvidenceCase `json:"samples"`
}

func TestBuildLearningMemoryResponse(t *testing.T) {
	deps := newLearningMemoryDeps(t)
	article := bestblogs.ArticleResponse{
		Provider: "bestblogs", Strategy: "public_api", ArticleID: "42acaf7d",
		Meta:    bestblogs.ArticleMeta{Title: "浏览器自动化：从 GUI 到 OpenCLI", Tags: []string{"浏览器自动化", "AI Agent", "OpenCLI", "RPA"}, SourceURL: "https://www.bestblogs.dev/article/42acaf7d"},
		Summary: bestblogs.ArticleSummary{OneSentence: "从 GUI 自动化转向 API 驱动。", MainPoints: []bestblogs.MainPoint{{Point: "为什么我们需要浏览器自动化"}, {Point: "API 路径更稳定"}, {Point: "OpenCLI 更适合 Agent"}, {Point: "未来软件竞争维度发生变化"}}},
		Content: bestblogs.ArticleContent{Markdown: stringsRepeat("浏览器自动化 OpenCLI Agent 软件工程 ", 220), Images: []string{"a", "b", "c"}},
	}
	response, err := buildLearningMemoryResponse(deps.store, "main", article)
	require.NoError(t, err)
	require.Equal(t, "long_term_memory", response.Route)
	require.NotEmpty(t, response.MemoryDigest)
	require.NotEmpty(t, response.InjectionPreview)
}

func TestGenerateLearningMemoryRoutingEvidence(t *testing.T) {
	deps := newLearningMemoryDeps(t)
	report := buildLearningMemoryRoutingReport(t, deps)
	writeLearningMemoryRoutingEvidence(t, report)
	require.GreaterOrEqual(t, report.EffectiveHitRate, 0.8)
}

func buildLearningMemoryRoutingReport(t *testing.T, deps memoryRouteDeps) memoryRoutingEvidenceReport {
	samples := make([]memoryRoutingEvidenceCase, 0, len(bestblogsExtractEvidenceInputs))
	for _, item := range bestblogsExtractEvidenceInputs {
		samples = append(samples, runLearningMemorySample(t, deps, item))
	}
	success := countMemoryRoutingSuccess(samples)
	recall := countMemoryRoutingReady(samples, "recall_count")
	inject := countMemoryRoutingReady(samples, "inject_preview")
	rate := successRate(success, len(samples))
	return memoryRoutingEvidenceReport{
		GeneratedAt: time.Now().Format(time.RFC3339), TotalSamples: len(samples),
		SuccessCount: success, RecallReadyCount: recall, InjectReadyCount: inject,
		EffectiveHitRate: rate, Threshold: 0.8, Passed: rate >= 0.8, Samples: samples,
	}
}

func runLearningMemorySample(t *testing.T, deps memoryRouteDeps, sample extractEvidenceInput) memoryRoutingEvidenceCase {
	response := requestLearningMemoryResponse(t, deps, sample)
	checks := learningMemoryChecks(response)
	return memoryRoutingEvidenceCase{
		ArticleURL: sample.ArticleURL, ArticleID: response.ArticleID, Title: response.Title,
		OK: allChecksPass(checks), Checks: checks, Route: response.Route, WriteStatus: response.WriteStatus,
		RecallCount: response.RecallCount, DigestPreview: previewText(response.MemoryDigest),
		InjectPreview: previewText(response.InjectionPreview),
	}
}

func requestLearningMemoryResponse(t *testing.T, deps memoryRouteDeps, sample extractEvidenceInput) learningMemoryWriteResponse {
	recorder := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/v1/learning/memory/write", bytes.NewReader(learningMemoryRequestBody(t, sample)))
	req.Header.Set("Content-Type", "application/json")
	learningMemoryWriteHandler(deps).ServeHTTP(recorder, req)
	require.Equal(t, http.StatusOK, recorder.Code)
	return decodeLearningMemoryResponse(t, recorder.Body.Bytes())
}

func learningMemoryRequestBody(t *testing.T, sample extractEvidenceInput) []byte {
	body, err := json.Marshal(learningExtractRequest{
		ArticleURL: sample.ArticleURL, ProviderHint: "bestblogs", Language: "zh",
	})
	require.NoError(t, err)
	return body
}

func decodeLearningMemoryResponse(t *testing.T, body []byte) learningMemoryWriteResponse {
	var response learningMemoryWriteResponse
	require.NoError(t, json.Unmarshal(body, &response))
	return response
}

func learningMemoryChecks(response learningMemoryWriteResponse) map[string]bool {
	return map[string]bool{
		"ok":                response.OK,
		"article_id":        response.ArticleID != "",
		"route":             response.Route == "long_term_memory",
		"write_status":      response.WriteStatus == "written" || response.WriteStatus == "duplicate",
		"memory_id":         response.MemoryID != "",
		"recall_count":      response.RecallCount > 0,
		"memory_digest":     response.MemoryDigest != "" && response.MemoryDigest != "当前没有命中相关长期记忆。",
		"injection_preview": response.InjectionPreview != "" && response.InjectionPreview != "当前没有可注入的学习记忆。",
	}
}

func countMemoryRoutingSuccess(samples []memoryRoutingEvidenceCase) int {
	count := 0
	for _, sample := range samples {
		if sample.OK {
			count++
		}
	}
	return count
}

func countMemoryRoutingReady(samples []memoryRoutingEvidenceCase, field string) int {
	count := 0
	for _, sample := range samples {
		if field == "recall_count" && sample.RecallCount > 0 {
			count++
		}
		if field == "inject_preview" && sample.InjectPreview != "" {
			count++
		}
	}
	return count
}

func writeLearningMemoryRoutingEvidence(t *testing.T, report memoryRoutingEvidenceReport) {
	data, err := json.MarshalIndent(report, "", "  ")
	require.NoError(t, err)
	path := filepath.Join("..", "..", "..", "tmp", "stage-h-learning", "memory-routing.json")
	require.NoError(t, os.MkdirAll(filepath.Dir(path), 0o755))
	require.NoError(t, os.WriteFile(path, data, 0o644))
}

func newLearningMemoryDeps(t *testing.T) memoryRouteDeps {
	repoRoot := t.TempDir()
	cfg := learningMemoryTestConfig(repoRoot)
	initLearningMemoryDB(t, repoRoot)
	return memoryRouteDeps{store: memory.NewStore(repoRoot), state: state.NewSettingsStore(repoRoot, cfg)}
}

func learningMemoryTestConfig(repoRoot string) config.AppConfig {
	workspace := config.WorkspaceRef{WorkspaceID: "main", Name: "test", RootPath: repoRoot, IsActive: true}
	model := config.ModelRef{ProviderID: "test", ModelID: "test", DisplayName: "test", Enabled: true, Available: true}
	return config.AppConfig{DefaultMode: "standard", DefaultModel: model, AvailableModels: []config.ModelRef{model}, DefaultWorkspace: workspace, Workspaces: []config.WorkspaceRef{workspace}}
}

func initLearningMemoryDB(t *testing.T, repoRoot string) {
	path := filepath.Join(repoRoot, "data", "storage", "main.db")
	require.NoError(t, os.MkdirAll(filepath.Dir(path), 0o755))
	db, err := sql.Open("sqlite", path)
	require.NoError(t, err)
	defer db.Close()
	_, err = db.Exec(learningMemoryTableSQL)
	require.NoError(t, err)
}

const learningMemoryTableSQL = `
create table if not exists long_term_memory (
id text primary key,
workspace_id text not null,
memory_type text not null,
title text not null,
summary text not null,
content text not null,
source text not null,
source_run_id text not null default '',
source_type text not null default '',
source_title text not null default '',
source_event_type text not null default '',
source_artifact_path text not null default '',
governance_version text not null default '',
governance_reason text not null default '',
governance_source text not null default '',
governance_at text not null default '',
archive_reason text not null default '',
verified integer not null default 0,
priority integer not null default 0,
archived integer not null default 0,
archived_at text not null default '',
created_at text not null default '',
updated_at text not null default '',
scope text not null default '',
session_id text not null default '',
timestamp text not null default ''
)`
