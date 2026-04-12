# 技术方案

## 影响范围

- 聚合验收脚本：`scripts/run-stage-f-rc-acceptance.ps1`
- 证据输出：`tmp/stage-f-rc/latest.json`
- 状态文档：`docs/11-hermes-rebuild/changes/*`、`stage-plans/全路线最小任务分解总表.md`

## 方案

### 1. 回归链

每轮执行以下脚本并校验通过：

1. `run-stage-f-install-acceptance.ps1`（安装/升级回归）
2. `run-stage-f-doctor-acceptance.ps1`（诊断回归）
3. `run-stage-e-entry1-acceptance.ps1`（核心入口成功链）
4. `run-stage-e-consistency-acceptance.ps1`（跨入口一致性链）

### 2. 故障注入链

每轮执行：

1. `run-stage-e-entry-failure-acceptance.ps1`

要求：

1. 存在 `run_failed` 与 `run_finished` 失败收口链。
2. `error_code=runtime_unavailable` 可复核。

### 3. 聚合判定

- 脚本支持 `-Rounds`，本轮按 `3` 轮执行。
- 汇总字段：
  - `regression_rate`
  - `fault_injection_rate`
  - `round_pass_rate`
  - `release_candidate.ready`
- 判定条件：
  - 每轮通过且最终 `status=passed`。

## 风险与回退

- 风险：当前是最小可行样本规模，不能替代长时压力验证。
- 缓解：`F-05` 补新机实机时长维度验证，`F-G1` 再做阶段级评审。
- 回退：若聚合脚本失败，按 `rounds_detail` 定位失败子链，单链回退修复后再重跑。
