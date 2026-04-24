package api

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"strings"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/state"
)

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

func providersHandler(
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		writeJSON(w, http.StatusOK, buildProviderSettingsResponse(cfg, credentials, runtimeStore))
	}
}

func providerTestHandler(cfg config.AppConfig) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, provider, ok := decodeProviderTest(w, r, cfg)
		if !ok {
			return
		}
		writeJSON(w, http.StatusOK, testProvider(cfg, provider, payload))
	}
}

func providerSaveHandler(
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, provider, ok := decodeProviderSave(w, r, cfg)
		if !ok {
			return
		}
		status, response := saveProvider(provider, payload, credentials, runtimeStore)
		writeJSON(w, status, response)
	}
}

func providerApplyHandler(
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, provider, ok := decodeProviderApply(w, r, cfg)
		if !ok {
			return
		}
		status, response := applyProvider(provider, payload, credentials, runtimeStore)
		writeJSON(w, status, response)
	}
}

func providerRemoveHandler(
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, provider, ok := decodeProviderRemove(w, r, cfg)
		if !ok {
			return
		}
		status, response := removeProvider(provider, payload, credentials, runtimeStore)
		writeJSON(w, status, response)
	}
}

func decodeProviderTest(
	w http.ResponseWriter,
	r *http.Request,
	cfg config.AppConfig,
) (ProviderTestRequest, config.ProviderConfig, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ProviderTestRequest{}, config.ProviderConfig{}, false
	}
	var payload ProviderTestRequest
	if !decodeJSONBody(w, r, &payload) || payload.APIKey == "" {
		http.Error(w, "provider_id and api_key are required", http.StatusBadRequest)
		return ProviderTestRequest{}, config.ProviderConfig{}, false
	}
	provider, ok := findProvider(cfg, payload.ProviderID)
	if !ok {
		http.Error(w, "provider not found", http.StatusBadRequest)
	}
	return payload, provider, ok
}

func decodeProviderSave(
	w http.ResponseWriter,
	r *http.Request,
	cfg config.AppConfig,
) (ProviderSaveRequest, config.ProviderConfig, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ProviderSaveRequest{}, config.ProviderConfig{}, false
	}
	var payload ProviderSaveRequest
	if !decodeJSONBody(w, r, &payload) || payload.APIKey == "" {
		http.Error(w, "provider_id and api_key are required", http.StatusBadRequest)
		return ProviderSaveRequest{}, config.ProviderConfig{}, false
	}
	provider, ok := findProvider(cfg, payload.ProviderID)
	if !ok {
		http.Error(w, "provider not found", http.StatusBadRequest)
	}
	return payload, provider, ok
}

func decodeProviderApply(
	w http.ResponseWriter,
	r *http.Request,
	cfg config.AppConfig,
) (ProviderApplyRequest, config.ProviderConfig, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ProviderApplyRequest{}, config.ProviderConfig{}, false
	}
	var payload ProviderApplyRequest
	if !decodeJSONBody(w, r, &payload) || payload.ProviderID == "" {
		http.Error(w, "provider_id is required", http.StatusBadRequest)
		return ProviderApplyRequest{}, config.ProviderConfig{}, false
	}
	provider, ok := findProvider(cfg, payload.ProviderID)
	if !ok {
		http.Error(w, "provider not found", http.StatusBadRequest)
	}
	return payload, provider, ok
}

func decodeProviderRemove(
	w http.ResponseWriter,
	r *http.Request,
	cfg config.AppConfig,
) (ProviderRemoveRequest, config.ProviderConfig, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ProviderRemoveRequest{}, config.ProviderConfig{}, false
	}
	var payload ProviderRemoveRequest
	if !decodeJSONBody(w, r, &payload) || payload.ProviderID == "" {
		http.Error(w, "provider_id is required", http.StatusBadRequest)
		return ProviderRemoveRequest{}, config.ProviderConfig{}, false
	}
	provider, ok := findProvider(cfg, payload.ProviderID)
	if !ok {
		http.Error(w, "provider not found", http.StatusBadRequest)
	}
	return payload, provider, ok
}

func decodeJSONBody(w http.ResponseWriter, r *http.Request, target any) bool {
	if err := json.NewDecoder(r.Body).Decode(target); err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return false
	}
	return true
}

func buildProviderSettingsResponse(
	cfg config.AppConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) ProviderSettingsResponse {
	activeProviderID, _ := runtimeStore.Snapshot()
	items := make([]ProviderSettingsItem, 0, len(cfg.Providers))
	for _, provider := range cfg.Providers {
		items = append(items, buildProviderSettingsItem(provider, credentials, runtimeStore))
	}
	return ProviderSettingsResponse{ActiveProviderID: activeProviderID, Providers: items}
}

func buildProviderSettingsItem(
	provider config.ProviderConfig,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) ProviderSettingsItem {
	credential, credentialOK := credentials.Get(provider.ProviderID)
	runtimeRecord, runtimeOK := runtimeStore.Get(provider.ProviderID)
	view := providerSettingsView(provider, credential, credentialOK, runtimeRecord, runtimeOK)
	return ProviderSettingsItem{
		ProviderID: provider.ProviderID, DisplayName: view.DisplayName, BaseURL: view.BaseURL,
		ChatCompletionsPath: view.ChatCompletionsPath, ModelsPath: view.ModelsPath,
		CredentialKind: "api_key", SupportsTest: true, Editable: true,
		CredentialStatus: buildProviderCredentialStatus(credential, credentialOK, runtimeRecord, runtimeOK),
	}
}

func providerSettingsView(
	provider config.ProviderConfig,
	credential state.ProviderCredentialRecord,
	credentialOK bool,
	runtimeRecord state.RuntimeProviderRecord,
	runtimeOK bool,
) config.ProviderConfig {
	if credentialOK && savedConfigPending(credential, runtimeRecord, runtimeOK) {
		return config.ProviderConfig{
			ProviderID: provider.ProviderID, DisplayName: firstNonEmpty(credential.DisplayName, provider.DisplayName),
			BaseURL:             firstNonEmpty(credential.BaseURL, provider.BaseURL),
			ChatCompletionsPath: firstNonEmpty(credential.ChatCompletionsPath, provider.ChatCompletionsPath),
			ModelsPath:          firstNonEmpty(credential.ModelsPath, provider.ModelsPath),
		}
	}
	if runtimeOK && runtimeRecord.Status == "applied" {
		return config.ProviderConfig{
			ProviderID: provider.ProviderID, DisplayName: runtimeRecord.DisplayName, BaseURL: runtimeRecord.BaseURL,
			ChatCompletionsPath: runtimeRecord.ChatCompletionsPath, ModelsPath: runtimeRecord.ModelsPath,
		}
	}
	if credentialOK {
		return config.ProviderConfig{
			ProviderID: provider.ProviderID, DisplayName: firstNonEmpty(credential.DisplayName, provider.DisplayName),
			BaseURL:             firstNonEmpty(credential.BaseURL, provider.BaseURL),
			ChatCompletionsPath: firstNonEmpty(credential.ChatCompletionsPath, provider.ChatCompletionsPath),
			ModelsPath:          firstNonEmpty(credential.ModelsPath, provider.ModelsPath),
		}
	}
	return provider
}

func savedConfigPending(
	credential state.ProviderCredentialRecord,
	runtimeRecord state.RuntimeProviderRecord,
	runtimeOK bool,
) bool {
	if !credential.HasCredential {
		return false
	}
	if !runtimeOK || runtimeRecord.Status != "applied" {
		return true
	}
	return runtimeRecord.ConfigVersion != credential.UpdatedAt
}

func buildProviderCredentialStatus(
	credential state.ProviderCredentialRecord,
	credentialOK bool,
	runtimeRecord state.RuntimeProviderRecord,
	runtimeOK bool,
) ProviderCredentialStatus {
	status := ProviderCredentialStatus{
		HasCredential: credentialOK && credential.HasCredential, APIKeyMasked: credential.APIKeyMasked,
		UpdatedAt: credential.UpdatedAt, LastTestStatus: defaultTestStatus(credential.LastTestStatus),
		LastTestMessage: credential.LastTestMessage, LastTestAt: credential.LastTestAt,
		ApplyStatus: "not_configured", AppliedAt: runtimeRecord.AppliedAt, PendingReload: false,
	}
	if runtimeOK && runtimeRecord.Status == "applied" && !runtimeRecord.PendingReload {
		status.ApplyStatus = "applied"
		return status
	}
	if credentialOK && credential.HasCredential {
		status.ApplyStatus = "saved_not_applied"
		status.PendingReload = true
	}
	if runtimeOK && runtimeRecord.PendingReload {
		status.PendingReload = true
	}
	return status
}

func defaultTestStatus(status string) string {
	if status == "" {
		return "idle"
	}
	return status
}

func testProvider(
	cfg config.AppConfig,
	provider config.ProviderConfig,
	payload ProviderTestRequest,
) ProviderTestResponse {
	checkedAt := fmt.Sprintf("%d", time.Now().UnixMilli())
	message, code := validateProviderConnection(cfg, provider, payload)
	return ProviderTestResponse{
		OK: code == "", ProviderID: provider.ProviderID, Message: message,
		CheckedAt: checkedAt, ErrorCode: code,
	}
}

func validateProviderConnection(
	cfg config.AppConfig,
	provider config.ProviderConfig,
	payload ProviderTestRequest,
) (string, string) {
	target, err := modelsURL(resolveProviderBaseURL(provider, payload), resolveModelsPath(provider, payload))
	if err != nil {
		return "provider 地址无效", "provider_unreachable"
	}
	body, statusCode, err := requestProviderModels(target, payload.APIKey)
	if err != nil {
		return classifyProviderError(err)
	}
	if message, code, handled := providerHTTPResult(statusCode); handled {
		return message, code
	}
	return validateProviderModels(body, expectedModelID(cfg, provider.ProviderID))
}

func requestProviderModels(target string, apiKey string) ([]byte, int, error) {
	req, err := http.NewRequest(http.MethodGet, target, nil)
	if err != nil {
		return nil, 0, err
	}
	req.Header.Set("Authorization", "Bearer "+strings.TrimSpace(apiKey))
	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return nil, 0, err
	}
	defer resp.Body.Close()
	body, err := ioReadAll(resp)
	return body, resp.StatusCode, err
}

func ioReadAll(resp *http.Response) ([]byte, error) {
	return io.ReadAll(resp.Body)
}

func providerHTTPResult(statusCode int) (string, string, bool) {
	if statusCode == http.StatusUnauthorized || statusCode == http.StatusForbidden {
		return "认证失败，请检查 API Key", "authentication_failed", true
	}
	if statusCode < http.StatusOK || statusCode >= http.StatusMultipleChoices {
		return fmt.Sprintf("provider 返回异常状态: %d", statusCode), "invalid_provider_response", true
	}
	return "", "", false
}

func validateProviderModels(body []byte, modelID string) (string, string) {
	var payload providerModelsPayload
	if err := json.Unmarshal(body, &payload); err != nil {
		return "provider 响应不是有效 JSON", "invalid_provider_response"
	}
	if modelID != "" && !containsModel(payload.Data, modelID) {
		return "默认模型当前不可用", "model_unavailable"
	}
	return "连接校验成功", ""
}

func containsModel(items []struct {
	ID string `json:"id"`
}, modelID string) bool {
	if len(items) == 0 {
		return false
	}
	for _, item := range items {
		if item.ID == modelID {
			return true
		}
	}
	return false
}

func saveProvider(
	provider config.ProviderConfig,
	payload ProviderSaveRequest,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) (int, ProviderSaveResponse) {
	record, err := credentials.Save(savedProviderRecord(provider, payload))
	if err != nil {
		return http.StatusInternalServerError, ProviderSaveResponse{OK: false, ProviderID: provider.ProviderID, Message: err.Error()}
	}
	_ = runtimeStore.MarkPending(provider.ProviderID, record.UpdatedAt, "凭据已保存，尚未应用到运行时")
	runtimeRecord, runtimeOK := runtimeStore.Get(provider.ProviderID)
	status := buildProviderCredentialStatus(record, true, runtimeRecord, runtimeOK)
	return http.StatusOK, ProviderSaveResponse{OK: true, ProviderID: provider.ProviderID, Message: "凭据已保存，尚未应用到运行时", CredentialStatus: status}
}

func applyProvider(
	provider config.ProviderConfig,
	payload ProviderApplyRequest,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) (int, ProviderApplyResponse) {
	record, ok := credentials.Get(payload.ProviderID)
	if !ok || !record.HasCredential {
		return http.StatusBadRequest, ProviderApplyResponse{OK: false, ProviderID: provider.ProviderID, Message: "当前 provider 尚未保存凭据", ApplyMode: "restart_required", RestartRequired: true}
	}
	appliedAt := fmt.Sprintf("%d", time.Now().UnixMilli())
	runtimeRecord := appliedRuntimeRecord(provider, record, appliedAt)
	if _, err := runtimeStore.Apply(runtimeRecord); err != nil {
		return http.StatusInternalServerError, ProviderApplyResponse{OK: false, ProviderID: provider.ProviderID, Message: err.Error(), ApplyMode: "restart_required", RestartRequired: true}
	}
	return http.StatusOK, ProviderApplyResponse{OK: true, ProviderID: provider.ProviderID, Message: "已应用到运行时", ApplyMode: "hot_reload", AppliedAt: appliedAt, RestartRequired: false}
}

func appliedRuntimeRecord(
	provider config.ProviderConfig,
	record state.ProviderCredentialRecord,
	appliedAt string,
) state.RuntimeProviderRecord {
	return state.RuntimeProviderRecord{
		ProviderID: provider.ProviderID, DisplayName: firstNonEmpty(record.DisplayName, provider.DisplayName),
		BaseURL:             firstNonEmpty(record.BaseURL, provider.BaseURL),
		ChatCompletionsPath: firstNonEmpty(record.ChatCompletionsPath, provider.ChatCompletionsPath),
		ModelsPath:          firstNonEmpty(record.ModelsPath, provider.ModelsPath),
		APIKey:              record.APIKey, AppliedAt: appliedAt, ConfigVersion: record.UpdatedAt,
		Status: "applied", PendingReload: false, LastApplyMessage: "已应用到 gateway 运行配置",
	}
}

func removeProvider(
	provider config.ProviderConfig,
	payload ProviderRemoveRequest,
	credentials *state.ProviderCredentialStore,
	runtimeStore *state.RuntimeProviderStore,
) (int, ProviderRemoveResponse) {
	if err := credentials.Remove(payload.ProviderID); err != nil {
		return http.StatusInternalServerError, ProviderRemoveResponse{OK: false, ProviderID: provider.ProviderID, Message: err.Error()}
	}
	if !runtimeStore.IsActive(payload.ProviderID) {
		return http.StatusOK, ProviderRemoveResponse{OK: true, ProviderID: provider.ProviderID, Message: "已移除凭据"}
	}
	return http.StatusOK, ProviderRemoveResponse{
		OK: true, ProviderID: provider.ProviderID, StateCode: "saved_removed_but_runtime_still_active",
		Message: "已移除已保存凭据，当前运行态仍保留原配置，请重启或切换后再清理",
	}
}

func resolveProviderBaseURL(provider config.ProviderConfig, payload ProviderTestRequest) string {
	if strings.TrimSpace(payload.BaseURL) != "" {
		return payload.BaseURL
	}
	return provider.BaseURL
}

func resolveModelsPath(provider config.ProviderConfig, payload ProviderTestRequest) string {
	return firstNonEmpty(payload.ModelsPath, provider.ModelsPath)
}

func savedProviderRecord(provider config.ProviderConfig, payload ProviderSaveRequest) state.ProviderCredentialRecord {
	return state.ProviderCredentialRecord{
		ProviderID: provider.ProviderID, DisplayName: firstNonEmpty(payload.DisplayName, provider.DisplayName),
		BaseURL:             firstNonEmpty(payload.BaseURL, provider.BaseURL),
		ChatCompletionsPath: firstNonEmpty(payload.ChatCompletionsPath, provider.ChatCompletionsPath),
		ModelsPath:          firstNonEmpty(payload.ModelsPath, provider.ModelsPath), APIKey: payload.APIKey,
	}
}

func firstNonEmpty(values ...string) string {
	for _, value := range values {
		if strings.TrimSpace(value) != "" {
			return value
		}
	}
	return ""
}

func modelsURL(baseURL string, modelsPath string) (string, error) {
	base, err := url.Parse(strings.TrimRight(baseURL, "/") + "/")
	if err != nil {
		return "", err
	}
	relative, err := url.Parse(strings.TrimLeft(modelsPath, "/"))
	if err != nil {
		return "", err
	}
	return base.ResolveReference(relative).String(), nil
}

func classifyProviderError(err error) (string, string) {
	var netErr net.Error
	if errors.As(err, &netErr) {
		return "provider 不可达，请检查网络或服务地址", "provider_unreachable"
	}
	return "provider 不可达，请检查网络或服务地址", "provider_unreachable"
}

func expectedModelID(cfg config.AppConfig, providerID string) string {
	for _, model := range cfg.AvailableModels {
		if model.ProviderID == providerID && model.Enabled {
			return model.ModelID
		}
	}
	if cfg.DefaultModel.ProviderID == providerID {
		return cfg.DefaultModel.ModelID
	}
	return ""
}

func findProvider(cfg config.AppConfig, providerID string) (config.ProviderConfig, bool) {
	for _, provider := range cfg.Providers {
		if provider.ProviderID == providerID {
			return provider, true
		}
	}
	return config.ProviderConfig{}, false
}
