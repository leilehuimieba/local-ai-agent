# 验证记录

## 验证方式

- 文档一致性检查：
  1. 校验总路线、阶段计划、最小任务总表均存在阶段 G 正式口径。
  2. 校验 `current-state.md` 与 `changes/INDEX.md` 的活跃 change 一致。
- 人工验证：
  1. 校验阶段状态已切为“阶段 G / Gate-G（执行中，未签收）”。
  2. 校验 `G-stage-definition-prep-20260414` 与本 change 之间的承接关系明确。

## 证据位置

- 路线与计划：
  1. `docs/11-hermes-rebuild/Hermes重构总路线图_完整计划.md`
  2. `docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
  3. `docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`
- 状态与索引：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
- 签收结论：
  1. `docs/11-hermes-rebuild/changes/G-stage-switch-signoff-20260414/review.md`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-G（执行中，未签收）。
- 当前覆盖情况：
  1. 已完成阶段切换签收与口径升级。
  2. 未进入 Gate-G 指标验收，后续由阶段 G 实现类 change 承接。
