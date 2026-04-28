package main

import (
	"fmt"
	"os"
	"strings"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/knowledge"
)

func main() {
	root := `D:\newwork\本地智能体`
	workspaceID := "main"
	sourcePrefix := os.Getenv("REINDEX_SOURCE_PREFIX")

	cfg, err := config.Load(root)
	if err != nil {
		fmt.Fprintf(os.Stderr, "加载配置失败: %v\n", err)
		os.Exit(1)
	}

	provider := knowledge.FindProvider(cfg, cfg.Embedding.ProviderID)
	if provider.ProviderID == "" {
		fmt.Fprintf(os.Stderr, "Embedding provider 未配置: %s\n", cfg.Embedding.ProviderID)
		os.Exit(1)
	}
	if provider.EmbeddingModel == "" {
		fmt.Fprintf(os.Stderr, "Embedding model 未配置\n")
		os.Exit(1)
	}

	fmt.Printf("Embedding provider: %s, model: %s\n", provider.ProviderID, provider.EmbeddingModel)
	if sourcePrefix != "" {
		fmt.Printf("过滤 source 前缀: %s\n", sourcePrefix)
	}

	store := knowledge.NewStore(root)

	items, err := store.List(workspaceID)
	if err != nil {
		fmt.Fprintf(os.Stderr, "列知识条目失败: %v\n", err)
		os.Exit(1)
	}

	var total, skipped, failed int
	for _, item := range items {
		if item.Content == "" {
			skipped++
			continue
		}
		if sourcePrefix != "" && !strings.HasPrefix(item.Source, sourcePrefix) {
			skipped++
			continue
		}

		// 删除旧 chunks 并重建
		_ = store.DeleteChunksByItemID(item.ID)

		chunks := knowledge.BuildChunks(item.ID, item.Content)
		if len(chunks) == 0 {
			skipped++
			continue
		}

		if err := store.CreateChunks(chunks); err != nil {
			fmt.Fprintf(os.Stderr, "[FAIL] %s 创建chunks失败: %v\n", item.Title, err)
			failed++
			continue
		}

		ok := 0
		for i := range chunks {
			text := item.Title + "\n" + item.Summary + "\n" + chunks[i].Content
			if len(text) > 6000 {
				text = text[:6000]
			}
			embed, err := knowledge.GetEmbedding(text, provider, provider.EmbeddingModel)
			if err != nil {
				if i < 3 {
					fmt.Fprintf(os.Stderr, "  chunk %d embedding失败: %v\n", i, err)
				}
				continue
			}
			if err := store.UpdateChunkEmbedding(chunks[i].ID, embed); err != nil {
				continue
			}
			ok++
			time.Sleep(20 * time.Millisecond)
		}

		total++
		fmt.Printf("[%d] %s (%d/%d chunks embedded)\n", total, item.Title, ok, len(chunks))
	}

	fmt.Printf("\n完成！成功 %d 条，跳过 %d 条，失败 %d 条\n", total, skipped, failed)
}
