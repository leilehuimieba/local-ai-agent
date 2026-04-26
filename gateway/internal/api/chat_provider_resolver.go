package api

import (
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/service"
)

func (h *ChatHandler) resolveProviderRef(providerID string) (contracts.ProviderRef, error) {
	return service.ResolveProviderRef(providerID, h.runtimeStore, h.credentialStore, h.appConfig)
}
