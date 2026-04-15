# 变更提案

## 背景

- 本次变更要解决的问题：
  1. 当前阶段 G 的 `G-01~G-03` 已完成，`G-04` 仍缺独立 change 的值守演练证据与职责链收口。
  2. `G-运行手册与值守规范` 目前为文档定义，需通过真实脚本演练把责任字段、发布窗口阈值与分流入口落到证据。
  3. Gate-G 阶段签收前，需要验证 runbook 在 routine 与 release_window 两种模式下都可执行并可追溯。
- 对应阶段目标：
  1. 完成 `G-04`：运行手册与值守职责固化。

## 目标

- 本次要完成什么：
  1. 按 runbook 完成一轮值守演练，覆盖 `routine` 与 `release_window`。
  2. 固化 `owner/tracking_id/due_at` 责任字段并回写证据链。
  3. 同步更新 `G-04` change 五件套与阶段文档，形成可提审口径。

## 非目标

- 本次明确不做什么：
  1. 不修改 `runtime-core` 业务能力与前端功能。
  2. 不直接做 Gate-G 最终签收（`G-G1`）。
  3. 不扩展新的巡检脚本类型。

## 验收口径

- 通过标准：
  1. `tmp/stage-g-evidence-freshness/latest.json` 在 release_window 下 `status=passed`，策略阈值为 30 分钟。
  2. `tmp/stage-g-gate/latest.json` 为 `passed` 且 `gate_g.ready=true`。
  3. `tmp/stage-g-ops/latest.json` 为 `passed` 且 `checks.governance_ready=true`。
  4. `tmp/stage-g-regression/latest.json` 保持 `pass_rate>=95`。
  5. change 五件套与 `current-state/INDEX/全路线最小任务分解总表` 口径一致。
