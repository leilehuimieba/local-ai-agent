# 验证记录

## 验证方式

- 文档验证：
  - 核对 `proposal.md`、`design.md`、`tasks.md`、`status.md` 是否都只覆盖浏览器高风险交互增量。
- 测试验证：
  - request-scoped spec / risk / confirmation 测试
  - browser interaction verify / replan / handoff 测试
- 联调验证：
  - 至少 1 条 `select`、`submit` 或 `upload` 的最小闭环证据

## 证据位置

- 已完成：
  - Browser MCP: `node --check frontend/scripts/browser-mcp-server.mjs`
  - Go: `cd gateway && go test ./internal/api -run BrowserMCP -count=1`
  - Rust: `cargo fmt --all`
  - Rust: `cargo test -p runtime-core`
  - Go: `cd gateway && go test ./internal/api -run BrowserMCPRuntimeBridgeE2ERiskyInteractions -count=1`

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期
- 当前覆盖情况：
  - 已完成独立 change 建档
  - 已完成 `select / submit / upload` 的 Browser MCP 能力暴露、策略配置与 Gateway 确认放行联调
  - 已完成 Runtime 侧浏览器交互 verify 扩展与单次回读收口测试
  - 已完成 `chat run -> runtime -> mcp gateway -> browser mcp` 的高风险交互端到端联调证据
