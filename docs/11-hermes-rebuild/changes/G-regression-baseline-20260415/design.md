# 技术方案

## 影响范围

- 涉及模块：
  1. 脚本层：阶段 G 回归基线聚合入口。
- 涉及文档或 contract：
  1. `scripts/run-stage-g-regression-baseline.ps1`
  2. `tmp/stage-g-regression/latest.json`
  3. `docs/11-hermes-rebuild/stage-plans/G-最小回归基线清单.md`
  4. `docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`
  5. `docs/11-hermes-rebuild/changes/G-regression-baseline-20260415/*`

## 方案

- 核心做法：
  1. 调整 `run-stage-g-regression-baseline.ps1` 为“先可选刷新，再按轮次校验快照”，避免每轮重复触发高成本链路。
  2. `-RefreshEvidence` 时仅在首轮刷新 E/G 关键证据，并记录 `refresh_log`。
  3. 每轮读取 E/F/G `latest.json` 快照，聚合判定 `samplePass`，同时输出失败分流 `route`（E/F/G）。
  4. 报告统一落盘到 `tmp/stage-g-regression/latest.json`，包含 `mode/summary/failed_samples/runs`。
- 状态流转或调用链变化：
  1. 新链路：`G-03 baseline -> (optional refresh) -> snapshot verify -> route classify`。
  2. 失败定位从“单一 failed”升级为“按 E/F/G 分流定位”。

## 风险与回退

- 主要风险：
  1. 上游快照被并行任务污染可能导致本轮判定抖动。
  2. `-RefreshEvidence` 全链路执行耗时较长，若运行窗口不足会影响连续轮次。
- 回退方式：
  1. 若回归脚本异常，回退到读取既有 `tmp/stage-g-regression/latest.json` 的 snapshot 校验模式。
  2. 若上游链路短时不稳，先固定 `Rounds=1` 收敛，再扩到 `Rounds=3` 取证。
