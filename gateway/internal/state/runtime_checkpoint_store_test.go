package state

import (
	"database/sql"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"local-agent/gateway/internal/contracts"

	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

func TestFindRetryableReturnsLatestRetryableCheckpoint(t *testing.T) {
	repoRoot := t.TempDir()
	store := NewRuntimeCheckpointStore(repoRoot)
	require.NoError(t, os.MkdirAll(filepath.Dir(store.path), 0o755))
	db, err := sql.Open("sqlite", store.path)
	require.NoError(t, err)
	defer db.Close()
	require.NoError(t, initCheckpointSchema(db))
	insertCheckpointRecord(t, db, checkpointFixture("checkpoint-old", "100", true, "retryable_failure"))
	insertCheckpointRecord(t, db, checkpointFixture("checkpoint-new", "200", true, "retryable_failure"))
	item, err := store.FindRetryable("run-1", "session-1", "")
	require.NoError(t, err)
	require.Equal(t, "checkpoint-new", item.CheckpointID)
	require.Equal(t, "retryable_failure", item.ResumeReason)
	require.Equal(t, "provider-1", item.Request.ModelRef.ProviderID)
}

func TestFindRetryableByCheckpointIDReturnsNotFound(t *testing.T) {
	repoRoot := t.TempDir()
	store := NewRuntimeCheckpointStore(repoRoot)
	require.NoError(t, os.MkdirAll(filepath.Dir(store.path), 0o755))
	db, err := sql.Open("sqlite", store.path)
	require.NoError(t, err)
	defer db.Close()
	require.NoError(t, initCheckpointSchema(db))
	_, err = store.FindRetryable("run-1", "session-1", "missing-checkpoint")
	require.ErrorIs(t, err, ErrRuntimeCheckpointNotFound())
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

func insertCheckpointRecord(t *testing.T, db *sql.DB, item checkpointRow) {
	t.Helper()
	requestPayload, err := json.Marshal(item.Request)
	require.NoError(t, err)
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
	require.NoError(t, err)
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
			ModelRef: contracts.ModelRef{
				ProviderID:  "provider-1",
				ModelID:     "model-1",
				DisplayName: "Model 1",
			},
			WorkspaceRef: contracts.WorkspaceRef{
				WorkspaceID: "workspace-1",
				Name:        "Workspace 1",
				RootPath:    "D:/workspace",
				IsActive:    true,
			},
			ContextHints: map[string]string{"repo_root": "D:/repo"},
		},
	}
}

func boolFlag(value bool) int {
	if value {
		return 1
	}
	return 0
}
