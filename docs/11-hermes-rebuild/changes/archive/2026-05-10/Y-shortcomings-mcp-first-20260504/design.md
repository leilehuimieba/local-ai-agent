# 设计：短板补齐第一阶段

## 范围

本阶段按“最小闭环优先”处理 P0 的第一项：MCP 添加/删除持久化 + 热重载。

## 后端设计

1. `gateway/internal/config/config.go`
   - 增加 `ConfigPath`、`LoadFile`、`SaveFile`。
   - `Load` 继续用于启动时加载，并保留环境变量覆盖。
   - `LoadFile`/`SaveFile` 用于编辑磁盘配置，避免把环境变量覆盖值写回配置文件。
2. `gateway/internal/api/router_settings.go`
   - `POST /api/v1/settings` 新增 `add_mcp_name`、`add_mcp_url`、`remove_mcp_id`。
   - settings 更新仍先处理模式、模型、工作区和目录授权，再处理 MCP 配置。
3. `gateway/internal/api/mcp_settings.go`
   - 校验 MCP URL 只允许 `http`/`https`。
   - 拒绝重复 URL。
   - 自动生成稳定且唯一的 server id。
   - 写回 `config/app.json` 后调用 Manager 热替换。
4. `gateway/internal/mcp/manager.go`
   - 新增 `ReplaceServers`，整组替换 clients/servers 并执行 `ConnectAll`。

## 前端设计

1. `frontend/lib/local-agent/api.ts`
   - `SettingsUpdatePayload` 增加 MCP 添加/删除字段。
   - `MCPTool` 补 `server_id` 字段。
2. `frontend/components/local-agent/views/settings-view.tsx`
   - 添加 MCP Server 后重新加载 settings。
   - MCP Server 展开后提供删除按钮和确认弹窗。

## 风险与回退

1. 如果新增 Server 连接失败，只影响 Manager ready 状态，不阻断配置持久化。
2. 如果写配置失败，不执行 Manager 热替换。
3. 如热重载出现异常，可通过恢复 `config/app.json` 中 `mcp.servers` 并重启 Gateway 回退。

## 后续接口预留

1. Runtime tool registry 集成时读取 Gateway 聚合后的 MCP tool 列表。
2. allowlist/risk level 可扩展到 `MCPServerConfig` 或单独的 MCP policy store。
3. 审计字段应从 `/api/v1/mcp/call` 进入日志体系，并映射 server id、tool name、risk level、elapsed ms。

## Runtime MCP tool registry 切片 A：可见性注入

1. Gateway 在构造 `RunRequest` 时读取 `mcp.Manager.AllTools()`，只注入 ready server 的工具摘要。
2. 注入字段暂放 `context_hints`：
   - `mcp_tool_count`
   - `mcp_servers`
   - `mcp_tool_preview`
3. `mcp_tool_preview` 只包含 `server_id/tool_name` 与截断描述，默认最多 12 个工具，不注入完整 JSON schema。
4. Runtime 将 `mcp_tool_preview` 合并到动态上下文的 `tool_preview`，用于给模型展示 Gateway 聚合后的 MCP 工具摘要。
5. 本切片只解决工具可见性和上下文调度；自动执行闭环由切片 B 承接。

## Runtime MCP tool registry 切片 B：调用闭环

1. Gateway 侧：`RunRequest.context_hints` 注入 `mcp_tool_specs_json`，只包含 allowlisted 且 `requires_confirmation=false` 的 MCP tools。
2. 模型侧：Runtime 将 `mcp_tool_specs_json` 映射为 OpenAI-style tool schema，函数名规范为 `mcp__{server_id}__{tool_name}`。
3. 解码侧：`action_decode` 识别 `mcp__` 前缀，转换为 `PlannedAction::MCPCall`，并从 wrapper `arguments` 中提取实际 MCP 参数。
4. 执行侧：Runtime 通过 Gateway `/api/v1/mcp/call` 调用工具，使用 `mcp_gateway_url` 与 `mcp_gateway_token` 作为内部桥接配置。
5. 安全侧：Runtime 不直接连接 MCP server，不绕过 Gateway；Gateway 继续执行 allowlist、risk、requires confirmation 与审计。
6. 失败语义：缺少桥接配置返回 `mcp_bridge_not_configured`；Gateway 拒绝 allowlist 或 confirmation 时返回对应错误文本；Runtime 将失败写入 tool trace。
7. 大结果：MCP 原始输出外置为 `mcp-raw-output` artifact，事件流只保留摘要、预览和 artifact 引用。

## MCP allowlist / risk / audit 策略

1. 配置层：`MCPServerConfig` 增加 `tool_policies`，每个工具策略包含：
   - `tool_name`
   - `allowed`
   - `risk_level`
   - `requires_confirmation`
   - `audit_enabled`
2. 默认规则：未配置策略的 MCP 工具一律 `allowed=false`、`risk_level=medium`、`requires_confirmation=true`、`audit_enabled=true`。
3. 可见性规则：`/api/v1/mcp/tools` 返回所有 ready tools 及策略字段；Runtime `context_hints` 只注入 allowlisted tools。
4. 调用规则：`/api/v1/mcp/call` 先执行 allowlist 检查；未 allowlisted 的工具拒绝调用；标记 requires confirmation 的工具在当前直接调用入口拒绝执行。
5. 审计规则：每次 MCP 调用成功或拒绝都会写入 `logs/mcp-audit.jsonl`，记录 server id、tool name、risk level、policy source、arguments hash、elapsed ms、outcome、error code。
6. 前端规则：设置页 MCP 工具行展示 allowed/blocked、risk、audit 标记，并支持调整 allowlist、risk level 与 confirmation 开关。

## P1 后续切片候选

### Aider/Codex 风格 diff apply

1. 目标：补齐“生成 diff -> 预览 -> 应用 -> 回滚/验收”的代码修改闭环。
2. 首个 change 建议只做 unified diff parser、路径边界校验、dry-run 预览和单文件 apply。
3. 权限边界：写入动作走现有 workspace 授权和 confirmation 主线，不允许 diff 越过工作区根目录。
4. 验证：用 fixture 覆盖新增、修改、删除、冲突、路径逃逸、CRLF/LF 保持。

### Windows doctor + 一键启动器 + 服务状态自检

1. 目标：把 Windows 原生体验从“可启动”推进到“可诊断、可恢复、可提示”。
2. 首个 change 建议落到 `gateway/cmd/launcher` 和独立 doctor route，检查端口占用、runtime/gateway 可达性、token 文件、config/app.json、Node/Rust/Go 构建依赖。
3. 输出形态：CLI/启动器返回结构化健康项，前端设置页只展示状态和修复建议。
4. 验证：构造缺 token、端口占用、runtime 未启动、config 缺字段等 fixture，并保留 Windows 命令输出样本。
