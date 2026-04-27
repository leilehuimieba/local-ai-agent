package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"syscall"
	"time"

	"local-agent/gateway/internal/config"
)

func main() {
	root, cfg, logDir := mustPrepareLauncher()
	mustStartSystem(root, cfg, logDir)
	mustOpenEntry(cfg.GatewayPort)
	printReady(cfg.GatewayPort, logDir)
}

func mustPrepareLauncher() (string, config.AppConfig, string) {
	root, err := findRepoRoot()
	if err != nil {
		fail("resolve repo root", err)
	}
	cfg, err := config.Load(root)
	if err != nil {
		fail("load config", err)
	}
	if err := ensureFrontendBuilt(root); err != nil {
		fail("prepare frontend", err)
	}
	logDir := filepath.Join(root, "logs")
	if err := os.MkdirAll(logDir, 0o755); err != nil {
		fail("create log dir", err)
	}
	return root, cfg, logDir
}

func mustStartSystem(root string, cfg config.AppConfig, logDir string) {
	if systemReady(cfg.GatewayPort, root) {
		return
	}
	if err := ensureRuntime(root, cfg, logDir); err != nil {
		fail("start runtime", err)
	}
	if err := ensureGateway(root, cfg, logDir); err != nil {
		fail("start gateway", err)
	}
	if err := waitForSystemReady(cfg.GatewayPort, root, 20*time.Second); err != nil {
		fail("wait system ready", err)
	}
}

func mustOpenEntry(port int) {
	if os.Getenv("LOCAL_AGENT_NO_BROWSER") == "1" {
		return
	}
	if err := openBrowser(gatewayURL(port)); err != nil {
		fail("open browser", err)
	}
}

func printReady(port int, logDir string) {
	fmt.Printf("[local-agent-launcher] ready: %s\n", gatewayURL(port))
	fmt.Printf("[local-agent-launcher] logs: %s\n", logDir)
}

func fail(step string, err error) {
	fmt.Fprintf(os.Stderr, "[local-agent-launcher] %s failed: %v\n", step, err)
	os.Exit(1)
}

func findRepoRoot() (string, error) {
	if envRoot := os.Getenv("LOCAL_AGENT_REPO_ROOT"); envRoot != "" {
		if ok, resolved := isRepoRoot(envRoot); ok {
			return resolved, nil
		}
	}

	candidates := []string{}
	if cwd, err := os.Getwd(); err == nil {
		candidates = append(candidates, cwd)
	}
	if exePath, err := os.Executable(); err == nil {
		candidates = append(candidates, filepath.Dir(exePath))
	}

	for _, start := range candidates {
		current := start
		for {
			if ok, resolved := isRepoRoot(current); ok {
				return resolved, nil
			}
			parent := filepath.Dir(current)
			if parent == current {
				break
			}
			current = parent
		}
	}

	return "", errors.New("config/app.json not found from current directory or executable path")
}

func isRepoRoot(path string) (bool, string) {
	resolved, err := filepath.Abs(path)
	if err != nil {
		return false, ""
	}
	configPath := filepath.Join(resolved, "config", "app.json")
	gatewayPath := filepath.Join(resolved, "gateway", "go.mod")
	if fileExists(configPath) && fileExists(gatewayPath) {
		return true, resolved
	}
	return false, ""
}

func ensureFrontendBuilt(root string) error {
	indexFile := filepath.Join(root, "frontend", "dist", "index.html")
	if fileExists(indexFile) && !frontendBuildStale(root, indexFile) {
		return nil
	}

	frontendDir := filepath.Join(root, "frontend")
	if !fileExists(filepath.Join(frontendDir, "node_modules")) {
		if err := runCommand(frontendDir, nil, filepath.Join(root, "logs", "frontend-install.log"), "npm", "install"); err != nil {
			return err
		}
	}

	return runCommand(frontendDir, nil, filepath.Join(root, "logs", "frontend-build.log"), "npm", "run", "build")
}

func ensureRuntime(root string, cfg config.AppConfig, logDir string) error {
	runtimeURL := fmt.Sprintf("http://127.0.0.1:%d/health", cfg.RuntimePort)
	if healthOK(runtimeURL) {
		return nil
	}

	env := append(os.Environ(), fmt.Sprintf("LOCAL_AGENT_RUNTIME_PORT=%d", cfg.RuntimePort))
	logPath := filepath.Join(logDir, "runtime.log")
	if err := spawnRuntimeProcess(root, env, logPath); err != nil {
		return err
	}

	return waitForHealth(runtimeURL, 20*time.Second)
}

func ensureGateway(root string, cfg config.AppConfig, logDir string) error {
	gatewayURL := fmt.Sprintf("http://127.0.0.1:%d/health", cfg.GatewayPort)
	if healthOK(gatewayURL) {
		return nil
	}

	env := append(
		os.Environ(),
		fmt.Sprintf("LOCAL_AGENT_GATEWAY_PORT=%d", cfg.GatewayPort),
		fmt.Sprintf("LOCAL_AGENT_RUNTIME_PORT=%d", cfg.RuntimePort),
	)
	logPath := filepath.Join(logDir, "gateway.log")
	gatewayDir := filepath.Join(root, "gateway")
	if err := spawnGatewayProcess(root, gatewayDir, env, logPath); err != nil {
		return err
	}

	return waitForHealth(gatewayURL, 20*time.Second)
}

func spawnRuntimeProcess(root string, env []string, logPath string) error {
	binary := filepath.Join(root, "target", "debug", executableName("runtime-host"))
	if fileExists(binary) {
		return spawnProcess(root, env, logPath, binary)
	}
	return spawnProcess(root, env, logPath, "cargo", "run", "-p", "runtime-host")
}

func spawnGatewayProcess(root string, gatewayDir string, env []string, logPath string) error {
	binary := filepath.Join(root, "gateway", executableName("server"))
	if !fileExists(binary) || gatewayBuildStale(gatewayDir, binary) {
		if err := buildGateway(root, gatewayDir, env); err != nil {
			return err
		}
	}
	return spawnProcess(root, env, logPath, binary)
}

func buildGateway(root string, gatewayDir string, env []string) error {
	logPath := filepath.Join(root, "logs", "gateway-build.log")
	return runCommand(gatewayDir, env, logPath, "go", "build", "-o", executableName("server"), "./cmd/server")
}

func gatewayBuildStale(gatewayDir string, binary string) bool {
	binaryInfo, err := os.Stat(binary)
	if err != nil {
		return true
	}
	return hasNewerFile(filepath.Join(gatewayDir, "cmd"), binaryInfo.ModTime(), ".go") ||
		hasNewerFile(filepath.Join(gatewayDir, "internal"), binaryInfo.ModTime(), ".go")
}

func hasNewerFile(root string, builtAt time.Time, suffix string) bool {
	stale := false
	_ = filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if stale || err != nil || info == nil || info.IsDir() {
			return nil
		}
		stale = strings.HasSuffix(path, suffix) && info.ModTime().After(builtAt)
		return nil
	})
	return stale
}

func frontendBuildStale(root string, indexFile string) bool {
	indexInfo, err := os.Stat(indexFile)
	if err != nil {
		return true
	}
	builtAt := indexInfo.ModTime()

	checkPaths := []string{
		filepath.Join(root, "frontend", "index.html"),
		filepath.Join(root, "frontend", "package.json"),
		filepath.Join(root, "frontend", "package-lock.json"),
		filepath.Join(root, "frontend", "tsconfig.json"),
		filepath.Join(root, "frontend", "vite.config.ts"),
	}
	for _, path := range checkPaths {
		info, err := os.Stat(path)
		if err == nil && info.ModTime().After(builtAt) {
			return true
		}
	}

	srcRoot := filepath.Join(root, "frontend", "src")
	stale := false
	_ = filepath.Walk(srcRoot, func(path string, info os.FileInfo, err error) error {
		if stale || err != nil || info == nil || info.IsDir() {
			return nil
		}
		if info.ModTime().After(builtAt) {
			stale = true
		}
		return nil
	})

	return stale
}

func runCommand(workdir string, env []string, logPath string, name string, args ...string) error {
	logFile, err := os.OpenFile(logPath, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o644)
	if err != nil {
		return err
	}
	defer logFile.Close()

	cmd := exec.Command(name, args...)
	cmd.Dir = workdir
	if env != nil {
		cmd.Env = env
	}
	cmd.Stdout = logFile
	cmd.Stderr = logFile
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("%s %v failed: %w", name, args, err)
	}
	return nil
}

func spawnProcess(workdir string, env []string, logPath string, name string, args ...string) error {
	logFile, err := os.OpenFile(logPath, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0o644)
	if err != nil {
		return err
	}

	cmd := exec.Command(name, args...)
	cmd.Dir = workdir
	cmd.Env = env
	cmd.Stdout = logFile
	cmd.Stderr = logFile
	configureProcess(cmd)
	if err := cmd.Start(); err != nil {
		_ = logFile.Close()
		return fmt.Errorf("%s %v failed: %w", name, args, err)
	}

	go func() {
		_ = cmd.Wait()
		_ = logFile.Close()
	}()

	return nil
}

func configureProcess(cmd *exec.Cmd) {
	if runtime.GOOS != "windows" {
		return
	}
	const detachedFlags = 0x00000008 | 0x01000000 | 0x08000000
	cmd.SysProcAttr = &syscall.SysProcAttr{
		CreationFlags: detachedFlags | syscall.CREATE_NEW_PROCESS_GROUP,
		HideWindow:    true,
	}
}

func waitForHealth(url string, timeout time.Duration) error {
	deadline := time.Now().Add(timeout)
	for time.Now().Before(deadline) {
		if healthOK(url) {
			return nil
		}
		time.Sleep(500 * time.Millisecond)
	}
	return fmt.Errorf("health check timeout: %s", url)
}

func healthOK(url string) bool {
	client := http.Client{Timeout: time.Second}
	resp, err := client.Get(url)
	if err != nil {
		return false
	}
	defer resp.Body.Close()
	return resp.StatusCode == http.StatusOK
}

func systemReady(port int, repoRoot string) bool {
	info, ok := systemInfo(gatewayURL(port), repoRoot)
	return ok && info.matches(repoRoot)
}

func systemInfo(gatewayURL string, repoRoot string) (launcherSystemInfo, bool) {
	tok := readToken(repoRoot)
	req, err := http.NewRequest("GET", gatewayURL+"/api/v1/system/info", nil)
	if err != nil {
		return launcherSystemInfo{}, false
	}
	if tok != "" {
		req.Header.Set("X-Local-Agent-Token", tok)
	}
	client := http.Client{Timeout: time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return launcherSystemInfo{}, false
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		return launcherSystemInfo{}, false
	}
	var payload launcherSystemInfo
	if err := json.NewDecoder(resp.Body).Decode(&payload); err != nil {
		return launcherSystemInfo{}, false
	}
	return payload, true
}

func readToken(repoRoot string) string {
	path := filepath.Join(repoRoot, "data", ".gateway_token")
	raw, err := os.ReadFile(path)
	if err != nil {
		return ""
	}
	return strings.TrimSpace(string(raw))
}

func waitForSystemReady(port int, repoRoot string, timeout time.Duration) error {
	deadline := time.Now().Add(timeout)
	for time.Now().Before(deadline) {
		if systemReady(port, repoRoot) {
			return nil
		}
		time.Sleep(500 * time.Millisecond)
	}
	return fmt.Errorf("system info timeout: %s", gatewayURL(port))
}

func gatewayURL(port int) string {
	return fmt.Sprintf("http://127.0.0.1:%d", port)
}

type launcherSystemInfo struct {
	RepoRoot      string `json:"repo_root"`
	FormalEntry   string `json:"formal_entry"`
	RuntimeStatus struct {
		OK bool `json:"ok"`
	} `json:"runtime_status"`
}

func (info launcherSystemInfo) matches(repoRoot string) bool {
	return info.RepoRoot == repoRoot &&
		info.FormalEntry == "desktop launcher -> local web console" &&
		info.RuntimeStatus.OK
}

func openBrowser(url string) error {
	switch runtime.GOOS {
	case "windows":
		return exec.Command("rundll32", "url.dll,FileProtocolHandler", url).Start()
	case "darwin":
		return exec.Command("open", url).Start()
	default:
		return exec.Command("xdg-open", url).Start()
	}
}

func fileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func executableName(base string) string {
	if runtime.GOOS == "windows" {
		return base + ".exe"
	}
	return base
}
