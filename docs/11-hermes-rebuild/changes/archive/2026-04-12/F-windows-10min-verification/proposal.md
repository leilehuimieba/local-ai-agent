# 变更提案

## 背景

- `F-04` 发布候选回归与故障注入已通过，阶段 F 下一步为 `F-05`。
- 总表要求完成“Windows 新机 10 分钟验证”，当前缺少结构化验收脚本与证据。
- 用户要求继续后端主线推进，本轮不做前端改造。

## 目标

- 新增 `F-05` 验收脚本，模拟新机安装并执行首任务。
- 产出可复核证据：`tmp/stage-f-windows/latest.json` 与 `latest.md`。
- 明确“总耗时 <= 10 分钟”的判定字段，支持 Gate-F 前评审。

## 非目标

- 本轮不做 Gate-F 最终评审签收（归属 `F-G1`）。
- 本轮不新增发布渠道（MSI/winget/scoop）。
- 本轮不修改前端功能。

## 验收口径

- `run-stage-f-windows-acceptance.ps1 -RequirePass` 执行通过。
- 报告中 `status=passed` 且 `within_time_budget=true`。
- 首任务完成状态可复核（`run_finished` + `completion_status=completed`）。
