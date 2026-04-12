# 变更提案

## 背景

- `F-01`（安装/升级主路径）与 `F-02`（doctor 核心检查）已完成。
- 阶段 F 的 `F-03` 要求收口“发布清单 + 回滚预案”，当前缺少统一执行口径与验收记录。
- `E-backend-reverify-pack` 已输出 `failed_checks` 与 `non_blocking_warnings` 双通道协议，以及 `latest/warning-sample/failure-sample` 三态证据。
- 用户当前要求继续后端主线推进，本轮不改前端实现。

## 目标

- 输出阶段 F 的发布清单（发布前检查、发布动作、发布后核验、审计留痕）。
- 输出阶段 F 的回滚预案（触发条件、回滚步骤、回滚后核验、失败升级路径）。
- 将 warning 协议接入发布/回滚决策，形成“可发布但需记录告警”的固定流程。
- 以 `changes` 工作区沉淀可复用的 F-03 文档资产，支撑 F-04/F-05/F-G1。

## 非目标

- 本轮不执行发布候选回归（归属 `F-04`）。
- 本轮不做新机 10 分钟实机验证（归属 `F-05`）。
- 本轮不触碰前端代码与界面样式。

## 验收口径

- `changes/F-*/design.md` 已完整覆盖发布清单与回滚预案。
- 补充文档可直接执行且与现有脚本口径一致。
- 三态决策口径可复核：
  - `failed_checks.count > 0`：阻断。
  - `failed_checks.count = 0` 且 `non_blocking_warnings.count > 0`：可发布但必须记录告警。
  - `failed_checks.count = 0` 且 `non_blocking_warnings.count = 0`：正常放行。
