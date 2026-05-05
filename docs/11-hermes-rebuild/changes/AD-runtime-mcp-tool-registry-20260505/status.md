# 当前状态

- 最近更新时间：2026-05-05
- 状态：进行中
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`
- 已完成：已切换主推进项，建立 AD change 工作区并完成现状盘点。
- 已完成：已确认 Runtime 当前具备 `mcp_tool_specs_json -> model tools -> action_decode -> executors/mcp.rs` 的半链路。
- 已完成：已把 MCP tool spec 映射为 request-scoped `ToolDefinition`，并接入 `tool_registry.rs` 的统一可见工具出口。
- 已完成：`model_client.rs` 已改为通过 registry 统一拿取模型 tools，原生工具与 MCP schema 不再双轨拼接。
- 已完成：`context_builder.rs` 已避免 MCP tools 与 `mcp_tool_preview` hint 重复展示，`connector_slot` 已纳入 `mcp_gateway`。
- 已完成：`capability_catalog` 已升级为 request-scoped 查询链路，Gateway `/api/v1/capabilities` 会携带当前 mode、workspace 与 MCP hints 向 Runtime 查询能力目录。
- 已完成：已补 Gateway 真路由接口级验收，`/api/v1/capabilities` 真实返回 request-scoped MCP capability。
- 进行中：正在整理 AD 的最终收口口径，准备提交与推送。
- 阻塞点：暂无。
- 下一步：补齐 AD 收口记录并提交推送，随后切下一项 change。
