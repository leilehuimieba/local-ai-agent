package api

type ProviderCredentialStatus struct {
	HasCredential   bool   `json:"has_credential"`
	APIKeyMasked    string `json:"api_key_masked,omitempty"`
	UpdatedAt       string `json:"updated_at,omitempty"`
	LastTestStatus  string `json:"last_test_status,omitempty"`
	LastTestMessage string `json:"last_test_message,omitempty"`
	LastTestAt      string `json:"last_test_at,omitempty"`
	ApplyStatus     string `json:"apply_status"`
	AppliedAt       string `json:"applied_at,omitempty"`
	PendingReload   bool   `json:"pending_reload"`
}

type ProviderSettingsItem struct {
	ProviderID          string                   `json:"provider_id"`
	DisplayName         string                   `json:"display_name"`
	BaseURL             string                   `json:"base_url"`
	ChatCompletionsPath string                   `json:"chat_completions_path"`
	ModelsPath          string                   `json:"models_path"`
	CredentialKind      string                   `json:"credential_kind"`
	SupportsTest        bool                     `json:"supports_test"`
	Editable            bool                     `json:"editable"`
	CredentialStatus    ProviderCredentialStatus `json:"credential_status"`
}

type ProviderSettingsResponse struct {
	ActiveProviderID string                 `json:"active_provider_id,omitempty"`
	Providers        []ProviderSettingsItem `json:"providers"`
}

type ProviderTestRequest struct {
	ProviderID          string `json:"provider_id"`
	DisplayName         string `json:"display_name,omitempty"`
	BaseURL             string `json:"base_url,omitempty"`
	ChatCompletionsPath string `json:"chat_completions_path,omitempty"`
	ModelsPath          string `json:"models_path,omitempty"`
	APIKey              string `json:"api_key"`
}

type ProviderTestResponse struct {
	OK         bool   `json:"ok"`
	ProviderID string `json:"provider_id"`
	Message    string `json:"message"`
	CheckedAt  string `json:"checked_at,omitempty"`
	ErrorCode  string `json:"error_code,omitempty"`
}

type ProviderSaveRequest struct {
	ProviderID          string `json:"provider_id"`
	DisplayName         string `json:"display_name,omitempty"`
	BaseURL             string `json:"base_url,omitempty"`
	ChatCompletionsPath string `json:"chat_completions_path,omitempty"`
	ModelsPath          string `json:"models_path,omitempty"`
	APIKey              string `json:"api_key"`
}

type ProviderSaveResponse struct {
	OK               bool                     `json:"ok"`
	ProviderID       string                   `json:"provider_id"`
	Message          string                   `json:"message"`
	CredentialStatus ProviderCredentialStatus `json:"credential_status"`
}

type ProviderApplyRequest struct {
	ProviderID string `json:"provider_id"`
}

type ProviderApplyResponse struct {
	OK              bool   `json:"ok"`
	ProviderID      string `json:"provider_id"`
	Message         string `json:"message"`
	ApplyMode       string `json:"apply_mode"`
	AppliedAt       string `json:"applied_at,omitempty"`
	RestartRequired bool   `json:"restart_required"`
}

type ProviderRemoveRequest struct {
	ProviderID string `json:"provider_id"`
}

type ProviderRemoveResponse struct {
	OK         bool   `json:"ok"`
	ProviderID string `json:"provider_id"`
	Message    string `json:"message"`
	StateCode  string `json:"state_code,omitempty"`
}

type providerModelsPayload struct {
	Data []struct {
		ID string `json:"id"`
	} `json:"data"`
}

