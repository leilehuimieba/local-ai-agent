package main

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"

	"local-agent/gateway/internal/api"
	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/mcp"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/session"
	"local-agent/gateway/internal/state"
	"local-agent/gateway/internal/token"
)

func main() {
	root := repoRoot()
	cfg, err := config.Load(root)
	if err != nil {
		fmt.Fprintf(os.Stderr, "[local-agent] load config failed: %v\n", err)
		os.Exit(1)
	}

	tok, err := token.LoadOrCreate(root)
	if err != nil {
		fmt.Fprintf(os.Stderr, "[local-agent] token init failed: %v\n", err)
		os.Exit(1)
	}

	addr := fmt.Sprintf("127.0.0.1:%d", cfg.GatewayPort)
	fmt.Printf("[local-agent] gateway control plane listening on http://%s\n", addr)
	fmt.Printf("[local-agent] runtime target http://127.0.0.1:%d\n", cfg.RuntimePort)

	runtimeClient := runtimeclient.NewClient(cfg.RuntimePort)
	eventBus := session.NewEventBus(root)
	settingsStore := state.NewSettingsStore(root, cfg)
	confirmationStore := state.NewConfirmationStore()
	credentialStore := state.NewProviderCredentialStore(root)
	runtimeStore := state.NewRuntimeProviderStore(root)

	mgr := mcp.NewManager(cfg.MCP.Servers)
	defer mgr.ShutdownAll()
	if len(mgr.Status()) > 0 {
		mgr.ConnectAll()
		for _, st := range mgr.Status() {
			if st.Ready {
				fmt.Printf("[local-agent] mcp connected: %s (%d tools)\n", st.Name, st.ToolCount)
			} else if st.Enabled {
				fmt.Fprintf(os.Stderr, "[local-agent] mcp not ready: %s (%s)\n", st.Name, st.URL)
			}
		}
	}

	if err := http.ListenAndServe(addr, api.NewRouter(
		root,
		cfg,
		runtimeClient,
		eventBus,
		settingsStore,
		confirmationStore,
		credentialStore,
		runtimeStore,
		tok,
		mgr,
	)); err != nil {
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
