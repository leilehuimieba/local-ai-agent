# 技术方案

## 影响范围

- 涉及模块：`crates/runtime-core/src/tool_registry.rs`、`crates/runtime-core/src/capabilities/registry.rs`、`crates/runtime-core/src/capabilities/spec.rs`、`crates/runtime-core/src/mcp_bridge.rs`、`crates/runtime-core/src/model_client.rs`、必要时的 `query_engine.rs / executors/agent_resolve.rs / lib.rs`。
- 涉及文档或 contract：`docs/11-hermes-rebuild/current-state.md`、`docs/11-hermes-rebuild/changes/INDEX.md`、本 change 的 `tasks.md / status.md / verify.md`。

## 方案

- 核心做法：把 MCP tool spec 从“只在 `model_client.rs` 临时追加 schema”提升为“request-scoped ToolDefinition”，由 Runtime tool registry 统一输出 visible tools、capability specs 和 model tool schemas。
- 状态流转或调用链变化：Gateway 继续注入 MCP hints；Runtime 先从 `mcp_tool_specs_json` 解析出可执行 MCP tool definition，再与静态 native tools 合并；`tool_registry` 成为模型 tools 与能力视图的共同来源；真正执行时仍走 `action_decode -> PlannedAction::MCPCall -> executors/mcp.rs -> Gateway /api/v1/mcp/call`。

## 现状判断

1. 现有主链路并不是“完全没有 MCP 自动执行”，而是已经存在半套：
   - Gateway 会注入 `mcp_tool_specs_json`。
   - `model_client.rs` 会把 `mcp_tool_schemas(request)` 直接并到模型 tools。
   - `action_decode` 与 `executors/mcp.rs` 已能落到执行。
2. 真正缺的是统一注册表：
   - `tool_registry.rs` 与 `capabilities/registry.rs` 只认静态工具目录。
   - `capability_specs()` 不包含 MCP，导致 capability 输出与真实可执行工具集分裂。
   - 模型 tools 现在有两套拼装路径，后续继续扩 MCP risk / source / connector 信息会越来越散。

## 最小改动路径

1. 先补 request-scoped MCP ToolDefinition 映射：
   - 以 `mcp_bridge.rs` 当前解析出的 spec 为源，生成 Runtime 内部统一 ToolDefinition。
2. 再补 registry 合并能力：
   - 给 `ToolRegistry` 增加按 `RunRequest` 或按 MCP spec 合并动态工具的入口。
3. 最后收口模型 tool schema 出口：
   - `model_client.rs` 改为从 registry 统一拿 tools，避免 native / MCP 分两套来源。

## 风险与回退

- 主要风险：`ToolDefinition` 当前 `input_schema` 是静态字符串枚举，MCP 原生 JSON schema 如果硬塞进去，容易把 native tool schema 生成逻辑打坏。
- 回退方式：保留现有 `mcp_tool_schemas(request)` 生成逻辑作为短期 fallback；如果 registry 化改动不稳定，可先只让 capability 列表与 visible tools 接入 MCP，模型 schema 维持旧路径。
