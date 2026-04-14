# 验证记录

## 验证方式

- 集成验收：
  1. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-evidence-freshness.ps1 -RefreshEvidence -WarningAuditExecutor g-duty -WarningAuditTrackingId G01-20260414-001 -WarningAuditDueAt 2026-04-15T10:00:00+08:00`
  2. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-gate-acceptance.ps1 -RefreshEvidence -WarningAuditExecutor g-duty -WarningAuditTrackingId G01-20260414-002 -WarningAuditDueAt 2026-04-15T10:30:00+08:00 -RequirePass`
- 人工验证：
  1. 校验 `stage-g-gate/latest.json` 的 `gate_g.ready=true`。
  2. 校验 `warning-audit-latest.json` 可读且字段完整。

## 证据位置

- 阶段 G 证据：
  1. `tmp/stage-g-evidence-freshness/latest.json`
  2. `tmp/stage-g-evidence-freshness/warning-audit-latest.json`
  3. `tmp/stage-g-gate/latest.json`
- 上游复核：
  1. `tmp/stage-backend-reverify/latest.json`
- 文档交付：
  1. `docs/11-hermes-rebuild/stage-plans/G-证据保鲜策略.md`
  2. `docs/11-hermes-rebuild/stage-plans/G-发布后巡检与告警治理.md`
  3. `docs/11-hermes-rebuild/stage-plans/G-最小回归基线清单.md`
  4. `docs/11-hermes-rebuild/stage-plans/G-运行手册与值守规范.md`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-G（执行中，未签收）。
- 当前覆盖情况：
  1. 已完成 `G-01` 的入口建设与首轮通过证据。
  2. `G-02/G-03/G-04/G-G1` 仍待后续 change 推进。
