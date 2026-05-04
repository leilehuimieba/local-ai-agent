# 设计：Windows doctor 与服务状态自检

## 范围

本 change 覆盖 Gateway 诊断 API、launcher 启动流程与前端设置页状态面板。目标是形成第一版可用的本地诊断闭环。

## Gateway 设计

1. 在现有 diagnostics 能力上增加统一服务状态响应。
2. 状态项包含 `id`、`label`、`status`、`severity`、`detail`、`hint`。
3. 状态类别包含 Gateway、Runtime、MCP、Provider、Session store、Knowledge base、Logs、Frontend dist。
4. 复用现有 `buildSettingsResponse`、Runtime health、MCP manager 和 provider/settings 状态。
5. API 保持在 `/api/v1/diagnostics/check` 或新增轻量读取入口，继续走 Gateway Token 认证。

## Launcher 设计

1. 启动流程改为 preflight -> start -> verify。
2. preflight 检查 repo root、config、logs 目录、端口占用和前端构建状态。
3. start 阶段只启动未就绪的 Runtime 与 Gateway。
4. verify 阶段读取 system info 或 diagnostics check，确认 Gateway 与 Runtime 已就绪。
5. 失败时打印失败阶段、日志路径和建议动作。

## Frontend 设计

1. 在设置页诊断区域增加服务状态面板。
2. 使用紧凑列表展示服务名称、状态、说明和建议。
3. 支持手动刷新诊断。
4. 状态颜色只表达健康程度，不替代文字说明。
5. 保持现有设置页布局，不新建独立页面。

## 后续队列

1. `AB-diff-preview-ui`：展示 diff dry-run report，支持用户确认后 apply。
2. `AC-mcp-observability-ui`：MCP 工具风险、allowlist 和审计记录的可运营界面。

## 风险与回退

1. 若 doctor 覆盖面过宽导致误报，先以 warning 呈现，不阻断启动。
2. 若前端状态面板接口异常，保留现有诊断按钮。
3. 若 launcher 验活失败，保留日志路径和手动启动命令提示。
