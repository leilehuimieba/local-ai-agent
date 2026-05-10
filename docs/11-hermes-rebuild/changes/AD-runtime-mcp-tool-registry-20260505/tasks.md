# 任务清单

- [x] 任务 1：盘点 Runtime 当前 MCP 半链路与 registry 缺口
  完成判据：明确 `Gateway hints -> model tools -> action decode -> mcp executor` 的现状，以及 registry 仍未接入的具体断点。
- [x] 任务 2：实现 request-scoped MCP ToolDefinition 与 registry 合并
  完成判据：Runtime registry 能按当前请求合并可执行 MCP 工具，且不会破坏现有 native tools 的 visible / capability 输出。
- [x] 任务 3：收口模型 tool schema 的统一出口
  完成判据：模型请求中的 tools 来源统一经过 registry，不再由 `model_client.rs` 旁路单独拼接 MCP schema。
- [x] 任务 4：补最小验证与回归
  完成判据：至少覆盖 MCP tool spec 解析、registry 合并、schema 输出和现有 Gateway bridge 自动调用链中的一条验证证据。
