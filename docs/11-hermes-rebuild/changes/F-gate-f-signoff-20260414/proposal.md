# 变更提案

## 背景

- 本次变更要解决的问题：
  1. 阶段 F 的分项 `F-01/F-02/F-03/F-05` 已有本轮证据，但缺少本轮 `F-G1` 独立签收包。
  2. `scripts/run-stage-f-gate-acceptance.ps1` 仍引用旧 change 路径，导致 Gate-F 聚合验收不可直接复跑。
  3. 需要形成可追溯的 Gate-F 决策文档，支撑阶段切换评审。
- 对应阶段目标：
  1. 阶段 F（Windows 产品化与发布）`F-G1`：Gate-F 门禁评审与发布决策。

## 目标

- 本次要完成什么：
  1. 修复 Gate-F 聚合脚本中的 change 路径映射，使其匹配当前活跃 change 命名。
  2. 执行 `scripts/run-stage-f-gate-acceptance.ps1 -RequireGateF` 并产出本轮 Gate-F 报告。
  3. 输出本轮 Gate-F 评审结论（`review.md`）并回写五件套状态。

## 非目标

- 本次明确不做什么：
  1. 不新增前端或运行时功能实现。
  2. 不并行推进阶段 G 任务实现。
  3. 不处理并行规划项 `F-memory-progressive-disclosure-20260414`。

## 验收口径

- 通过标准：
  1. `scripts/run-stage-f-gate-acceptance.ps1 -RequireGateF` 可执行通过。
  2. `tmp/stage-f-gate/latest.json` 中 `status=passed` 且 `gate_f.ready=true`。
  3. 本 change 五件套与 `review.md` 对齐本轮 Gate-F 评审结论。
