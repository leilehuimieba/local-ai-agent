# 变更提案

## 背景

- `AE-browser-automation-runtime-20260505` 已把 `open_page / read_page` 接入 Runtime 主链。
- `AF-browser-interaction-confirmation-20260505` 已把 `click / type` 纳入确认分层与审批后执行闭环。
- `AL-verify-matrix-minimal-20260505` 已把浏览器交互的最小 verify 接回主循环，并在失败时先触发一次 `browser/read_page` 回读，再决定 handoff。
- 当前浏览器主链的下一处薄弱点是更高风险交互仍未正式收口：`select`、`submit`、`upload` 这类动作没有统一 contract，也没有和 verify / recovery 形成受控闭环。

## 目标

- 产出一个只覆盖“浏览器高风险交互增量”的独立 change 工作区。
- 冻结本刀范围：只做 `select / submit / upload` 的最小 contract、风险分层、verify 信号与 recovery/handoff 收口。
- 明确该 change 以浏览器主链延长为目标，不回头混做知识检索、记忆路由或通用 verify 扩面。

## 非目标

- 不在本 change 中扩新的知识回答策略、记忆层路由或 verify UI 面板。
- 不在本 change 中引入浏览器录制器、复杂多标签编排、登录态管理或下载流水线。
- 不在本 change 中重写通用权限系统，只在现有 confirmation / risk / audit 主链上做最小增量。

## 验收口径

- 文档范围只覆盖浏览器高风险交互增量，与 `AL`、`AK`、`AJ` 保持边界清晰。
- 已明确 `select / submit / upload` 的最小 contract、风险等级与 `requires_confirmation` 口径。
- 已明确浏览器高风险交互的 verify 最小通过标准、失败标准与 recovery/handoff 顺序。
- 后续实现型智能体可以直接以本 change 为入口，先盘点 Runtime / Gateway / Browser MCP 当前实现，再按最小顺序落代码和测试。
