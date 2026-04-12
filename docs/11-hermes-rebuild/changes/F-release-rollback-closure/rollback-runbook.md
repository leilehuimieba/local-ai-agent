# 回滚预案（F-03）

## 1. 触发条件

1. 发布后 `gateway/runtime` 健康检查失败且 15 分钟未恢复。
2. 主链路接口不可用，影响当前发布可用性。
3. 发布候选回归出现阻断级问题（P0/P1）。

## 2. 回滚执行步骤

1. 停止当前进程：
   - 停止监听发布端口的网关与运行时进程。
2. 确认备份版本：
   - 定位 `backups/backup-<timestamp>` 最近稳定版本。
3. 切换版本：
   - 用备份目录覆盖 `current`，并更新 `current-version.txt`。
4. 重启系统：
   - 通过 `launcher.exe` 或 `start-agent.ps1` 启动。
5. 复测健康：
   - `gateway /health`、`runtime /health`、`/api/v1/system/info`。
6. 重跑发布窗口复核：
   - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate -ReleaseWindow -RequirePass`
   - 一键审计模式（推荐）：
   - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate -ReleaseWindow -RequirePass -EmitWarningAuditRecord -WarningAuditOutPath tmp/stage-backend-reverify/warning-audit-from-pack.json -WarningAuditExecutor <owner> -WarningAuditTrackingId <ticket> -WarningAuditDueAt <iso8601> -RequireWarningAuditReady`
   - 读取 `tmp/stage-backend-reverify/latest.json` 的 `failed_checks.count` 与 `non_blocking_warnings.count`。
   - 读取 `evidence.warning_audit_record`，确认回滚后审计快照已落盘。

## 3. 回滚后核验

1. 健康检查恢复为可达。
2. 最小任务执行成功，输出可追溯。
3. `doctor` 结果恢复到 `passed`。
4. 回滚后决策（固定协议）：
   - 阻断态：`failed_checks.count > 0`，回滚结果不通过，继续升级处置。
   - 告警态：`failed_checks.count = 0` 且 `non_blocking_warnings.count > 0`，允许继续服务，但必须登记告警与后续动作。
   - 通过态：`failed_checks.count = 0` 且 `non_blocking_warnings.count = 0`，回滚验收通过。

## 4. 失败升级路径

1. 若单次回滚失败，立即切换到上一个更早稳定备份。
2. 若连续两次失败，停止发布窗口并进入人工值守排障。
3. 排障完成后必须补回滚复盘记录与根因说明。

## 5. 告警登记模板（仅告警态）

1. 检查时间（`checked_at`）：
2. 结论（`status`）：
3. `summary.identity_diff_severity`：
4. `non_blocking_warnings.warning_codes`：
5. `details[].title/description/priority/ui_hint`：
6. `details[].action_label/action_command`：
7. 跟踪项编号（Issue/Ticket）：
8. 责任人：
9. 预计完成时间：
10. 记录生成命令（推荐）：
   - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-warning-audit-record.ps1 -ReportPath tmp/stage-backend-reverify/latest.json -OutPath tmp/stage-backend-reverify/warning-audit-warning.json -Executor <owner> -TrackingId <ticket> -DueAt <iso8601> -RequireReady`

## 6. 审计记录模板

1. 触发时间：
2. 触发人：
3. 故障版本：
4. 回滚目标版本：
5. 触发原因：
6. 回滚结果：
7. 后续动作：
