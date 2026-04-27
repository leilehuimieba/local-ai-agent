# 当前状态

- 最近更新时间：2026-04-14
- 状态：已完成（`T01-T03` 已完成）
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`
- 已完成：
  1. 已创建 `F-release-candidate-regression-20260414` 并补齐五件套。
  2. 已冻结 `F-03` 验收入口为 `scripts/run-stage-f-rc-acceptance.ps1 -Rounds 3`。
  3. 已执行 `scripts/run-stage-f-rc-acceptance.ps1 -Rounds 3`，`tmp/stage-f-rc/latest.json` 为 `passed`。
  4. 已沉淀轮次摘要证据：`artifacts/T02-rc-round-summary-20260414.md`。
- 进行中：
  1. 无。
- 阻塞点：
  1. 无硬阻塞。
- 下一步：
  1. 进入 `F-05` Windows 10 分钟验收链路（`scripts/run-stage-f-windows-acceptance.ps1`）。
  2. 在 Gate-F 汇总验收中复用 `tmp/stage-f-rc/latest.json` 作为发布候选证据输入。
