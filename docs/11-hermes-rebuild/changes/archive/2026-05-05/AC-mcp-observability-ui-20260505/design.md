# 技术方案

## 影响范围

- 涉及模块：`frontend/components/local-agent/views/settings-view.tsx`、`frontend/lib/local-agent/types.ts`、`frontend/lib/local-agent/api.ts`、必要时的 `gateway/internal/api/*` 只读字段。
- 涉及文档或 contract：`docs/frontend-acceptance.md`、`docs/11-hermes-rebuild/changes/INDEX.md`、`docs/11-hermes-rebuild/current-state.md`。

## 方案

- 核心做法：在设置页补一个只读的 MCP 可观测面板，优先展示工具名称、服务器来源、风险等级、allowlist 命中、审计状态和最近动作。
- 状态流转或调用链变化：优先复用现有 settings / mcp API；如果字段不足，只补最小只读字段，不改执行链、不改工具路由。

## 字段盘点结论

### 已可直接复用的字段

1. `server_id / name / url / ready / enabled / tool_count / allowed_tool_count / blocked_tool_count / requires_policy`
   来源：`GET /api/v1/settings` 的 `mcp.servers`，由 `gateway/internal/api/settings_response.go` -> `buildMCPInfo` 返回。
2. `tool.name / description / inputSchema / server_id / allowed / risk_level / requires_confirmation / audit_enabled / policy_source`
   来源：`GET /api/v1/settings` 的 `mcp.tools`，底层是 `mcp.Manager.AllTools()` 和 `mcp.Tool`。
3. 工具策略修改入口
   来源：`POST /api/v1/settings`，支持 `mcp_policy_server_id / mcp_policy_tool_name / mcp_policy_allowed / mcp_policy_risk_level / mcp_policy_requires_confirmation`。
4. 设置页现有基础落点
   来源：`frontend/components/local-agent/views/settings-view.tsx` 中现有 `MCPSection / MCPServerCard / MCPToolRow`。

### 现阶段不够的字段

1. `按服务器或工具筛选的审计列表`
   现状：已补 `GET /api/v1/mcp/audits`，支持 `limit / server_id / tool_name` 三个最小查询参数。
2. `最近动作 / 最近调用时间 / 最近结果`
   现状：已通过只读审计 API 暴露给前端，可展示最近调用结果与耗时摘要。
3. `audit_id / outcome / error_code / elapsed_ms / arguments_hash`
   现状：已在只读接口中可读，但前端当前只展示 `outcome / error_message / elapsed_ms / timestamp`，继续避免暴露参数正文。

## 页面展示项与来源矩阵

1. 工具名
   页面字段：`tool.name`
   来源：`mcp.tools[].name`
   结论：已具备
2. 服务器归属
   页面字段：`tool.server_id` + `server.name`
   来源：`mcp.tools[].server_id` + `mcp.servers[]`
   结论：已具备
3. 风险等级
   页面字段：`tool.risk_level`
   来源：`mcp.tools[].risk_level`
   结论：已具备
4. allowlist 命中状态
   页面字段：`tool.allowed`
   来源：`mcp.tools[].allowed`
   结论：已具备
5. 是否需确认
   页面字段：`tool.requires_confirmation`
   来源：`mcp.tools[].requires_confirmation`
   结论：已具备
6. 审计开关
   页面字段：`tool.audit_enabled`
   来源：`mcp.tools[].audit_enabled`
   结论：已具备
7. 策略来源
   页面字段：`tool.policy_source`
   来源：`mcp.tools[].policy_source`
   结论：已具备
8. 服务器汇总状态
   页面字段：`ready / tool_count / allowed_tool_count / blocked_tool_count / requires_policy`
   来源：`mcp.servers[]`
   结论：已具备
9. 最近动作
   页面字段：最近一次调用时间、结果、耗时
   来源：`GET /api/v1/mcp/audits`
   结论：已具备
10. 审计明细
    页面字段：`audit_id / outcome / error_code / elapsed_ms / arguments_hash`
    来源：`GET /api/v1/mcp/audits`
    结论：后端已具备，前端当前只展示安全摘要

## 本轮结论

1. AC 当前已经具备两段能力：基于 `settings.mcp` 的只读运营面板，以及基于 `mcp-audit.jsonl` 的最近动作卡片。
2. 审计接口目前只返回安全摘要，不回传原始参数正文，保持“可观测”而不是“泄漏更多上下文”。
3. 本轮已补按服务器联动的最近动作筛选与错误码聚合，便于运营快速定位是哪个 server 在持续报错。
4. 服务端分页仍可作为后续增强项，但不再阻塞本轮收口。

## 风险与回退

- 主要风险：信息维度过多导致设置页变得拥挤，或者前端字段缺失造成误读。
- 回退方式：保留现有 MCP 服务器列表与原设置项，只隐藏新的观测面板；如字段暂缺，先降级为“缺失值占位 + 只读列表”。
