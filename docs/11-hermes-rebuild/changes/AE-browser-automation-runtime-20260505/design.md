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
   - 现有 skill / 插件能做本地浏览器操作。
   - 但 Runtime 尚无稳定的浏览器 tool contract 与执行出口。
2. 这轮优先目标不是“浏览器能力做多强”，而是先让浏览器能力进入主链、能被调用、能被验证。

## 最小改动路径

1. 先盘点并冻结一条首刀浏览器 contract：
   - 明确支持哪些动作，结果如何回传，风险等级如何定义。
2. 再把浏览器能力接入 Runtime registry：
   - 让模型能在正式工具列表中看到它。
3. 最后补执行桥与验收：
   - 至少补一条真实接口或 E2E 证据，证明浏览器动作能被调起并返回。

## 风险与回退

- 主要风险：浏览器自动化天然涉及长时动作、环境波动与页面不稳定，如果第一刀动作面太大，容易把主链拉散。
- 回退方式：首刀只保留只读或低复杂度动作；如执行桥不稳定，可先保留 registry / contract 与接口级验收，不扩到复杂交互链。
