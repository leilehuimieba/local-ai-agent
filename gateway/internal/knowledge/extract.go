package knowledge

import (
	"archive/zip"
	"encoding/xml"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"unicode"
	"unicode/utf8"

	"rsc.io/pdf"
)

type ExtractResult struct {
	Title   string
	Content string
	Error   error
}

var ocrAPIKey, ocrSecretKey string

func SetOCRConfig(apiKey, secretKey string) {
	ocrAPIKey = apiKey
	ocrSecretKey = secretKey
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
	res.Content = strings.ReplaceAll(res.Content, "\x00", "")
	res.Content = cleanWatermark(res.Content)
	res.Title = pickTitle(res.Title, res.Content, path)
	return res
}

var watermarkKeywords = []string{
	"淘宝店铺：", "淘宝：", "掌柜旺旺：", "认准淘宝店铺：",
	"叮当考研", "谈辰图书", "光速考研工作室", "学海无涯教育",
}

func cleanWatermark(content string) string {
	lines := strings.Split(content, "\n")
	var out []string
	lastEmpty := false
	for _, line := range lines {
		if hasWatermarkLine(line) {
			lastEmpty = true
			continue
		}
		if strings.TrimSpace(line) == "" {
			if !lastEmpty {
				out = append(out, "")
				lastEmpty = true
			}
			continue
		}
		out = append(out, line)
		lastEmpty = false
	}
	return strings.TrimSpace(strings.Join(out, "\n"))
}

func hasWatermarkLine(line string) bool {
	for _, kw := range watermarkKeywords {
		if strings.Contains(line, kw) {
			return true
		}
	}
	return false
}

func pickTitle(title, content, path string) string {
	candidates := []string{title}
	for _, line := range strings.Split(content, "\n") {
		s := strings.TrimSpace(line)
		if s != "" {
			candidates = append(candidates, s)
			if len(candidates) >= 5 {
				break
			}
		}
	}
	for _, c := range candidates {
		truncated := c
		if len(truncated) > 200 {
			truncated = truncated[:200]
		}
		if !isMostlyGarbled(truncated) && !strings.ContainsRune(truncated, utf8.RuneError) && !containsControlChars(truncated) {
			return truncated
		}
	}
	return filepath.Base(path)
}

func containsControlChars(s string) bool {
	for _, r := range s {
		if r < 32 && r != '\n' && r != '\t' && r != '\r' {
			return true
		}
	}
	return false
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
		if r >= 0x80 && r <= 0x9F {
			continue
		}
		if r == ' ' {
			continue
		}
		if unicode.Is(unicode.Cf, r) {
			continue
		}
		valid++
	}
	return float64(valid)/float64(len(runes)) < 0.3
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

	// 优先使用 pdftotext（对中文支持更好），不可用时回退到 rsc.io/pdf
	if findPdftotext() != "" {
		res = fallbackPdftotext(path, nil)
		if res.Error == nil && strings.TrimSpace(res.Content) != "" && !isMostlyGarbled(res.Content) && !strings.Contains(res.Content, "\x00") {
			// 如果控制字符比例过高，说明字体映射有问题，尝试 OCR
			runes := []rune(res.Content)
			ctrl := 0
			for _, r := range runes {
				if r < 32 && r != '\n' && r != '\t' && r != '\r' {
					ctrl++
				}
			}
			if float64(ctrl)/float64(len(runes)) < 0.15 {
				cleaned := cleanWatermark(res.Content)
				if len(strings.TrimSpace(cleaned)) >= 100 {
					res.Content = cleaned
					return res
				}
				// 清理后内容太短（可能只有水印），继续尝试 OCR
			}
		}
		// pdftotext 提取为空或乱码，尝试 OCR
		if ocrAPIKey != "" && ocrSecretKey != "" {
			ocrRes := extractPdfWithOCR(path, ocrAPIKey, ocrSecretKey)
			if ocrRes.Error == nil {
				return ocrRes
			}
			// OCR 失败时标记错误，让上层知道原因
			res.Error = ocrRes.Error
		}
		return res
	}

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
	if isMostlyGarbled(result) || strings.TrimSpace(result) == "" {
		return ExtractResult{Error: fmt.Errorf("PDF内容为空或乱码")}
	}

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

func findPdftotext() string {
	if runtime.GOOS == "windows" {
		candidates := []string{
			`third_party\poppler\poppler-24.08.0\Library\bin\pdftotext.exe`,
			`..\third_party\poppler\poppler-24.08.0\Library\bin\pdftotext.exe`,
		}
		if exe, err := os.Executable(); err == nil {
			exeDir := filepath.Dir(exe)
			candidates = append(candidates,
				filepath.Join(exeDir, `third_party\poppler\poppler-24.08.0\Library\bin\pdftotext.exe`),
				filepath.Join(exeDir, `..\third_party\poppler\poppler-24.08.0\Library\bin\pdftotext.exe`),
			)
		}
		for _, c := range candidates {
			if _, err := os.Stat(c); err == nil {
				abs, _ := filepath.Abs(c)
				return abs
			}
		}
	}
	if p, err := exec.LookPath("pdftotext"); err == nil {
		return p
	}
	return ""
}

func hasNonASCII(s string) bool {
	for _, r := range s {
		if r > 127 {
			return true
		}
	}
	return false
}

func popplerPrefix() string {
	pt := findPdftotext()
	if pt == "" {
		return ""
	}
	dir := filepath.Dir(pt)
	// bin is under Library/bin, share is under Library/../share
	share := filepath.Join(dir, "..", "..", "share")
	if abs, err := filepath.Abs(share); err == nil {
		if _, err := os.Stat(abs); err == nil {
			prefix := filepath.Dir(abs)
			if runtime.GOOS == "windows" && hasNonASCII(prefix) {
				junction := filepath.Join(os.TempDir(), "poppler-prefix")
				_ = os.RemoveAll(junction)
				_ = exec.Command("cmd", "/c", "mklink", "/J", junction, prefix).Run()
				if _, err := os.Stat(junction); err == nil {
					return junction
				}
			}
			return prefix
		}
	}
	return ""
}

func fallbackPdftotext(path string, priorErr error) ExtractResult {
	pt := findPdftotext()
	if pt == "" {
		if priorErr != nil {
			return ExtractResult{Error: priorErr}
		}
		return ExtractResult{Error: fmt.Errorf("pdftotext 不可用")}
	}

	workPath := path
	cleanPath := false
	if runtime.GOOS == "windows" {
		tmpDir := os.TempDir()
		base := filepath.Base(path)
		tmpPath := filepath.Join(tmpDir, base)
		if data, err := os.ReadFile(path); err == nil {
			if err := os.WriteFile(tmpPath, data, 0o600); err == nil {
				workPath = tmpPath
				cleanPath = true
			}
		}
	}
	if cleanPath {
		defer os.Remove(workPath)
	}

	cmd := exec.Command(pt, "-enc", "UTF-8", "-layout", workPath, "-")
	if prefix := popplerPrefix(); prefix != "" {
		cmd.Env = append(os.Environ(), "POPPLER_PREFIX="+prefix)
	}
	var stderrBuf strings.Builder
	cmd.Stderr = &stderrBuf
	out, err := cmd.Output()
	if err != nil {
		if priorErr != nil {
			return ExtractResult{Error: priorErr}
		}
		return ExtractResult{Error: fmt.Errorf("pdftotext 失败: %w", err)}
	}

	result := string(out)
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
