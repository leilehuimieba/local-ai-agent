# 技术方案

## 影响范围

- Runtime：`crates/runtime-core/src/mcp_bridge.rs`、`risk.rs`、相关测试。
- Gateway：`gateway/internal/api/mcp_context_hints.go`、`router_mcp.go`、`internal/mcp/manager.go`、相关测试。
- Browser MCP：`frontend/scripts/browser-mcp-server.mjs`
- 配置：`config/app.json`

## 方案

1. 冻结第二刀浏览器交互合同：
   - `browser/click`
   - `browser/type`
2. request-scoped MCP spec 不再只注入“无需确认”的工具，而是把 allowlisted 的确认工具也暴露给 Runtime。
3. Runtime 对 MCP 动作新增基于 tool spec 的确认判断：
   - `requires_confirmation=false` 继续直连执行
   - `requires_confirmation=true` 先进入现有 confirmation 主线
4. Gateway MCP 调用链在收到已批准的 confirmation 上下文后，允许一次确认后执行，避免重复被策略层挡回。

## 风险分层

1. `open_page`
   - `risk=medium`
   - `requires_confirmation=false`
2. `read_page`
   - `risk=low`
   - `requires_confirmation=false`
3. `click`
   - `risk=medium`
   - `requires_confirmation=true`
4. `type`
   - `risk=high`
   - `requires_confirmation=true`

## 验证路径

1. Go / Rust 测试覆盖：
   - request-scoped spec 包含确认型 MCP 工具
   - Runtime 风险门会为确认型 MCP 工具生成 confirmation request
   - Gateway 在确认批准后允许对应 MCP 调用通过
2. 最小联调：
   - `open_page`
   - `click` 或 `type` 先触发 confirmation
   - approval 后执行成功
