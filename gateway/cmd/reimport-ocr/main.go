package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/knowledge"
)

func main() {
	repoRoot := `D:\newwork\本地智能体`

	cfg, err := config.Load(repoRoot)
	if err == nil && cfg.OCR.Baidu.APIKey != "" {
		knowledge.SetOCRConfig(cfg.OCR.Baidu.APIKey, cfg.OCR.Baidu.SecretKey)
	}
	if knowledge.FindTesseract() != "" {
		fmt.Println("Tesseract OCR available")
	} else if cfg.OCR.Baidu.APIKey != "" {
		fmt.Println("Baidu OCR enabled")
	} else {
		fmt.Println("OCR not available")
	}

	limit := 30
	if len(os.Args) > 1 {
		if n, err := strconv.Atoi(os.Args[1]); err == nil && n > 0 {
			limit = n
		}
	}
	fmt.Printf("limit=%d\n", limit)

	dbPath := filepath.Join(repoRoot, "data", "storage", "main.db")
	query := fmt.Sprintf(`SELECT id, workspace_id, source FROM knowledge_items WHERE LENGTH(content) = 0 ORDER BY source LIMIT %d;`, limit)
	cmd := exec.Command("sqlite3", dbPath, query)
	out, err := cmd.Output()
	if err != nil {
		fmt.Fprintf(os.Stderr, "query failed: %v\n", err)
		os.Exit(1)
	}

	var updated, skipped, failed int
	for _, line := range strings.Split(string(out), "\n") {
		parts := strings.Split(line, "|")
		if len(parts) < 3 {
			continue
		}
		id := strings.TrimSpace(parts[0])
		ws := strings.TrimSpace(parts[1])
		source := strings.TrimSpace(parts[2])

		start := time.Now()
		extracted := knowledge.ExtractText(source)
		elapsed := time.Since(start)
		if extracted.Error != nil {
			fmt.Printf("SKIP (%v): %s -> %v\n", elapsed, filepath.Base(source), extracted.Error)
			skipped++
			continue
		}

		updateCmd := exec.Command("sqlite3", dbPath)
		updateCmd.Stdin = strings.NewReader(fmt.Sprintf(
			"UPDATE knowledge_items SET title = %s, content = %s, updated_at = datetime('now') WHERE id = %s AND workspace_id = %s;\n",
			escapeSQL(extracted.Title), escapeSQL(extracted.Content), escapeSQL(id), escapeSQL(ws),
		))
		if _, err := updateCmd.Output(); err != nil {
			fmt.Printf("ERROR: %s -> %v\n", filepath.Base(source), err)
			failed++
			continue
		}

		updated++
		action := "OK"
		if extracted.Content == "" {
			action = "OK (empty)"
		}
		fmt.Printf("%s (%v): %s -> title=%q content_len=%d\n", action, elapsed, filepath.Base(source), extracted.Title, len(extracted.Content))
	}
	fmt.Printf("\nupdated=%d skipped=%d failed=%d\n", updated, skipped, failed)
}

func escapeSQL(s string) string {
	return "'" + strings.ReplaceAll(s, "'", "''") + "'"
}
