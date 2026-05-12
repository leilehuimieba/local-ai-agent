package session

import (
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"time"

	_ "modernc.org/sqlite"
)

type Session struct {
	ID        string `json:"id"`
	Title     string `json:"title"`
	CreatedAt string `json:"created_at"`
	UpdatedAt string `json:"updated_at"`
}

type ChatMessage struct {
	ID         string `json:"id"`
	SessionID  string `json:"session_id"`
	Role       string `json:"role"`
	Content    string `json:"content"`
	BlocksJSON string `json:"-"`
	Timestamp  string `json:"timestamp"`
}

type MessageBlock struct {
	Type     string     `json:"type"`
	Content  string     `json:"content,omitempty"`
	Language string     `json:"language,omitempty"`
	Headers  []string   `json:"headers,omitempty"`
	Rows     [][]string `json:"rows,omitempty"`
	Items    []string   `json:"items,omitempty"`
}

type SessionStore struct {
	storageRoot string
}

func NewSessionStore(repoRoot string) *SessionStore {
	return &SessionStore{storageRoot: filepath.Join(repoRoot, "data")}
}

func (s *SessionStore) openDB() (*sql.DB, error) {
	dbPath := filepath.Join(s.storageRoot, "storage", "main.db")
	if err := os.MkdirAll(filepath.Dir(dbPath), 0o755); err != nil {
		return nil, err
	}
	db, err := sql.Open("sqlite", dbPath)
	if err != nil {
		return nil, err
	}
	return db, ensureSessionTables(db)
}

func (s *SessionStore) ListSessions() ([]Session, error) {
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer func() { _ = db.Close() }()

	rows, err := db.Query(`SELECT id, title, created_at, updated_at FROM sessions ORDER BY updated_at DESC`)
	if err != nil {
		if isMissingTable(err) {
			return []Session{}, nil
		}
		return nil, err
	}
	defer func() { _ = rows.Close() }()

	sessions := make([]Session, 0)
	for rows.Next() {
		var s Session
		if err := rows.Scan(&s.ID, &s.Title, &s.CreatedAt, &s.UpdatedAt); err != nil {
			continue
		}
		sessions = append(sessions, s)
	}
	return sessions, nil
}

func (s *SessionStore) GetSession(id string) (*Session, error) {
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer func() { _ = db.Close() }()

	row := db.QueryRow(`SELECT id, title, created_at, updated_at FROM sessions WHERE id = ?`, id)
	var sess Session
	if err := row.Scan(&sess.ID, &sess.Title, &sess.CreatedAt, &sess.UpdatedAt); err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, errors.New("session not found")
		}
		return nil, err
	}
	return &sess, nil
}

func (s *SessionStore) CreateSession(id string, title string) (*Session, error) {
	now := time.Now().Format(time.RFC3339)
	sess := Session{ID: id, Title: title, CreatedAt: now, UpdatedAt: now}

	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer func() { _ = db.Close() }()

	_, err = db.Exec(
		`INSERT INTO sessions (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)`,
		sess.ID, sess.Title, sess.CreatedAt, sess.UpdatedAt,
	)
	if err != nil {
		return nil, err
	}
	return &sess, nil
}

func (s *SessionStore) TouchSession(id string) error {
	db, err := s.openDB()
	if err != nil {
		return err
	}
	defer func() { _ = db.Close() }()

	now := time.Now().Format(time.RFC3339)
	_, err = db.Exec(`UPDATE sessions SET updated_at = ? WHERE id = ?`, now, id)
	return err
}

func (s *SessionStore) GetMessages(sessionID string) ([]ChatMessage, error) {
	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer func() { _ = db.Close() }()

	rows, err := db.Query(
		`SELECT id, session_id, role, content, blocks_json, timestamp FROM chat_messages WHERE session_id = ? ORDER BY timestamp ASC`,
		sessionID,
	)
	if err != nil {
		if isMissingTable(err) {
			return []ChatMessage{}, nil
		}
		return nil, err
	}
	defer func() { _ = rows.Close() }()

	var messages []ChatMessage
	for rows.Next() {
		var m ChatMessage
		var blocksJSON sql.NullString
		if err := rows.Scan(&m.ID, &m.SessionID, &m.Role, &m.Content, &blocksJSON, &m.Timestamp); err != nil {
			continue
		}
		m.BlocksJSON = blocksJSON.String
		messages = append(messages, m)
	}
	return messages, nil
}

func (s *SessionStore) AddMessage(sessionID string, role string, content string, blocks []MessageBlock, timestamp string) (*ChatMessage, error) {
	if timestamp == "" {
		timestamp = time.Now().Format(time.RFC3339)
	}
	id := fmt.Sprintf("msg_%d_%s", time.Now().UnixNano(), randomSuffix(6))

	var blocksJSON string
	if len(blocks) > 0 {
		b, _ := json.Marshal(blocks)
		blocksJSON = string(b)
	}

	db, err := s.openDB()
	if err != nil {
		return nil, err
	}
	defer func() { _ = db.Close() }()

	// Ensure session exists
	var count int
	_ = db.QueryRow(`SELECT COUNT(*) FROM sessions WHERE id = ?`, sessionID).Scan(&count)
	if count == 0 {
		title := content
		if len(title) > 50 {
			title = title[:50] + "..."
		}
		now := time.Now().Format(time.RFC3339)
		_, _ = db.Exec(`INSERT INTO sessions (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)`, sessionID, title, now, now)
	}

	_, err = db.Exec(
		`INSERT INTO chat_messages (id, session_id, role, content, blocks_json, timestamp) VALUES (?, ?, ?, ?, ?, ?)`,
		id, sessionID, role, content, blocksJSON, timestamp,
	)
	if err != nil {
		return nil, err
	}

	// Update session timestamp
	now := time.Now().Format(time.RFC3339)
	_, _ = db.Exec(`UPDATE sessions SET updated_at = ? WHERE id = ?`, now, sessionID)

	return &ChatMessage{
		ID:         id,
		SessionID:  sessionID,
		Role:       role,
		Content:    content,
		BlocksJSON: blocksJSON,
		Timestamp:  timestamp,
	}, nil
}

func (s *SessionStore) DeleteSession(id string) error {
	db, err := s.openDB()
	if err != nil {
		return err
	}
	defer func() { _ = db.Close() }()

	_, err = db.Exec(`DELETE FROM chat_messages WHERE session_id = ?`, id)
	if err != nil {
		return err
	}
	_, err = db.Exec(`DELETE FROM sessions WHERE id = ?`, id)
	return err
}

func ensureSessionTables(db *sql.DB) error {
	for _, stmt := range createSessionTableSQLs {
		if _, err := db.Exec(stmt); err != nil {
			return err
		}
	}
	return nil
}

func isMissingTable(err error) bool {
	return err != nil && (errors.Is(err, sql.ErrNoRows) ||
		(errors.Is(err, sql.ErrConnDone)) ||
		(err.Error() == "no such table: sessions") ||
		(err.Error() == "no such table: chat_messages"))
}

func randomSuffix(n int) string {
	const chars = "abcdefghijklmnopqrstuvwxyz0123456789"
	b := make([]byte, n)
	for i := range b {
		b[i] = chars[time.Now().UnixNano()%int64(len(chars))]
	}
	return string(b)
}

var createSessionTableSQLs = []string{
	`create table if not exists sessions (
		id text primary key,
		title text not null default '',
		created_at text not null,
		updated_at text not null
	)`,
	`create index if not exists idx_sessions_updated on sessions(updated_at)`,
	`create table if not exists chat_messages (
		id text primary key,
		session_id text not null,
		role text not null,
		content text not null default '',
		blocks_json text,
		timestamp text not null
	)`,
	`create index if not exists idx_messages_session on chat_messages(session_id)`,
	`create index if not exists idx_messages_timestamp on chat_messages(timestamp)`,
}
