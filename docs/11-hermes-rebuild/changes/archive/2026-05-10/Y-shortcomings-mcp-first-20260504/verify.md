# 验证记录

更新时间：2026-05-04

## 已执行

1. `cd gateway; go build -o gateway.exe ./cmd/server`
   - 结果：通过。
2. `cd gateway; go test ./internal/api -run "TestNextMCPServers|TestApplyMCPSettings" -count=1`
   - 结果：通过，`ok local-agent/gateway/internal/api`。
3. `cd gateway; go test ./internal/mcp ./internal/config`
   - 结果：通过，无测试文件。
4. `cd gateway; go test ./internal/...` 排除 `internal/api` 后逐包执行。
   - 结果：通过，`knowledge`、`memory`、`providers/bestblogs`、`service`、`session`、`state`、`token` 等包均通过。
5. `cd frontend; npx tsc --noEmit`
   - 结果：通过。
6. `cd frontend; npm test`
   - 结果：通过，2 个测试文件、11 个测试全绿。
7. `cd frontend; npm run build`
   - 结果：通过。
8. `cd frontend; npm run test:e2e`
   - 结果：通过，4 个 Playwright 移动端 E2E 全绿。
9. `cd gateway; go test ./...`
   - 结果：通过，`internal/api` 已不再依赖外部文章样本导致 502 或超时。
10. `cd gateway; go test ./internal/api -run "TestMCPTool|TestBuildRetry|TestCancel" -count=1`
    - 结果：通过，覆盖 MCP 工具摘要排序、重试构建与取消流程相关回归。
11. `cargo test -p runtime-core`
    - 结果：通过，174 个测试全绿；覆盖 context builder 与 AgentResolve prompt 的 MCP 可见性注入；保留既有 `redact_sensitive_text` 未使用 warning。
12. `cd gateway; go build -o gateway.exe ./cmd/server`
    - 结果：通过。
13. `cd gateway; go test ./internal/mcp ./internal/api -run "TestResolveToolPolicy|TestNextMCPServers|TestMCPCall|TestMCPTool" -count=1`
    - 结果：通过，覆盖 MCP 默认 deny、配置 allowlist、调用拒绝、低风险允许调用和工具预览风险字段。
14. `cd frontend; npx tsc --noEmit`
    - 结果：通过。
15. `cd frontend; npm test`
    - 结果：通过，2 个测试文件、11 个测试全绿。
16. `cd frontend; npm run build`
    - 结果：通过。
17. `cargo test -p runtime-core memory::tests::search_includes_current_memory_object_for_duplicate_entry -- --nocapture`
    - 结果：通过，确认 memory 测试夹具临时目录碰撞已收敛。
18. `cargo test -p runtime-core`
    - 结果：通过，174 个测试全绿；保留既有 `redact_sensitive_text` 未使用 warning。
19. `cd gateway; go test ./internal/api ./internal/mcp -run "TestMCP|TestResolveToolPolicy" -count=1`
    - 结果：通过，覆盖 MCP tool specs、bridge hints、allowlist、调用与策略解析相关回归。
20. `cargo test -p runtime-core mcp -- --nocapture`
    - 结果：通过，8 个 MCP 相关测试全绿，覆盖 schema 注入、tool_call 解码、payload 构造和缺桥接配置失败。
21. `cd gateway; go test ./...`
    - 结果：通过，Gateway 全量测试通过。
22. `cargo test -p runtime-core`
    - 结果：通过，180 个测试全绿；保留既有 `redact_sensitive_text` 未使用 warning。
23. `cd gateway; go build -o gateway.exe ./cmd/server`
    - 结果：通过。
24. `cd gateway; go test ./internal/api -run "TestMCPRuntimeBridgeE2EUsesGatewayPolicyAndAudit" -count=1`
    - 结果：通过，启动 fake MCP、真实 Gateway router 与 fake Runtime；fake Runtime 使用 `mcp_gateway_url/mcp_gateway_token` 回调 `/api/v1/mcp/call`，确认 allowlist 调用与 `logs/mcp-audit.jsonl` 落盘。

## 未通过 / 残余风险

1. `cargo test -p runtime-core` 仍有既有 warning：`crate::sensitive_data::redact_sensitive_text` 未使用；不影响本次验收。
2. Runtime 只把 allowlisted 且 `requires_confirmation=false` 的 MCP tools 注入为模型可执行 schema；需要确认的工具继续由 Gateway 拒绝，后续接统一 confirmation 主线。
3. 本轮已补本地 fake MCP E2E 验收夹具；真实第三方 MCP server 仍建议在人工验收时补一轮。

## 验收映射

1. MCP 添加/删除持久化：由 `gateway/internal/api/mcp_settings_test.go` 覆盖。
2. Manager 热重载：由代码路径 `applyMCPSettings -> mgr.ReplaceServers` 覆盖，后续可补 handler 集成测试。
3. 前端操作入口：由 TypeScript 构建覆盖，后续可补 Playwright 设置页 E2E。
4. 全量 Go 外部依赖收敛：由 `gateway/internal/api/bestblogs_test_fixture_test.go` 与 `go test ./...` 覆盖。
5. Runtime MCP 可见性注入：由 `gateway/internal/api/mcp_context_hints_test.go` 与 `context_builder::tests::tool_preview_includes_mcp_hints_when_policy_allows_tools` 覆盖。
6. MCP 安全策略：由 `gateway/internal/mcp/policy_test.go`、`gateway/internal/api/mcp_settings_test.go`、`gateway/internal/api/router_mcp_test.go` 覆盖。
7. Runtime MCP 调用闭环：由 `mcp_bridge::tests`、`action_decode::tests::decodes_mcp_tool_call_arguments_wrapper`、`executors::mcp::tests` 与 Gateway MCP route 测试覆盖。
