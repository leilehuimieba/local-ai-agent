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

type OCRConfig struct {
	Provider string         `json:"provider"`
	Baidu    BaiduOCRConfig `json:"baidu"`
}

type EmbeddingConfig struct {
	ProviderID string `json:"provider_id"`
}

type AppConfig struct {
	AppName          string           `json:"app_name"`
	GatewayPort      int              `json:"gateway_port"`
	RuntimePort      int              `json:"runtime_port"`
	DefaultMode      string           `json:"default_mode"`
	DefaultModel     contracts.ModelRef         `json:"default_model"`
	AvailableModels  []contracts.ModelRef       `json:"available_models"`
	Providers        []ProviderConfig           `json:"providers"`
	DefaultWorkspace contracts.WorkspaceRef     `json:"default_workspace"`
	Workspaces       []contracts.WorkspaceRef   `json:"workspaces"`
	OCR              OCRConfig        `json:"ocr"`
	Siyuan           SiyuanConfig     `json:"siyuan"`
	Embedding        EmbeddingConfig  `json:"embedding"`
}

func Load(repoRoot string) (AppConfig, error) {
	path := filepath.Join(repoRoot, "config", "app.json")
	raw, err := os.ReadFile(path)
	if err != nil {
		return AppConfig{}, fmt.Errorf("read config: %w", err)
	}

	var cfg AppConfig
	if err := json.Unmarshal(raw, &cfg); err != nil {
		return AppConfig{}, fmt.Errorf("parse config: %w", err)
	}

	applyEnvOverrides(&cfg)

	if cfg.AppName == "" {
		return AppConfig{}, errors.New("app_name is required")
	}
	if cfg.GatewayPort == 0 || cfg.RuntimePort == 0 {
		return AppConfig{}, errors.New("gateway_port and runtime_port are required")
	}
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

	return cfg, nil
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
