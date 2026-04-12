# 任务清单

- [x] 输出发布清单
  完成判据：`release-checklist.md` 覆盖发布前检查、发布动作、发布后核验、审计留痕。
- [x] 输出回滚预案
  完成判据：`rollback-runbook.md` 覆盖触发条件、回滚步骤、回滚后核验、失败升级路径。
- [x] 在 design 中汇总 F-03 口径
  完成判据：`design.md` 可独立回答“怎么发布、怎么回滚、失败怎么办”。
- [x] 回写索引与总表状态
  完成判据：`INDEX.md` 与 `全路线最小任务分解总表.md` 对齐到 F-03 已完成。
- [x] 接入 warning 协议到发布清单
  完成判据：`release-checklist.md` 明确 `failed/warn/passed` 三态决策与告警留痕字段。
- [x] 接入 warning 协议到回滚预案
  完成判据：`rollback-runbook.md` 明确回滚后复测三态处理分支（阻断/告警/放行）。
- [x] 补齐三态可复核证据链
  完成判据：`verify.md` 同时引用 `latest.json`、`warning-sample.json`、`failure-sample.json` 的关键字段。
- [x] 固化 warning 审计记录脚本
  完成判据：新增 `scripts/run-stage-f-warning-audit-record.ps1`，可将复核报告产出 `pass/warn/blocked` 审计快照。
- [x] 发布/回滚清单接入审计记录命令
  完成判据：`release-checklist.md` 与 `rollback-runbook.md` 均包含可直接执行的审计记录命令示例。
- [x] 复核包接入一键审计参数
  完成判据：`run-stage-backend-reverify-pack.ps1` 支持同次执行产出 `warning-audit` 快照并可开启 `RequireWarningAuditReady`。
