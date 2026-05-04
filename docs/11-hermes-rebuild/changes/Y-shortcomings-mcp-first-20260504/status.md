# 状态记录

更新时间：2026-05-04

## 当前状态

已收口。

## 已完成

1. 新建短板补齐 change 工作区。
2. 后端已增加 `config/app.json` 编辑用读写函数。
3. Settings API 已支持 MCP 添加/删除字段。
4. MCP Manager 已支持服务器列表热替换。
5. 设置页已支持添加后刷新和展开后删除服务器。
6. 已补 MCP settings 后端单元测试。
7. 已修正 Vitest 配置，避免单测误加载 Playwright E2E。
8. Gateway 构建、MCP 相关 Go 测试、前端类型检查、单测、构建和 Playwright E2E 已通过。
9. 已用本地 fixture 收敛 learning evidence 测试外部网络依赖，`go test ./...` 已恢复通过。
10. Gateway 已把 ready MCP tools 注入 `RunRequest.context_hints`，Runtime 已在工具预览与 AgentResolve prompt 中展示 MCP 工具摘要。
11. MCP 工具已纳入默认 deny 的 allowlist 策略，工具清单、调用入口和前端设置页均展示 risk/confirmation/audit 字段。
12. `/api/v1/mcp/call` 已接入 allowlist 校验与 `logs/mcp-audit.jsonl` 审计记录。
13. 已修复 Runtime memory 测试夹具临时目录碰撞导致的并发抖动。
14. Runtime 已将可执行 MCP specs 注入模型 tool schema，支持 `mcp__{server_id}__{tool_name}` tool_call 解码。
15. Runtime 已新增 MCP executor，通过 Gateway `/api/v1/mcp/call` 执行 MCP 工具，并复用 Gateway allowlist/risk/audit。
16. 已将 diff apply 与 Windows doctor/启动器拆为 P1 后续独立 change 候选。

## 进行中

1. 已切换到 P1 diff apply 独立 change。

## 阻塞点

1. 暂无阻塞。

## 下一步

1. 后续如接真实第三方 MCP server，可补一轮人工验收记录。
