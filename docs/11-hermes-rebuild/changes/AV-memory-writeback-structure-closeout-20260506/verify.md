# 验证记录

## 验证方式

- 文档入口检查：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AU-worktree-ownership-and-closeout-routing-20260506/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/AO-memory-writeback-governance-20260506/`
- 热点文件检查：
  - `memory.rs`
  - `memory_router/mod.rs`
  - `sqlite_store/mod.rs`
- 第一刀结构检查：
  - 确认 `memory_router/mod.rs` 只抽出知识层写回子模块
  - 确认 `sqlite_store/mod.rs` 与 `memory.rs` 本轮未被混入
- 第二刀结构检查：
  - 确认 `memory_router/mod.rs` 只抽出 write policy 子模块
  - 确认 `audit`、`entries`、`sqlite_store/mod.rs` 与 `memory.rs` 本轮未被混入
- 第三刀结构检查：
  - 确认 `memory_router/mod.rs` 只抽出 audit 子模块
  - 确认 `entries`、`sqlite_store/mod.rs` 与 `memory.rs` 本轮未被混入
- `sqlite_store` 第一轮结构检查：
  - 确认 `sqlite_store/mod.rs` 只抽出 `schema.rs` 与 `cleanup_rules.rs`
  - 确认本轮未混入 `memory.rs`
- `memory.rs` 第一刀结构检查：
  - 确认 `memory.rs` 只抽出 `policy.rs`
  - 确认本轮未混入评分排序、对象召回主链或 `sqlite_store`
- 定向测试：
  - `cargo test -p runtime-core memory_router -- --nocapture`
  - `cargo test -p runtime-core memory_write_ -- --nocapture`
  - `cargo test -p runtime-core memory_object_store -- --nocapture`
  - `cargo test -p runtime-core checkpoint -- --nocapture`
  - `cargo test -p runtime-core memory::tests -- --nocapture`
  - `cargo test -p runtime-core memory_recall -- --nocapture`
- 聚合验证：
  - `cargo test -p runtime-core memory_router -- --nocapture`
  - `cargo test -p runtime-core memory_write_ -- --nocapture`
  - `cargo test -p runtime-core memory_object_store -- --nocapture`
  - `cargo test -p runtime-core checkpoint -- --nocapture`
  - `cargo test -p runtime-core memory::tests -- --nocapture`
  - `cargo test -p runtime-core memory_recall -- --nocapture`

## 证据位置

- 上游裁决：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AU-worktree-ownership-and-closeout-routing-20260506/status.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AU-worktree-ownership-and-closeout-routing-20260506/verify.md`
- 上游已签收实现：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/AO-memory-writeback-governance-20260506/status.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/AO-memory-writeback-governance-20260506/review.md`
- 当前第一刀实现：
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/knowledge_write.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/tests.rs`
- 当前第二刀实现：
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/write_policy.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/tests.rs`
- 当前第三刀实现：
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/audit.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/tests.rs`
- 当前 `sqlite_store` 第一轮实现：
  - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/schema.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/cleanup_rules.rs`
- 当前 `memory.rs` 第一刀实现：
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory/policy.rs`

## 当前验证结论

1. `AV` 建档理由成立：memory 写回治理簇是当前已知未提交改动中最成体系、也最适合作为下一实现入口的一组。
2. 当前三大目标文件在第一刀前均越过项目热点红线：
   - `memory.rs`：658 行
   - `memory_router/mod.rs`：744 行
   - `sqlite_store/mod.rs`：712 行
3. 第一刀已完成：已从 `memory_router/mod.rs` 抽出 `knowledge_write.rs`，且未同时混入 `sqlite_store/mod.rs` 与 `memory.rs`。
4. 第二刀已完成：已从 `memory_router/mod.rs` 抽出 `write_policy.rs`，且未同时混入 `audit`、`entries`、`sqlite_store/mod.rs` 与 `memory.rs`。
5. 第三刀已完成：已从 `memory_router/mod.rs` 抽出 `audit.rs`，且未同时混入 `entries`、`sqlite_store/mod.rs` 与 `memory.rs`。
6. `memory_router/mod.rs` 当前为 `504` 行，进一步低于 `600` 行热点红线。
7. `sqlite_store` 第一轮已完成：已从 `sqlite_store/mod.rs` 抽出 `schema.rs` 与 `cleanup_rules.rs`，且未混入 `memory.rs`。
8. `sqlite_store/mod.rs` 已从 `774` 行下降到 `551` 行，热点红线已解除。
9. `memory.rs` 第一刀已完成：已从 `memory.rs` 抽出 `policy.rs`，且未混入评分排序、对象召回主链或 `sqlite_store`。
10. `memory.rs` 已从 `721` 行下降到 `556` 行，热点红线已解除。
11. 定向测试通过：
   - `cargo test -p runtime-core memory_router -- --nocapture`：`10 passed; 0 failed`
   - `cargo test -p runtime-core memory_write_ -- --nocapture`：`2 passed; 0 failed`
   - `cargo test -p runtime-core memory_object_store -- --nocapture`：`6 passed; 0 failed`
   - `cargo test -p runtime-core checkpoint -- --nocapture`：`10 passed; 0 failed`
   - `cargo test -p runtime-core memory::tests -- --nocapture`：`10 passed; 0 failed`
   - `cargo test -p runtime-core memory_recall -- --nocapture`：`7 passed; 0 failed`
12. `AO` 已签收语义未被扩面；当前 `memory_router` 三刀、`sqlite_store` 第一轮与 `memory.rs` 第一刀均属于纯结构收口。
13. 当前三个热点文件都已低于红线：`memory_router/mod.rs` 504 行、`sqlite_store/mod.rs` 551 行、`memory.rs` 556 行；后续不宜默认继续大拆。
14. 本轮聚合验证再次全绿，说明 memory 写回治理簇的 write / recall / object / checkpoint 主链在结构收口后仍保持稳定。
15. 以 `AV` 的目标范围来看，当前已满足“结构收口 + 热点治理准备完成”的收口条件，可切下一主推进项。

## Gate 映射

- 对应阶段 Gate：`Gate-I（已收口；当前处于自由迭代期）`
- 当前覆盖情况：
  - 已完成下一把实现 change 的正式建档
  - 已完成 memory 写回治理簇范围冻结
  - 已完成热点目标排序与入口切换
  - 已完成 `memory_router/mod.rs` 第一刀结构收口与定向验证
  - 已完成 `memory_router/mod.rs` 第二刀结构收口与定向验证
  - 已完成 `memory_router/mod.rs` 第三刀结构收口与定向验证
  - 已完成 `sqlite_store/mod.rs` 第一轮结构收口与定向验证
  - 已完成 `memory.rs` 第一刀结构收口与定向验证
  - 已完成 `AV` 聚合验证与收口条件判断
