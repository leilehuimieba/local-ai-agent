# 验证记录

## 验证方式

- 单元测试：优先覆盖 MCP tool spec 到 ToolDefinition 的映射、registry 合并结果、schema 输出结果。
- 集成测试：复用或扩展 Runtime / Gateway 现有 MCP bridge 测试，确认自动 MCP 调用没有回退。
- 人工验证：必要时通过日志或 capability 输出检查当前请求下的 MCP tools 是否真正进入统一注册表。

## 证据位置

- 测试记录：
  - `cargo test -p runtime-core request_visible_tools_merges_native_and_mcp_tools`
  - `cargo test -p runtime-core request_tool_schemas_keep_mcp_arguments_wrapper`
  - `cargo test -p runtime-core request_capability_specs_include_mcp_connector_slot`
  - `cargo test -p runtime-core mcp_tool_schema_is_loaded_from_context_hints`
  - `cargo test -p runtime-core mcp_schemas_use_executable_specs_only`
  - `cargo test -p runtime-core decodes_mcp_tool_call_arguments_wrapper`
  - `cargo test -p runtime-host`
  - `go test ./internal/api -run TestHandleCapabilitiesUsesRequestScopedRuntimeCatalog`
  - `go test ./internal/api -run TestCapabilitiesAPIIncludesRequestScopedMCPCapability`
  - `go test ./internal/api -run TestMCPToolSpecsOnlyExposeExecutableTools`
  - `go test ./internal/api -run TestMCPRuntimeBridgeE2EUsesGatewayPolicyAndAudit`
- 日志或截图：
  - 接口级验收已通过 `gateway/internal/api/mcp_runtime_bridge_e2e_test.go` 中的真实路由测试留证，覆盖 token 鉴权、Gateway MCP hints 注入、Runtime capability query 与 MCP capability 返回。
- 代码盘点：
  - `crates/runtime-core/src/tool_registry.rs`
  - `crates/runtime-core/src/capabilities/registry.rs`
  - `crates/runtime-core/src/capabilities/spec.rs`
  - `crates/runtime-core/src/mcp_bridge.rs`
  - `crates/runtime-core/src/model_client.rs`
  - `crates/runtime-core/src/lib.rs`
  - `crates/runtime-core/src/executors/mcp.rs`
  - `crates/runtime-host/src/main.rs`
  - `gateway/internal/api/catalog.go`
  - `gateway/internal/api/router.go`
  - `gateway/internal/api/chat_context_resolver.go`
  - `gateway/internal/api/mcp_context_hints.go`
  - `gateway/internal/api/mcp_runtime_bridge_e2e_test.go`

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期。
- 当前覆盖情况：已完成 Runtime request-scoped MCP ToolDefinition、registry 合并、模型 schema 统一出口、capability catalog request-scoped 化，以及最小跨 Rust/Go 回归留证；剩余工作主要是 AD 文档收口与提审决策。
