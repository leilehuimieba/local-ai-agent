# 技术方案

## 影响范围

- 聚合脚本：`scripts/run-stage-backend-reverify-pack.ps1`
- 聚合证据：`tmp/stage-backend-reverify/latest.json`
- 脚本登记：`scripts/README.md`

## 方案

### 1. 复核输入

- `tmp/stage-e-cli-history/latest.json`
- `tmp/stage-e-cli-cancel/latest.json`
- `tmp/stage-e-consistency/latest.json`
- `tmp/stage-f-gate/latest.json`

### 2. 复核规则

- `e01_history_ready`：历史切片报告通过且关键检查为 true。
- `e01_cancel_ready`：中断切片报告通过且关键检查为 true。
- `e04_consistency_ready`：一致性报告通过且 run/session/trace 与终态一致性检查为 true。
- `fg1_gate_ready`：Gate-F 报告通过且 `gate_f.ready=true`。
- 四项都通过则 `backend_reverify_ready=true`。
- 聚合报告透传接口级证据摘要：`run_identity`、runtime/gateway 终态事件与一致性检查结果。
- 聚合报告透传一致性差异分组：`identity_diff_summary`、`identity_diff_groups`（`run_id/session_id/trace_id` 的冲突与缺失计数）与 `severity`（`ok/warn/error`）。
- 一致性差异以 `/api/v1/logs` 为 gateway 侧输入时，要求日志项透传 `trace_id`，避免将字段缺失误判为身份冲突。

### 2.1 严格门禁（StrictGate）

- 通过 `-StrictGate` 启用。
- 可叠加 `-ReleaseWindow` 启用发布窗口口径，证据时效阈值固定为 30 分钟。
- 证据时效：
  - 对 `E-01` 历史、`E-01` 中断、`E-04` 一致性、`F-G1` 门禁四份输入证据读取 `checked_at`。
  - 默认要求证据年龄 `<=180` 分钟（可通过 `-MaxEvidenceAgeMinutes` 调整）。
- 结构完整性：
  - 一致性证据必须含 `run_identity.request_id/run_id/session_id/trace_id`。
  - 一致性证据必须含 runtime 与 gateway 两侧终态节点。
  - Gate-F 证据必须含 `install_ready/doctor_ready/release_candidate_ready/windows_10min_ready/no_open_p0_p1/ready`。
- 严格模式下，`strict_gate_ready=true` 才能判定 `backend_reverify_ready=true`。
- 无论严格模式是否开启，聚合报告都输出 `failed_checks.reason_codes`，用于失败定位。
  - 示例：`strict_evidence_age_exceeded`、`e04_consistency_not_ready`。
- 聚合报告同步输出：
  - `failed_checks.suggestions`：失败项的人类可读建议。
  - `failed_checks.recommended_commands_minimal`：最小修复命令数组（按失败项定向修复）。
  - `failed_checks.recommended_commands_full_refresh`：全量刷新命令数组（统一重跑后再严格校验）。
  - `failed_checks.recommended_commands`：兼容字段，当前与 `recommended_commands_minimal` 等价。
  - `non_blocking_warnings`：非阻断告警区（与 `failed_checks` 并列），用于承接 `severity=warn` 的提示信息。
  - `non_blocking_warnings.details`：固定输出 `title/description/priority/ui_hint/action_label/action_command`，前端无需二次编排告警文案。
- 对 `strict_evidence_age_exceeded`：
  - `recommended_commands_minimal` 只包含过期证据对应脚本（历史/中断/一致性/Gate-F）。
  - `recommended_commands_full_refresh` 固定为复核包全刷新严格校验命令。
  - 在 `-ReleaseWindow` 下，最小修复命令会基于 30 分钟阈值动态收敛到过期项。

### 3. 复跑模式

- 默认读取现有证据聚合。
- `-RefreshEvidence` 时先重跑四个子脚本，再聚合。
- `-StrictGate` 可与 `-RefreshEvidence` 组合使用，形成“全刷新 + 严格校验”。
- `run-stage-backend-reverify-warning-sample.ps1` 可一键生成 `warning-sample.json`，用于 `passed+warn` 路径回归，同时自动恢复 `latest.json` 到标准通过态。

## 风险与回退

- 风险：聚合层只反映输入证据质量，不能替代子脚本自身验证。
- 缓解：保留子脚本原始报告路径并在聚合报告中透传。
- 回退：若聚合层异常，仍可直接按子脚本逐项复跑。
