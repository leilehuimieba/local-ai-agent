package config

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
)

type ModelRef struct {
	ProviderID  string `json:"provider_id"`
	ModelID     string `json:"model_id"`
	DisplayName string `json:"display_name"`
	Enabled     bool   `json:"enabled"`
	Available   bool   `json:"available"`
}

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

type WorkspaceRef struct {
	WorkspaceID string `json:"workspace_id"`
	Name        string `json:"name"`
	RootPath    string `json:"root_path"`
	IsActive    bool   `json:"is_active"`
}

type SiyuanConfig struct {
	RootDir          string `json:"root_dir"`
	ExportDir        string `json:"export_dir"`
	AutoWriteEnabled bool   `json:"auto_write_enabled"`
	SyncEnabled      bool   `json:"sync_enabled"`
}

type AppConfig struct {
	AppName          string           `json:"app_name"`
	GatewayPort      int              `json:"gateway_port"`
	RuntimePort      int              `json:"runtime_port"`
	DefaultMode      string           `json:"default_mode"`
	DefaultModel     ModelRef         `json:"default_model"`
	AvailableModels  []ModelRef       `json:"available_models"`
	Providers        []ProviderConfig `json:"providers"`
	DefaultWorkspace WorkspaceRef     `json:"default_workspace"`
	Workspaces       []WorkspaceRef   `json:"workspaces"`
	Siyuan           SiyuanConfig     `json:"siyuan"`
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
		cfg.AvailableModels = []ModelRef{cfg.DefaultModel}
	}
	if cfg.DefaultModel.ModelID == "" && len(cfg.AvailableModels) > 0 {
		cfg.DefaultModel = cfg.AvailableModels[0]
	}
	if len(cfg.Workspaces) == 0 {
		cfg.Workspaces = []WorkspaceRef{cfg.DefaultWorkspace}
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
}
