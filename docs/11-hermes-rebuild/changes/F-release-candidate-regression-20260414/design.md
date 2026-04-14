# 技术方案

## 影响范围

- 涉及模块：
  1. `scripts/run-stage-f-rc-acceptance.ps1`
  2. `scripts/run-stage-f-install-acceptance.ps1`
  3. `scripts/run-stage-f-doctor-acceptance.ps1`
  4. `scripts/run-stage-e-entry1-acceptance.ps1`
  5. `scripts/run-stage-e-consistency-acceptance.ps1`
  6. `scripts/run-stage-e-entry-failure-acceptance.ps1`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/F-release-candidate-regression-20260414/*`
  4. `tmp/stage-f-rc/latest.json`

## 方案

- 核心做法：
  1. 以 `run-stage-f-rc-acceptance.ps1 -Rounds 3` 作为 `F-03` 唯一验收入口。
  2. 每轮串行复用 install/doctor/entry/consistency/failure 五类验收，避免引入新判定逻辑。
  3. 以脚本输出字段判定是否通过：
     - `regression_ready=true`
     - `fault_injection_ready=true`
     - `ready=true`
- 状态流转或调用链变化：
  1. 本刀只消费现有脚本与证据，不改 runtime/gateway 实现。
  2. `F-03` 通过后，下一步进入 `F-05`（Windows 10 分钟验收）或 Gate-F 汇总验收。

## 风险与回退

- 主要风险：
  1. 多脚本串行执行可能受端口/环境抖动影响导致偶发失败。
  2. 复用阶段 E 脚本时若环境脏状态残留，可能误伤 RC 统计。
- 回退方式：
  1. 若单轮失败，先保留 `rounds_detail` 失败样本，再按失败脚本最小修复后重跑。
  2. 若连续失败且无明确根因，冻结 `F-03` 并回退到最近一次通过证据，不推进 Gate-F 结论。
