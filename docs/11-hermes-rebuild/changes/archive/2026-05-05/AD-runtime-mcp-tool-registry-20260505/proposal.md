# 变更提案

## 背景

- 本次变更要解决的问题：Gateway 已经把 `mcp_tool_specs_json / mcp_tool_preview / mcp_gateway_url` 注入 Runtime，上层模型也能收到 MCP function schema，但 Runtime 自己的 `tool_registry`、`capability_specs`、`visible_tools` 仍是静态工具集，导致 MCP 能力没有真正进入统一主链路。
- 对应阶段目标：自由迭代期里，把 MCP 从“网关桥接 + 模型可见”推进到“Runtime 注册表统一管理、可验证、可持续扩展”。

## 目标

- 本次要完成什么：让 Runtime tool registry 按请求上下文合并可执行的 MCP 工具，并把这些工具纳入统一的 capability 输出、模型 tool schema 装配和执行留痕主线。

## 非目标

- 本次明确不做什么：不新增 stdio transport 支持，不重写 Gateway MCP Manager，不补前端新的 MCP 控制台，不在这一轮解决“需确认 MCP 工具”的完整审批闭环。

## 验收口径

- 通过标准：Runtime 能在当前请求范围内识别并暴露可执行 MCP 工具；模型 tool schema 不再绕过 registry 单独拼装；自动执行后仍复用现有 Gateway allowlist、risk、audit；并补齐最小单测或集成验证证据。
