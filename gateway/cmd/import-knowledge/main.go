package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"local-agent/gateway/internal/knowledge"
)

func main() {
	root := `D:\newwork\本地智能体`
	workspaceID := "main"
	sourceDir := `C:\Users\33371\Desktop\【最全】英语学习资料（2025.5.21更新）`

	store := knowledge.NewStore(root)

	existingItems, _ := store.List(workspaceID)
	existingSources := make(map[string]bool, len(existingItems))
	for _, it := range existingItems {
		existingSources[it.Source] = true
	}

	var count, skipped int
	_ = filepath.Walk(sourceDir, func(path string, info os.FileInfo, err error) error {
		if err != nil || info.IsDir() {
			return nil
		}
		ext := strings.ToLower(filepath.Ext(path))
		if ext != ".pdf" {
			return nil
		}

		title := strings.TrimSuffix(info.Name(), ext)
		var content, summary string

		func() {
			defer func() {
				if r := recover(); r != nil {
					fmt.Printf("解析失败 %s: panic %v\n", path, r)
				}
			}()
			extracted := knowledge.ExtractText(path)
			if extracted.Error == nil && len(extracted.Content) > 100 && isReadable(extracted.Content) {
				content = extracted.Content
				summary = extracted.Content
				if len(summary) > 200 {
					summary = summary[:200] + "..."
				}
				if extracted.Title != "" && len(extracted.Title) < 200 {
					title = extracted.Title
				}
			}
		}()

		if content == "" {
			content = "文件：" + info.Name() + "\n路径：" + path
			summary = content
			if len(summary) > 200 {
				summary = summary[:200] + "..."
			}
		}

		category := classify(path, sourceDir)
		tags := tagsFromPath(path, sourceDir)

		if existingSources[path] {
			skipped++
			return nil
		}

		_, err = store.Create(workspaceID, knowledge.CreateRequest{
			Title:    title,
			Summary:  summary,
			Content:  content,
			Category: category,
			Tags:     tags,
			Source:   path,
		})
		if err != nil {
			fmt.Printf("插入失败 %s: %v\n", path, err)
			skipped++
			return nil
		}

		count++
		fmt.Printf("[%d] 已导入: %s\n", count, title)
		return nil
	})

	fmt.Printf("\n导入完成，成功 %d 条，跳过 %d 条\n", count, skipped)
}

func isReadable(s string) bool {
	if len(s) == 0 {
		return false
	}
	var printable int
	for _, r := range s {
		if r >= 0x20 && r < 0x7F || r >= 0x4E00 && r <= 0x9FFF {
			printable++
		}
	}
	return float64(printable)/float64(len([]rune(s))) > 0.5
}

func classify(fullPath, baseDir string) string {
	rel, _ := filepath.Rel(baseDir, fullPath)
	parts := strings.Split(rel, string(filepath.Separator))
	if len(parts) == 0 {
		return "英语学习"
	}
	top := parts[0]
	switch {
	case strings.Contains(top, "TED"):
		return "TED演讲"
	case strings.Contains(top, "四六级") || strings.Contains(top, "六级") || strings.Contains(top, "四级"):
		return "四六级"
	case strings.Contains(top, "单词") || strings.Contains(top, "词汇"):
		return "词汇"
	default:
		return "英语学习"
	}
}

func tagsFromPath(fullPath, baseDir string) []string {
	rel, _ := filepath.Rel(baseDir, fullPath)
	parts := strings.Split(rel, string(filepath.Separator))
	var tags []string
	for _, p := range parts {
		if p == "" || strings.HasSuffix(p, ".pdf") {
			continue
		}
		tags = append(tags, p)
		if len(tags) >= 3 {
			break
		}
	}
	if len(tags) == 0 {
		return []string{"英语学习"}
	}
	return tags
}
