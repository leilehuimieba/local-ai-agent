package knowledge

import (
	"archive/zip"
	"encoding/xml"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"unicode/utf8"

	"rsc.io/pdf"
)

type ExtractResult struct {
	Title   string
	Content string
	Error   error
}

func ExtractText(path string) ExtractResult {
	ext := strings.ToLower(filepath.Ext(path))
	var res ExtractResult
	switch ext {
	case ".txt", ".md", ".markdown":
		res = extractTxt(path)
	case ".pdf":
		res = extractPdf(path)
	case ".docx":
		res = extractDocx(path)
	default:
		return ExtractResult{Error: fmt.Errorf("不支持的文件类型: %s", ext)}
	}
	if res.Error != nil {
		return res
	}
	if isMostlyGarbled(res.Content) {
		res.Content = ""
	}
	if isMostlyGarbled(res.Title) {
		res.Title = filepath.Base(path)
	}
	return res
}

func isMostlyGarbled(s string) bool {
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

func extractTxt(path string) ExtractResult {
	data, err := os.ReadFile(path)
	if err != nil {
		return ExtractResult{Error: err}
	}
	content := string(data)
	lines := strings.Split(content, "\n")
	title := filepath.Base(path)
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if trimmed != "" {
			title = trimmed
			break
		}
	}
	return ExtractResult{Title: title, Content: content}
}

func extractPdf(path string) (res ExtractResult) {
	defer func() {
		if r := recover(); r != nil {
			res = ExtractResult{Error: fmt.Errorf("PDF解析panic: %v", r)}
		}
	}()

	file, err := pdf.Open(path)
	if err != nil {
		return ExtractResult{Error: err}
	}

	var content strings.Builder
	for i := 1; i <= file.NumPage(); i++ {
		page := file.Page(i)
		if page.V.IsNull() {
			continue
		}
		text := page.Content().Text
		for _, t := range text {
			content.WriteString(t.S)
		}
		content.WriteString("\n")
	}

	result := content.String()
	title := filepath.Base(path)
	lines := strings.Split(result, "\n")
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if trimmed != "" {
			title = trimmed
			break
		}
	}
	return ExtractResult{Title: title, Content: result}
}

func extractDocx(path string) ExtractResult {
	zr, err := zip.OpenReader(path)
	if err != nil {
		return ExtractResult{Error: err}
	}
	defer zr.Close()

	var docFile *zip.File
	for _, f := range zr.File {
		if f.Name == "word/document.xml" {
			docFile = f
			break
		}
	}
	if docFile == nil {
		return ExtractResult{Error: fmt.Errorf("无效的 DOCX 文件: 未找到 word/document.xml")}
	}

	rc, err := docFile.Open()
	if err != nil {
		return ExtractResult{Error: err}
	}
	defer rc.Close()

	var content strings.Builder
	decoder := xml.NewDecoder(rc)
	var inText bool
	for {
		tok, err := decoder.Token()
		if err == io.EOF {
			break
		}
		if err != nil {
			return ExtractResult{Error: err}
		}
		switch se := tok.(type) {
		case xml.StartElement:
			switch se.Name.Local {
			case "t":
				inText = true
			case "tab":
				content.WriteByte('\t')
			case "br":
				content.WriteByte('\n')
			}
		case xml.EndElement:
			if se.Name.Local == "t" {
				inText = false
			}
		case xml.CharData:
			if inText {
				content.Write(se)
			}
		}
	}

	result := content.String()
	title := filepath.Base(path)
	lines := strings.Split(result, "\n")
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if trimmed != "" {
			title = trimmed
			break
		}
	}
	return ExtractResult{Title: title, Content: result}
}

func SaveUploadedFile(src io.Reader, dstPath string) error {
	if err := os.MkdirAll(filepath.Dir(dstPath), 0o755); err != nil {
		return err
	}
	out, err := os.Create(dstPath)
	if err != nil {
		return err
	}
	defer out.Close()
	_, err = io.Copy(out, src)
	return err
}
