package knowledge

import (
	"archive/zip"
	"bytes"
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestExtractText_docx(t *testing.T) {
	tmp := t.TempDir()
	path := filepath.Join(tmp, "test.docx")
	require.NoError(t, writeMinimalDocx(path, "Hello World\n第二行内容"))

	res := ExtractText(path)
	require.NoError(t, res.Error)
	assert.Equal(t, "Hello World", res.Title)
	assert.Contains(t, res.Content, "Hello World")
	assert.Contains(t, res.Content, "第二行内容")
}

func TestIsMostlyGarbled(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected bool
	}{
		{"empty", "", false},
		{"normal chinese", "这是正常的中文内容", false},
		{"normal mixed", "Hello World 你好世界", false},
		{"mostly garbled", "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09", true},
		{"pdf garbage", "9!B \u0004��\u0077\u0077-Q��\u0001�?�&�A�F\u00009!�#", true},
		{"partial garbled", "你好\x00\x00\x00\x00\x00\x00\x00\x00", true},
		{"control chars ok", "line1\nline2\tline3", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, tt.expected, isMostlyGarbled(tt.input))
		})
	}
}

func writeMinimalDocx(path string, text string) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	defer f.Close()
	zw := zip.NewWriter(f)
	defer zw.Close()

	w, err := zw.Create("word/document.xml")
	if err != nil {
		return err
	}
	lines := bytes.Split([]byte(text), []byte("\n"))
	_, _ = w.Write([]byte(`<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>`))
	for i, line := range lines {
		if i > 0 {
			_, _ = w.Write([]byte("<w:p><w:r><w:br/></w:r></w:p>"))
		}
		_, _ = w.Write([]byte("<w:p><w:r><w:t>"))
		_, _ = w.Write(line)
		_, _ = w.Write([]byte("</w:t></w:r></w:p>"))
	}
	_, _ = w.Write([]byte("</w:body></w:document>"))
	return zw.Close()
}
