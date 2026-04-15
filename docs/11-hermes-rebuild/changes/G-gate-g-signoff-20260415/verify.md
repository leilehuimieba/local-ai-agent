# 验证记录

## 验证方式

- 集成验收：
  1. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-gate-acceptance.ps1 -RefreshEvidence -RequireGateF`
  2. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-signoff-acceptance.ps1 -RequireGateG`
- 人工验证：
  1. 校验 `tmp/stage-g-signoff/latest.json` 中 `status=passed` 与 `gate_g_signoff.ready=true`。
  2. 校验 `gate_g_signoff.g01_ready/g02_ready/g03_ready=true`。
  3. 校验 `gate_g_signoff.warning_audit_fields_ready=true` 与 `gate_g_signoff.no_open_p0_p1=true`。

## 证据位置

- G-G1 聚合证据：
  1. `tmp/stage-g-signoff/latest.json`
- 依赖证据：
  1. `tmp/stage-f-gate/latest.json`
  2. `tmp/stage-g-gate/latest.json`
  3. `tmp/stage-g-ops/latest.json`
  4. `tmp/stage-g-regression/latest.json`
  5. `tmp/stage-g-evidence-freshness/warning-audit-latest.json`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-G（已签收）。
- 当前覆盖情况：
  1. `G-01~G-03` 聚合就绪且 warning 审计字段完整。
  2. `G-04` 无阻塞，`G-G1` 签收通过。
