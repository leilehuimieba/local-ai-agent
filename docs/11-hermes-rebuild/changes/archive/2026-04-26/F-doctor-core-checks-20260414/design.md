# 技术方案

## 影响范围

- 涉及模块：
  1. `scripts/doctor.ps1`
  2. `scripts/run-stage-f-doctor-acceptance.ps1`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/F-doctor-core-checks-20260414/*`
  4. `tmp/stage-f-doctor/latest.json`

## 方案

- 核心做法：
  1. 复用 `run-stage-f-doctor-acceptance.ps1` 作为唯一验收入口，固定 doctor 自检基线。
  2. 以验收脚本返回结果为准，核对 10 项核心检查：`go/rust/node/npm/config/ports/frontend/runtime/gateway/logs`。
  3. 若验收失败，仅做最小修复（脚本缺陷或诊断输出缺失），不扩展到新能力。
- 状态流转或调用链变化：
  1. 本刀优先文档收口与验收证据，不引入 runtime/gateway 结构性改动。
  2. `F-02` 通过后再进入 `F-03` 发布候选与故障注入验收链路。

## 风险与回退

- 主要风险：
  1. 本地依赖版本或端口冲突导致 doctor 误报失败。
  2. 诊断脚本通过但日志证据不完整，影响 Gate-F 组合验收可追溯性。
- 回退方式：
  1. 验收失败时冻结当前主线，仅修复 doctor 核心链路后重跑。
  2. 如短时环境抖动，保留失败样本与日志，不直接变更 Gate 结论。
