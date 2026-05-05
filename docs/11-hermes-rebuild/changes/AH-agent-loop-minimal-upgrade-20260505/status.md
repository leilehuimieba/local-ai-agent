# 当前状态

- 最近更新时间：2026-05-05
- 状态：已完成
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`
- 已完成：已从 `AG-agent-loop-memory-knowledge-20260505` 蓝图中拆出第一刀独立实现 change。
- 已完成：已冻结本 change 只覆盖 `PlanEnvelope + 小步回路 + replan 事件 + budget handoff`。
- 已完成：已明确第一刀优先改造 `planner.rs`、`query_engine.rs`、`lib.rs`、`run_state_builder.rs`、`events.rs`。
- 已完成：已为 `RuntimeRunState` 接入 `PlanEnvelope`，并把主链从单步收口改成受控多步迭代。
- 已完成：已补 `plan_iteration_started`、`plan_iteration_completed`、`replan_requested`、`iteration_budget_exhausted` 事件，以及 iteration metadata 到上下文快照。
- 已完成：已补 observation 生命周期映射与相关测试，保证新增事件进入既有观测链。
- 已完成：`cargo test -p runtime-core` 全绿，当前为 196 项测试通过。
- 阻塞点：暂无。
- 下一步：若继续第二刀，按既定顺序拆上下文 profile change，不在本项内继续混做记忆、知识检索或 verify 矩阵。
