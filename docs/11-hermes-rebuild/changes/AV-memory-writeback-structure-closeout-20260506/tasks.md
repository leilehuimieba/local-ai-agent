# 任务清单

- [x] 任务 1：读取 AU 与 AO 当前口径
  完成判据：已读取 `current-state.md`、`changes/INDEX.md`、`AU` 五件套关键文档，以及 `AO` 的 `status.md` / `review.md`。
- [x] 任务 2：确认 memory 写回治理簇仍是下一优先实现项
  完成判据：已确认 `memory.rs / memory_router/mod.rs / sqlite_store/mod.rs` 既属于同一未提交改动主簇，又全部越过 600 行红线。
- [x] 任务 3：建立 `AV-memory-writeback-structure-closeout-20260506` 正式 change 工作区
  完成判据：已补齐 `proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md`。
- [x] 任务 4：冻结本 change 范围
  完成判据：已明确只处理 memory 写回治理簇，不混 browser / knowledge / verify / query_engine。
- [x] 任务 5：同步状态入口
  完成判据：`current-state.md` 与 `changes/INDEX.md` 已切到 `AV` 作为当前活跃 change。
- [x] 任务 6：给出后续最小实现顺序
  完成判据：已明确后续优先顺序为 `memory_router/mod.rs -> sqlite_store/mod.rs -> memory.rs`。
- [x] 任务 7：实施第一刀结构收口
  完成判据：已从 `memory_router/mod.rs` 抽出 `knowledge_write.rs`，且不改 AO 已签收语义。
- [x] 任务 8：执行第一刀定向验证并补证据
  完成判据：`cargo test -p runtime-core memory_router -- --nocapture` 与 `cargo test -p runtime-core memory_write_ -- --nocapture` 通过，并写回 `verify.md`。
- [x] 任务 9：实施第二刀结构收口
  完成判据：已从 `memory_router/mod.rs` 抽出 `write_policy.rs`，仅迁移 write policy 相关职责，不混入 `audit`、`entries`、`sqlite_store/mod.rs` 或 `memory.rs`。
- [x] 任务 10：执行第二刀定向验证并补证据
  完成判据：第二刀后再次通过 `cargo test -p runtime-core memory_router -- --nocapture` 与 `cargo test -p runtime-core memory_write_ -- --nocapture`，并把剩余拆分建议写回 `status.md` / `verify.md`。
- [x] 任务 11：实施第三刀结构收口
  完成判据：已从 `memory_router/mod.rs` 抽出 `audit.rs`，仅迁移审计留痕相关函数，不混入 `entries`、`sqlite_store/mod.rs` 或 `memory.rs`。
- [x] 任务 12：执行第三刀定向验证并补证据
  完成判据：第三刀后再次通过 `cargo test -p runtime-core memory_router -- --nocapture` 与 `cargo test -p runtime-core memory_write_ -- --nocapture`，并明确是否继续留在 `memory_router` 做第四刀。
- [x] 任务 13：切换 `AV` 主热点到 `sqlite_store/mod.rs`
  完成判据：已完成 `sqlite_store/mod.rs` 结构盘点，并确认优先抽离 `schema / migration` 与清理判定规则，而不混入 `memory.rs`。
- [x] 任务 14：实施 `sqlite_store/mod.rs` 第一刀与第二刀最小结构收口
  完成判据：已抽出 `schema.rs` 与 `cleanup_rules.rs`，仅迁移 schema / migration 常量和清理判定规则，不混入 `memory.rs` 或其它模块。
- [x] 任务 15：执行 `sqlite_store/mod.rs` 定向验证并补证据
  完成判据：`sqlite_store/mod.rs` 已降到项目红线以下，并通过 `cargo test -p runtime-core memory_object_store -- --nocapture` 与 `cargo test -p runtime-core checkpoint -- --nocapture`。
- [x] 任务 16：切换 `AV` 主热点到 `memory.rs`
  完成判据：已完成 `memory.rs` 结构盘点，并确认第一刀优先抽离记忆治理 / 过滤策略，而不混入评分检索主链。
- [x] 任务 17：实施 `memory.rs` 第一刀最小结构收口
  完成判据：已抽出 `policy.rs`，仅迁移标准化治理、归档判定与过滤策略，不混入评分排序、对象召回或 `sqlite_store`。
- [x] 任务 18：执行 `memory.rs` 定向验证并补证据
  完成判据：`memory.rs` 已降到项目红线以下，并通过 `cargo test -p runtime-core memory::tests -- --nocapture`、`cargo test -p runtime-core memory_object_store -- --nocapture` 与 `cargo test -p runtime-core memory_recall -- --nocapture`。
- [x] 任务 19：执行 `AV` 聚合验证
  完成判据：已通过覆盖 `memory write / recall / object / checkpoint` 主链的最小聚合验证。
- [x] 任务 20：完成 `AV` 收口裁决
  完成判据：已确认 `AV` 满足收口条件，并切换到下一主推进项。
