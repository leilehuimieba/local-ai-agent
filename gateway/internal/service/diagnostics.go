package service

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"time"

	"local-agent/gateway/internal/util"
)

type logsRemediationState struct {
	LogsDir  string `json:"logs_dir"`
	Exists   bool   `json:"exists"`
	IsDir    bool   `json:"is_dir"`
	Writable bool   `json:"writable"`
}

type logsRemediationResponse struct {
	OK            bool                 `json:"ok"`
	RemediationID string               `json:"remediation_id"`
	Strategy      string               `json:"strategy"`
	Scope         string               `json:"scope"`
	Before        logsRemediationState `json:"before"`
	After         logsRemediationState `json:"after"`
	Actions       []string             `json:"actions"`
	NextStep      string               `json:"next_step"`
	ManualGuide   string               `json:"manual_guide"`
}

type frontendRemediationState struct {
	FrontendDir       string `json:"frontend_dir"`
	DistIndex         string `json:"dist_index"`
	Exists            bool   `json:"exists"`
	PackageJSONExists bool   `json:"package_json_exists"`
	NodeModulesExists bool   `json:"node_modules_exists"`
	BuildReady        bool   `json:"build_ready"`
}

type frontendRemediationResponse struct {
	OK            bool                     `json:"ok"`
	RemediationID string                   `json:"remediation_id"`
	Strategy      string                   `json:"strategy"`
	Scope         string                   `json:"scope"`
	Before        frontendRemediationState `json:"before"`
	After         frontendRemediationState `json:"after"`
	Actions       []string                 `json:"actions"`
	NextStep      string                   `json:"next_step"`
	ManualGuide   string                   `json:"manual_guide"`
}

type gatewayRemediationState struct {
	GatewayDir  string `json:"gateway_dir"`
	EntryPath   string `json:"entry_path"`
	HealthURL   string `json:"health_url"`
	SourceReady bool   `json:"source_ready"`
	GoAvailable bool   `json:"go_available"`
	Reachable   bool   `json:"reachable"`
}

type gatewayRemediationResponse struct {
	OK            bool                    `json:"ok"`
	RemediationID string                  `json:"remediation_id"`
	Strategy      string                  `json:"strategy"`
	Scope         string                  `json:"scope"`
	Before        gatewayRemediationState `json:"before"`
	After         gatewayRemediationState `json:"after"`
	Actions       []string                `json:"actions"`
	NextStep      string                  `json:"next_step"`
	ManualGuide   string                  `json:"manual_guide"`
}

type configRemediationState struct {
	ConfigPath      string   `json:"config_path"`
	Exists          bool     `json:"exists"`
	ValidJSON       bool     `json:"valid_json"`
	MissingFields   []string `json:"missing_fields"`
	RecommendedOnly bool     `json:"recommended_only"`
}

type configRemediationResponse struct {
	OK            bool                   `json:"ok"`
	RemediationID string                 `json:"remediation_id"`
	Strategy      string                 `json:"strategy"`
	Scope         string                 `json:"scope"`
	Before        configRemediationState `json:"before"`
	After         configRemediationState `json:"after"`
	Actions       []string               `json:"actions"`
	NextStep      string                 `json:"next_step"`
	ManualGuide   string                 `json:"manual_guide"`
}

var frontendBuildRunner = func(frontendDir string) error {
	cmd := exec.Command("npm", "run", "build")
	cmd.Dir = frontendDir
	return cmd.Run()
}

var gatewayHealthChecker = func(port int) bool {
	client := http.Client{Timeout: time.Second}
	resp, err := client.Get(fmt.Sprintf("http://127.0.0.1:%d/health", port))
	if err != nil {
		return false
	}
	defer resp.Body.Close()
	return resp.StatusCode == http.StatusOK
}

var gatewayStartRunner = func(repoRoot string, port int) error {
	cmd := exec.Command("go", "run", "./cmd/server")
	cmd.Dir = filepath.Join(repoRoot, "gateway")
	cmd.Env = append(os.Environ(), fmt.Sprintf("LOCAL_AGENT_GATEWAY_PORT=%d", port))
	return cmd.Start()
}
func RemediateLogsWritable(repoRoot string) logsRemediationResponse {
	before := ReadLogsRemediationState(repoRoot)
	after, actions, nextStep, ok := ApplyLogsRemediation(repoRoot, before)
	return logsRemediationResponse{
		OK: ok, RemediationID: "H02-S04", Strategy: "low_risk_auto_fix", Scope: "logs_writable",
		Before: before, After: after, Actions: actions, NextStep: nextStep,
		ManualGuide: filepath.ToSlash(filepath.Join(repoRoot, "tmp", "stage-h-remediation", "manual-guides", "logs-not-writable.md")),
	}
}

func RemediateFrontendDist(repoRoot string) frontendRemediationResponse {
	before := ReadFrontendRemediationState(repoRoot)
	after, actions, nextStep, ok := ApplyFrontendRemediation(repoRoot, before)
	return frontendRemediationResponse{
		OK: ok, RemediationID: "H02-S03", Strategy: "low_risk_auto_fix", Scope: "frontend_dist",
		Before: before, After: after, Actions: actions, NextStep: nextStep,
		ManualGuide: filepath.ToSlash(filepath.Join(repoRoot, "tmp", "stage-h-remediation", "manual-guides", "frontend-dist-missing.md")),
	}
}

func RemediateGatewayUnreachable(repoRoot string, port int) gatewayRemediationResponse {
	before := ReadGatewayRemediationState(repoRoot, port)
	after, actions, nextStep, ok := ApplyGatewayRemediation(repoRoot, port, before)
	return gatewayRemediationResponse{
		OK: ok, RemediationID: "H02-S02", Strategy: "low_risk_auto_fix", Scope: "gateway_unreachable",
		Before: before, After: after, Actions: actions, NextStep: nextStep,
		ManualGuide: filepath.ToSlash(filepath.Join(repoRoot, "tmp", "stage-h-remediation", "manual-guides", "gateway-unreachable.md")),
	}
}

func InspectConfigRemediation(repoRoot string) configRemediationResponse {
	before := ReadConfigRemediationState(repoRoot)
	after, actions, nextStep, ok, strategy := ApplyConfigRemediation(repoRoot, before)
	return configRemediationResponse{
		OK: ok, RemediationID: "H02-S05", Strategy: strategy, Scope: "config_missing_or_invalid",
		Before: before, After: after, Actions: actions, NextStep: nextStep,
		ManualGuide: filepath.ToSlash(filepath.Join(repoRoot, "tmp", "stage-h-remediation", "manual-guides", "config-missing-or-invalid.md")),
	}
}

func ReadLogsRemediationState(repoRoot string) logsRemediationState {
	logsDir := filepath.Join(repoRoot, "logs")
	info, err := os.Stat(logsDir)
	if err != nil {
		return logsRemediationState{LogsDir: logsDir}
	}
	return logsRemediationState{
		LogsDir: logsDir, Exists: true, IsDir: info.IsDir(),
		Writable: info.IsDir() && ProbeLogsWritable(logsDir),
	}
}

func ReadFrontendRemediationState(repoRoot string) frontendRemediationState {
	frontendDir := filepath.Join(repoRoot, "frontend")
	distIndex := filepath.Join(frontendDir, "dist", "index.html")
	packageJSON := filepath.Join(frontendDir, "package.json")
	nodeModules := filepath.Join(frontendDir, "node_modules")
	return frontendRemediationState{
		FrontendDir: frontendDir, DistIndex: distIndex, Exists: util.PathExists(distIndex),
		PackageJSONExists: util.PathExists(packageJSON), NodeModulesExists: util.PathExists(nodeModules),
		BuildReady: util.PathExists(packageJSON) && util.PathExists(nodeModules),
	}
}

func ReadGatewayRemediationState(repoRoot string, port int) gatewayRemediationState {
	gatewayDir := filepath.Join(repoRoot, "gateway")
	entryPath := filepath.Join(gatewayDir, "cmd", "server", "main.go")
	return gatewayRemediationState{
		GatewayDir: gatewayDir, EntryPath: entryPath, HealthURL: fmt.Sprintf("http://127.0.0.1:%d/health", port),
		SourceReady: util.PathExists(entryPath), GoAvailable: ToolAvailable("go"), Reachable: gatewayHealthChecker(port),
	}
}

func ReadConfigRemediationState(repoRoot string) configRemediationState {
	path := filepath.Join(repoRoot, "config", "app.json")
	state := configRemediationState{ConfigPath: path, RecommendedOnly: true}
	data, err := os.ReadFile(path)
	if err != nil {
		return state
	}
	state.Exists = true
	var payload map[string]any
	if err := json.Unmarshal(data, &payload); err != nil {
		return state
	}
	state.ValidJSON = true
	state.MissingFields = MissingConfigFields(payload)
	return state
}

func ApplyLogsRemediation(
	repoRoot string,
	before logsRemediationState,
) (logsRemediationState, []string, string, bool) {
	if before.Writable {
		return before, []string{"logs_already_writable"}, "logs 已可写，无需修复。", true
	}
	err := EnsureLogsDirectory(before.LogsDir)
	if err != nil {
		next := "logs 路径不可自动修复，请按手动接管指引处理。"
		return ReadLogsRemediationState(repoRoot), []string{"manual_takeover_required"}, next, false
	}
	after := ReadLogsRemediationState(repoRoot)
	if after.Writable {
		return after, []string{"create_logs_dir", "write_probe_passed"}, "修复完成，请重新执行 diagnostics check。", true
	}
	return after, []string{"write_probe_failed"}, "logs 仍不可写，请转人工接管。", false
}

func ApplyFrontendRemediation(
	repoRoot string,
	before frontendRemediationState,
) (frontendRemediationState, []string, string, bool) {
	if before.Exists {
		return before, []string{"frontend_dist_already_exists"}, "frontend dist 已存在，无需修复。", true
	}
	if !before.BuildReady {
		next := "frontend 构建前置条件不足，请按手动接管指引处理。"
		return ReadFrontendRemediationState(repoRoot), []string{"manual_takeover_required"}, next, false
	}
	if err := frontendBuildRunner(before.FrontendDir); err != nil {
		next := "frontend 构建失败，请按手动接管指引处理。"
		return ReadFrontendRemediationState(repoRoot), []string{"build_failed_manual_takeover"}, next, false
	}
	after := ReadFrontendRemediationState(repoRoot)
	if after.Exists {
		return after, []string{"npm_build_passed"}, "构建完成，请重新执行 diagnostics check。", true
	}
	return after, []string{"dist_still_missing"}, "frontend dist 仍缺失，请转人工接管。", false
}

func ApplyGatewayRemediation(
	repoRoot string,
	port int,
	before gatewayRemediationState,
) (gatewayRemediationState, []string, string, bool) {
	if before.Reachable {
		return before, []string{"gateway_already_reachable"}, "gateway 已可达，无需修复。", true
	}
	if !before.SourceReady || !before.GoAvailable {
		next := "gateway 缺少启动前置条件，请按手动接管指引处理。"
		return ReadGatewayRemediationState(repoRoot, port), []string{"manual_takeover_required"}, next, false
	}
	if err := gatewayStartRunner(repoRoot, port); err != nil {
		next := "gateway 启动失败，请按手动接管指引处理。"
		return ReadGatewayRemediationState(repoRoot, port), []string{"start_failed_manual_takeover"}, next, false
	}
	after := WaitGatewayRemediationState(repoRoot, port)
	if after.Reachable {
		return after, []string{"gateway_restart_attempted", "gateway_health_restored"}, "gateway 已恢复，请重新执行 diagnostics check。", true
	}
	return after, []string{"gateway_health_still_failed"}, "gateway 仍不可达，请转人工接管。", false
}

func EnsureLogsDirectory(logsDir string) error {
	info, err := os.Stat(logsDir)
	if err == nil && !info.IsDir() {
		return fmt.Errorf("logs path is file")
	}
	if err == nil {
		return nil
	}
	return os.MkdirAll(logsDir, 0o755)
}

func ProbeLogsWritable(logsDir string) bool {
	probe := filepath.Join(logsDir, fmt.Sprintf("h02-write-%d.tmp", time.Now().UnixNano()))
	if err := os.WriteFile(probe, []byte("ok"), 0o644); err != nil {
		return false
	}
	_ = os.Remove(probe)
	return true
}

func ToolAvailable(name string) bool {
	_, err := exec.LookPath(name)
	return err == nil
}

func WaitGatewayRemediationState(repoRoot string, port int) gatewayRemediationState {
	for attempt := 0; attempt < 5; attempt++ {
		state := ReadGatewayRemediationState(repoRoot, port)
		if state.Reachable {
			return state
		}
		time.Sleep(200 * time.Millisecond)
	}
	return ReadGatewayRemediationState(repoRoot, port)
}

func MissingConfigFields(payload map[string]any) []string {
	required := []string{"app_name", "gateway_port", "runtime_port", "default_workspace"}
	missing := make([]string, 0, len(required))
	for _, key := range required {
		if !ConfigFieldPresent(payload[key]) {
			missing = append(missing, key)
		}
	}
	return missing
}

func ConfigFieldPresent(value any) bool {
	if value == nil {
		return false
	}
	switch item := value.(type) {
	case string:
		return item != ""
	default:
		return true
	}
}

func ApplyConfigRemediation(
	repoRoot string,
	before configRemediationState,
) (configRemediationState, []string, string, bool, string) {
	if !before.Exists || !before.ValidJSON {
		actions := ConfigActions(before)
		return before, actions, ConfigNextStep(before), false, "read_only_validation"
	}
	after, changed, writeOK := TryConfigSafeDefaults(repoRoot, before)
	if changed && writeOK && len(after.MissingFields) == 0 {
		actions := []string{"config_auto_write_defaults", "config_recheck_passed"}
		return after, actions, "配置缺失字段已自动补齐，请重新执行 diagnostics check。", true, "low_risk_auto_fix"
	}
	if changed {
		actions := append([]string{"config_auto_write_partial"}, after.MissingFields...)
		nextStep := fmt.Sprintf("配置仍缺字段：%v，请按手动接管指引处理。", after.MissingFields)
		return after, actions, nextStep, false, "low_risk_auto_fix"
	}
	actions := ConfigActions(before)
	return before, actions, ConfigNextStep(before), len(before.MissingFields) == 0, "read_only_validation"
}

func TryConfigSafeDefaults(
	repoRoot string,
	before configRemediationState,
) (configRemediationState, bool, bool) {
	path := filepath.Join(repoRoot, "config", "app.json")
	data, err := os.ReadFile(path)
	if err != nil {
		return before, false, false
	}
	updated, changed, ok := ApplySafeConfigDefaults(data)
	if !ok {
		return before, false, false
	}
	if changed {
		if err := os.WriteFile(path, updated, 0o644); err != nil {
			return ReadConfigRemediationState(repoRoot), false, false
		}
	}
	return ReadConfigRemediationState(repoRoot), changed, true
}

func ApplySafeConfigDefaults(data []byte) ([]byte, bool, bool) {
	var payload map[string]any
	if err := json.Unmarshal(data, &payload); err != nil {
		return nil, false, false
	}
	changed := false
	if !ConfigFieldPresent(payload["runtime_port"]) {
		payload["runtime_port"] = 8898
		changed = true
	}
	if !ConfigFieldPresent(payload["default_workspace"]) {
		payload["default_workspace"] = map[string]any{"workspace_id": "main"}
		changed = true
	}
	updated, err := json.MarshalIndent(payload, "", "  ")
	if err != nil {
		return nil, false, false
	}
	return updated, changed, true
}

func ConfigActions(state configRemediationState) []string {
	if !state.Exists {
		return []string{"manual_takeover_required", "config_missing"}
	}
	if !state.ValidJSON {
		return []string{"manual_takeover_required", "config_invalid_json"}
	}
	if len(state.MissingFields) > 0 {
		return append([]string{"manual_takeover_required", "missing_required_fields"}, state.MissingFields...)
	}
	return []string{"config_read_only_check_passed"}
}

func ConfigNextStep(state configRemediationState) string {
	if !state.Exists {
		return "配置文件缺失，请按手动接管指引恢复 config/app.json。"
	}
	if !state.ValidJSON {
		return "配置文件不是有效 JSON，请按手动接管指引修正格式。"
	}
	if len(state.MissingFields) > 0 {
		return fmt.Sprintf("配置缺少字段：%v，请按手动接管指引补齐。", state.MissingFields)
	}
	return "配置校验通过，无需修复。"
}
