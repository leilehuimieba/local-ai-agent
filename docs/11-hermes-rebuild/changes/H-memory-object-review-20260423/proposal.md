# H-memory-object-review-20260423（proposal）

更新时间：2026-04-23  
状态：草案（待启动，不切主推进）

## 1. 背景

1. 当前仓库已具备长期记忆、知识沉淀、观察记录、写回治理与注入预算等基础能力，主落点集中在：
   - `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/memory_recall.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/knowledge.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/knowledge_store.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/observation.rs`
2. 已签收的 `H-memory-routing-kb-20260415` 证明了“学习记忆路由与知识沉淀”的最小闭环，但当前仍主要基于扁平 `MemoryEntry` 记录与 recall digest，尚未形成：
   - 稳定的长期记忆对象身份；
   - 面向 agent 的固定系统视图入口；
   - 面向记忆对象本体的版本审查与回退。
3. 结合对 `nocturne_memory` 的架构学习，可确认本仓库下一步最值得吸收的不是完整图谱后端，而是：
   - `system views`
   - `memory object + version`
   - `review / rollback`

## 2. 问题定义

1. 当前长期记忆更像“治理过的可搜索记录”，而不是“可稳定寻址、可版本演进、可审查回退的长期对象”。
2. 当前 recall 入口偏搜索式，缺少 `system://boot`、`system://recent`、`system://index` 这类固定系统入口，导致启动与续跑阶段仍较依赖散点召回。
3. 当前已有 observation 与回退演练证据，但缺少“针对某个长期记忆对象查看历史 diff 并恢复旧版本”的独立治理面。

## 3. 本次草案目标

1. 为后续 memory 升级提供一个不打断 Gate-H 的并行 change 工作区。
2. 冻结 memory 子系统的下一阶段最小改造边界：
   - 新增 system views；
   - 引入 memory object/version 最小模型；
   - 提供最小 review/rollback 能力。
3. 明确哪些能力后置，不在本轮草案中扩 scope。

## 4. 约束

1. 当前主推进仍以 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md` 为准，即 `H-gate-h-signoff-20260416`。
2. 本 change 仅作为并行草案，不切主推进，不改写 Gate-H 现有状态结论。
3. 本草案默认以 `runtime-core` 为主落点，不提前引入新的外部 memory 主系统。

## 5. 预期收益

1. 让长期记忆从“可写入记录”升级为“可治理对象”。
2. 让上下文装配从“偏搜索 recall”升级为“系统入口 + 规则召回 + 搜索补充”。
3. 让 memory 治理与 observation/verify 治理口径对齐，补齐记忆对象本体的版本审查与回退。
