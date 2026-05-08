# 技术方案

## 影响范围

- 当前主目标文件：
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/mod.rs`
- 直接支撑文件：
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_schema.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/run_memory_metadata.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/storage_migration.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/executors/memory.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_object_store.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_recall.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/memory_object.rs`
- 状态与 change 入口：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`

## 方案

### 1. 范围冻结原则

本 change 只处理 **memory 写回治理簇**，并按以下边界冻结：

1. 允许处理：
   - memory 写回字段在 entry/schema/sqlite/event metadata 链上的结构收口；
   - `memory.rs / memory_router/mod.rs / sqlite_store/mod.rs` 的热点治理与模块化准备；
   - 与上述链路直接相关的最小测试迁移与接线修正。
2. 不允许处理：
   - browser 失败恢复、verify snapshot 扩面、handoff artifact 扩面；
   - knowledge answer 主链行为；
   - context/profile/replan/query_engine 再拆分；
   - 新一轮长期记忆语义策略扩面。

### 2. 当前优先级顺序

1. **第一优先：`memory_router/mod.rs`**
   - 原因：它是当前写回决策主入口，且 744 行越线最明显；
   - 负责的主题集中：
     - `evaluate_finish_memory_writes`
     - `working_memory_outcome`
     - `write_long_term_memory`
     - `write_preference_memory`
     - `write_failure_lesson_memory`
     - audit / skipped / duplicate / reject outcome
2. **第二优先：`sqlite_store/mod.rs`**
   - 原因：712 行，且承接 schema/migration/persistence；
   - 负责的主题集中：
     - `long_term_memory` schema 与 migration
     - memory entry insert/load/map/update governance
3. **第三优先：`memory.rs`**
   - 原因：658 行，承接 entry 归一化与回填；
   - 负责的主题集中：
     - `MemoryEntry`
     - `normalized_memory_entry`
     - governance/write-layer fallback

### 3. 建议模块边界（本轮仅备案，不实现）

1. `memory_router/`
   - `write_policy.rs`：归层与 accepted/rejected/duplicate 决策
   - `audit.rs`：`MemoryAuditTrail / MemoryWriteOutcome` 与 skipped/written audit
   - `entries.rs`：`preference_entry / failure_lesson_entry / auto_memory_entry`
2. `sqlite_store/`
   - `memory_entry_store.rs`：`insert/load/map/update governance`
   - `memory_schema_migrations.rs`：schema 与 migrations
3. `memory.rs`
   - 保留核心类型与最小 normalize 入口
   - 把 derived/fallback/governance helper 外提

### 4. 本轮与 AO 的关系

- `AO` 的职责是“功能治理闭环成立并签收”；
- `AV` 的职责是“围绕 AO 已形成的未提交 memory 改动继续做结构收口与热点治理”；
- 因此 `AV` 不是否定 `AO`，而是承接 `AO` 在真实工作区中的结构化落地与入库准备。

### 5. 风险与回退

- 风险 1：把 memory 写回治理和 knowledge/base/browser 混在同一轮改动里。
  - 回退：仅保留 memory 簇文件，发现跨簇实现企图时立即停止并回到 AV 文档边界。
- 风险 2：为降热点行数而顺手改语义。
  - 回退：坚持“职责搬移优先，行为不变优先”，每一刀都要能回到 AO 已签收语义。
- 风险 3：拆分时测试夹具膨胀，再踩函数长度红线。
  - 回退：优先按职责拆 tests/mod，保持新增或修改函数 <= 30 行。
