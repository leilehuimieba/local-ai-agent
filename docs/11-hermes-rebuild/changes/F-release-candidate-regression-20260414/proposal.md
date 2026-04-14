# 变更提案

## 背景

- 本次变更要解决的问题：
  1. `F-01`（install）和 `F-02`（doctor）已完成，阶段 F 需要推进 `F-03` 发布候选回归与故障注入验收。
  2. 当前缺少 `F-03` 独立 change 工作区，不利于证据收口和回退管理。
  3. Gate-F 组合验收依赖 `tmp/stage-f-rc/latest.json`，需先形成稳定 RC 证据。
- 对应阶段目标：
  1. 阶段 F（Windows 产品化与发布）`F-03`：发布候选回归通过并具备故障注入闭环。

## 目标

- 本次要完成什么：
  1. 建立 `F-release-candidate-regression-20260414` 五件套并冻结 `F-03` 范围。
  2. 执行 `scripts/run-stage-f-rc-acceptance.ps1 -Rounds 3`，回写结果与关键指标。
  3. 形成可复用的 RC 验收摘要，供后续 Gate-F 汇总使用。

## 非目标

- 本次明确不做什么：
  1. 不并行推进 `F-05`（Windows 10 分钟验收）与 Gate-F 总签收。
  2. 不修改阶段 E 的历史脚本或口径定义。
  3. 不做 Gate-F 最终完成声明。

## 验收口径

- 通过标准：
  1. 本 change 五件套完整且状态一致（`proposal/design/tasks/status/verify`）。
  2. `tmp/stage-f-rc/latest.json` 中 `status=passed` 且 `release_candidate.ready=true`。
  3. `verify.md` 回写轮次结果、关键统计与失败回退建议。
