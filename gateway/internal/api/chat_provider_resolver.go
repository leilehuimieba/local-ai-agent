package api

import (
	"fmt"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/state"
)

func (h *ChatHandler) resolveProviderRef(providerID string) (contracts.ProviderRef, error) {
	if ref, ok := runtimeProviderRef(h.runtimeStore, providerID); ok {
		return ref, nil
	}
	if ref, ok := credentialProviderRef(h.appConfig, h.credentialStore, providerID); ok {
		return ref, nil
	}
	if ref, ok := configProviderRef(h.appConfig, providerID); ok {
		return ref, nil
	}
	return contracts.ProviderRef{}, fmt.Errorf("provider %s 缺少可用凭据，请先保存并应用或检查配置", providerID)
}

func runtimeProviderRef(store *state.RuntimeProviderStore, providerID string) (contracts.ProviderRef, bool) {
	record, ok := store.Get(providerID)
	if !ok || record.Status != "applied" || record.APIKey == "" {
		return contracts.ProviderRef{}, false
	}
	return runtimeRecordRef(record), true
}

func credentialProviderRef(cfg config.AppConfig, store *state.ProviderCredentialStore, providerID string) (contracts.ProviderRef, bool) {
	record, ok := store.Get(providerID)
	if !ok || !record.HasCredential || record.APIKey == "" {
		return contracts.ProviderRef{}, false
	}
	provider, ok := catalogProvider(cfg, providerID)
	if !ok {
		return contracts.ProviderRef{}, false
	}
	return credentialRecordRef(provider, record), true
}

func configProviderRef(cfg config.AppConfig, providerID string) (contracts.ProviderRef, bool) {
	provider, ok := catalogProvider(cfg, providerID)
	if !ok || provider.APIKey == "" {
		return contracts.ProviderRef{}, false
	}
	return providerConfigRef(provider, provider.APIKey), true
}

func catalogProvider(cfg config.AppConfig, providerID string) (config.ProviderConfig, bool) {
	for _, item := range cfg.Providers {
		if item.ProviderID == providerID {
			return item, true
		}
	}
	return config.ProviderConfig{}, false
}

func providerConfigRef(provider config.ProviderConfig, apiKey string) contracts.ProviderRef {
	return contracts.ProviderRef{
		ProviderID: provider.ProviderID, DisplayName: provider.DisplayName, BaseURL: provider.BaseURL,
		ChatCompletionsPath: provider.ChatCompletionsPath, ModelsPath: provider.ModelsPath, APIKey: apiKey,
	}
}

func runtimeRecordRef(record state.RuntimeProviderRecord) contracts.ProviderRef {
	return contracts.ProviderRef{
		ProviderID: record.ProviderID, DisplayName: record.DisplayName, BaseURL: record.BaseURL,
		ChatCompletionsPath: record.ChatCompletionsPath, ModelsPath: record.ModelsPath, APIKey: record.APIKey,
	}
}

func credentialRecordRef(provider config.ProviderConfig, record state.ProviderCredentialRecord) contracts.ProviderRef {
	return contracts.ProviderRef{
		ProviderID: provider.ProviderID, DisplayName: firstNonEmptyValue(record.DisplayName, provider.DisplayName),
		BaseURL:             firstNonEmptyValue(record.BaseURL, provider.BaseURL),
		ChatCompletionsPath: firstNonEmptyValue(record.ChatCompletionsPath, provider.ChatCompletionsPath),
		ModelsPath:          firstNonEmptyValue(record.ModelsPath, provider.ModelsPath), APIKey: record.APIKey,
	}
}

func firstNonEmptyValue(values ...string) string {
	for _, value := range values {
		if value != "" {
			return value
		}
	}
	return ""
}
