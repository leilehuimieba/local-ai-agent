package service

import (
	"fmt"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/state"
)

func ResolveProviderRef(
	providerID string,
	runtimeStore *state.RuntimeProviderStore,
	credentialStore *state.ProviderCredentialStore,
	appConfig config.AppConfig,
) (contracts.ProviderRef, error) {
	if ref, ok := RuntimeProviderRef(runtimeStore, providerID); ok {
		return ref, nil
	}
	if ref, ok := CredentialProviderRef(appConfig, credentialStore, providerID); ok {
		return ref, nil
	}
	if ref, ok := ConfigProviderRef(appConfig, providerID); ok {
		return ref, nil
	}
	return contracts.ProviderRef{}, fmt.Errorf("provider %s 缺少可用凭据，请先保存并应用或检查配置", providerID)
}

func RuntimeProviderRef(store *state.RuntimeProviderStore, providerID string) (contracts.ProviderRef, bool) {
	record, ok := store.Get(providerID)
	if !ok || record.Status != "applied" || record.APIKey == "" {
		return contracts.ProviderRef{}, false
	}
	return RuntimeRecordRef(record), true
}

func CredentialProviderRef(cfg config.AppConfig, store *state.ProviderCredentialStore, providerID string) (contracts.ProviderRef, bool) {
	record, ok := store.Get(providerID)
	if !ok || !record.HasCredential || record.APIKey == "" {
		return contracts.ProviderRef{}, false
	}
	provider, ok := CatalogProvider(cfg, providerID)
	if !ok {
		return contracts.ProviderRef{}, false
	}
	return CredentialRecordRef(provider, record), true
}

func ConfigProviderRef(cfg config.AppConfig, providerID string) (contracts.ProviderRef, bool) {
	provider, ok := CatalogProvider(cfg, providerID)
	if !ok || provider.APIKey == "" {
		return contracts.ProviderRef{}, false
	}
	return ProviderConfigRef(provider, provider.APIKey), true
}

func CatalogProvider(cfg config.AppConfig, providerID string) (config.ProviderConfig, bool) {
	for _, item := range cfg.Providers {
		if item.ProviderID == providerID {
			return item, true
		}
	}
	return config.ProviderConfig{}, false
}

func ProviderConfigRef(provider config.ProviderConfig, apiKey string) contracts.ProviderRef {
	return contracts.ProviderRef{
		ProviderID: provider.ProviderID, DisplayName: provider.DisplayName, BaseURL: provider.BaseURL,
		ChatCompletionsPath: provider.ChatCompletionsPath, ModelsPath: provider.ModelsPath, APIKey: apiKey,
	}
}

func RuntimeRecordRef(record state.RuntimeProviderRecord) contracts.ProviderRef {
	return contracts.ProviderRef{
		ProviderID: record.ProviderID, DisplayName: record.DisplayName, BaseURL: record.BaseURL,
		ChatCompletionsPath: record.ChatCompletionsPath, ModelsPath: record.ModelsPath, APIKey: record.APIKey,
	}
}

func CredentialRecordRef(provider config.ProviderConfig, record state.ProviderCredentialRecord) contracts.ProviderRef {
	return contracts.ProviderRef{
		ProviderID: provider.ProviderID, DisplayName: FirstNonEmptyValue(record.DisplayName, provider.DisplayName),
		BaseURL:             FirstNonEmptyValue(record.BaseURL, provider.BaseURL),
		ChatCompletionsPath: FirstNonEmptyValue(record.ChatCompletionsPath, provider.ChatCompletionsPath),
		ModelsPath:          FirstNonEmptyValue(record.ModelsPath, provider.ModelsPath), APIKey: record.APIKey,
	}
}

func FirstNonEmptyValue(values ...string) string {
	for _, value := range values {
		if value != "" {
			return value
		}
	}
	return ""
}
