# 验证记录

## 验证方式

- 单元测试：
  1. 本刀不新增单测；聚合脚本以执行验收作为主验证。
- 集成测试：
  1. `scripts/run-stage-f-gate-acceptance.ps1 -RequireGateF`（已执行，最新复核时间 `2026-04-14T22:40:25.7301978+08:00`）。
- 人工验证：
  1. 核对 `blocker_checks` 中四个 change 状态文档均 `exists=true` 且 `no_blocker=true`。

## 证据位置

- 测试记录：
  1. `tmp/stage-f-gate/latest.json`（`checked_at=2026-04-14T22:40:25.7301978+08:00`，`status=passed`）
- 日志或截图：
  1. `scripts/run-stage-f-gate-acceptance.ps1`
  2. `docs/11-hermes-rebuild/changes/F-gate-f-signoff-20260414/artifacts/T02-gate-f-summary-20260414.md`
  3. `tmp/stage-f-install/latest.json`
  4. `tmp/stage-f-doctor/latest.json`
  5. `tmp/stage-f-rc/latest.json`
  6. `tmp/stage-f-windows/latest.json`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-F（已签收）
- 当前覆盖情况：
  1. `gate_f.install_ready=true`、`doctor_ready=true`、`release_candidate_ready=true`、`windows_10min_ready=true`。
  2. `gate_f.no_open_p0_p1=true`、`gate_f.ready=true`。
  3. 本 change 已完成本轮 Gate-F 提审签收，不做下一阶段实现声明。
