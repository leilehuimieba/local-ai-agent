package main

import (
	"database/sql"
	"fmt"
	"os"
	"path/filepath"
	"unicode/utf8"

	_ "modernc.org/sqlite"
)

func main() {
	repoRoot := `D:\newwork\本地智能体`
	dbPath := filepath.Join(repoRoot, "data", "storage", "main.db")
	db, err := sql.Open("sqlite", dbPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "open db failed: %v\n", err)
		os.Exit(1)
	}
	defer db.Close()

	rows, err := db.Query(`select id, workspace_id, title, content, source from knowledge_items`)
	if err != nil {
		fmt.Fprintf(os.Stderr, "query failed: %v\n", err)
		os.Exit(1)
	}
	defer rows.Close()

	type record struct {
		id, workspaceID, title, content, source string
	}
	var records []record
	for rows.Next() {
		var r record
		rows.Scan(&r.id, &r.workspaceID, &r.title, &r.content, &r.source)
		records = append(records, r)
	}
	rows.Close()

	var cleaned int
	for _, r := range records {
		needsUpdate := false
		newTitle := r.title
		newContent := r.content

		if isGarbled(r.title) {
			needsUpdate = true
			newTitle = filepath.Base(r.source)
			if newTitle == "" {
				newTitle = r.title
			}
		}
		if isGarbled(r.content) {
			needsUpdate = true
			newContent = ""
		}

		if !needsUpdate {
			continue
		}

		_, err := db.Exec(
			`update knowledge_items set title = ?, content = ? where id = ? and workspace_id = ?`,
			newTitle, newContent, r.id, r.workspaceID,
		)
		if err != nil {
			fmt.Printf("ERROR: %s -> %v\n", r.source, err)
			continue
		}
		cleaned++
		fmt.Printf("CLEAN: %s -> title=%q content_len=%d\n", filepath.Base(r.source), newTitle, len(newContent))
	}

	fmt.Printf("\n=== cleaned %d records ===\n", cleaned)
}

func isGarbled(s string) bool {
	if s == "" {
		return false
	}
	runes := []rune(s)
	valid := 0
	for _, r := range runes {
		if r == utf8.RuneError {
			continue
		}
		if r < 32 && r != '\n' && r != '\t' && r != '\r' {
			continue
		}
		valid++
	}
	return float64(valid)/float64(len(runes)) < 0.7
}
