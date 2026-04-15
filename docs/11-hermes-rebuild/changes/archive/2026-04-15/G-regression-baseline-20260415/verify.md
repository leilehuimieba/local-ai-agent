# 验证记录

## 验证方式

- 集成验收：
  1. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-regression-baseline.ps1 -Rounds 1 -RequirePass`
  2. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-regression-baseline.ps1 -RefreshEvidence -Rounds 3 -RequirePass`
- 人工验证：
  1. 校验 `tmp/stage-g-regression/latest.json` 中 `status=passed`。
  2. 校验 `summary.pass_rate>=95` 且 `summary.ready=true`。
  3. 校验 `runs[*].route` 与 `failed_samples[*].route` 字段存在。

## 证据位置

- G-03 证据：
  1. `tmp/stage-g-regression/latest.json`
- 依赖快照：
  1. `tmp/stage-e-cli-history/latest.json`
  2. `tmp/stage-e-cli-cancel/latest.json`
  3. `tmp/stage-e-consistency/latest.json`
  4. `tmp/stage-f-gate/latest.json`
  5. `tmp/stage-g-evidence-freshness/latest.json`
  6. `tmp/stage-g-gate/latest.json`
  7. `tmp/stage-g-ops/latest.json`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-G（执行中，未签收）。
- 当前覆盖情况：
  1. 已完成 `G-03` 最小回归基线脚本化与批量取证，当前 `pass_rate=100`。
  2. `G-04/G-G1` 仍待推进。
