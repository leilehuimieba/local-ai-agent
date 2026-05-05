# 验证记录

## 验证方式

- Rust / Go 单元测试：覆盖 request-scoped spec、risk gate、Gateway approval 执行链。
- 接口或端到端验证：至少补一条浏览器交互确认后执行证据。

## 证据位置

- 测试记录：
  - `cargo test -p runtime-core`
  - `cd gateway && go test ./internal/mcp ./internal/api`
  - 重点覆盖：
    - `risk::tests::requires_confirmation_for_mcp_tool_marked_by_request_spec`
    - `mcp_bridge::tests::mcp_schemas_keep_confirmation_specs_visible`
    - `TestMCPCallAllowsApprovedConfirmationTool`
    - `TestBrowserMCPGatewayAllowsApprovedInteractionTools`
- 日志或截图：本轮以 Rust / Go 接口级与真实 browser MCP E2E 作为主证据，未额外落截图。
- 代码盘点：
  - `frontend/scripts/browser-mcp-server.mjs`
  - `gateway/internal/api/mcp_context_hints.go`
  - `gateway/internal/api/router_mcp.go`
  - `gateway/internal/mcp/manager.go`
  - `crates/runtime-core/src/mcp_bridge.rs`
  - `crates/runtime-core/src/risk.rs`
  - `crates/runtime-core/src/tool_registry.rs`
  - `crates/runtime-core/src/tool_trace.rs`
  - `crates/runtime-core/src/run_risk_flow.rs`
  - `crates/runtime-core/src/executors/mcp.rs`
  - `gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`
  - `gateway/internal/api/router_mcp_test.go`

## 当前判断

1. request-scoped MCP spec 已包含确认型工具，不再只给 Runtime 暴露“可直连”的那部分 browser 能力。
2. Runtime 已能对确认型 MCP 动作返回 `awaiting_confirmation`，而不是把它们提前过滤掉。
3. Gateway 已能在批准后的 confirmation 上下文里放行确认型 MCP 调用，不再形成“批准后仍被策略层挡回”的死链。
4. 真实 browser MCP E2E 已证明：
   - `click` 未批准前返回 `requires confirmation`
   - `click` 批准后执行成功
   - `type` 批准后执行成功
   - `read_page` 能读回 `Clicked once` 与 `approved text`
