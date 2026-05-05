# 技术方案

## 影响范围

- 涉及模块：`crates/runtime-core/src/tool_registry.rs`、浏览器相关 executor 或桥接模块、`gateway/internal/api/` 的浏览器承接入口、必要时的前端验收入口。
- 涉及文档或 contract：`docs/11-hermes-rebuild/current-state.md`、`docs/11-hermes-rebuild/changes/INDEX.md`、本 change 的 `tasks.md / status.md / verify.md`。

## 方案

- 核心做法：先盘点现有 Playwright / dev-browser / MCP browser 能力，选择一条最小主链承接 Runtime 请求，再把该能力纳入 registry、风险边界与验收主线。
- 状态流转或调用链变化：Runtime 生成浏览器动作 -> registry 暴露浏览器工具 -> 执行层调用 Gateway 或既有浏览器桥接入口 -> 返回结构化结果 -> 进入验证与事件留痕。

## 现状判断

1. 仓库已经具备浏览器相关资产，但它们仍偏“独立工具”而非 Runtime 主链能力：
   - 前端有 Playwright E2E 基础设施。
   - 仓库根目录存在 `.playwright-cli`、`.playwright-mcp` 调试产物目录，可证明此前已有浏览器自动化与联调实践。
   - 现有 skill / 插件能做本地浏览器操作，但仍停留在独立调试与验收层。
   - Runtime 尚无稳定的浏览器 tool contract 与执行出口。
2. 这轮优先目标不是“浏览器能力做多强”，而是先让浏览器能力进入主链、能被调用、能被验证。

## 资产盘点结论

1. Runtime 侧：
   - `crates/runtime-core/src/executors/mcp.rs` 已提供成熟的外部执行桥，从 `request.context_hints` 读取 `mcp_gateway_url` / `mcp_gateway_token`，统一通过 Gateway `POST /api/v1/mcp/call` 调外部工具。
   - `crates/runtime-core/src/tool_registry.rs` 已支持 request-scoped MCP tool definitions 合并到模型工具列表，但当前没有 Runtime native browser tool。
   - `crates/runtime-core/src/capabilities/slots.rs` 中的 `browser_capture_ingest` 仍是 `reserved`，只保留“浏览器摘录与网页入库”的规划位。
2. Gateway 侧：
   - `gateway/internal/api/router_mcp.go` 已具备 `/api/v1/mcp/servers`、`/api/v1/mcp/tools`、`/api/v1/mcp/audits`、`/api/v1/mcp/call` 完整入口。
   - `gateway/internal/mcp/manager.go` 已提供 allowlist、risk、requires_confirmation、audit 和多 server 管理，浏览器能力若走 MCP 可直接复用治理链。
   - `gateway/internal/api/settings_response.go` 中的 `browserCaptureConnection()` 仍是控制面保留位，不承接真实浏览器动作校验。
3. 前端与调试资产：
   - `frontend/playwright.config.ts` 与 `frontend/e2e/mobile-layout.spec.ts` 已形成 Playwright E2E 基础设施，但目标是前端页面回归，不是 Runtime 工具执行桥。
   - `frontend/app/acceptance/mcp-observability/page.tsx` 有 `browser/open_page` 的只读示例数据，可作为后续 UI 验收素材，但当前不是后端真实链路。
   - `.playwright-cli`、`.playwright-mcp` 目录当前主要保存日志、页面快照和截图，说明仓库已有浏览器操作痕迹，但还没有沉淀为主链 contract。
4. 配置与历史口径：
   - `config/app.json` 当前只配置 `rust-mcp-local`，没有浏览器类 MCP server。
   - 历史 change `H-learning-mode-browser-20260415` 已明确：browser fallback 未产品化，若长期依赖 UI/DOM 抽取，生产稳定性会下降。

## 主链断点

1. 配置断点：浏览器 MCP server 尚未进入真实配置，Runtime 请求无法获得浏览器工具清单。
2. contract 断点：Runtime registry 已能接 request-scoped MCP tools，但尚无冻结的浏览器首刀工具名、输入输出与结果回传格式。
3. 能力视图断点：控制面与 connector slot 对浏览器仍是保留位，缺少“已接主链”的能力呈现。
4. 验收断点：当前只有前端 E2E 与只读观测样例，没有浏览器动作真实调起的接口级或端到端证据。

## 首刀 contract 建议

1. 首刀入口优先复用 MCP 路线，不新增 Runtime 内生 Playwright executor。
2. 首刀动作面收在低复杂度、低状态依赖、结果结构化的两步：
   - `browser_open_page`
   - `browser_read_page`
3. `browser_click` 可作为次一级候选，放到 `open_page + read_page` 稳定后再接。
4. 首刀明确不做：
   - 登录态管理
   - 下载链路
   - 多标签复杂切换
   - 长链 DOM 脆弱自动化
   - 视觉回放或重型浏览器编排

## 首刀 contract 冻结（v1）

### 1. 工具身份

1. 统一 MCP server：`browser`
2. MCP 原始工具名：
   - `open_page`
   - `read_page`
3. Runtime 暴露给模型的函数名：
   - `mcp__browser__open_page`
   - `mcp__browser__read_page`
4. 不单独新增 Runtime native browser tool，继续复用 request-scoped MCP ToolDefinition 注入机制。

### 2. 策略字段冻结

1. `open_page`
   - `risk_level`: `medium`
   - `requires_confirmation`: `false`
   - `audit_enabled`: `true`
2. `read_page`
   - `risk_level`: `low`
   - `requires_confirmation`: `false`
   - `audit_enabled`: `true`
3. 冻结理由：
   - 当前 `crates/runtime-core/src/mcp_bridge.rs` 只会把 `requires_confirmation=false` 的 MCP spec 暴露给模型。
   - 当前 `gateway/internal/mcp/manager.go` 对 `requires_confirmation=true` 的工具会直接拒绝调用。
   - 因此首刀若想进入现有主链并可真实调度，这两个只读工具必须先冻结为“无需确认但必须审计”。
4. 后续扩展规则：
   - `click`、`type`、`download`、`upload` 等更强动作默认按 `risk>=medium` 且优先评估 `requires_confirmation=true`。

### 3. `open_page` 输入 contract

```json
{
  "type": "object",
  "properties": {
    "url": { "type": "string", "description": "要打开的页面 URL" },
    "wait_until": {
      "type": "string",
      "enum": ["domcontentloaded", "load", "networkidle"],
      "description": "导航完成判定，默认 load"
    },
    "timeout_ms": {
      "type": "integer",
      "minimum": 1000,
      "maximum": 30000,
      "description": "导航超时，默认 15000"
    }
  },
  "required": ["url"]
}
```

1. `url` 必填。
2. `wait_until` 可选，默认 `load`。
3. `timeout_ms` 可选，默认 `15000`。
4. 不在首刀引入 `selector`、`cookies`、`headers`、`auth`、`viewport` 等附加参数。

### 4. `open_page` 成功 payload

```json
{
  "ok": true,
  "page_id": "page_01",
  "requested_url": "https://example.com",
  "final_url": "https://example.com/home",
  "title": "Example",
  "status_code": 200,
  "loaded": true,
  "elapsed_ms": 842
}
```

1. `page_id` 必返，供后续 `read_page` 显式引用，避免多会话串页。
2. `requested_url` 与 `final_url` 同时保留，用于处理跳转链。
3. `status_code`、`title`、`elapsed_ms` 作为最小可观测元数据固定返回。

### 5. `read_page` 输入 contract

```json
{
  "type": "object",
  "properties": {
    "page_id": { "type": "string", "description": "open_page 返回的页面标识" },
    "format": {
      "type": "string",
      "enum": ["markdown", "text"],
      "description": "返回正文格式，默认 markdown"
    },
    "max_chars": {
      "type": "integer",
      "minimum": 1000,
      "maximum": 20000,
      "description": "最大返回字符数，默认 12000"
    }
  },
  "required": ["page_id"]
}
```

1. `page_id` 必填，不允许依赖“当前激活页”隐式状态。
2. `format` 可选，默认 `markdown`。
3. `max_chars` 可选，默认 `12000`。
4. 首刀不支持 `selector` 定位读取，统一按页面级正文读取。

### 6. `read_page` 成功 payload

```json
{
  "ok": true,
  "page_id": "page_01",
  "url": "https://example.com/home",
  "title": "Example",
  "content_format": "markdown",
  "content": "# Example\n\nReadable body...",
  "truncated": false,
  "returned_char_count": 4821,
  "elapsed_ms": 231
}
```

1. `content` 为主结果字段。
2. `content_format` 与请求的 `format` 对齐，避免模型猜测正文编码。
3. `truncated` 与 `returned_char_count` 必返，用于 Runtime 预览和后续验收口径。

### 7. 错误 payload 约定

```json
{
  "ok": false,
  "error_code": "browser_page_not_found",
  "error_message": "page_id page_01 not found"
}
```

1. 页面不存在、导航超时、正文为空、页面不可读等“可恢复业务错误”，优先返回 `200 + ok=false` 的结构化 JSON。
2. 仅当 MCP server 不可用、协议崩溃、结果不可解析等基础设施错误时，才抬升为 Gateway / MCP 调用失败。
3. 首批保留错误码：
   - `browser_invalid_url`
   - `browser_navigation_timeout`
   - `browser_navigation_failed`
   - `browser_page_not_found`
   - `browser_read_failed`
   - `browser_empty_document`

### 8. MCP spec 示例

```json
[
  {
    "server_id": "browser",
    "name": "open_page",
    "function_name": "mcp__browser__open_page",
    "description": "Open a page and return navigation metadata.",
    "risk_level": "medium",
    "requires_confirmation": false,
    "audit_enabled": true,
    "input_schema": {
      "type": "object",
      "properties": {
        "url": { "type": "string" },
        "wait_until": { "type": "string", "enum": ["domcontentloaded", "load", "networkidle"] },
        "timeout_ms": { "type": "integer", "minimum": 1000, "maximum": 30000 }
      },
      "required": ["url"]
    }
  },
  {
    "server_id": "browser",
    "name": "read_page",
    "function_name": "mcp__browser__read_page",
    "description": "Read the current page into structured text.",
    "risk_level": "low",
    "requires_confirmation": false,
    "audit_enabled": true,
    "input_schema": {
      "type": "object",
      "properties": {
        "page_id": { "type": "string" },
        "format": { "type": "string", "enum": ["markdown", "text"] },
        "max_chars": { "type": "integer", "minimum": 1000, "maximum": 20000 }
      },
      "required": ["page_id"]
    }
  }
]
```

## 最小改动路径

1. 先盘点并冻结一条首刀浏览器 contract：
   - 明确支持哪些动作，结果如何回传，风险等级如何定义。
2. 再把浏览器能力接入 Runtime registry：
   - 让模型能在正式工具列表中看到它。
3. 最后补执行桥与验收：
   - 至少补一条真实接口或 E2E 证据，证明浏览器动作能被调起并返回。

## 本轮落地结果

1. 已采用“MCP browser server -> Gateway MCP -> Runtime request-scoped registry”作为首刀正式主链。
2. 已新增真实 browser MCP server：
   - `GET /health`
   - `POST /mcp`
   - MCP 方法：`initialize`、`tools/list`、`tools/call`
3. 已把 browser server 接入 `config/app.json`，并冻结：
   - `open_page`: `risk=medium`、`requires_confirmation=false`、`audit=true`
   - `read_page`: `risk=low`、`requires_confirmation=false`、`audit=true`
4. 已在 Gateway capability request 路径上，把 browser MCP tool spec 作为 request-scoped hints 注入 Runtime。
5. 已补 launcher 侧 Runtime 过期二进制重建，防止旧 `runtime-host` 让能力目录和联调结果失真。

## 风险与回退

- 主要风险：浏览器自动化天然涉及长时动作、环境波动与页面不稳定，如果第一刀动作面太大，容易把主链拉散。
- 回退方式：首刀只保留只读或低复杂度动作；如执行桥不稳定，可先保留 registry / contract 与接口级验收，不扩到复杂交互链。
