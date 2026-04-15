# 变更提案

## 背景

- 本次变更要解决的问题：
  1. 阶段 G 已启动，但尚未形成“证据保鲜策略 + 自动复跑入口”的可执行闭环。
  2. 当前复核能力集中在 `run-stage-backend-reverify-pack.ps1`，缺少面向阶段 G 的统一入口与报告口径。
  3. Gate-G 需要以“证据时效 + warning 责任字段 + 严格门禁”作为常态判定基线。
- 对应阶段目标：
  1. 完成 `G-01`：证据保鲜策略与自动复跑入口落版。

## 目标

- 本次要完成什么：
  1. 新增阶段 G 证据保鲜脚本入口 `run-stage-g-evidence-freshness.ps1`。
  2. 新增阶段 G G-01 聚合验收脚本 `run-stage-g-gate-acceptance.ps1`。
  3. 补齐阶段 G 交付文档：`G-证据保鲜策略.md`、`G-发布后巡检与告警治理.md`、`G-最小回归基线清单.md`、`G-运行手册与值守规范.md`。
  4. 产出首轮证据并回写 change 五件套。

## 非目标

- 本次明确不做什么：
  1. 不修改 `runtime-core` 与 `frontend` 代码。
  2. 不完成 `G-02/G-03/G-04/G-G1` 全量实施，仅建立入口与首轮基线。
  3. 不变更阶段状态裁决（仍由 `current-state.md` 维护阶段 G）。

## 验收口径

- 通过标准：
  1. `scripts/run-stage-g-evidence-freshness.ps1` 与 `scripts/run-stage-g-gate-acceptance.ps1` 可执行。
  2. 运行 `run-stage-g-gate-acceptance.ps1 -RefreshEvidence -RequirePass` 输出 `tmp/stage-g-gate/latest.json` 且 `gate_g.ready=true`。
  3. 阶段 G 四份交付文档已创建。
  4. 本 change 的 `tasks.md`、`status.md`、`verify.md` 与证据路径一致。
