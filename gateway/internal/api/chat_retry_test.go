package api

import (
	"database/sql"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/session"
	"local-agent/gateway/internal/state"

	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

func TestBuildRetryRunRequestRestoresProviderAndResumeStrategy(t *testing.T) {
	repoRoot := t.TempDir()
	cfg := sampleAppConfig()
	require.NoError(t, os.MkdirAll(filepath.Join(repoRoot, "data", "settings"), 0o755))
	credentialStore := state.NewProviderCredentialStore(repoRoot)
	_, err := credentialStore.Save(state.ProviderCredentialRecord{
		ProviderID: "provider-1",
		APIKey:     "secret-key-001",
	})
	require.NoError(t, err)
	require.NoError(t, seedRetryCheckpoint(repoRoot, retryRequestFixture("retryable_failure")))
	handler := newRetryTestHandler(repoRoot, cfg, credentialStore)
	request, err := handler.buildRetryRunRequest(ChatRetryRequest{
		SessionID: "session-1",
		RunID:     "run-1",
	})
	require.NoError(t, err)
	require.Equal(t, "run-1", request.RunID)
	require.Equal(t, "session-1", request.SessionID)
	require.Equal(t, "retry_failure", request.ResumeStrategy)
	require.Equal(t, "checkpoint-1", request.ResumeFromCheckpointID)
	require.Equal(t, "secret-key-001", request.ProviderRef.APIKey)
	require.Nil(t, request.ConfirmationDecision)
	require.Equal(t, "D:/repo", request.ContextHints["repo_root"])
}

func TestBuildRetryRunRequestRejectsNonRetryableCheckpoint(t *testing.T) {
	repoRoot := t.TempDir()
	cfg := sampleAppConfig()
	require.NoError(t, seedRetryCheckpoint(repoRoot, retryRequestFixture("confirmation_required")))
	handler := newRetryTestHandler(repoRoot, cfg, state.NewProviderCredentialStore(repoRoot))
	_, err := handler.buildRetryRunRequest(ChatRetryRequest{
		SessionID:    "session-1",
		RunID:        "run-1",
		CheckpointID: "checkpoint-1",
	})
	require.EqualError(t, err, "当前 checkpoint 不支持失败重试")
}

func newRetryTestHandler(
	repoRoot string,
	cfg config.AppConfig,
	credentialStore *state.ProviderCredentialStore,
) *ChatHandler {
	return NewChatHandler(
		repoRoot,
		cfg,
		runtimeclient.NewClient(19090),
		session.NewEventBus(repoRoot),
		state.NewSettingsStore(repoRoot, cfg),
		state.NewConfirmationStore(),
		credentialStore,
		state.NewRuntimeProviderStore(repoRoot),
	)
}

func seedRetryCheckpoint(repoRoot string, item checkpointRow) error {
	path := filepath.Join(repoRoot, "data", "storage", "main.db")
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	db, err := sql.Open("sqlite", path)
	if err != nil {
		return err
	}
	defer db.Close()
	if err := initCheckpointSchema(db); err != nil {
		return err
	}
	return insertRetryCheckpoint(db, item)
}

func insertRetryCheckpoint(db *sql.DB, item checkpointRow) error {
	requestPayload, err := json.Marshal(item.Request)
	if err != nil {
		return err
	}
	_, err = db.Exec(
		`insert into runtime_checkpoints (
			checkpoint_id, run_id, session_id, trace_id, workspace_id, status, final_stage,
			resumable, resume_reason, resume_stage, event_count, request_payload, response_payload, created_at
		) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		item.CheckpointID,
		item.Request.RunID,
		item.Request.SessionID,
		item.Request.TraceID,
		item.Request.WorkspaceRef.WorkspaceID,
		"failed",
		"Finish",
		boolFlag(item.Resumable),
		item.ResumeReason,
		"Execute",
		2,
		string(requestPayload),
		`{"events":[],"result":{"run_id":"`+item.Request.RunID+`","status":"failed","final_answer":"","summary":"","final_stage":"Finish"}}`,
		item.CreatedAt,
	)
	return err
}

func sampleAppConfig() config.AppConfig {
	return config.AppConfig{
		AppName:      "local-agent",
		GatewayPort:  8080,
		RuntimePort:  19090,
		DefaultMode:  "standard",
		DefaultModel: sampleModel(),
		AvailableModels: []config.ModelRef{
			sampleModel(),
		},
		Providers: []config.ProviderConfig{
			{
				ProviderID:          "provider-1",
				DisplayName:         "Provider 1",
				BaseURL:             "https://example.invalid",
				ChatCompletionsPath: "/chat",
				ModelsPath:          "/models",
			},
		},
		DefaultWorkspace: sampleWorkspace(),
		Workspaces: []config.WorkspaceRef{
			sampleWorkspace(),
		},
		Siyuan: config.SiyuanConfig{},
	}
}

func sampleModel() config.ModelRef {
	return config.ModelRef{
		ProviderID:  "provider-1",
		ModelID:     "model-1",
		DisplayName: "Model 1",
		Enabled:     true,
		Available:   true,
	}
}

func sampleWorkspace() config.WorkspaceRef {
	return config.WorkspaceRef{
		WorkspaceID: "workspace-1",
		Name:        "Workspace 1",
		RootPath:    "D:/workspace",
		IsActive:    true,
	}
}

func retryRequestFixture(resumeReason string) checkpointRow {
	item := checkpointFixture("checkpoint-1", "100", true, resumeReason)
	item.Request.ProviderRef = contracts.ProviderRef{
		ProviderID:          "provider-1",
		DisplayName:         "Provider 1",
		BaseURL:             "https://example.invalid",
		ChatCompletionsPath: "/chat",
		ModelsPath:          "/models",
	}
	return item
}

type checkpointRow struct {
	CheckpointID string
	CreatedAt    string
	Resumable    bool
	ResumeReason string
	Request      contracts.RunRequest
}

func checkpointFixture(
	checkpointID string,
	createdAt string,
	resumable bool,
	resumeReason string,
) checkpointRow {
	return checkpointRow{
		CheckpointID: checkpointID,
		CreatedAt:    createdAt,
		Resumable:    resumable,
		ResumeReason: resumeReason,
		Request: contracts.RunRequest{
			RequestID: "request-1",
			RunID:     "run-1",
			SessionID: "session-1",
			TraceID:   "trace-1",
			UserInput: "retry me",
			Mode:      "standard",
			ModelRef:  sampleModel(),
			WorkspaceRef: config.WorkspaceRef{
				WorkspaceID: "workspace-1",
				Name:        "Workspace 1",
				RootPath:    "D:/workspace",
				IsActive:    true,
			},
			ContextHints: map[string]string{"repo_root": "D:/repo"},
		},
	}
}

func initCheckpointSchema(db *sql.DB) error {
	_, err := db.Exec(`create table runtime_checkpoints (
		checkpoint_id text primary key,
		run_id text not null,
		session_id text not null,
		trace_id text not null,
		workspace_id text not null,
		status text not null,
		final_stage text not null,
		resumable integer not null default 0,
		resume_reason text not null default '',
		resume_stage text not null default '',
		event_count integer not null default 0,
		request_payload text not null,
		response_payload text not null,
		created_at text not null
	)`)
	return err
}

func boolFlag(value bool) int {
	if value {
		return 1
	}
	return 0
}
