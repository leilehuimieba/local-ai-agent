# 技术方案

## 影响范围

- 涉及模块：
  - `crates/runtime-core/src/memory_router/mod.rs`
  - `crates/runtime-core/src/memory_recall.rs`
  - `crates/runtime-core/src/memory_layer.rs`
  - `crates/runtime-core/src/context_builder.rs`
  - `crates/runtime-core/src/events.rs`
  - `gateway/internal/api/learning_memory.go`
- 涉及文档或 contract：
  - `docs/11-hermes-rebuild/changes/AG-agent-loop-memory-knowledge-20260505/design.md`
  - Runtime memory digest metadata / context snapshot / Gateway preview 口径

## Harness 判断

- 第一刀已经把“多步回路”立起来，第二刀已经把“上下文 profile”立起来。
- 第三刀现在最值得补的，不是再扩新记忆层，而是让现有记忆入口先学会“按任务选层”。
- 因此本刀只补 `memory route`，不补 `memory write governance`，也不提前扩知识检索和 verify。

## 范围冻结

- 本刀只做四件事：
  - 定义第三刀最小 `memory route`
  - 让 Runtime 能按当前 profile / 动作类型 / 恢复态选记忆读层
  - 让 `memory_recall.rs` 输出更小、更可解释的 digest
  - 让 route 结果进入 metadata / context snapshot / learning preview

## 最小路由口径

### 读层边界

- 本刀不新建物理存储层，先基于现有入口做逻辑路由：
  - `working hint`
    - 来自第二刀已成立的 `session.short_term` / 当前上下文
    - 本刀不重做，只作为记忆读侧的外部前提
  - `system views`
    - 当前仓库级规则、项目总览、长期稳定结论
  - `current memory object`
    - 当前对象、当前任务、当前工作对象相关条目
  - `history entries`
    - 其他长期记忆记录，包括 lesson / preference / workflow pattern

### 路由目标

- `ask_profile`
  - 优先：`system views + history entries`
  - 目的：回答定义、关系、规则、项目背景时优先稳定知识
- `act_profile`
  - 优先：`current memory object`
  - 其次：与当前动作相关的少量 `history entries`
  - 目的：执行时避免被大段历史和抽象规则淹没
- `repair_profile`
  - 优先：`current memory object + history entries`
  - 其次：必要时补 `system views`
  - 目的：失败恢复时优先看到“上次为什么失败、怎么恢复”
- `learn_profile`
  - 优先：`system views + history entries`
  - 目的：沉淀和复用时优先看可复用规则、经验、workflow

## MemoryRecall 改造

- `memory_recall.rs` 不再默认把三类命中直接并排拼接。
- 本刀应至少新增下面这些读侧结果字段：
  - `memory_route`
  - `selected_layers`
  - `match_reason`
  - `reuse_confidence`
  - `skipped_layers`
- 最小 digest 目标：
  - 先说明“本次为什么读这一层”
  - 再输出 1 到 3 条最相关摘要
  - 不再默认把所有命中层全文压到同一个长摘要里

## MemoryRouter 改造

- `memory_router/mod.rs` 当前更偏写侧治理。
- 本刀不重做写侧，只补一个最小读侧路由入口，用来回答：
  - 当前属于哪种 `memory route`
  - 当前应优先哪些层
  - 当前应抑制哪些层
- 路由信号至少应综合：
  - `assembly_profile`
  - `prompt_profile`
  - 当前 `PlannedAction`
  - 是否处于 checkpoint / failure recovery
  - 当前查询是否更偏规则、对象、恢复、流程

## Context / Event 出口

- `context_builder.rs`
  - 继续负责把记忆摘要挂入动态上下文
  - 新增 route 解释字段，而不是只给最终拼接后的 `memory_digest`
- `events.rs`
  - 让 route 结果进入 `context_snapshot`
  - 至少保留：
    - `memory_route`
    - `memory_selected_layers`
    - `memory_match_reason`
    - `memory_reuse_confidence`
- `memory_layer.rs`
  - 当前只负责 object-aware 标签
  - 本刀应补 route 层的简短可读描述，避免 metadata 和 prompt 表达不一致

## Gateway 边界

- `gateway/internal/api/learning_memory.go`
  - 只保留学习模式下的 preview / digest 展示
  - 不承担 Runtime 主路由判断
- 第三刀的主判断仍应收敛在 Rust Runtime，避免 Go / Rust 各算一套路由

## 风险与回退

- 主要风险：
  - 路由条件过宽，导致 ask / learn / repair 误读同一批摘要
  - 恢复态没优先命中失败经验，仍然被系统视图淹没
  - metadata 写了 route，但 digest 实际还是旧拼接逻辑
- 回退方式：
  - 保留当前全量拼接 digest 作为 fallback
  - 当 route 判断失败时，退回当前稳定摘要路径
  - 先补 route 选择测试和 digest 差异测试，再决定是否删除旧拼接分支
