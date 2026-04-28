package api

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/knowledge"
	"local-agent/gateway/internal/memory"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/session"
	"local-agent/gateway/internal/state"
	"local-agent/gateway/internal/token"
)

func NewRouter(
	repoRoot string,
	cfg config.AppConfig,
	runtimeClient *runtimeclient.Client,
	eventBus *session.EventBus,
	settingsStore *state.SettingsStore,
	confirmationStore *state.ConfirmationStore,
	credentialStore *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
	tok *token.Manager,
) http.Handler {
	mux := http.NewServeMux()
	chat := NewChatHandler(repoRoot, cfg, runtimeClient, eventBus, settingsStore, confirmationStore, credentialStore, runtimeStore)
	memoryDeps := memoryRouteDeps{store: memory.NewStore(repoRoot), state: settingsStore}
	registerCoreRoutes(mux, cfg)
	registerProvidersRoutes(mux, cfg, credentialStore, runtimeStore)
	registerLearningRoutes(mux, memoryDeps)
	registerSettingsRoutes(mux, repoRoot, cfg, settingsStore)
	registerLogsRoutes(mux, repoRoot, cfg.RuntimePort, eventBus)
	registerReleaseRoutes(mux, repoRoot)
	registerMemoryRoutes(mux, memoryDeps)
	registerChatRoutes(mux, chat)
	knowledge.NewHandler(repoRoot).RegisterRoutes(mux, settingsStore, repoRoot, cfg)
	mux.Handle("/", spaHandler(repoRoot, tok.Value()))
	return tok.Middleware(mux)
}

func registerCoreRoutes(mux *http.ServeMux, cfg config.AppConfig) {
	mux.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		writeJSON(w, http.StatusOK, map[string]any{
			"status":  "ok",
			"app":     cfg.AppName,
			"gateway": cfg.GatewayPort,
		})
	})
}

func providerOptions(items []config.ProviderConfig) []ProviderOption {
	options := make([]ProviderOption, 0, len(items))
	for _, item := range items {
		options = append(options, ProviderOption{
			ProviderID:          item.ProviderID,
			DisplayName:         item.DisplayName,
			BaseURL:             item.BaseURL,
			ChatCompletionsPath: item.ChatCompletionsPath,
			ModelsPath:          item.ModelsPath,
		})
	}
	return options
}

func spaHandler(repoRoot string, tokenValue string) http.Handler {
	distDir := filepath.Join(repoRoot, "frontend", "dist")
	indexFile := filepath.Join(distDir, "index.html")
	fileServer := http.FileServer(http.Dir(distDir))

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if _, err := os.Stat(indexFile); err != nil {
			w.Header().Set("Content-Type", "text/html; charset=utf-8")
			_, _ = fmt.Fprint(w, `<!doctype html><html lang="zh-CN"><head><meta charset="utf-8"><title>本地智能体</title></head><body style="font-family:Segoe UI,Microsoft YaHei,sans-serif;padding:32px"><h1>前端尚未构建</h1><p>请先在 <code>frontend/</code> 下执行 <code>npm install</code> 和 <code>npm run build</code>。</p></body></html>`)
			return
		}

		requestPath := filepath.Join(distDir, filepath.Clean(r.URL.Path))
		if info, err := os.Stat(requestPath); err == nil && !info.IsDir() {
			fileServer.ServeHTTP(w, r)
			return
		}

		if tokenValue != "" {
			injectTokenAndServe(w, r, indexFile, tokenValue)
			return
		}
		http.ServeFile(w, r, indexFile)
	})
}

func injectTokenAndServe(w http.ResponseWriter, r *http.Request, indexFile string, tokenValue string) {
	raw, err := os.ReadFile(indexFile)
	if err != nil {
		http.ServeFile(w, r, indexFile)
		return
	}
	body := string(raw)
	meta := fmt.Sprintf(`<meta name="local-agent-token" content="%s" />`, tokenValue)
	if strings.Contains(body, `<meta name="local-agent-token"`) {
		body = strings.ReplaceAll(body, meta, "")
	}
	if idx := strings.Index(body, `<meta charset="UTF-8"`); idx != -1 {
		before := body[:idx]
		after := body[idx:]
		body = before + meta + "\n    " + after
	}
	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	_, _ = w.Write([]byte(body))
}

func fetchRuntimeStatus(runtimePort int) RuntimeStatus {
	for attempt := 0; attempt < 3; attempt++ {
		status, ok := requestRuntimeStatus(runtimePort)
		if ok {
			return status
		}
		time.Sleep(120 * time.Millisecond)
	}
	return RuntimeStatus{OK: false, Name: "runtime-host", Version: "unreachable"}
}

func requestRuntimeStatus(runtimePort int) (RuntimeStatus, bool) {
	client := http.Client{Timeout: time.Second}
	resp, err := client.Get(fmt.Sprintf("http://127.0.0.1:%d/health", runtimePort))
	if err != nil {
		return RuntimeStatus{}, false
	}
	defer resp.Body.Close()
	var payload RuntimeStatus
	if err := json.NewDecoder(resp.Body).Decode(&payload); err != nil {
		return RuntimeStatus{OK: false, Name: "runtime-host", Version: "invalid-response"}, true
	}
	return payload, true
}

func writeJSON(w http.ResponseWriter, status int, payload any) {
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(payload)
}
