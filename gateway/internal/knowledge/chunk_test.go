package knowledge

import (
	"strings"
	"testing"
)

func TestSplitText_Short(t *testing.T) {
	result := SplitText("短文本", 500)
	if len(result) != 1 {
		t.Fatalf("short text should produce 1 chunk, got %d", len(result))
	}
	if result[0] != "短文本" {
		t.Fatalf("got %q", result[0])
	}
}

func TestSplitText_Paragraphs(t *testing.T) {
	longPara := strings.Repeat("这是一段比较长的测试内容，用于验证段落分块功能。", 20)
	paragraphs := strings.Repeat(longPara+"\n\n", 10)
	result := SplitText(paragraphs, 500)
	if len(result) < 5 {
		t.Fatalf("expected at least 5 chunks, got %d", len(result))
	}
	for _, c := range result {
		if len(c) > 600 {
			t.Fatalf("chunk too large: len=%d", len(c))
		}
	}
}

func TestSplitText_ChineseParagraphs(t *testing.T) {
	// 用句号分句的长中文文本
	sentences := strings.Repeat("这是一段中文测试内容，用于验证分块功能是否正常工作。", 30)
	result := SplitText(sentences, 500)
	if len(result) < 3 {
		t.Fatalf("expected at least 3 chunks, got %d", len(result))
	}
	for _, c := range result {
		if len(c) > 600 {
			t.Fatalf("chunk too large: len=%d", len(c))
		}
	}
}

func TestSplitText_Empty(t *testing.T) {
	result := SplitText("", 500)
	if len(result) != 0 {
		t.Fatalf("empty text should produce 0 chunks, got %d", len(result))
	}
}

func TestSplitText_MergeSmall(t *testing.T) {
	// 几个短段落应该被合并
	text := "第一段。\n\n第二段。\n\n第三段。"
	result := SplitText(text, 500)
	if len(result) != 1 {
		t.Fatalf("small paragraphs should be merged, got %d chunks: %v", len(result), result)
	}
}

func TestBuildChunks(t *testing.T) {
	content := strings.Repeat("测试内容。", 100)
	chunks := BuildChunks("item1", content)
	if len(chunks) < 2 {
		t.Fatalf("expected multiple chunks, got %d", len(chunks))
	}
	for i, c := range chunks {
		if c.ItemID != "item1" {
			t.Fatalf("chunk %d has wrong ItemID: %s", i, c.ItemID)
		}
		if c.ChunkIndex != i {
			t.Fatalf("chunk %d has wrong ChunkIndex: %d", i, c.ChunkIndex)
		}
		if c.ID == "" {
			t.Fatalf("chunk %d has empty ID", i)
		}
		if c.Content == "" {
			t.Fatalf("chunk %d has empty Content", i)
		}
	}
}

func TestSplitText_RRFMerge(t *testing.T) {
	vecRanks := map[string]int{"a": 1, "b": 2, "c": 3}
	kwRanks := map[string]int{"a": 3, "b": 1, "c": 2}

	merged := make(map[string]float64)
	for id, rank := range vecRanks {
		merged[id] = 1.0 / float64(60+rank)
	}
	for id, rank := range kwRanks {
		merged[id] += 1.0 / float64(60+rank)
	}

	// b 在关键词中排第1，向量中排第2，综合应最好
	best := ""
	bestScore := 0.0
	for id, score := range merged {
		if score > bestScore {
			bestScore = score
			best = id
		}
	}
	if best != "b" {
		t.Fatalf("RRF should rank 'b' highest, got %s (scores: %v)", best, merged)
	}
}
