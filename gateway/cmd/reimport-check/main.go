package main

import (
	"database/sql"
	"fmt"
	"path/filepath"
	"unicode/utf8"

	_ "modernc.org/sqlite"
)

func main() {
	db, _ := sql.Open("sqlite", filepath.Join(`D:\newwork\本地智能体`, "data", "storage", "main.db"))
	defer db.Close()
	var total, empty, garbled int
	rows, _ := db.Query(`select title, content from knowledge_items`)
	defer rows.Close()
	for rows.Next() {
		var t, c string
		rows.Scan(&t, &c)
		total++
		if c == "" {
			empty++
		}
		if isGarbled(t) || isGarbled(c) {
			garbled++
		}
	}
	fmt.Printf("total=%d empty=%d garbled=%d\n", total, empty, garbled)
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
