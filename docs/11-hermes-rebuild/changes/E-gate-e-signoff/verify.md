# 验证记录

## 验证方式

- Gate-E 批量验收：
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-gate-batch.ps1 -Rounds 5 -RequireGateE`
- 抽样验收（由批量脚本内部串行调用）：
  - `scripts/run-stage-e-entry1-acceptance.ps1`
  - `scripts/run-stage-e-consistency-acceptance.ps1`
  - `scripts/run-stage-e-entry-failure-acceptance.ps1`
- 后端回归：
  - `go test ./...`（`gateway/`）

## 证据位置

- 批量报告：
  - `tmp/stage-e-batch/latest.json`
- 子样本报告：
  - `tmp/stage-e-entry1/latest.json`
  - `tmp/stage-e-consistency/latest.json`
  - `tmp/stage-e-entry-failure/latest.json`
- 脚本：
  - `scripts/run-stage-e-gate-batch.ps1`

## Gate 映射

- 对应阶段 Gate：Gate-E。
- 当前覆盖情况：
  - `entry_rate=1.0`（阈值 `>=0.95`）
  - `consistency_rate=1.0`（阈值 `>=0.95`）
  - `failure_closure_rate=1.0`（阈值 `>=0.95`）
  - `rounds=5`（满足最小轮次要求）
  - `gate_e.ready=true`
