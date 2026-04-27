# T02 发布候选回归摘要（2026-04-14）

更新时间：2026-04-14  
范围：`F-release-candidate-regression-20260414` / `F-03`

## 1. 验收命令

1. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-rc-acceptance.ps1 -Rounds 3`

## 2. 结果摘要

1. 报告文件：`tmp/stage-f-rc/latest.json`
2. `checked_at`：`2026-04-14T22:16:49.0986040+08:00`
3. `status`：`passed`
4. 轮次：`rounds=3`
5. 指标：
   - `regression_count=3`，`regression_rate=1.0`
   - `fault_injection_count=3`，`fault_injection_rate=1.0`
   - `round_pass_count=3`，`round_pass_rate=1.0`

## 3. 关键判定

1. `release_candidate.regression_ready=true`
2. `release_candidate.fault_injection_ready=true`
3. `release_candidate.ready=true`

## 4. 轮次检查结论

1. Round 1：install/doctor/entry/consistency/failure 全通过。
2. Round 2：install/doctor/entry/consistency/failure 全通过。
3. Round 3：install/doctor/entry/consistency/failure 全通过。

## 5. 证据路径

1. `tmp/stage-f-rc/latest.json`
2. `tmp/stage-f-install/latest.json`
3. `tmp/stage-f-doctor/latest.json`
4. `tmp/stage-e-entry1/latest.json`
5. `tmp/stage-e-consistency/latest.json`
6. `tmp/stage-e-entry-failure/latest.json`
