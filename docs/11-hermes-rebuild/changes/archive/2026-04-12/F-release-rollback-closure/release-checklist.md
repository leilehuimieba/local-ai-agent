# 发布清单（F-03）

## 1. 发布前检查

1. 一键复核包执行成功：
   - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate -ReleaseWindow -RequirePass`
   - 发布窗口推荐命令（含审计快照）：
   - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate -ReleaseWindow -RequirePass -EmitWarningAuditRecord -WarningAuditOutPath tmp/stage-backend-reverify/warning-audit-from-pack.json -WarningAuditExecutor <owner> -WarningAuditTrackingId <ticket> -WarningAuditDueAt <iso8601> -RequireWarningAuditReady`
   - 读取 `tmp/stage-backend-reverify/latest.json` 的三态字段：
   - `failed_checks.count`
   - `non_blocking_warnings.count`
   - 读取 `evidence.warning_audit_record`，确认审计快照已产出。
2. 发布放行决策（固定协议）：
   - 阻断态：`failed_checks.count > 0`，不得发布。
   - 告警态：`failed_checks.count = 0` 且 `non_blocking_warnings.count > 0`，允许发布但必须先完成告警登记。
   - 通过态：`failed_checks.count = 0` 且 `non_blocking_warnings.count = 0`，正常放行。
3. `tmp/stage-f-install/latest.json` 为 `status=passed`。
4. `tmp/stage-f-doctor/latest.json` 为 `status=passed`。
5. `config/app.json` 可解析，`gateway_port/runtime_port` 合法。
6. `frontend/dist/index.html` 存在。
7. 发布目录具备以下最小产物：
   - `target/debug/runtime-host.exe`
   - `gateway/server.exe`
   - `gateway/launcher.exe`
   - `start-agent.ps1`
8. `logs/` 目录可写。

## 2. 告警登记（仅告警态执行）

1. 记录 warning 总览字段：
   - `checked_at`
   - `status`
   - `summary.identity_diff_severity`
   - `non_blocking_warnings.count`
   - `non_blocking_warnings.warning_codes`
2. 逐条记录 warning 明细字段：
   - `details[].title`
   - `details[].description`
   - `details[].priority`
   - `details[].ui_hint`
   - `details[].action_label`
   - `details[].action_command`
3. 记录责任与动作字段：
   - 发布执行人
   - 决策时间
   - 跟踪项编号（Issue/Ticket）
   - 预期完成时间
4. 生成审计记录（推荐脚本化）：
   - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-warning-audit-record.ps1 -ReportPath tmp/stage-backend-reverify/latest.json -OutPath tmp/stage-backend-reverify/warning-audit-warning.json -Executor <owner> -TrackingId <ticket> -DueAt <iso8601> -RequireReady`
5. 若使用复核包的一键审计参数，当前步骤可直接复用复核包输出的 `warning-audit-from-pack.json`。

## 3. 发布动作

1. 生成发布版本号（推荐 `v<timestamp>`）。
2. 执行安装/升级脚本，部署到 `current`。
3. 若命中告警态，先完成“告警登记”再继续发布。
4. 记录发布元数据：
   - 版本号
   - 时间
   - 提交标识
   - 验收证据路径

## 4. 发布后核验

1. 网关健康检查通过：`/health`。
2. 运行时健康检查通过：`runtime /health`。
3. 系统信息接口可达：`/api/v1/system/info`。
4. 至少执行一次最小任务链路验证。
5. 复核包报告留档：
   - 保留 `tmp/stage-backend-reverify/latest.json`
   - 告警态额外保留 `tmp/stage-backend-reverify/warning-sample.json` 作为字段对照样本

## 5. 审计留痕

1. 保留发布版本信息和执行人。
2. 保留发布前后检查结果。
3. 保留触发的脚本命令和输出路径。
4. 告警态必须保留告警登记记录与跟踪项编号。
5. 建议同时保留 `warning-audit-*.json` 作为审计快照。
