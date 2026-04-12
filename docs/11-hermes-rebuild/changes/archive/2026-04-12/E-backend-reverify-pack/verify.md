# 验证记录

## 验证方式

- 一键复核：
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RequirePass`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -RequirePass`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -StrictGate -RequirePass`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate -RequirePass`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate -ReleaseWindow -RequirePass`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -StrictGate -MaxEvidenceAgeMinutes 0`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-warning-sample.ps1`
- 合规复测（2026-04-12）：
  - 工作区改动 Go 文件函数行数扫描（结果：无 `>30` 函数）
  - `go test ./...`（工作目录：`gateway/`）
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-cli-history-acceptance.ps1`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-cli-cancel-acceptance.ps1`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-consistency-acceptance.ps1`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RequirePass`
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -RequirePass`

## 证据位置

- 聚合报告：
  - `tmp/stage-backend-reverify/latest.json`
  - `tmp/stage-backend-reverify/failure-sample.json`
  - `tmp/stage-backend-reverify/warning-sample.json`
- 输入证据：
  - `tmp/stage-e-cli-history/latest.json`
  - `tmp/stage-e-cli-cancel/latest.json`
  - `tmp/stage-e-consistency/latest.json`
  - `tmp/stage-f-gate/latest.json`

## Gate 映射

- 对应阶段 Gate：Gate-E / Gate-F 复核入口（聚合层）。
- 当前覆盖情况：
  - `E-01` 历史与中断切片通过。
  - `E-04` 跨入口一致性通过（同 run/session/trace + 终态一致）。
  - `F-G1` Gate-F 门禁通过。
  - 聚合判定 `backend_reverify_ready=true`。

## 本轮证据快照（2026-04-12）

- `tmp/stage-e-cli-history/latest.json`
  - `checked_at=2026-04-12T20:28:44.9283425+08:00`
  - `status=passed`
  - `checks.logs_runs_view=true`
  - `checks.cli_history_slice_ready=true`
- `tmp/stage-e-cli-cancel/latest.json`
  - `checked_at=2026-04-12T20:28:46.9736666+08:00`
  - `status=passed`
  - `checks.cancel_endpoint_ready=true`
  - `checks.cancel_terminal_status=true`
- `tmp/stage-e-consistency/latest.json`
  - `checked_at=2026-04-12T20:58:09.1159802+08:00`
  - `status=passed`
  - `checks.accepted_id_matched=true`
  - `checks.runtime_result_matched=true`
  - `checks.all_gateway_run_matched=true`
  - `checks.all_gateway_session_matched=true`
  - `checks.terminal_type_matched=true`
  - `checks.terminal_tool_matched=true`
  - `checks.completion_status_matched=true`
  - `checks.gateway_trace_matched=true`
  - `checks.identity_diff_all_matched=true`
  - `checks.identity_diff_severity=ok`
  - `identity_diff_summary.mismatch_count=0`
  - `identity_diff_summary.missing_count=0`
  - `identity_diff_summary.missing_dimensions=`
  - `identity_diff_summary.severity=ok`
- `tmp/stage-backend-reverify/latest.json`
  - `checked_at=2026-04-12T20:58:12.1977439+08:00`
  - `status=passed`
  - `summary.e04_consistency_ready=true`
  - `summary.strict_gate_ready=true`
  - `summary.identity_diff_severity=ok`
  - `summary.non_blocking_warning_count=0`
  - `summary.backend_reverify_ready=true`
  - `failed_checks.count=0`
  - `non_blocking_warnings.count=0`
  - `strict_gate.enabled=true`
  - `strict_gate.release_window=true`
  - `strict_gate.max_evidence_age_minutes=30`

## 接口级证据锚点（E-04）

- `tmp/stage-backend-reverify/latest.json`
  - `interface_evidence.run_identity.run_id=run-e4-1775998685583`
  - `interface_evidence.runtime_terminal.event_type=run_finished`
  - `interface_evidence.gateway_terminal.event_type=run_finished`
  - `interface_evidence.consistency_checks.*=true`
  - `interface_evidence.identity_diff_summary.all_matched=true`
  - `interface_evidence.identity_diff_summary.mismatch_count=0`
  - `interface_evidence.identity_diff_summary.missing_count=0`
  - `interface_evidence.identity_diff_summary.missing_dimensions=`
  - `interface_evidence.identity_diff_summary.severity=ok`
  - `interface_evidence.identity_diff_groups.trace_id.runtime.missing_count=0`
  - `interface_evidence.identity_diff_groups.trace_id.gateway.missing_count=0`

## StrictGate 证据锚点

- `tmp/stage-backend-reverify/latest.json`
  - `strict_gate.checks.evidence_age_within_limit=true`
  - `strict_gate.checks.consistency_identity_present=true`
  - `strict_gate.checks.consistency_terminal_present=true`
  - `strict_gate.checks.gate_fields_complete=true`
  - `strict_gate.evidence_age_minutes.e01_history_age_minutes=0.85`
  - `strict_gate.evidence_age_minutes.e01_cancel_age_minutes=0.75`
  - `strict_gate.evidence_age_minutes.e04_consistency_age_minutes=0.05`
  - `strict_gate.evidence_age_minutes.fg1_gate_age_minutes=0.4`

## 非阻断告警样本（Warn）

- `tmp/stage-backend-reverify/warning-sample.json`（通过将 `tmp/stage-e-consistency/latest.json` 的 `identity_diff_summary.severity` 临时置为 `warn` 生成后留档）
  - `status=passed`
  - `summary.strict_gate_ready=true`
  - `summary.backend_reverify_ready=true`
  - `summary.identity_diff_severity=warn`
  - `summary.non_blocking_warning_count=1`
  - `failed_checks.count=0`
  - `non_blocking_warnings.count=1`
  - `non_blocking_warnings.warning_codes[0]=e04_identity_diff_warn`
  - `non_blocking_warnings.suggestions[0]=Identity diff has non-blocking missing fields.`
  - `non_blocking_warnings.details[0].title=E-04 identity diff warning`
  - `non_blocking_warnings.details[0].description=Identity diff has missing fields but no blocking mismatch.`
  - `non_blocking_warnings.details[0].priority=medium`
  - `non_blocking_warnings.details[0].ui_hint=warning_card`
  - `non_blocking_warnings.details[0].action_label=Refresh E-04 evidence`
  - `non_blocking_warnings.details[0].action_command=powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-consistency-acceptance.ps1`
  - `interface_evidence.run_identity.run_id=run-e4-1775998661643`
  - `checked_at=2026-04-12T20:57:51.4266674+08:00`
  - `interface_evidence.identity_diff_summary.severity=warn`
  - `interface_evidence.identity_diff_summary.missing_count=1`

## 失败路径样本（StrictGate）

- `tmp/stage-backend-reverify/failure-sample.json`（将 `tmp/stage-e-cli-history/latest.json` 的 `checked_at` 回拨到 31 分钟前后执行 `-StrictGate -ReleaseWindow` 留档）
  - 建议触发命令（稳定失败）：
    - `powershell -Command "$p='tmp/stage-e-cli-history/latest.json';$d=Get-Content -Raw $p|ConvertFrom-Json;$d.checked_at=(Get-Date).AddMinutes(-31).ToString('o');$d|ConvertTo-Json -Depth 6|Set-Content -Path $p -Encoding UTF8"`
    - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -StrictGate -ReleaseWindow`
  - `status=failed`
  - `summary.strict_gate_ready=false`
  - `failed_checks.count=1`
  - `failed_checks.reason_codes[0]=strict_evidence_age_exceeded`
  - `failed_checks.suggestions[0]=Evidence age exceeded threshold.`
  - `failed_checks.recommended_commands_minimal[0]=powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-cli-history-acceptance.ps1`
  - `failed_checks.recommended_commands_full_refresh[0]=powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate -ReleaseWindow -RequirePass`
