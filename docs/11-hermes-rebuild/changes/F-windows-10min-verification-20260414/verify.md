# 验证记录

## 验证方式

- 单元测试：
  1. 本刀默认不新增单测；如验收失败触发脚本修复再补测试记录。
- 集成测试：
  1. `scripts/run-stage-f-windows-acceptance.ps1 -MaxMinutes 10`（已执行，最新复核时间 `2026-04-14T22:29:27.2119836+08:00`）。
- 人工验证：
  1. 核对 `elapsed_ms`、`within_time_budget` 与首任务终态字段。

## 证据位置

- 测试记录：
  1. `tmp/stage-f-windows/latest.json`（`checked_at=2026-04-14T22:29:27.2119836+08:00`，`status=passed`）
  2. `tmp/stage-f-windows/latest.md`
- 日志或截图：
  1. `scripts/run-stage-f-windows-acceptance.ps1`
  2. `scripts/install-local-agent.ps1`
  3. `docs/11-hermes-rebuild/changes/F-windows-10min-verification-20260414/artifacts/T02-windows-10min-summary-20260414.md`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-F（执行中）
- 当前覆盖情况：
  1. `elapsed_ms=20603`，`threshold_ms=600000`，`within_time_budget=true`。
  2. `gateway_ready=true`、`runtime_ready=true`、`first_task_completed=true`。
  3. 首任务终态：`event_type=run_finished` 且 `completion_status=completed`。
  4. 本 change 已完成 `T01-T03`，不做 Gate-F 完成声明。
