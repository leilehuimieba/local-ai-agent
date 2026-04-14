# 验证记录

## 验证方式

- 单元测试：
  1. 本刀默认不新增单测；若脚本修复引入新函数再补对应测试记录。
- 集成测试：
  1. `scripts/run-stage-f-rc-acceptance.ps1 -Rounds 3`（已执行，最新复核时间 `2026-04-14T22:16:49.0986040+08:00`）。
- 人工验证：
  1. 核对 `rounds_detail` 中 install/doctor/entry/consistency/failure 五类检查结果与时间戳连续性。

## 证据位置

- 测试记录：
  1. `tmp/stage-f-rc/latest.json`（`checked_at=2026-04-14T22:16:49.0986040+08:00`，`status=passed`）
- 日志或截图：
  1. `scripts/run-stage-f-rc-acceptance.ps1`
  2. `docs/11-hermes-rebuild/changes/F-release-candidate-regression-20260414/artifacts/T02-rc-round-summary-20260414.md`
  3. `tmp/stage-f-install/latest.json`
  4. `tmp/stage-f-doctor/latest.json`
  5. `tmp/stage-e-entry1/latest.json`
  6. `tmp/stage-e-consistency/latest.json`
  7. `tmp/stage-e-entry-failure/latest.json`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-F（执行中）
- 当前覆盖情况：
  1. `round_pass_rate=1.0`，`regression_rate=1.0`，`fault_injection_rate=1.0`。
  2. `release_candidate.regression_ready=true`、`fault_injection_ready=true`、`ready=true`。
  3. 本 change 已完成 `T01-T03`，不做 Gate-F 完成声明。
