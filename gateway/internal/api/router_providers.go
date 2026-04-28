package api

import (
	"net/http"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/state"
)

func registerProvidersRoutes(
	mux *http.ServeMux,
	cfg config.AppConfig,
	credentialStore *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
	repoRoot string,
) {
	registerProviderSettingsRoutes(mux, cfg, credentialStore, runtimeStore)
	registerProviderArticleRoutes(mux, repoRoot)
}

func registerProviderSettingsRoutes(
	mux *http.ServeMux,
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) {
	mux.HandleFunc("/api/v1/settings/providers", providersHandler(cfg, credentials, runtimeStore))
	mux.HandleFunc("/api/v1/settings/providers/test", providerTestHandler(cfg))
	mux.HandleFunc("/api/v1/settings/providers/save", providerSaveHandler(cfg, credentials, runtimeStore))
	mux.HandleFunc("/api/v1/settings/providers/apply", providerApplyHandler(cfg, credentials, runtimeStore))
	mux.HandleFunc("/api/v1/settings/providers/remove", providerRemoveHandler(cfg, credentials, runtimeStore))
}

func registerProviderArticleRoutes(mux *http.ServeMux, repoRoot string) {
	mux.HandleFunc("/api/v1/providers/bestblogs/article/read", bestblogsArticleReadHandler())
	mux.HandleFunc("/api/v1/providers/bestblogs/articles", bestblogsArticleListHandler())
	mux.HandleFunc("/api/v1/providers/bestblogs/scrape", bestblogsScrapeHandler(repoRoot))
}
