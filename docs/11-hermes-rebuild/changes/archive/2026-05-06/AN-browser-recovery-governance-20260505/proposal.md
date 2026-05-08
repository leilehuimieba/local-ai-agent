# 变更提案

## 背景

- `AE-browser-automation-runtime-20260505` 已把 `open_page / read_page` 接入 Runtime 主链。
- `AF-browser-interaction-confirmation-20260505` 已把 `click / type` 纳入确认分层与审批后执行闭环。
- `AM-browser-risky-interaction-20260505` 已把 `select / submit / upload` 纳入 browser MCP、risk/confirmation、Runtime verify，并补齐端到端联调证据。
- 当前浏览器主链的下一处薄弱点不再是“缺动作”，而是“失败后怎么更稳定地交接与恢复”：高风险交互失败后虽然已具备单次 `read_page` 回读和 handoff 收口，但失败分类、handoff artifact 和恢复提示仍然偏粗。

## 目标

- 产出一个只覆盖“浏览器恢复治理细化”的独立 change 工作区。
- 冻结本刀范围：只做高风险浏览器交互失败后的恢复分层、handoff artifact 收紧与恢复提示治理。
- 明确该 change 以浏览器失败路径可操作性为目标，不回头混做新动作、知识检索或通用 verify 扩面。

## 非目标

- 不在本 change 中新增新的 browser MCP 动作。
- 不在本 change 中重写通用 verify 矩阵、知识回答策略或记忆路由。
- 不在本 change 中引入浏览器录制器、复杂工作台或多标签任务编排。

## 验收口径

- 文档范围只覆盖浏览器恢复治理，与 `AM`、`AL` 保持边界清晰。
- 已明确浏览器高风险交互失败后的最小失败分型、recovery 入口、handoff artifact 和最终停止条件。
- 已明确后续实现入口，后续实现型智能体可以直接据此先盘点 `query_engine.rs`、`events.rs`、失败元数据与 checkpoint/handoff 输出。
