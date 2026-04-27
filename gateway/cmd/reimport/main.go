package main

import (
	"database/sql"
	"fmt"
	"os"
	"path/filepath"
	"time"

	_ "modernc.org/sqlite"
	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/knowledge"
)

func main() {
	repoRoot := `D:\newwork\本地智能体`
	if len(os.Args) > 1 {
		repoRoot = os.Args[1]
	}

	cfg, err := config.Load(repoRoot)
	if err == nil && cfg.OCR.Baidu.APIKey != "" {
		knowledge.SetOCRConfig(cfg.OCR.Baidu.APIKey, cfg.OCR.Baidu.SecretKey)
	}

	dbPath := filepath.Join(repoRoot, "data", "storage", "main.db")
	db, err := sql.Open("sqlite", dbPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "open db failed: %v\n", err)
		os.Exit(1)
	}
	defer db.Close()

	rows, err := db.Query(`select id, workspace_id, title, source from knowledge_items`)
	if err != nil {
		fmt.Fprintf(os.Stderr, "query failed: %v\n", err)
		os.Exit(1)
	}
	defer rows.Close()

	type record struct {
		id          string
		workspaceID string
		title       string
		source      string
	}

	var records []record
	for rows.Next() {
		var r record
		if err := rows.Scan(&r.id, &r.workspaceID, &r.title, &r.source); err != nil {
			continue
		}
		records = append(records, r)
	}
	rows.Close()

	fmt.Printf("found %d records\n", len(records))

	var updated, skipped, failed, deleted int
	for _, r := range records {
		if r.source == "" {
			skipped++
			fmt.Printf("SKIP (no source): %s\n", r.title)
			continue
		}

		if _, err := os.Stat(r.source); os.IsNotExist(err) {
			if _, err := db.Exec(`delete from knowledge_items where id = ? and workspace_id = ?`, r.id, r.workspaceID); err != nil {
				fmt.Printf("ERROR (delete): %s -> %v\n", r.source, err)
				failed++
			} else {
				fmt.Printf("DELETE (missing): %s\n", r.source)
				deleted++
			}
			continue
		}

		extracted := knowledge.ExtractText(r.source)
		if extracted.Error != nil {
			fmt.Printf("SKIP (extract error): %s -> %v\n", r.source, extracted.Error)
			skipped++
			continue
		}

		now := time.Now().Format(time.RFC3339)
		_, err := db.Exec(
			`update knowledge_items set title = ?, content = ?, updated_at = ? where id = ? and workspace_id = ?`,
			extracted.Title, extracted.Content, now, r.id, r.workspaceID,
		)
		if err != nil {
			fmt.Printf("ERROR (update): %s -> %v\n", r.source, err)
			failed++
			continue
		}

		updated++
		action := "OK"
		if extracted.Content == "" {
			action = "OK (empty)"
		}
		fmt.Printf("%s: %s -> title=%q content_len=%d\n", action, filepath.Base(r.source), extracted.Title, len(extracted.Content))
	}

	fmt.Printf("\n=== summary ===\n")
	fmt.Printf("updated: %d\n", updated)
	fmt.Printf("deleted (missing source): %d\n", deleted)
	fmt.Printf("skipped: %d\n", skipped)
	fmt.Printf("failed: %d\n", failed)
}
