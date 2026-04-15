# 变更提案

## 背景

- 本次变更要解决的问题：
  1. 阶段 G 的 `G-01~G-04` 已完成，但尚未形成 `G-G1` 独立签收包。
  2. 当前缺少统一 Gate-G 聚合签收脚本，导致阶段签收仍依赖人工拼接证据。
  3. 需要一次性完成“聚合复核 + 签收结论 + 状态切换”闭环，避免阶段口径漂移。
- 对应阶段目标：
  1. 完成 `G-G1`：Gate-G 评审与阶段签收。

## 目标

- 本次要完成什么：
  1. 新增 `scripts/run-stage-g-signoff-acceptance.ps1`，统一聚合 `G-01/G-02/G-03` 与阻塞项检查。
  2. 执行 Gate-G 聚合复核并产出 `tmp/stage-g-signoff/latest.json`。
  3. 输出 `G-G1` 评审结论并将主线状态切换到“阶段 G / Gate-G（已签收）”。
  4. 同步更新 `current-state.md`、`changes/INDEX.md` 与最小任务总表。

## 非目标

- 本次明确不做什么：
  1. 不新增运行时或前端功能。
  2. 不提前切换到下一阶段执行。
  3. 不修改阶段 G 既有 Gate 指标定义。

## 验收口径

- 通过标准：
  1. `scripts/run-stage-g-signoff-acceptance.ps1 -RequireGateG` 执行通过。
  2. `tmp/stage-g-signoff/latest.json` 中 `status=passed` 且 `gate_g_signoff.ready=true`。
  3. `review.md` 给出明确签收结论与后续建议。
  4. `current-state.md` 显示 `Gate-G（已签收）`，且与 `INDEX.md` 口径一致。
