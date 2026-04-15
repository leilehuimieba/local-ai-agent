# G-G1 Gate 签收演练记录（2026-04-15）

- 执行时间：2026-04-15 11:42 ~ 11:43 (UTC+08:00)
- 执行人：codex-g-signoff
- 命令：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-signoff-acceptance.ps1 -RequireGateG`

## 1. 结果摘要

1. `tmp/stage-g-signoff/latest.json`：`status=passed`，`gate_g_signoff.ready=true`
2. `tmp/stage-g-gate/latest.json`：`gate_g.ready=true`
3. `tmp/stage-g-ops/latest.json`：`checks.governance_ready=true`
4. `tmp/stage-g-regression/latest.json`：`summary.pass_rate=100`，`summary.ready=true`

## 2. 关键字段

1. `warning_audit_fields_ready=true`
2. `no_open_p0_p1=true`
3. `g01_ready/g02_ready/g03_ready=true`

## 3. 证据路径

1. `tmp/stage-g-signoff/latest.json`
2. `tmp/stage-g-gate/latest.json`
3. `tmp/stage-g-ops/latest.json`
4. `tmp/stage-g-regression/latest.json`
5. `tmp/stage-g-evidence-freshness/warning-audit-latest.json`
