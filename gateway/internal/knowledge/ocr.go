package knowledge

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"time"
)

type baiduToken struct {
	AccessToken string `json:"access_token"`
	ExpiresIn   int    `json:"expires_in"`
}

type baiduOCRResponse struct {
	WordsResultNum int `json:"words_result_num"`
	WordsResult    []struct {
		Words string `json:"words"`
	} `json:"words_result"`
	ErrorCode int    `json:"error_code"`
	ErrorMsg  string `json:"error_msg"`
}

type baiduOCRClient struct {
	apiKey      string
	secretKey   string
	token       string
	expiresAt   time.Time
	tokenMu     sync.Mutex
	httpClient  *http.Client
}

func newBaiduOCRClient(apiKey, secretKey string) *baiduOCRClient {
	return &baiduOCRClient{
		apiKey:     apiKey,
		secretKey:  secretKey,
		httpClient: &http.Client{Timeout: 30 * time.Second},
	}
}

func (c *baiduOCRClient) getToken() (string, error) {
	c.tokenMu.Lock()
	defer c.tokenMu.Unlock()

	if c.token != "" && time.Now().Before(c.expiresAt.Add(-5*time.Minute)) {
		return c.token, nil
	}

	u := fmt.Sprintf(
		"https://aip.baidubce.com/oauth/2.0/token?grant_type=client_credentials&client_id=%s&client_secret=%s",
		url.QueryEscape(c.apiKey), url.QueryEscape(c.secretKey),
	)
	resp, err := c.httpClient.Post(u, "", nil)
	if err != nil {
		return "", fmt.Errorf("获取百度token失败: %w", err)
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	var t baiduToken
	if err := json.Unmarshal(body, &t); err != nil {
		return "", fmt.Errorf("解析百度token失败: %w", err)
	}
	if t.AccessToken == "" {
		return "", fmt.Errorf("百度token为空: %s", string(body))
	}

	c.token = t.AccessToken
	c.expiresAt = time.Now().Add(time.Duration(t.ExpiresIn) * time.Second)
	return c.token, nil
}

func (c *baiduOCRClient) recognize(imageBase64 string) (string, error) {
	token, err := c.getToken()
	if err != nil {
		return "", err
	}

	u := fmt.Sprintf("https://aip.baidubce.com/rest/2.0/ocr/v1/accurate_basic?access_token=%s", token)
	data := url.Values{}
	data.Set("image", imageBase64)
	data.Set("language_type", "CHN_ENG")

	resp, err := c.httpClient.Post(u, "application/x-www-form-urlencoded", strings.NewReader(data.Encode()))
	if err != nil {
		return "", fmt.Errorf("百度OCR请求失败: %w", err)
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	var result baiduOCRResponse
	if err := json.Unmarshal(body, &result); err != nil {
		return "", fmt.Errorf("解析百度OCR结果失败: %w", err)
	}
	if result.ErrorCode != 0 {
		return "", fmt.Errorf("百度OCR错误 %d: %s", result.ErrorCode, result.ErrorMsg)
	}

	var words []string
	for _, w := range result.WordsResult {
		words = append(words, w.Words)
	}
	return strings.Join(words, "\n"), nil
}

func findPdftoppm() string {
	if pt := findPdftotext(); pt != "" {
		ppm := filepath.Join(filepath.Dir(pt), "pdftoppm.exe")
		if _, err := os.Stat(ppm); err == nil {
			return ppm
		}
	}
	if p, err := exec.LookPath("pdftoppm"); err == nil {
		return p
	}
	return ""
}

func extractPdfWithOCR(path string, apiKey, secretKey string) ExtractResult {
	ppm := findPdftoppm()
	if ppm == "" {
		return ExtractResult{Error: fmt.Errorf("pdftoppm 不可用，无法进行 OCR")}
	}

	workPath := path
	if runtime.GOOS == "windows" {
		tmpPath := filepath.Join(os.TempDir(), filepath.Base(path))
		if data, err := os.ReadFile(path); err == nil {
			_ = os.WriteFile(tmpPath, data, 0o600)
			workPath = tmpPath
			defer os.Remove(tmpPath)
		}
	}

	tmpDir, err := os.MkdirTemp("", "pdf-ocr-*")
	if err != nil {
		return ExtractResult{Error: fmt.Errorf("创建临时目录失败: %w", err)}
	}
	defer os.RemoveAll(tmpDir)

	cmd := exec.Command(ppm, "-png", "-f", "1", "-l", "8", "-r", "100", workPath, filepath.Join(tmpDir, "page"))
	if prefix := popplerPrefix(); prefix != "" {
		cmd.Env = append(os.Environ(), "POPPLER_PREFIX="+prefix)
	}
	if err := cmd.Run(); err != nil {
		return ExtractResult{Error: fmt.Errorf("PDF转图片失败: %w", err)}
	}

	client := newBaiduOCRClient(apiKey, secretKey)
	var pages []string
	entries, _ := os.ReadDir(tmpDir)
	var pngFiles []string
	for _, entry := range entries {
		if strings.HasSuffix(strings.ToLower(entry.Name()), ".png") {
			pngFiles = append(pngFiles, entry.Name())
		}
	}
	// 限制最多处理前 8 页，避免耗时过长
	if len(pngFiles) > 8 {
		pngFiles = pngFiles[:8]
	}
	for _, name := range pngFiles {
		imgPath := filepath.Join(tmpDir, name)
		imgData, err := os.ReadFile(imgPath)
		if err != nil {
			continue
		}
		base64Img := base64.StdEncoding.EncodeToString(imgData)
		text, err := client.recognize(base64Img)
		if err != nil {
			continue
		}
		if strings.TrimSpace(text) != "" {
			pages = append(pages, text)
		}
	}

	result := strings.Join(pages, "\n")
	if strings.TrimSpace(result) == "" {
		return ExtractResult{Error: fmt.Errorf("OCR 未识别到文字")}
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
