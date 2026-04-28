package api

import (
	"context"
	"errors"
	"net/http"
	"strconv"
	"sync"
	"time"

	"local-agent/gateway/internal/knowledge"
	"local-agent/gateway/internal/providers/bestblogs"
)

func bestblogsArticleReadHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeBestblogsReadRequest(w, r)
		if !ok {
			return
		}
		ctx, cancel := context.WithTimeout(r.Context(), 20*time.Second)
		defer cancel()
		result, err := bestblogsArticleReader(ctx, payload)
		if err != nil {
			writeBestblogsError(w, err)
			return
		}
		writeJSON(w, http.StatusOK, result)
	}
}

func bestblogsArticleListHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		page := queryInt(r, "page", 1)
		pageSize := queryInt(r, "page_size", 20)
		language := r.URL.Query().Get("language")
		if language == "" {
			language = "zh"
		}

		ctx, cancel := context.WithTimeout(r.Context(), 20*time.Second)
		defer cancel()

		client := bestblogs.NewClient(nil)
		result, err := client.ListArticles(ctx, bestblogs.ListArticlesRequest{
			Language: language,
			Page:     page,
			PageSize: pageSize,
		})
		if err != nil {
			var providerErr bestblogs.Error
			if errors.As(err, &providerErr) {
				writeJSON(w, providerErr.Status, map[string]any{
					"ok": false, "error_code": providerErr.Code, "message": providerErr.Message,
				})
				return
			}
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		writeJSON(w, http.StatusOK, result)
	}
}

type bestblogsScrapeRequest struct {
	StartPage    int    `json:"start_page"`
	EndPage      int    `json:"end_page"`
	PageSize     int    `json:"page_size"`
	Language     string `json:"language"`
	FullContent  bool   `json:"full_content"`
	MaxArticles  int    `json:"max_articles"`
}

type bestblogsScrapeResponse struct {
	OK           bool   `json:"ok"`
	Message      string `json:"message"`
	TotalPages   int    `json:"total_pages,omitempty"`
	Imported     int    `json:"imported"`
	Skipped      int    `json:"skipped"`
	Errors       int    `json:"errors"`
	FinishedPage int    `json:"finished_page"`
}

var (
	scrapeJobMu   sync.Mutex
	scrapeJobDone = true
	scrapeJobResult bestblogsScrapeResponse
)

func bestblogsScrapeHandler(repoRoot string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodGet {
			scrapeJobMu.Lock()
			done := scrapeJobDone
			result := scrapeJobResult
			scrapeJobMu.Unlock()
			result.OK = done
			if done {
				result.Message = "上次抓取已完成"
			} else {
				result.Message = "抓取进行中..."
			}
			writeJSON(w, http.StatusOK, result)
			return
		}
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		scrapeJobMu.Lock()
		if !scrapeJobDone {
			scrapeJobMu.Unlock()
			writeJSON(w, http.StatusConflict, bestblogsScrapeResponse{
				OK: false, Message: "已有抓取任务在运行中",
			})
			return
		}
		scrapeJobDone = false
		scrapeJobMu.Unlock()

		var payload bestblogsScrapeRequest
		if !decodeJSONBody(w, r, &payload) {
			scrapeJobMu.Lock()
			scrapeJobDone = true
			scrapeJobMu.Unlock()
			return
		}
		if payload.StartPage < 1 {
			payload.StartPage = 1
		}
		if payload.EndPage < payload.StartPage {
			payload.EndPage = payload.StartPage
		}
		if payload.PageSize < 1 || payload.PageSize > 100 {
			payload.PageSize = 20
		}
		if payload.Language == "" {
			payload.Language = "zh"
		}

		writeJSON(w, http.StatusAccepted, bestblogsScrapeResponse{
			OK: true, Message: "抓取已启动",
			TotalPages: payload.EndPage - payload.StartPage + 1,
		})

		go runBestblogsScrape(repoRoot, payload)
	}
}

func runBestblogsScrape(repoRoot string, req bestblogsScrapeRequest) {
	defer func() {
		scrapeJobMu.Lock()
		scrapeJobDone = true
		scrapeJobMu.Unlock()
	}()

	client := bestblogs.NewClient(nil)
	store := knowledge.NewStore(repoRoot)

	// 尝试获取默认工作区
	workspaceID := "main"

	existing, _ := store.List(workspaceID)
	existingSources := make(map[string]bool, len(existing))
	for _, it := range existing {
		if it.Source != "" {
			existingSources[it.Source] = true
		}
	}

	result := bestblogsScrapeResponse{}

	for page := req.StartPage; page <= req.EndPage; page++ {
		ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
		resp, err := client.ListArticles(ctx, bestblogs.ListArticlesRequest{
			Language: req.Language,
			Page:     page,
			PageSize: req.PageSize,
		})
		cancel()

		if err != nil {
			result.Errors++
			continue
		}

		for _, item := range resp.Items {
			if req.MaxArticles > 0 && result.Imported >= req.MaxArticles {
				result.FinishedPage = page
				scrapeJobMu.Lock()
				scrapeJobResult = result
				scrapeJobMu.Unlock()
				return
			}

			sourceURL := item.URL
			if sourceURL == "" {
				sourceURL = item.ReadURL
			}
			if existingSources[sourceURL] {
				result.Skipped++
				continue
			}

			category := item.Category
			if category == "" {
				category = "未分类"
			}
			tags := item.Tags
			if len(tags) == 0 && item.Domain != "" {
				tags = []string{item.Domain}
			}

			content := item.Summary
			if req.FullContent {
				articleResult, err := fetchArticleContent(item.ReadURL, req.Language)
				if err == nil && articleResult != "" {
					content = articleResult
				}
				time.Sleep(200 * time.Millisecond)
			}

			_, err := store.Create(workspaceID, knowledge.CreateRequest{
				Title:    item.Title,
				Summary:  item.OneSentenceSummary,
				Content:  content,
				Category: category,
				Tags:     tags,
				Source:   sourceURL,
			})
			if err != nil {
				result.Errors++
				continue
			}
			existingSources[sourceURL] = true
			result.Imported++
		}
		result.FinishedPage = page

		scrapeJobMu.Lock()
		scrapeJobResult = result
		scrapeJobMu.Unlock()
	}

	result.FinishedPage = req.EndPage
	scrapeJobMu.Lock()
	scrapeJobResult = result
	scrapeJobMu.Unlock()
}

func fetchArticleContent(readURL, language string) (string, error) {
	client := bestblogs.NewClient(nil)
	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Second)
	defer cancel()

	result, err := client.ReadArticle(ctx, bestblogs.ReadArticleRequest{
		ArticleURL:      readURL,
		Language:        language,
		IncludeMarkdown: true,
	})
	if err != nil {
		return "", err
	}
	return result.Content.Markdown, nil
}

func queryInt(r *http.Request, key string, defaultVal int) int {
	raw := r.URL.Query().Get(key)
	if raw == "" {
		return defaultVal
	}
	v, err := strconv.Atoi(raw)
	if err != nil {
		return defaultVal
	}
	return v
}

func decodeBestblogsReadRequest(
	w http.ResponseWriter,
	r *http.Request,
) (bestblogs.ReadArticleRequest, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return bestblogs.ReadArticleRequest{}, false
	}
	var payload bestblogs.ReadArticleRequest
	if !decodeJSONBody(w, r, &payload) {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return bestblogs.ReadArticleRequest{}, false
	}
	return payload, true
}

func writeBestblogsError(w http.ResponseWriter, err error) {
	var providerErr bestblogs.Error
	if errors.As(err, &providerErr) {
		writeJSON(w, providerErr.Status, map[string]any{
			"ok": false, "error_code": providerErr.Code, "message": providerErr.Message,
		})
		return
	}
	http.Error(w, err.Error(), http.StatusInternalServerError)
}
