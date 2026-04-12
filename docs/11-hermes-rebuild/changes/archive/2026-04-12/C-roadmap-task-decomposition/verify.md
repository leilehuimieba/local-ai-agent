# 验证记录

## 验证方式

- 文档检查：确认新增总表覆盖 A~F 且包含 Gate 任务。
- 入口检查：确认 `阶段计划总表` 已挂接总表链接。
- 索引检查：确认 `changes/INDEX.md` 已登记本 change。
- 一致性检查：确认总表状态与当前阶段事实一致（A/B 已收口，C 待评审，D/E/F 待推进）。

## 证据位置

- 任务总表：`docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`
- 入口文档：`docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
- change 索引：`docs/11-hermes-rebuild/changes/INDEX.md`
- 本 change 五件套：
  - `docs/11-hermes-rebuild/changes/C-roadmap-task-decomposition/proposal.md`
  - `docs/11-hermes-rebuild/changes/C-roadmap-task-decomposition/design.md`
  - `docs/11-hermes-rebuild/changes/C-roadmap-task-decomposition/tasks.md`
  - `docs/11-hermes-rebuild/changes/C-roadmap-task-decomposition/status.md`
  - `docs/11-hermes-rebuild/changes/C-roadmap-task-decomposition/verify.md`

## Gate 映射

- 对应阶段 Gate：执行治理增强（跨阶段支持，不改变既有 Gate 阈值）。
- 当前覆盖情况：已建立“阶段目标 -> 最小任务 -> 验证证据”的可执行映射，可直接用于后续逐轮推进。
