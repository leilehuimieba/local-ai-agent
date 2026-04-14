# 技术方案

## 影响范围

- 涉及模块：
  1. 文档治理层（阶段定义、状态裁决、任务入口）。
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/Hermes重构总路线图_完整计划.md`
  2. `docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
  3. `docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`
  4. `docs/11-hermes-rebuild/current-state.md`
  5. `docs/11-hermes-rebuild/changes/INDEX.md`
  6. `docs/11-hermes-rebuild/changes/G-stage-definition-prep-20260414/status.md`

## 方案

- 核心做法：
  1. 采用“先定义口径，再切状态”的顺序：先补路线/计划/任务，再切 `current-state`。
  2. 阶段 G 正式定义采用 `G-stage-definition-prep-20260414` 草案口径，避免二次分叉。
  3. 切换完成后，以 `G-stage-switch-signoff-20260414` 作为新的活跃 change，承接切换签收证据。
- 状态流转或调用链变化：
  1. 阶段状态：`阶段 F / Gate-F（已签收）` -> `阶段 G / Gate-G（执行中，未签收）`。
  2. 活跃 change：`G-stage-definition-prep-20260414` -> `G-stage-switch-signoff-20260414`。

## 风险与回退

- 主要风险：
  1. 若总路线、阶段计划、任务总表存在字段不一致，会造成执行口径冲突。
  2. 若先切 `current-state` 再补定义，短时会出现“阶段已切换但无正式任务入口”。
- 回退方式：
  1. 如发现口径冲突，立即回退 `current-state.md` 与 `changes/INDEX.md` 至阶段 F 口径。
  2. 保留阶段 G 变更在独立 change 中修正后再重新切换。
