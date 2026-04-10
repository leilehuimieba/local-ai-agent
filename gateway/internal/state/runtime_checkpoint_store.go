package state

import (
	"database/sql"
	"encoding/json"
	"errors"
	"path/filepath"

	"local-agent/gateway/internal/contracts"

	_ "modernc.org/sqlite"
)

var errRuntimeCheckpointNotFound = errors.New("checkpoint not found")

func ErrRuntimeCheckpointNotFound() error {
	return errRuntimeCheckpointNotFound
}

type RuntimeCheckpointRecord struct {
	CheckpointID string
	RunID        string
	SessionID    string
	WorkspaceID  string
	Resumable    bool
	ResumeReason string
	Request      contracts.RunRequest
}

type RuntimeCheckpointStore struct {
	path string
}

func NewRuntimeCheckpointStore(repoRoot string) *RuntimeCheckpointStore {
	return &RuntimeCheckpointStore{
		path: filepath.Join(repoRoot, "data", "storage", "main.db"),
	}
}

func (s *RuntimeCheckpointStore) FindRetryable(
	runID string,
	sessionID string,
	checkpointID string,
) (RuntimeCheckpointRecord, error) {
	if checkpointID != "" {
		return s.findByID(checkpointID)
	}
	return s.findLatestRetryable(runID, sessionID)
}

func (s *RuntimeCheckpointStore) findByID(checkpointID string) (RuntimeCheckpointRecord, error) {
	return s.queryCheckpoint(
		`select checkpoint_id, run_id, session_id, workspace_id, resumable, resume_reason, request_payload
		from runtime_checkpoints where checkpoint_id = ? limit 1`,
		checkpointID,
	)
}

func (s *RuntimeCheckpointStore) findLatestRetryable(
	runID string,
	sessionID string,
) (RuntimeCheckpointRecord, error) {
	return s.queryCheckpoint(
		`select checkpoint_id, run_id, session_id, workspace_id, resumable, resume_reason, request_payload
		from runtime_checkpoints
		where run_id = ? and session_id = ? and resumable = 1 and resume_reason = 'retryable_failure'
		order by cast(created_at as integer) desc limit 1`,
		runID,
		sessionID,
	)
}

func (s *RuntimeCheckpointStore) queryCheckpoint(
	query string,
	args ...any,
) (RuntimeCheckpointRecord, error) {
	db, err := sql.Open("sqlite", s.path)
	if err != nil {
		return RuntimeCheckpointRecord{}, err
	}
	defer db.Close()
	row := db.QueryRow(query, args...)
	return scanRuntimeCheckpoint(row)
}

func scanRuntimeCheckpoint(row *sql.Row) (RuntimeCheckpointRecord, error) {
	var item RuntimeCheckpointRecord
	var payload string
	if err := row.Scan(
		&item.CheckpointID,
		&item.RunID,
		&item.SessionID,
		&item.WorkspaceID,
		&item.Resumable,
		&item.ResumeReason,
		&payload,
	); err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return RuntimeCheckpointRecord{}, errRuntimeCheckpointNotFound
		}
		return RuntimeCheckpointRecord{}, err
	}
	if err := json.Unmarshal([]byte(payload), &item.Request); err != nil {
		return RuntimeCheckpointRecord{}, err
	}
	return item, nil
}
