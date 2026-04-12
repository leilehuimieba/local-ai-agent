# 技术方案

## 影响范围

- 文档资产：
  - `docs/11-hermes-rebuild/changes/F-release-rollback-closure/release-checklist.md`
  - `docs/11-hermes-rebuild/changes/F-release-rollback-closure/rollback-runbook.md`
  - `docs/11-hermes-rebuild/changes/F-release-rollback-closure/verify.md`
- 变更状态：
  - `docs/11-hermes-rebuild/changes/INDEX.md`
  - `docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`

## 三态决策矩阵（warning 协议）

1. 阻断态（`failed`）：
   - 判定条件：`failed_checks.count > 0`
   - 决策：禁止发布；回滚后复测若仍为 `failed`，继续升级处置。
2. 告警态（`warn`）：
   - 判定条件：`failed_checks.count = 0` 且 `non_blocking_warnings.count > 0`
   - 决策：允许发布或继续服务，但必须记录 warning 审计项并创建跟踪动作。
3. 通过态（`passed`）：
   - 判定条件：`failed_checks.count = 0` 且 `non_blocking_warnings.count = 0`
   - 决策：按正常流程放行。

## 告警态必填审计字段

1. 结论字段：
   - `checked_at`
   - `status`
   - `summary.identity_diff_severity`
2. 告警聚合字段：
   - `non_blocking_warnings.count`
   - `non_blocking_warnings.warning_codes`
3. 告警明细字段（逐条）：
   - `details[].title`
   - `details[].description`
   - `details[].priority`
   - `details[].ui_hint`
   - `details[].action_label`
   - `details[].action_command`
4. 执行责任字段：
   - 发布/回滚执行人
   - 决策时间
   - 跟踪项编号（Issue/Ticket/任务单）

脚本化建议：

1. 使用 `scripts/run-stage-f-warning-audit-record.ps1` 统一生成审计快照。
2. `warn` 态要求带入 `Executor/TrackingId/DueAt`，并启用 `-RequireReady` 阻止缺字段放行。
3. 复核包可启用 `-EmitWarningAuditRecord` 与 `-RequireWarningAuditReady`，实现“复核+审计”一条命令落盘。

## 发布清单（F-03 口径）

发布前必须满足：

1. 代码与阶段证据：
   - `F-01` 证据：`tmp/stage-f-install/latest.json` 为 `passed`。
   - `F-02` 证据：`tmp/stage-f-doctor/latest.json` 为 `passed`。
2. 构建与验收脚本可执行：
   - `scripts/install-local-agent.ps1`
   - `scripts/doctor.ps1`
3. 发布配置完整：
   - `config/app.json` 可解析，端口配置合法。
   - `frontend/dist/index.html` 存在。
4. 安全合规：
   - 不在发布产物中包含密钥、调试日志、测试快照。

发布放行判定：

1. 若 `failed_checks.count > 0`：阻断，不得发布。
2. 若 `failed_checks.count = 0` 且 `non_blocking_warnings.count > 0`：可发布，但必须先完成告警审计记录。
3. 若 `failed_checks.count = 0` 且 `non_blocking_warnings.count = 0`：正常放行。

发布动作（最小路径）：

1. 执行安装/升级脚本生成当前发布目录。
2. 执行 doctor 验证本次发布目录可健康启动。
3. 如命中告警态，写入 warning 审计记录并关联跟踪项编号。
4. 写入发布元数据（版本、时间、来源提交、验收报告路径）。

发布后核验：

1. `gateway/runtime` 健康可达。
2. `/api/v1/system/info` 返回 `formal_entry` 与 `repo_root` 一致。
3. 关键证据路径可追溯。

## 回滚预案（F-03 口径）

触发条件（任一命中即触发）：

1. 发布后健康检查连续失败。
2. 关键接口不可用且 15 分钟内无法修复。
3. 关键回归失败且影响主链路使用。

回滚步骤（最小脚本化流程）：

1. 停止当前网关与运行时进程。
2. 将 `current` 切回 `backups` 下最近稳定版本目录。
3. 复核 `current-version.txt` 与回滚版本一致。
4. 启动并执行 doctor 复测。
5. 重跑后端复核包并按三态矩阵判定。

回滚后核验：

1. `gateway/runtime` 健康恢复。
2. 主入口可访问并可执行一条最小任务。
3. 记录回滚审计信息（触发时间、触发人、版本、原因、结果）。
4. 若为告警态，保留服务但必须登记 warning 审计并创建跟踪项；若为阻断态，继续升级处置。

## 风险与回退

- 风险：当前为最小文档收口，尚未覆盖完整发布渠道（如 MSI/winget）。
- 缓解：在 `F-04` 增加发布候选回归与故障注入，验证文档可操作性。
- 回退：若发布文档与脚本口径冲突，优先按脚本真实行为修正文档并补审计记录。
