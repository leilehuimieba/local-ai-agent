package main

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"

	"local-agent/gateway/internal/api"
	"local-agent/gateway/internal/config"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/session"
	"local-agent/gateway/internal/state"
)

func main() {
	root := repoRoot()
	cfg, err := config.Load(root)
	if err != nil {
		fmt.Fprintf(os.Stderr, "[local-agent] load config failed: %v\n", err)
		os.Exit(1)
	}

	addr := fmt.Sprintf("127.0.0.1:%d", cfg.GatewayPort)
	fmt.Printf("[local-agent] gateway control plane listening on http://%s\n", addr)
	fmt.Printf("[local-agent] runtime target http://127.0.0.1:%d\n", cfg.RuntimePort)

	runtimeClient := runtimeclient.NewClient(cfg.RuntimePort)
	eventBus := session.NewEventBus(root)
	settingsStore := state.NewSettingsStore(root, cfg)
	confirmationStore := state.NewConfirmationStore()

	if err := http.ListenAndServe(addr, api.NewRouter(root, cfg, runtimeClient, eventBus, settingsStore, confirmationStore)); err != nil {
		fmt.Fprintf(os.Stderr, "[local-agent] gateway stopped: %v\n", err)
		os.Exit(1)
	}
}

func repoRoot() string {
	cwd, err := os.Getwd()
	if err != nil {
		return "."
	}
	if filepath.Base(cwd) == "gateway" {
		return filepath.Dir(cwd)
	}
	return cwd
}
