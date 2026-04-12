# 验证记录

## 验证方式

- 文档复核：
  - 检查 `release-checklist.md` 与 `rollback-runbook.md` 的字段完整性。
- 口径一致性复核：
  - 对照 `F-01/F-02` 已落地脚本与证据路径，确认文档不偏离实际执行链。
  - 对照后端复核包 StrictGate + ReleaseWindow 参数，确认发布前与回滚后都采用发布窗口严格门禁命令。
- 三态样本复核：
  - 通过态：`tmp/stage-backend-reverify/latest.json`
    - `status=passed`
    - `failed_checks.count=0`
    - `non_blocking_warnings.count=0`
  - 告警态：`tmp/stage-backend-reverify/warning-sample.json`
    - `status=passed`
    - `summary.identity_diff_severity=warn`
    - `failed_checks.count=0`
    - `non_blocking_warnings.count=1`
    - `non_blocking_warnings.warning_codes[0]=e04_identity_diff_warn`
    - `non_blocking_warnings.details[0].priority=medium`
    - `non_blocking_warnings.details[0].ui_hint=warning_card`
  - 阻断态：`tmp/stage-backend-reverify/failure-sample.json`
    - `status=failed`
    - `failed_checks.count=1`
    - `failed_checks.reason_codes[0]=strict_evidence_age_exceeded`
- 审计记录脚本复核：
  - 通过态命令：
    - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-warning-audit-record.ps1 -ReportPath tmp/stage-backend-reverify/latest.json -OutPath tmp/stage-backend-reverify/warning-audit-latest.json -RequireReady`
    - 结果：`decision=pass`、`ready_for_release=true`
  - 告警态命令：
    - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-warning-audit-record.ps1 -ReportPath tmp/stage-backend-reverify/warning-sample.json -OutPath tmp/stage-backend-reverify/warning-audit-warning.json -Executor release-ops -TrackingId OPS-20260412-WARN-01 -DueAt 2026-04-13T18:00:00+08:00 -RequireReady`
    - 结果：`decision=warn`、`ready_for_release=true`、`warning_codes[0]=e04_identity_diff_warn`
  - 阻断态命令：
    - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-warning-audit-record.ps1 -ReportPath tmp/stage-backend-reverify/failure-sample.json -OutPath tmp/stage-backend-reverify/warning-audit-failure.json`
    - 结果：`decision=blocked`、`ready_for_release=false`
- 复核包一键审计复核：
  - 命令：
    - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -StrictGate -ReleaseWindow -RequirePass -EmitWarningAuditRecord -WarningAuditOutPath tmp/stage-backend-reverify/warning-audit-from-pack.json -RequireWarningAuditReady`
  - 结果：
    - 终端输出同时返回 `latest.json` 与 `warning-audit-from-pack.json` 路径。
    - `tmp/stage-backend-reverify/latest.json` 中 `evidence.warning_audit_record` 已回写为审计快照路径。
    - `tmp/stage-backend-reverify/warning-audit-from-pack.json` 中 `decision=pass`、`ready_for_release=true`。

## 证据位置

- 发布清单：
  - `docs/11-hermes-rebuild/changes/F-release-rollback-closure/release-checklist.md`
- 回滚预案：
  - `docs/11-hermes-rebuild/changes/F-release-rollback-closure/rollback-runbook.md`
- 一键复核聚合报告：
  - `tmp/stage-backend-reverify/latest.json`
- 告警样本：
  - `tmp/stage-backend-reverify/warning-sample.json`
- 阻断样本：
  - `tmp/stage-backend-reverify/failure-sample.json`
- 审计快照样本：
  - `tmp/stage-backend-reverify/warning-audit-latest.json`
  - `tmp/stage-backend-reverify/warning-audit-warning.json`
  - `tmp/stage-backend-reverify/warning-audit-failure.json`
  - `tmp/stage-backend-reverify/warning-audit-from-pack.json`
- 关联样本：
  - `tmp/stage-f-install/latest.json`
  - `tmp/stage-f-doctor/latest.json`
- 告警样本生成脚本：
  - `scripts/run-stage-backend-reverify-warning-sample.ps1`
- 审计记录脚本：
  - `scripts/run-stage-f-warning-audit-record.ps1`
- 一键复核包（含审计参数）：
  - `scripts/run-stage-backend-reverify-pack.ps1`

## Gate 映射

- 对应阶段 Gate：Gate-F（已在 `F-G1` 提审签收）。
- 当前覆盖情况：
  - 已完成 `F-03` 的发布与回滚文档收口。
  - 已完成 warning 协议接入，形成“可发布但需记录告警”固定流程。
  - 后续 `F-04`、`F-05` 与 `F-G1` 已完成，见 `changes/F-gate-f-signoff/review.md`。
