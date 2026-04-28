package main

import (
	"context"
	"flag"
	"fmt"
	"os"
	"time"

	"local-agent/gateway/internal/knowledge"
	"local-agent/gateway/internal/providers/bestblogs"
)

func main() {
	root := flag.String("root", ".", "项目根目录")
	workspaceID := flag.String("workspace", "main", "工作区 ID")
	startPage := flag.Int("start-page", 1, "起始页码")
	endPage := flag.Int("end-page", 0, "结束页码（0 = 仅起始页）")
	pageSize := flag.Int("page-size", 20, "每页条数（最大 100）")
	language := flag.String("lang", "zh", "语言")
	fullContent := flag.Bool("full", false, "逐篇抓取正文（慢，每篇一次请求）")
	maxArticles := flag.Int("max", 0, "最大导入条数（0 = 不限制）")
	dryRun := flag.Bool("dry-run", false, "仅列出，不写入知识库")
	flag.Parse()

	if *endPage < *startPage {
		*endPage = *startPage
	}

	client := bestblogs.NewClient(nil)
	store := knowledge.NewStore(*root)

	existing, _ := store.List(*workspaceID)
	existingSources := make(map[string]bool, len(existing))
	for _, it := range existing {
		if it.Source != "" {
			existingSources[it.Source] = true
		}
	}

	var totalImported, totalSkipped, totalErrors int

	for page := *startPage; page <= *endPage; page++ {
		fmt.Printf("\n=== 第 %d 页 ===\n", page)
		ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)

		resp, err := client.ListArticles(ctx, bestblogs.ListArticlesRequest{
			Language: *language,
			Page:     page,
			PageSize: *pageSize,
		})
		cancel()

		if err != nil {
			fmt.Fprintf(os.Stderr, "获取列表失败 (第 %d 页): %v\n", page, err)
			totalErrors++
			continue
		}

		fmt.Printf("本页 %d 条，总计 %d 条，共 %d 页\n", len(resp.Items), resp.TotalCount, resp.PageCount)

		for _, item := range resp.Items {
			if *maxArticles > 0 && totalImported >= *maxArticles {
				fmt.Println("\n已达到最大导入数限制。")
				printSummary(totalImported, totalSkipped, totalErrors)
				return
			}

			sourceURL := item.URL
			if sourceURL == "" {
				sourceURL = item.ReadURL
			}

			if existingSources[sourceURL] {
				totalSkipped++
				fmt.Printf("  [SKIP] 已存在: %s\n", truncate(item.Title, 60))
				continue
			}

			category := item.Category
			if category == "" {
				category = "未分类"
			}
			tags := item.Tags
			if len(tags) == 0 {
				tags = []string{bestblogsLabel(item.Domain)}
			}

			content := item.Summary
			if *fullContent {
				fmt.Printf("  [FETCH] 获取正文: %s\n", truncate(item.Title, 50))
				content = fetchFullContent(item.ReadURL, *language)
				time.Sleep(200 * time.Millisecond) // 温和限速
			}

			req := knowledge.CreateRequest{
				Title:    item.Title,
				Summary:  item.OneSentenceSummary,
				Content:  content,
				Category: category,
				Tags:     tags,
				Source:   sourceURL,
			}

			if *dryRun {
				fmt.Printf("  [DRY-RUN] %s | %s | %s\n", truncate(item.Title, 50), category, sourceURL)
				totalImported++
				continue
			}

			created, err := store.Create(*workspaceID, req)
			if err != nil {
				totalErrors++
				fmt.Fprintf(os.Stderr, "  [ERR] 写入失败: %s - %v\n", truncate(item.Title, 50), err)
				continue
			}

			existingSources[sourceURL] = true
			totalImported++
			fmt.Printf("  [OK %s] %s\n", created.ID, truncate(item.Title, 55))
		}
	}

	printSummary(totalImported, totalSkipped, totalErrors)
}

func printSummary(imported, skipped, errors int) {
	fmt.Printf("\n===== 抓取完成 =====\n")
	fmt.Printf("导入: %d 条\n", imported)
	fmt.Printf("跳过: %d 条\n", skipped)
	fmt.Printf("失败: %d 条\n", errors)
}

func fetchFullContent(readURL string, language string) string {
	client := bestblogs.NewClient(nil)
	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Second)
	defer cancel()

	result, err := client.ReadArticle(ctx, bestblogs.ReadArticleRequest{
		ArticleURL:      readURL,
		Language:        language,
		IncludeMarkdown: true,
	})
	if err != nil {
		return ""
	}
	return result.Content.Markdown
}

func bestblogsLabel(domain string) string {
	if domain != "" {
		return domain
	}
	return "bestblogs"
}

func truncate(s string, maxLen int) string {
	runes := []rune(s)
	if len(runes) > maxLen {
		return string(runes[:maxLen]) + "..."
	}
	return s
}
