# 技术方案

## 影响范围

- 涉及模块：
  1. 文档层：阶段定义与切换决策，不涉及源码模块改动。
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/G-stage-definition-prep-20260414/*`
  4. `docs/11-hermes-rebuild/changes/G-stage-definition-prep-20260414/artifacts/G-stage-definition-draft-20260414.md`

## 方案

- 核心做法：
  1. 在本 change 内先输出“阶段 G 定义草案”，明确目标、必做任务、交付物、Gate、失败回退。
  2. 以“是否满足切换前置条件”为主线完成评审，不做阶段 G 实现。
  3. 维持当前阶段口径为“阶段 F / Gate-F（已签收）”，仅切换活跃 change 到本目录。
- 状态流转或调用链变化：
  1. 当前状态不切阶段，只做切换准备。
  2. 若草案通过评审，再进入“正式切换到阶段 G”的独立变更。

## 风险与回退

- 主要风险：
  1. 阶段 G 边界定义不清会导致后续实现范围蔓延。
  2. 若与既有总路线冲突，可能出现口径分叉。
- 回退方式：
  1. 发现冲突时，回退为“保持阶段 F 已签收”并冻结阶段 G 切换。
  2. 仅保留草案为候选文档，不更新阶段主状态。
