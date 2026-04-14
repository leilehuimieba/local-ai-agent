# 验证记录

## 验证方式

- 单元测试：
  1. 本刀默认不新增单测；如 doctor 脚本变更再补对应测试记录。
- 集成测试：
  1. `scripts/run-stage-f-doctor-acceptance.ps1`（已执行，最新复核时间 `2026-04-14T21:59:42.4713329+08:00`）。
- 人工验证：
  1. 核对 doctor 10 项检查与日志产物路径可追溯。

## 证据位置

- 测试记录：
  1. `tmp/stage-f-doctor/latest.json`（`checked_at=2026-04-14T21:59:42.4713329+08:00`，`status=passed`）
- 日志或截图：
  1. `scripts/doctor.ps1`
  2. `scripts/run-stage-f-doctor-acceptance.ps1`
  3. `docs/11-hermes-rebuild/changes/F-doctor-core-checks-20260414/artifacts/T02-doctor-check-summary-20260414.md`
  4. `tmp/stage-f-doctor/logs/runtime.log`
  5. `tmp/stage-f-doctor/logs/gateway.log`
  6. `tmp/stage-f-doctor/logs/runtime-build.stdout.log`
  7. `tmp/stage-f-doctor/logs/runtime-build.stderr.log`
  8. `tmp/stage-f-doctor/logs/gateway-build.stdout.log`
  9. `tmp/stage-f-doctor/logs/gateway-build.stderr.log`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-F（执行中）
- 当前覆盖情况：
  1. doctor 核心检查均通过：`go/rust/node/npm/config/ports/frontend/runtime/gateway/logs` 全为 `true`。
  2. 本 change 已完成 `T01-T03` 并形成可复用证据，不做 Gate-F 完成声明。
