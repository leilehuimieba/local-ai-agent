# 任务清单

- [x] 新增后端一键复核脚本
  完成判据：`run-stage-backend-reverify-pack.ps1` 可执行并输出聚合报告。
- [x] 跑通聚合复核
  完成判据：`tmp/stage-backend-reverify/latest.json` 为 `status=passed`。
- [x] 同步脚本文档
  完成判据：`scripts/README.md` 已登记脚本用途和证据路径。
- [x] 接入发布与回滚清单
  完成判据：`release-checklist.md` 与 `rollback-runbook.md` 已引用一键复核包。
- [x] 后端函数长度合规复核与修正
  完成判据：新增/修改函数满足 30 行约束，且 `go test ./...` 与一键复核链路通过。
- [x] 复核包纳入 E-04 一致性链
  完成判据：`latest.json` 包含 `e04_consistency_ready` 与接口级证据摘要字段。
- [x] 新增 StrictGate 严格门禁
  完成判据：支持 `-StrictGate -MaxEvidenceAgeMinutes`，并在报告输出 `strict_gate` 结构与判定结果。
- [x] 新增失败原因聚合输出
  完成判据：报告包含 `failed_checks.count/reason_codes`，严格门禁失败可直接定位 `reason_code`。
- [x] 新增失败修复建议输出
  完成判据：报告包含 `failed_checks.suggestions`，每个 `reason_code` 均有对应建议。
- [x] 新增失败修复命令数组输出
  完成判据：报告包含 `failed_checks.recommended_commands`，失败时可直接复制执行修复命令。
- [x] 失败修复命令分层输出
  完成判据：报告包含 `recommended_commands_minimal/full_refresh` 两组命令，兼容保留 `recommended_commands`。
- [x] 严格时效失败的定向修复命令
  完成判据：`strict_evidence_age_exceeded` 时 `recommended_commands_minimal` 仅包含过期证据对应脚本。
- [x] 发布窗口参数与证据口径
  完成判据：支持 `-ReleaseWindow`（30 分钟阈值），并通过 `-RefreshEvidence -StrictGate -ReleaseWindow` 复核。
- [x] 失败样本可复核触发链收口
  完成判据：`failure-sample.json` 可通过“回拨单项证据时间”稳定触发，且 `recommended_commands_full_refresh` 与发布窗口口径一致。
- [x] E-04 身份差异分组透传
  完成判据：`latest.json` 的 `interface_evidence` 包含 `identity_diff_summary/identity_diff_groups`，可直接按 `run_id/session_id/trace_id` 展示冲突与缺失计数。
- [x] Gateway 日志链 trace_id 补齐
  完成判据：`/api/v1/logs` 输出包含 `trace_id`，且 E-04 报告 `identity_diff_summary.missing_count=0`。
- [x] E-04 差异分级字段补齐
  完成判据：E-04 报告输出 `identity_diff_summary.severity` 与 `checks.identity_diff_severity`（`ok/warn/error`），前端无需二次推断即可渲染状态。
- [x] 复核包非阻断告警区
  完成判据：`latest.json` 输出 `non_blocking_warnings`（与 `failed_checks` 并列），`severity=warn` 时给出告警码和建议，且不阻断 `backend_reverify_ready`。
- [x] 告警详情字段标准化
  完成判据：`non_blocking_warnings.details` 固定输出 `title/description/action_label/action_command`，前端可直接渲染操作引导。
- [x] 告警展示提示字段补齐
  完成判据：`non_blocking_warnings.details` 新增 `priority/ui_hint`，前端可直接完成排序与样式映射。
- [x] 告警样本生成脚本化
  完成判据：提供 `run-stage-backend-reverify-warning-sample.ps1` 一键生成 `warning-sample.json`，并自动恢复 `latest.json` 为 passed。
