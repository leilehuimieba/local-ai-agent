# 当前状态

- 最近更新时间：2026-05-05
- 状态：已收口
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`
- 已完成：已新建独立 change `AM-browser-risky-interaction-20260505`。
- 已完成：已冻结本 change 只覆盖浏览器高风险交互增量，不回 `AG`、`AK`、`AL` 混做。
- 已确认：本刀优先级是 `select / submit / upload` 的最小 contract、risk/confirmation、verify 与 recovery/handoff。
- 已完成：已盘点 Browser MCP 当前只支持 `open_page / read_page / click / type`，并确认 Runtime / Gateway 的 risk 与 confirmation 主链可承接新动作。
- 已完成：`frontend/scripts/browser-mcp-server.mjs` 已新增 `select / submit / upload` 三类工具与最小执行逻辑。
- 已完成：`config/app.json` 已为 `select / submit / upload` 补齐 browser MCP tool policy。
- 已完成：`gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go` 已补工具暴露检查与确认后执行的最小桥接证据。
- 已完成：`crates/runtime-core/src/verify.rs` 已把 `select / submit / upload` 纳入浏览器交互完成信号识别。
- 已完成：`crates/runtime-core/src/query_engine.rs` 已补三类动作的 verify 失败测试，确认仍走“单次 `read_page` 回读 -> 回读后停止自动续跑”的既有收口路径。
- 已完成：`gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go` 已补 `chat run -> runtime -> mcp gateway -> browser mcp` 的高风险交互端到端联调证据。
- 验证结果：`node --check frontend/scripts/browser-mcp-server.mjs` 通过；`go test ./internal/api -run BrowserMCP -count=1` 通过。
- 验证结果：`cargo fmt --all` 与 `cargo test -p runtime-core` 通过，`runtime-core` 当前 227 项测试全绿。
- 阻塞点：暂无。
- 下一步：如继续推进，请新建下一项独立 change，不在 AM 内继续扩 scope。
