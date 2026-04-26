package api

import (
	"bytes"
	"encoding/json"
	"errors"
	"net/http"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

type releaseCommandRequest struct {
	Step string `json:"step"`
}

type releaseCommandResponse struct {
	Step       string `json:"step"`
	Command    string `json:"command"`
	Artifact   string `json:"artifact"`
	ExitCode   int    `json:"exit_code"`
	Status     string `json:"status"`
	DurationMS int64  `json:"duration_ms"`
	Stdout     string `json:"stdout"`
	Stderr     string `json:"stderr"`
}

type releaseCommandSpec struct {
	Script   string
	Args     []string
	Artifact string
}

func registerReleaseRoutes(mux *http.ServeMux, repoRoot string) {
	mux.HandleFunc("/api/v1/release/run", releaseRunHandler(repoRoot))
}

func releaseRunHandler(repoRoot string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		response, err := runReleaseStep(repoRoot, r)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		writeReleaseResponse(w, response)
	}
}

func writeReleaseResponse(w http.ResponseWriter, response releaseCommandResponse) {
	status := http.StatusOK
	if response.Status != "passed" {
		status = http.StatusInternalServerError
	}
	writeJSON(w, status, response)
}

func runReleaseStep(repoRoot string, r *http.Request) (releaseCommandResponse, error) {
	var payload releaseCommandRequest
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		return releaseCommandResponse{}, errors.New("invalid json body")
	}
	spec, err := releaseSpec(repoRoot, payload.Step)
	if err != nil {
		return releaseCommandResponse{}, err
	}
	return executeReleaseSpec(repoRoot, payload.Step, spec)
}

func releaseSpec(repoRoot string, step string) (releaseCommandSpec, error) {
	out := filepath.Join("tmp", "release-wizard", step+".json")
	specs := releaseSpecs(repoRoot, out)
	spec, ok := specs[step]
	if !ok {
		return releaseCommandSpec{}, errors.New("unsupported release step")
	}
	return spec, nil
}

func releaseSpecs(repoRoot string, out string) map[string]releaseCommandSpec {
	installRoot := filepath.Join(repoRoot, "tmp", "release-wizard-install")
	return map[string]releaseCommandSpec{
		"prelaunch": {"run-full-regression.ps1", []string{"-OutFile", out}, out},
		"package":   {"install-local-agent.ps1", []string{"-InstallRoot", installRoot}, installRoot},
		"doctor":    {"doctor.ps1", []string{"-RepoRoot", repoRoot, "-OutFile", out}, out},
		"rc":        {"run-stage-f-rc-acceptance.ps1", []string{"-Rounds", "1", "-RequirePass"}, filepath.Join("tmp", "stage-f-rc", "latest.json")},
	}
}

func executeReleaseSpec(repoRoot string, step string, spec releaseCommandSpec) (releaseCommandResponse, error) {
	started := time.Now()
	stdout, stderr, code := runPowerShellScript(repoRoot, spec)
	return releaseCommandResponse{
		Step:       step,
		Command:    renderReleaseCommand(spec),
		Artifact:   spec.Artifact,
		ExitCode:   code,
		Status:     readReleaseStatus(code),
		DurationMS: time.Since(started).Milliseconds(),
		Stdout:     trimCommandOutput(stdout),
		Stderr:     trimCommandOutput(stderr),
	}, nil
}

func runPowerShellScript(repoRoot string, spec releaseCommandSpec) (string, string, int) {
	args := releasePowerShellArgs(repoRoot, spec)
	cmd := exec.Command("powershell.exe", args...)
	cmd.Dir = repoRoot
	var stdout bytes.Buffer
	var stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err := cmd.Run()
	if err == nil {
		return stdout.String(), stderr.String(), 0
	}
	return stdout.String(), stderr.String(), exitCode(err)
}

func releasePowerShellArgs(repoRoot string, spec releaseCommandSpec) []string {
	script := filepath.Join(repoRoot, "scripts", spec.Script)
	args := []string{"-NoProfile", "-ExecutionPolicy", "Bypass", "-File", script}
	return append(args, spec.Args...)
}

func exitCode(err error) int {
	if exitErr, ok := err.(*exec.ExitError); ok {
		return exitErr.ExitCode()
	}
	return 1
}

func renderReleaseCommand(spec releaseCommandSpec) string {
	parts := append([]string{spec.Script}, spec.Args...)
	return strings.Join(parts, " ")
}

func readReleaseStatus(code int) string {
	if code == 0 {
		return "passed"
	}
	return "failed"
}

func trimCommandOutput(value string) string {
	text := strings.TrimSpace(value)
	if len(text) <= 4000 {
		return text
	}
	return text[len(text)-4000:]
}
