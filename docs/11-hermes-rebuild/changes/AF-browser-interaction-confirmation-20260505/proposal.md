# 变更提案

## 背景

- AE 已把 `browser/open_page` 与 `browser/read_page` 接入 Runtime 主链，但浏览器交互动作仍停留在只读能力。
- 当前 `requires_confirmation=true` 的 MCP 工具不会进入 request-scoped Runtime tool spec，也无法在审批后继续通过 Gateway 执行。

## 目标

- 为浏览器第二刀补齐 `click` / `type` 的正式 contract。
- 把 `risk` 与 `requires_confirmation` 分层接入 request-scoped tool spec、Runtime 风险门和 Gateway MCP 调用链。
- 证明“未审批先暂停、审批后可执行”的最小闭环成立。

## 非目标

- 不扩下载、上传、复杂多标签切换、登录态管理。
- 不做浏览器交互录制器或完整 IDE 级 DOM 工作台。

## 验收口径

- `click` / `type` 能出现在 request-scoped capability catalog 中，并带上正确的 `risk_level` 与 `requires_confirmation`。
- 运行时在用户批准前返回 `awaiting_confirmation`，批准后能够真实调起 browser MCP 并返回结果。
- 至少补齐一条接口级或端到端证据，覆盖 `click` 或 `type` 的确认后执行链。
