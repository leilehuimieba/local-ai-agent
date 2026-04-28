package knowledge

import (
	"fmt"
	"strings"
	"time"
)

const defaultChunkSize = 500

var chunkSeparators = []string{"\n\n", "\n", "。", ". ", "；", "，", " "}

func SplitText(text string, chunkSize int) []string {
	if chunkSize <= 0 {
		chunkSize = defaultChunkSize
	}
	text = strings.TrimSpace(text)
	if text == "" {
		return nil
	}
	if len(text) <= chunkSize {
		return []string{text}
	}

	chunks := RecursiveSplit(text, chunkSize)
	return mergeSmallChunks(chunks, chunkSize)
}

func RecursiveSplit(text string, chunkSize int) []string {
	for _, sep := range chunkSeparators {
		if !strings.Contains(text, sep) {
			continue
		}
		parts := strings.Split(text, sep)
		var result []string
		for _, part := range parts {
			part = strings.TrimSpace(part)
			if part == "" {
				continue
			}
			if len(part) <= chunkSize {
				result = append(result, part)
			} else {
				result = append(result, RecursiveSplit(part, chunkSize)...)
			}
		}
		return result
	}
	// 没有分隔符可用，强制按字符数切分
	var result []string
	runes := []rune(text)
	for i := 0; i < len(runes); i += chunkSize {
		end := i + chunkSize
		if end > len(runes) {
			end = len(runes)
		}
		result = append(result, string(runes[i:end]))
	}
	return result
}

func mergeSmallChunks(chunks []string, chunkSize int) []string {
	if len(chunks) <= 1 {
		return chunks
	}
	minSize := chunkSize / 2
	var result []string
	cur := chunks[0]
	for i := 1; i < len(chunks); i++ {
		if len(cur) < minSize {
			cur += "\n" + chunks[i]
		} else {
			result = append(result, cur)
			cur = chunks[i]
		}
	}
	if strings.TrimSpace(cur) != "" {
		result = append(result, cur)
	}
	return result
}

func BuildChunks(itemID string, content string) []Chunk {
	texts := SplitText(content, defaultChunkSize)
	now := time.Now().Format(time.RFC3339)
	chunks := make([]Chunk, 0, len(texts))
	for i, t := range texts {
		chunks = append(chunks, Chunk{
			ID:         fmt.Sprintf("chk_%s_%d", itemID, i),
			ItemID:     itemID,
			ChunkIndex: i,
			Content:    t,
			CreatedAt:  now,
		})
	}
	return chunks
}
