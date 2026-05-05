# 当前状态

- 最近更新时间：2026-05-05
- 状态：已完成实现并补齐验证
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`
- 已完成：已从 `AG-agent-loop-memory-knowledge-20260505` 蓝图中拆出第三刀独立实现 change。
- 已完成：已冻结本 change 只覆盖记忆最小路由，不混入知识 pack、知识检索增强或 verify 矩阵。
- 已完成：已明确第三刀优先改造 `memory_router/mod.rs`、`memory_recall.rs`、`memory_layer.rs`、`context_builder.rs`、`events.rs`、`gateway/internal/api/learning_memory.go`。
- 已完成：`memory_router/read_route.rs` 已形成 ask / act / repair / learn 四类最小读侧路由。
- 已完成：`memory_recall.rs` 已按 route 生成最小 digest，并输出 route、选层、跳层、命中理由与复用置信度。
- 已完成：`context_builder.rs`、`run_context_metadata.rs`、`events.rs`、`contracts.rs`、`gateway/internal/contracts/contracts.go` 已补 route metadata 与 snapshot 字段。
- 已完成：`memory_layer.rs` 已补 route summary helper，事件文案与 prompt 文案保持统一。
- 已完成：`cargo fmt --all` 与 `cargo test -p runtime-core` 已通过。
- 阻塞点：暂无。
- 下一步：如继续推进，优先从第四刀或后续独立 change 进入知识检索增强 / verify 矩阵，不在 AJ 内继续扩 scope。
