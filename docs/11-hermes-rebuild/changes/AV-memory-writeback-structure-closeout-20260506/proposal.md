# 变更提案

## 背景

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AU-worktree-ownership-and-closeout-routing-20260506/` 已完成当前未提交改动的归属梳理，并明确裁决：下一把真正的实现 change 应优先落到 **memory 写回治理簇**。
- 该簇虽然主体能力已在 `AO-memory-writeback-governance-20260506` 中完成签收，但真实工作区仍保留一整组未入库改动，且集中落在以下热点文件：
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`（658 行）
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`（744 行）
  - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/mod.rs`（712 行）
- 这组文件一方面承接了 `memory_write_layer / memory_write_decision / memory_write_reason / memory_duplicate_strategy` 的治理字段与事件透出，另一方面又都已超过项目对 Rust 源文件 600 行的热点红线。
- 如果继续把这组 memory 改动停留在“AO 已签收但未完全入库”的状态，后续无论切 browser、knowledge 还是其它模块，都容易把工作区继续叠加到一个更难收口的混合面上。

## 目标

- 新建独立实现 change `AV-memory-writeback-structure-closeout-20260506`。
- 只围绕 memory 写回治理簇推进最小结构收口与热点治理准备。
- 为下一步真正的实现推进冻结范围、模块边界、验证口径与回退方式。

## 非目标

- 不在本 change 内扩 browser 恢复、高风险交互、gateway/runtime bridge 或 knowledge 主链行为。
- 不回 `AO` 继续做语义扩面，例如 `memory_reuse_value`、`memory_verification_gate` 等第二批治理字段。
- 不顺手处理 `events.rs`、`context_builder.rs`、`executors/project.rs` 等其它热点文件。
- 本轮不直接进入大规模 Rust 实现，只建立正式工作区并切换执行入口。

## 验收口径

- 已建立 `AV-memory-writeback-structure-closeout-20260506` 的正式五件套工作区。
- 已明确本 change 只覆盖 memory 写回治理簇，不回头混入 browser / knowledge / verify 其它簇。
- `current-state.md` 与 `changes/INDEX.md` 已切换到 `AV` 作为当前活跃 change。
- 已明确 `memory.rs / memory_router/mod.rs / sqlite_store/mod.rs` 的优先级与拆分/收口边界。
- 全程没有在未建档前直接进入 memory 簇的大规模代码改写。
