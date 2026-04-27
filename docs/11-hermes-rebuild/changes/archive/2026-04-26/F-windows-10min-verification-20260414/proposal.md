# 变更提案

## 背景

- 本次变更要解决的问题：
  1. `F-01/F-02/F-03` 已完成，阶段 F 尚缺 `F-05` 的 Windows 10 分钟新机验证证据。
  2. Gate-F 判定依赖 `tmp/stage-f-windows/latest.json`，当前主线尚未完成本轮回写。
  3. 需要独立 change 管理 `F-05` 执行结果与失败回退路径。
- 对应阶段目标：
  1. 阶段 F（Windows 产品化与发布）`F-05`：新机 10 分钟内完成安装并跑通首任务。

## 目标

- 本次要完成什么：
  1. 建立 `F-windows-10min-verification-20260414` 五件套并冻结 `F-05` 验收口径。
  2. 执行 `scripts/run-stage-f-windows-acceptance.ps1 -MaxMinutes 10` 并回写证据。
  3. 形成可复用的 10 分钟验收摘要，供后续 Gate-F 汇总验收引用。

## 非目标

- 本次明确不做什么：
  1. 不在本刀内做 Gate-F 最终签收。
  2. 不修改 install/doctor/rc 已收口 change 的验收结论。
  3. 不扩展到并行规划项 `F-memory-progressive-disclosure-20260414`。

## 验收口径

- 通过标准：
  1. 本 change 五件套完整且状态一致（`proposal/design/tasks/status/verify`）。
  2. `tmp/stage-f-windows/latest.json` 中 `status=passed`，且 `within_time_budget=true`。
  3. `verify.md` 回写耗时、首任务终态与证据路径。
