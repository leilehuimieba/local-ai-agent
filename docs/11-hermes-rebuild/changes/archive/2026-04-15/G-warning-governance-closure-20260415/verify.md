# 验证记录

## 验证方式

- 集成验收：
  1. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-warning-governance.ps1 -RefreshEvidence -WarningAuditExecutor g-duty -WarningAuditTrackingId G02-20260415-001 -WarningAuditDueAt 2026-04-15T18:00:00+08:00 -RequirePass`
- 人工验证：
  1. 校验 `tmp/stage-g-ops/latest.json` 中 `status=passed`。
  2. 校验 `tmp/stage-g-ops/warning-tracker.json` 的 `updated_at/history` 已更新。

## 证据位置

- G-02 证据：
  1. `tmp/stage-g-ops/latest.json`
  2. `tmp/stage-g-ops/warning-tracker.json`
- 上游证据：
  1. `tmp/stage-g-evidence-freshness/latest.json`
  2. `tmp/stage-g-evidence-freshness/warning-audit-latest.json`
  3. `tmp/stage-backend-reverify/latest.json`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-G（执行中，未签收）。
- 当前覆盖情况：
  1. 已完成 `G-02` 告警治理闭环入口与首轮证据。
  2. `G-03/G-04/G-G1` 仍待推进。
