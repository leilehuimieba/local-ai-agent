package config

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"local-agent/gateway/internal/contracts"
)

type ProviderConfig struct {
	ProviderID          string `json:"provider_id"`
	DisplayName         string `json:"display_name"`
	BaseURL             string `json:"base_url"`
	ChatCompletionsPath string `json:"chat_completions_path"`
	EmbeddingsPath      string `json:"embeddings_path"`
	ModelsPath          string `json:"models_path"`
	APIKey              string `json:"api_key"`
	EmbeddingModel      string `json:"embedding_model"`
}

type SiyuanConfig struct {
	RootDir          string `json:"root_dir"`
	ExportDir        string `json:"export_dir"`
	AutoWriteEnabled bool   `json:"auto_write_enabled"`
	SyncEnabled      bool   `json:"sync_enabled"`
}

type BaiduOCRConfig struct {
	APIKey    string `json:"api_key"`
	SecretKey string `json:"secret_key"`
}

type TesseractConfig struct {
	CmdPath string `json:"cmd_path"`
}

type OCRConfig struct {
	Provider  string          `json:"provider"`
	Baidu     BaiduOCRConfig  `json:"baidu"`
	Tesseract TesseractConfig `json:"tesseract"`
}

type EmbeddingConfig struct {
	ProviderID string `json:"provider_id"`
}

type MCPServerConfig struct {
	ID           string          `json:"id"`
	Name         string          `json:"name"`
	Type         string          `json:"type"`
	URL          string          `json:"url"`
	Command      string          `json:"command,omitempty"`
	Args         []string        `json:"args,omitempty"`
	Enabled      bool            `json:"enabled"`
	ToolPolicies []MCPToolPolicy `json:"tool_policies,omitempty"`
}

type MCPToolPolicy struct {
	ToolName             string `json:"tool_name"`
	Allowed              bool   `json:"allowed"`
	RiskLevel            string `json:"risk_level"`
	RequiresConfirmation bool   `json:"requires_confirmation"`
	AuditEnabled         bool   `json:"audit_enabled"`
}

type MCPConfig struct {
	Servers []MCPServerConfig `json:"servers"`
}

type AppConfig struct {
	AppName          string                   `json:"app_name"`
	GatewayPort      int                      `json:"gateway_port"`
	RuntimePort      int                      `json:"runtime_port"`
	DefaultMode      string                   `json:"default_mode"`
	DefaultModel     contracts.ModelRef       `json:"default_model"`
	AvailableModels  []contracts.ModelRef     `json:"available_models"`
	Providers        []ProviderConfig         `json:"providers"`
	DefaultWorkspace contracts.WorkspaceRef   `json:"default_workspace"`
	Workspaces       []contracts.WorkspaceRef `json:"workspaces"`
	OCR              OCRConfig                `json:"ocr"`
	Siyuan           SiyuanConfig             `json:"siyuan"`
	Embedding        EmbeddingConfig          `json:"embedding"`
	MCP              MCPConfig                `json:"mcp"`
}

func Load(repoRoot string) (AppConfig, error) {
	cfg, err := LoadFile(repoRoot)
	if err != nil {
		return AppConfig{}, err
	}
	applyEnvOverrides(&cfg)
	normalizeConfig(&cfg)
	return cfg, validateConfig(cfg)
}

func ConfigPath(repoRoot string) string {
	return filepath.Join(repoRoot, "config", "app.json")
}

func LoadFile(repoRoot string) (AppConfig, error) {
	raw, err := os.ReadFile(ConfigPath(repoRoot))
	if err != nil {
		return AppConfig{}, fmt.Errorf("read config: %w", err)
	}
	var cfg AppConfig
	if err := json.Unmarshal(raw, &cfg); err != nil {
		return AppConfig{}, fmt.Errorf("parse config: %w", err)
	}
	return cfg, nil
}

func validateConfig(cfg AppConfig) error {
	if cfg.AppName == "" {
		return errors.New("app_name is required")
	}
	if cfg.GatewayPort == 0 || cfg.RuntimePort == 0 {
		return errors.New("gateway_port and runtime_port are required")
	}
	return nil
}

func normalizeConfig(cfg *AppConfig) {
	if cfg.DefaultMode == "" {
		cfg.DefaultMode = "standard"
	}
	if len(cfg.AvailableModels) == 0 {
		cfg.AvailableModels = []contracts.ModelRef{cfg.DefaultModel}
	}
	if cfg.DefaultModel.ModelID == "" && len(cfg.AvailableModels) > 0 {
		cfg.DefaultModel = cfg.AvailableModels[0]
	}
	if len(cfg.Workspaces) == 0 {
		cfg.Workspaces = []contracts.WorkspaceRef{cfg.DefaultWorkspace}
	}
}

func SaveFile(repoRoot string, cfg AppConfig) error {
	raw, err := json.MarshalIndent(cfg, "", "    ")
	if err != nil {
		return fmt.Errorf("marshal config: %w", err)
	}
	path := ConfigPath(repoRoot)
	tmpPath := path + ".tmp"
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return fmt.Errorf("create config dir: %w", err)
	}
	if err := os.WriteFile(tmpPath, append(raw, '\n'), 0o644); err != nil {
		return fmt.Errorf("write config: %w", err)
	}
	return os.Rename(tmpPath, path)
}

func applyEnvOverrides(cfg *AppConfig) {
	if value := os.Getenv("LOCAL_AGENT_GATEWAY_PORT"); value != "" {
		if port, err := strconv.Atoi(value); err == nil {
			cfg.GatewayPort = port
		}
	}
	if value := os.Getenv("LOCAL_AGENT_RUNTIME_PORT"); value != "" {
		if port, err := strconv.Atoi(value); err == nil {
			cfg.RuntimePort = port
		}
	}
	if value := os.Getenv("LOCAL_AGENT_DEFAULT_MODE"); value != "" {
		cfg.DefaultMode = value
	}
	if value := os.Getenv("LOCAL_AGENT_MODEL_ID"); value != "" {
		cfg.DefaultModel.ModelID = value
	}
	if value := os.Getenv("LOCAL_AGENT_MODEL_NAME"); value != "" {
		cfg.DefaultModel.DisplayName = value
	}
	if value := os.Getenv("LOCAL_AGENT_PROVIDER_ID"); value != "" {
		cfg.DefaultModel.ProviderID = value
	}
	cfg.DefaultModel.Enabled = true
	cfg.DefaultModel.Available = true
	if value := os.Getenv("LOCAL_AGENT_WORKSPACE_ROOT"); value != "" {
		cfg.DefaultWorkspace.RootPath = value
	}

	for i := range cfg.Providers {
		envKey := "LOCAL_AGENT_API_KEY_" + strings.ToUpper(cfg.Providers[i].ProviderID)
		if value := os.Getenv(envKey); value != "" {
			cfg.Providers[i].APIKey = value
		}
		envBase := "LOCAL_AGENT_BASE_URL_" + strings.ToUpper(cfg.Providers[i].ProviderID)
		if value := os.Getenv(envBase); value != "" {
			cfg.Providers[i].BaseURL = value
		}
	}
}
