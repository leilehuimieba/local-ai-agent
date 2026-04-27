# 变更提案

## 背景

- 本次变更要解决的问题：
  1. `F-install-upgrade-20260414` 已完成 `F-01`，阶段 F 下一刀需推进 `F-02 doctor`。
  2. 当前缺少 `F-02` 独立 change 的任务口径与证据收口入口。
  3. Gate-F 组合验收脚本依赖 doctor 证据，需先固化 doctor 核心检查通过链。
- 对应阶段目标：
  1. 阶段 F（Windows 产品化与发布）`F-02`：环境诊断与一键自检主路径可用。

## 目标

- 本次要完成什么：
  1. 建立 `F-doctor-core-checks-20260414` 五件套，冻结 `F-02` 范围与验收口径。
  2. 执行 `scripts/run-stage-f-doctor-acceptance.ps1` 并回写证据。
  3. 明确 doctor 核心检查项与失败回退路径，支撑后续 Gate-F 汇总验收。

## 非目标

- 本次明确不做什么：
  1. 不并行收口 `F-03/F-04/F-05` 的发布候选与 Windows 10 分钟全链路。
  2. 不修改 Gate-F 阈值定义。
  3. 不做 Gate-F 最终完成声明。

## 验收口径

- 通过标准：
  1. 本 change 五件套完整且状态一致（`proposal/design/tasks/status/verify`）。
  2. `scripts/run-stage-f-doctor-acceptance.ps1` 执行通过，`tmp/stage-f-doctor/latest.json` 为 `passed`。
  3. `verify.md` 回写检查项结果、证据路径与失败回退建议。
