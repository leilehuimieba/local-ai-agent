# 当前状态

- 最近更新时间：2026-05-05
- 状态：已实现，待后续刀口衔接
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`
- 已完成：已从 `AG-agent-loop-memory-knowledge-20260505` 蓝图中拆出第二刀独立实现 change。
- 已完成：已冻结本 change 只覆盖 `ask / act / repair / learn` 四类上下文 profile，不混入记忆分层、知识 pack 或 verify 矩阵。
- 已完成：已明确第二刀优先改造 `context_policy.rs`、`context_builder.rs`、`run_context_metadata.rs`、`events.rs`、`gateway/internal/api/chat_context_resolver.go`。
- 已完成：Runtime 已收口四类 `assembly_profile`，并新增 `prompt_profile` 兼容现有 prompt 渲染分支。
- 已完成：`context_builder.rs` 已按 profile 输出不同最小包，并把 `profile / prompt_profile / selection_reason / injection_summary` 挂入 metadata、context snapshot 与事件快照。
- 已完成：Go 侧 contracts 已补齐 `prompt_profile` 与 `injection_summary` 字段，Gateway 仍只承担 hints 透传，不承担 profile 主判断。
- 已完成：`cargo fmt --all` 与 `cargo test -p runtime-core` 已通过。
- 阻塞点：暂无。
- 下一步：按顺序进入下一刀上下文 profile 后续衔接，但不要在本刀回补记忆分层、知识 pack 或 verify 矩阵。
