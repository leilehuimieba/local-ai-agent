# 技术方案

## 影响范围

- 文档入口：
  - `docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
  - `docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`（新增）
- change 工作区：
  - `docs/11-hermes-rebuild/changes/C-roadmap-task-decomposition/`
- 索引：
  - `docs/11-hermes-rebuild/changes/INDEX.md`

## 方案

- 采用“两层任务模型”：
  - 层 1：阶段目标与 Gate（继续由 `阶段计划总表` 维护）。
  - 层 2：最小任务清单（由新增总表维护，任务 ID 可直接在对话中引用）。
- 每条最小任务统一字段：
  - `任务ID`
  - `阶段`
  - `前置依赖`
  - `单轮动作（<=2h）`
  - `完成判据（DoD）`
  - `验证方式`
  - `证据落点`
  - `状态`
- 执行协议收口：
  - 每次对话仅领取 1 条“依赖已满足”的任务。
  - 完成后必须回写 `tasks.md / status.md / verify.md` 或总表状态。
  - 未产出验证证据不得标记“完成”。

## 风险与回退

- 主要风险：任务拆分过细导致维护负担增加。
- 主要风险：阶段已完成任务与历史文档状态不一致。
- 回退方式：总表仅增量补充，不替换既有阶段文档；如发现冲突，以 `阶段计划总表` Gate 口径为准并在总表修正。
