# 当前状态

- 最近更新时间：2026-05-05
- 状态：已完成实现，待后续独立 change 继续扩知识质量或 verify 矩阵
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`
- 已完成：已从 `AG-agent-loop-memory-knowledge-20260505` 蓝图中拆出第四刀独立实现 change。
- 已完成：已冻结本 change 只覆盖知识检索增强与 knowledge pack，不混入 verify 矩阵。
- 已完成：已明确第四刀优先改造 `gateway/internal/knowledge/store.go`、`crates/runtime-core/src/knowledge.rs`、`knowledge_store.rs`、`context_builder.rs`、`events.rs`。
- 已完成：`gateway/internal/knowledge/store.go` 已改为“全量候选 + 轻量重排”，不再被 `%整句%` SQL 过滤提前截断弱命中项。
- 已完成：`crates/runtime-core/src/knowledge.rs` 已补 `source_kind`、`use_for`、`citation_ready`、`match_reason`，并产出最小 `knowledge pack`。
- 已完成：`context_builder.rs`、`prompt.rs`、`events.rs`、`run_context_metadata.rs`、`contracts.rs` 已贯通 `knowledge_pack_question_type`、`knowledge_pack_citations`、`knowledge_pack_match_reason`。
- 已完成：ask / learn profile 会优先注入 `knowledge pack` 摘要；project status 问答仍保留 current-state 优先策略。
- 验证结果：`cargo test -p runtime-core` 通过 208 项；`go test ./internal/knowledge/...` 在 `gateway/` 模块下通过。
- 阻塞点：暂无。
- 下一步：主推进已切到 `AL-verify-matrix-minimal-20260505`，由 verify 矩阵先验证当前知识回答主链，不回 AK 扩 scope。
