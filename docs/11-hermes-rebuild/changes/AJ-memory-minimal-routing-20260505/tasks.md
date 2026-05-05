# 任务清单

- [x] 任务 1：冻结第三刀记忆最小路由范围与落点
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只做记忆最小路由”保持一致。
- [x] 任务 2：实现最小 memory route 选择
  完成判据：`memory_router/mod.rs` 能根据 profile、动作类型、恢复态和查询意图选出最小读层。
- [x] 任务 3：实现 route-aware memory digest 与 metadata
  完成判据：`memory_recall.rs`、`context_builder.rs`、`events.rs` 能输出 route、层选择原因、最小 digest 与 snapshot 字段。
- [x] 任务 4：补第三刀验证证据
  完成判据：至少补齐 route 选择测试、不同 route 的 digest 差异证据，以及恢复态不会误回退为无差别全量摘要的回归。

## 实现记录

- 已完成：在 `memory_router/read_route.rs` 落最小读侧路由，形成 `ask_route`、`act_route`、`repair_route`、`learn_route` 四类选择。
- 已完成：路由信号收敛到 `ContextAssemblyPolicy.profile`、`RunRequest.resume_from_checkpoint_id` 与查询词启发式，不在 Go 侧重复计算主路由。
- 已完成：`memory_recall.rs` 改为 route-aware 最小 digest，新增 `memory_route`、`selected_layers`、`match_reason`、`reuse_confidence`、`skipped_layers` 五个字段。
- 已完成：`context_builder.rs`、`run_context_metadata.rs`、`events.rs`、`contracts.rs`、`gateway/internal/contracts/contracts.go` 已贯通 route 字段。
- 已完成：`memory_layer.rs` 增补 route summary helper，统一 digest / metadata / event 文案口径。
- 未纳入：知识 pack、知识检索增强、verify 矩阵、写侧长期记忆治理重构。
