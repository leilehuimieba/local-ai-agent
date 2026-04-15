# 变更提案

## 背景

- 本次变更要解决的问题：
  1. 阶段 G 已完成 `G-01/G-02`，但 `G-03` 最小回归基线仍缺独立变更与稳定汇总入口。
  2. 现有 `run-stage-g-regression-baseline.ps1` 曾因重复重建上游链路导致执行抖动，历史报告存在失败样本。
  3. Gate-G 需要“可复跑且可分流”的最小回归证据，作为后续 `G-04/G-G1` 的量化基础。
- 对应阶段目标：
  1. 完成 `G-03`：最小回归基线与失败分流规则落地。

## 目标

- 本次要完成什么：
  1. 稳定化 `scripts/run-stage-g-regression-baseline.ps1`，区分 `snapshot` 与 `refresh_then_verify` 两种执行模式。
  2. 输出 `tmp/stage-g-regression/latest.json`，包含通过率、失败分流路由与证据引用。
  3. 完成 `Rounds=1` 与 `Rounds=3` 的验收复跑，达到通过率阈值（>=95%）。
  4. 回写阶段 G 文档与本 change 五件套，形成 G-03 可提审口径。

## 非目标

- 本次明确不做什么：
  1. 不推进 `G-04` 运行手册职责细化实现。
  2. 不做 Gate-G 最终签收决策（`G-G1`）。
  3. 不改 `runtime-core/gateway/frontend` 主业务代码。

## 验收口径

- 通过标准：
  1. `scripts/run-stage-g-regression-baseline.ps1` 可执行，并支持 `-RefreshEvidence -Rounds <n> -RequirePass`。
  2. `tmp/stage-g-regression/latest.json` 为 `passed`，`summary.pass_rate>=95` 且 `summary.ready=true`。
  3. 报告包含失败分流字段（`route`）与完整证据落点。
  4. 文档状态与当前阶段口径一致，且活跃 change 已切换到本目录。
