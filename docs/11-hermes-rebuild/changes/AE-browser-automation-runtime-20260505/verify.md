# 验证记录

## 验证方式

- 代码盘点：确认 Runtime、Gateway、前端与现有浏览器能力入口的真实落点。
- 单元测试：优先覆盖浏览器 tool contract、registry 暴露与执行桥输入输出。
- 接口或端到端验证：至少补一条真实浏览器动作被调起并返回结果的证据。

## 证据位置

- 测试记录：
  - `cd gateway && go test ./cmd/launcher ./internal/mcp ./internal/api`
  - 重点覆盖：
    - `gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`
    - `gateway/internal/api/mcp_runtime_bridge_e2e_test.go`
- 日志或截图：
  - `tmp/ae-browser-mcp/launcher.txt`
  - `tmp/ae-browser-mcp/capabilities.json`
  - `tmp/ae-browser-mcp/mcp-tools.json`
  - `tmp/ae-browser-mcp/open-page.json`
  - `tmp/ae-browser-mcp/read-page.json`
  - `logs/runtime-build.log`
- 代码盘点：
  - `crates/runtime-core/src/executors/mcp.rs`
  - `crates/runtime-core/src/mcp_bridge.rs`
  - `crates/runtime-core/src/tool_registry.rs`
  - `crates/runtime-core/src/capabilities/slots.rs`
  - `gateway/internal/api/router_mcp.go`
  - `gateway/internal/mcp/manager.go`
  - `gateway/internal/api/settings_response.go`
  - `frontend/playwright.config.ts`
  - `frontend/e2e/mobile-layout.spec.ts`
  - `frontend/app/acceptance/mcp-observability/page.tsx`
  - `config/app.json`
  - `.playwright-cli/`
  - `.playwright-mcp/`
  - `docs/11-hermes-rebuild/changes/archive/2026-04-26/H-learning-mode-browser-20260415/status.md`
  - `docs/11-hermes-rebuild/changes/archive/2026-04-26/H-learning-mode-browser-20260415/verify.md`

## 本轮盘点结论

1. Runtime 当前唯一成熟的外部执行桥是 MCP Gateway 调用链，浏览器首刀优先复用这一入口。
2. 浏览器 connector slot 与 settings 外部连接位都还停留在 `reserved` / 保留位，没有正式 browser tool contract。
3. 前端 Playwright、`.playwright-cli`、`.playwright-mcp` 提供的是测试、调试和快照资产，不等价于 Runtime 主链浏览器能力。
4. 主配置没有浏览器 MCP server，说明“浏览器可调度”目前还没进入真实产品配置。
5. 历史口径已明确 browser fallback 未产品化，因此 AE 首刀不应从 DOM-heavy 回退链入手。

## 本轮 contract 冻结结论

1. 首刀工具名已冻结为 `browser/open_page` 与 `browser/read_page`，对应 Runtime 函数名 `mcp__browser__open_page` 与 `mcp__browser__read_page`。
2. `open_page` 已冻结为 `risk=medium`、`requires_confirmation=false`、`audit=true`。
3. `read_page` 已冻结为 `risk=low`、`requires_confirmation=false`、`audit=true`。
4. 两个工具都要求返回结构化 JSON；`open_page` 必返 `page_id`，`read_page` 必显式接收 `page_id`，避免共享“当前页”状态。
5. 可恢复业务错误统一采用 `ok=false + error_code + error_message`，为后续模型推理与验收保留稳定结构。

## 本轮实现验证结论

1. 真实 browser MCP server 已接入配置与 launcher：
   - `frontend/scripts/browser-mcp-server.mjs`
   - `frontend/package.json`
   - `config/app.json`
   - `gateway/cmd/launcher/main.go`
2. request-scoped tool spec 已进入 Runtime 能力目录：
   - `tmp/ae-browser-mcp/capabilities.json` 中可见
     - `mcp__browser__open_page`
     - `mcp__browser__read_page`
3. Gateway 实时工具视图已能读到 browser server 策略与 schema：
   - `tmp/ae-browser-mcp/mcp-tools.json`
4. 最小真实链已通过：
   - `tmp/ae-browser-mcp/open-page.json`
     - `ok=true`
     - `page_id=page_01`
     - `title=AE Browser Proof`
   - `tmp/ae-browser-mcp/read-page.json`
     - `ok=true`
     - `content` 含 `Bridge Ready`
     - `content` 含 `Hello from AE browser acceptance.`
5. launcher 已验证会自动重建过期 Runtime 二进制：
   - `logs/runtime-build.log` 显示 `cargo build -p runtime-host` 成功
   - 新 `target/debug/runtime-host.exe` 时间戳已刷新到本轮联调时间

## 最小联调命令

1. 启动：
   - `cd gateway`
   - `LOCAL_AGENT_NO_BROWSER=1 go run ./cmd/launcher`
2. 取能力目录：
   - `GET http://127.0.0.1:38471/api/v1/capabilities?mode=standard`
3. 调真实 browser MCP：
   - `POST http://127.0.0.1:38471/api/v1/mcp/call`
   - body: `{"server_id":"browser","name":"open_page","arguments":{"url":"file:///.../fixture.html"}}`
4. 二次读取：
   - `POST http://127.0.0.1:38471/api/v1/mcp/call`
   - body: `{"server_id":"browser","name":"read_page","arguments":{"page_id":"page_01","format":"markdown"}}`

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期。
- 当前覆盖情况：已完成任务 1 盘点留证、任务 2 contract 冻结、任务 3 主链接入与任务 4 联调验收，AE 当前目标已达成。
