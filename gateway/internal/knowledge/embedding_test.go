package knowledge

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCosineSimilarity(t *testing.T) {
	a := []float32{1, 0, 0}
	b := []float32{1, 0, 0}
	assert.InDelta(t, 1.0, CosineSimilarity(a, b), 1e-6)

	a = []float32{1, 0, 0}
	b = []float32{0, 1, 0}
	assert.InDelta(t, 0.0, CosineSimilarity(a, b), 1e-6)

	a = []float32{1, 1, 0}
	b = []float32{1, 1, 0}
	assert.InDelta(t, 1.0, CosineSimilarity(a, b), 1e-6)

	assert.InDelta(t, 0.0, CosineSimilarity([]float32{}, []float32{}), 1e-6)
	assert.InDelta(t, 0.0, CosineSimilarity(nil, []float32{1}), 1e-6)
}

func TestRankItemsKeyword(t *testing.T) {
	items := []Item{
		{ID: "1", Title: "Go 语言编程", Content: "Go 是一种静态类型语言"},
		{ID: "2", Title: "Python 教程", Content: "Python 是动态类型语言"},
		{ID: "3", Title: "Rust 入门", Content: "Rust 注重内存安全"},
	}
	result := rankItemsByKeyword(items, "Rust")
	assert.Len(t, result, 1)
	assert.Equal(t, "3", result[0].ID)
}
