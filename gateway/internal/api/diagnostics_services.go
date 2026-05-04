package api

import (
	"fmt"
	"os"
	"path/filepath"

	"local-agent/gateway/internal/service"
	"local-agent/gateway/internal/util"
)

func buildServiceStatuses(repoRoot string, settings SettingsResponse) []ServiceStatus {
	statuses := []ServiceStatus{
		okService("gateway", "Gateway", fmt.Sprintf("localhost:%d", settings.Ports["gateway"])),
		runtimeService(settings.RuntimeStatus),
		mcpService(settings.MCP),
		providerService(settings.Diagnostics.ProviderCount, settings.Diagnostics.ModelCount),
		sessionService(filepath.Join(repoRoot, "data", "storage")),
		knowledgeService(settings.Diagnostics.KnowledgeBasePathExists),
		logsService(repoRoot),
		frontendService(repoRoot),
	}
	return statuses
}

func okService(id string, label string, detail string) ServiceStatus {
	return ServiceStatus{ID: id, Label: label, Status: "ok", Severity: "info", Detail: detail, Hint: "状态正常。"}
}

func warnService(id string, label string, detail string, hint string) ServiceStatus {
	return ServiceStatus{ID: id, Label: label, Status: "warning", Severity: "warning", Detail: detail, Hint: hint}
}

func errorService(id string, label string, detail string, hint string) ServiceStatus {
	return ServiceStatus{ID: id, Label: label, Status: "error", Severity: "error", Detail: detail, Hint: hint}
}

func runtimeService(status RuntimeStatus) ServiceStatus {
	if status.OK {
		return okService("runtime", "Runtime", fmt.Sprintf("%s / %s", status.Name, status.Version))
	}
	return errorService("runtime", "Runtime", status.Version, "请通过一键启动器重启 Runtime，并检查 logs/runtime.log。")
}

func mcpService(info MCPInfo) ServiceStatus {
	total, ready := len(info.Servers), readyMCPServers(info.Servers)
	if total == 0 {
		return warnService("mcp", "MCP", "未配置 MCP Server", "可在设置页添加 MCP HTTP server。")
	}
	if ready == total {
		return okService("mcp", "MCP", fmt.Sprintf("%d/%d 个 server 可用", ready, total))
	}
	return warnService("mcp", "MCP", fmt.Sprintf("%d/%d 个 server 可用", ready, total), "检查不可用 MCP server 的 URL 和 allowlist 配置。")
}

func readyMCPServers(items []MCPServerStatus) int {
	ready := 0
	for _, item := range items {
		if item.Enabled && item.Ready {
			ready++
		}
	}
	return ready
}

func providerService(providerCount int, modelCount int) ServiceStatus {
	if providerCount > 0 && modelCount > 0 {
		return okService("providers", "模型服务商", fmt.Sprintf("%d 个服务商 / %d 个模型", providerCount, modelCount))
	}
	return errorService("providers", "模型服务商", "未配置可用模型服务商", "请先配置模型服务商和默认模型。")
}

func sessionService(path string) ServiceStatus {
	if probeDirWritable(path) {
		return okService("sessions", "会话存储", filepath.ToSlash(path))
	}
	return errorService("sessions", "会话存储", filepath.ToSlash(path), "请确认 data/storage 可创建并可写。")
}

func knowledgeService(exists bool) ServiceStatus {
	if exists {
		return okService("knowledge", "知识库", "knowledge_base 目录可访问")
	}
	return warnService("knowledge", "知识库", "knowledge_base 目录尚未生成", "首次上传知识文件后会自动生成。")
}

func logsService(repoRoot string) ServiceStatus {
	path := filepath.Join(repoRoot, "logs")
	if service.ProbeLogsWritable(path) {
		return okService("logs", "日志目录", filepath.ToSlash(path))
	}
	return errorService("logs", "日志目录", filepath.ToSlash(path), "请确认 logs 目录存在且当前用户可写。")
}

func frontendService(repoRoot string) ServiceStatus {
	path := filepath.Join(repoRoot, "frontend", "dist", "index.html")
	if util.PathExists(path) {
		return okService("frontend", "前端资源", "frontend/dist/index.html")
	}
	return warnService("frontend", "前端资源", "frontend/dist/index.html 不存在", "运行一键启动器或 npm run build 生成前端资源。")
}

func probeDirWritable(path string) bool {
	if err := os.MkdirAll(path, 0o755); err != nil {
		return false
	}
	probe := filepath.Join(path, ".doctor-write-probe")
	if err := os.WriteFile(probe, []byte("ok"), 0o644); err != nil {
		return false
	}
	_ = os.Remove(probe)
	return true
}
