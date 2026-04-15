# 变更提案

## 背景

- 本次变更要解决的问题：
  1. `G-01` 已完成证据保鲜入口，但 `G-02` 告警治理闭环尚未固化。
  2. warning 的持续追踪与升级缺少统一状态文件，难以形成“连续出现 -> 升级跟踪”的判定链。
  3. Gate-G 需要把 warning 治理从“人工判断”升级为“可脚本复核”。
- 对应阶段目标：
  1. 完成 `G-02`：发布后巡检与告警审计流程固化。

## 目标

- 本次要完成什么：
  1. 新增 `run-stage-g-warning-governance.ps1`，形成 G-02 的脚本化闭环。
  2. 产出 warning tracker（`tmp/stage-g-ops/warning-tracker.json`）与治理报告（`tmp/stage-g-ops/latest.json`）。
  3. 回写阶段 G 文档与本 change 五件套，明确升级阈值与责任字段口径。

## 非目标

- 本次明确不做什么：
  1. 不改 `runtime-core` 业务逻辑。
  2. 不推进 `G-03` 回归基线自动汇总实现。
  3. 不进行 Gate-G 最终签收。

## 验收口径

- 通过标准：
  1. `scripts/run-stage-g-warning-governance.ps1` 可执行并支持 `-RequirePass`。
  2. `tmp/stage-g-ops/latest.json` 为 `passed` 且 `checks.governance_ready=true`。
  3. `tmp/stage-g-ops/warning-tracker.json` 成功更新并记录历史。
  4. 本 change 文档与证据路径一致。
