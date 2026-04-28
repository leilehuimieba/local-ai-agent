package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"strings"

	"local-agent/gateway/internal/knowledge"
)

func main() {
	root := `D:\newwork\本地智能体`
	workspaceID := "main"
	jsonlPath := `D:\tmp\bestblogs_data\bestblogs_articles.jsonl`

	store := knowledge.NewStore(root)

	// 获取已有条目，按 source 去重
	existingItems, _ := store.List(workspaceID)
	existingSources := make(map[string]bool, len(existingItems))
	for _, it := range existingItems {
		existingSources[it.Source] = true
	}

	f, err := os.Open(jsonlPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "无法打开文件: %v\n", err)
		os.Exit(1)
	}
	defer f.Close()

	type articleInput struct {
		Title    string   `json:"title"`
		Content  string   `json:"content"`
		Category string   `json:"category"`
		Tags     []string `json:"tags"`
		Source   string   `json:"source"`
	}

	var count, skipped int
	scanner := bufio.NewScanner(f)
	scanner.Buffer(make([]byte, 1<<20), 10<<20)

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}

		var article articleInput
		if err := json.Unmarshal([]byte(line), &article); err != nil {
			fmt.Fprintf(os.Stderr, "解析失败: %v\n", err)
			continue
		}

		if existingSources[article.Source] {
			skipped++
			fmt.Printf("[SKIP] 已存在: %s\n", article.Title)
			continue
		}

		summary := article.Content
		runes := []rune(summary)
		if len(runes) > 200 {
			summary = string(runes[:200]) + "..."
		}

		_, err := store.Create(workspaceID, knowledge.CreateRequest{
			Title:    article.Title,
			Summary:  summary,
			Content:  article.Content,
			Category: article.Category,
			Tags:     article.Tags,
			Source:   article.Source,
		})
		if err != nil {
			fmt.Fprintf(os.Stderr, "插入失败 %s: %v\n", article.Title, err)
			skipped++
			continue
		}

		count++
		fmt.Printf("[%d] OK: %s\n", count, article.Title)
	}

	if err := scanner.Err(); err != nil {
		fmt.Fprintf(os.Stderr, "读取文件出错: %v\n", err)
	}

	fmt.Printf("\n导入完成！成功 %d 条，跳过 %d 条\n", count, skipped)
}
