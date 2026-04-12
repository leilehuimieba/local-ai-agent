# Scripts 目录说明

更新时间：2026-04-12
状态：当前有效

## 1. 当前保留脚本

1. `start-dev.ps1`
   - 用途：本地开发启动入口。
2. `run-v1-regression-check.ps1`
   - 用途：V1 固定回归检查入口。
   - 引用文档：`docs/07-test/V1回归检查入口_V1.md`
3. `sync-to-server.ps1`
   - 用途：将本地仓库完整同步到服务器 `/opt/hermes`。
   - 默认参数：`175.178.90.193` + `root` + `~/.ssh/labsafe_new.pem`。
4. `sync-from-server.ps1`
   - 用途：将服务器 `/opt/hermes` 回拉到本地仓库。
   - 默认行为：回拉完成后删除本地备份；如需保留，使用 `-KeepLocalBackup`。
5. `run-stage-d-migrate-acceptance.ps1`
   - 用途：阶段 D `D-02` 记忆迁移最小脚本验收（legacy JSONL -> SQLite）。
   - 证据输出：`tmp/stage-d-migrate-acceptance/latest.json`。
6. `run-stage-d-day1-acceptance.ps1`
   - 用途：阶段 D `D-06` 跨会话连续性首轮样本（会话 A 写记忆 -> 会话 B 召回）。
   - 证据输出：`tmp/stage-d-day1/latest.json`。
7. `run-stage-d-gate-batch.ps1`
   - 用途：阶段 D `D-G1` 7 天 Gate-D 批量验收（连续性与召回命中率阈值校验）。
   - 证据输出：`tmp/stage-d-batch/latest.json`。
8. `run-stage-e-entry1-acceptance.ps1`
   - 用途：阶段 E `E-02` 网关首入口会话协议验收（`chat/run` 协议字段 + `logs` 会话过滤）。
   - 证据输出：`tmp/stage-e-entry1/latest.json`。
9. `run-stage-e-consistency-acceptance.ps1`
   - 用途：阶段 E `E-04` 跨入口一致性验收（CLI/runtime 与 gateway 同 `run_id` 锚点对比）。
   - 差异分组：报告新增 `identity_diff_summary` 与 `identity_diff_groups`，按 `run_id/session_id/trace_id` 输出冲突与缺失计数，并给出 `severity`（`ok/warn/error`）。
   - 证据输出：`tmp/stage-e-consistency/latest.json`。
10. `run-stage-e-entry-failure-acceptance.ps1`
   - 用途：阶段 E `E-05` 失败样本验收（gateway 入口在 runtime 不可达时的失败收口链）。
   - 证据输出：`tmp/stage-e-entry-failure/latest.json`。
11. `run-stage-e-gate-batch.ps1`
   - 用途：阶段 E `E-G1` Gate-E 批量验收（聚合 E-02/E-04/E-05 三条链路并输出门禁结论）。
   - 证据输出：`tmp/stage-e-batch/latest.json`。
12. `run-stage-e-cli-history-acceptance.ps1`
   - 用途：阶段 E `E-01` CLI/TUI 交互切片 1（历史视图）后端验收。
   - 证据输出：`tmp/stage-e-cli-history/latest.json`。
13. `run-stage-e-cli-cancel-acceptance.ps1`
   - 用途：阶段 E `E-01` CLI/TUI 交互切片 2（中断接口）后端验收。
   - 证据输出：`tmp/stage-e-cli-cancel/latest.json`。
14. `install-local-agent.ps1`
   - 用途：阶段 F `F-01` 安装/升级主路径实现（构建并部署到安装目录，支持 install/upgrade 两种模式）。
   - 输出：安装结果 JSON（stdout）。
15. `run-stage-f-install-acceptance.ps1`
   - 用途：阶段 F `F-01` 安装与升级验收（安装 -> 启动校验 -> 升级 -> 启动校验）。
   - 证据输出：`tmp/stage-f-install/latest.json`。
16. `doctor.ps1`
   - 用途：阶段 F `F-02` 核心诊断命令（依赖、配置、端口、前端产物、服务健康、日志可写）。
   - 输出：诊断结果 JSON（stdout，可选落盘）。
17. `run-stage-f-doctor-acceptance.ps1`
   - 用途：阶段 F `F-02` `doctor` 命令验收（拉起 runtime/gateway 后执行 doctor 并校验字段）。
   - 证据输出：`tmp/stage-f-doctor/latest.json`。
18. `run-stage-f-rc-acceptance.ps1`
   - 用途：阶段 F `F-04` 发布候选回归与故障注入聚合验收（组合 F-01/F-02 与核心入口链路样本）。
   - 证据输出：`tmp/stage-f-rc/latest.json`。
19. `run-stage-f-windows-acceptance.ps1`
   - 用途：阶段 F `F-05` Windows 新机 10 分钟验证（安装后启动并完成首任务，校验总耗时门槛）。
   - 证据输出：`tmp/stage-f-windows/latest.json`、`tmp/stage-f-windows/latest.md`。
20. `run-stage-f-gate-acceptance.ps1`
   - 用途：阶段 F `F-G1` Gate-F 评审聚合验收（汇总 F-01/F-02/F-04/F-05 与阻塞状态检查）。
   - 证据输出：`tmp/stage-f-gate/latest.json`。
21. `run-stage-backend-reverify-pack.ps1`
   - 用途：后端一键复核包（聚合 E-01 历史/中断、E-04 跨入口一致性与 F-G1 门禁证据）。
   - 门禁参数：支持 `-StrictGate`、`-MaxEvidenceAgeMinutes`、`-ReleaseWindow`（30 分钟阈值），用于证据时效与关键字段完整性严格校验。
   - 审计参数：支持 `-EmitWarningAuditRecord`、`-WarningAuditOutPath`、`-WarningAuditExecutor`、`-WarningAuditTrackingId`、`-WarningAuditDueAt`、`-RequireWarningAuditReady`，可在同一次复核中生成 warning 审计快照并执行字段完整性校验。
   - 接口证据：透传 `identity_diff_summary` 与 `identity_diff_groups`，用于前端直接展示 run/session/trace 一致性差异。
   - 告警分层：输出 `non_blocking_warnings`（与 `failed_checks` 并列），承接 `severity=warn` 的非阻断告警，并提供 `details.title/description/priority/ui_hint/action_label/action_command` 供前端直连渲染。
   - 失败定位：报告输出 `reason_codes`、`suggestions`、`recommended_commands_minimal`、`recommended_commands_full_refresh`（兼容保留 `recommended_commands`）。
   - 命令策略：`recommended_commands_minimal` 按失败项定向修复；`recommended_commands_full_refresh` 提供一键全刷新严格校验命令。
   - 证据输出：`tmp/stage-backend-reverify/latest.json`。
22. `run-stage-backend-reverify-warning-sample.ps1`
   - 用途：生成后端复核包非阻断告警样本（`warning-sample.json`），并自动恢复 `latest.json` 到标准通过态。
   - 证据输出：`tmp/stage-backend-reverify/warning-sample.json`。
23. `run-stage-f-warning-audit-record.ps1`
   - 用途：把后端复核包报告转成发布/回滚统一告警审计记录（`pass/warn/blocked` 三态决策）。
   - 规则：`warn` 状态要求补齐 `Executor/TrackingId/DueAt`，否则 `ready_for_release=false`。
   - 证据输出：`tmp/stage-backend-reverify/warning-audit-*.json`。

## 2. 同步命令示例

1. 本地 -> 服务器
   - `powershell -NoProfile -File scripts/sync-to-server.ps1`
2. 服务器 -> 本地（保留本地备份）
   - `powershell -NoProfile -File scripts/sync-from-server.ps1 -KeepLocalBackup`
3. 服务器 -> 本地（回拉后自动删除本地备份）
   - `powershell -NoProfile -File scripts/sync-from-server.ps1`

## 3. 已归档脚本

以下脚本属于阶段性验收或专项检查，不作为当前默认入口：

1. `archive/run-mainline-acceptance.ps1`
2. `archive/run-runtime-host-lock-check.ps1`
3. `archive/run-stage-d-day13-day14.ps1`

说明：

1. 归档脚本可保留用于历史复盘与证据补跑。
2. 新阶段若需复用，应先复制回根 `scripts/` 并在对应执行入口文档中登记。
3. 不建议在当前主链路文档继续引用归档路径以外的旧脚本名。
