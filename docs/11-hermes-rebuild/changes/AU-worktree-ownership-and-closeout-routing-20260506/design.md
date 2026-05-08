# 技术方案

## 影响范围

- 状态入口：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
- 本 change 工作区：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AU-worktree-ownership-and-closeout-routing-20260506/`
- 只读核对范围：
  - `D:/newwork/本地智能体/crates/runtime-core/src/`
  - `D:/newwork/本地智能体/frontend/scripts/browser-mcp-server.mjs`
  - `D:/newwork/本地智能体/gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`
  - `D:/newwork/本地智能体/scripts/`
  - `D:/newwork/本地智能体/docs/07-test/evidence/`
  - `D:/newwork/本地智能体/tmp/`

## 方案

### 1. 归属矩阵分组

当前未提交改动按“已有 change 归属”先分成四个主簇：

1. **结构拆分簇**
   - 已有 change：`AS`、`AT`
   - 代表文件：
     - `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/verify/`
     - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/`
   - 结论：功能/结构已完成，当前问题不是再实现，而是“已收口但仍未入库”。

2. **记忆写回治理簇**
   - 已有 change：`AO`
   - 代表文件：
     - `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/tests.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/run_memory_metadata.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/mod.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/memory_schema.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/storage_migration.rs`
   - 结论：这是当前最成体系、且仍带热点文件压力的一组未收口代码。

3. **浏览器恢复 / 高风险交互 / verify 元数据簇**
   - 已有 change：`AN`、`AM`、部分 `AL`
   - 代表文件：
     - `D:/newwork/本地智能体/crates/runtime-core/src/contracts.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/events.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/run_verification_metadata.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/handoff.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/run_failure_metadata.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/run_finish_events.rs`
     - `D:/newwork/本地智能体/frontend/scripts/browser-mcp-server.mjs`
     - `D:/newwork/本地智能体/gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`
   - 结论：这组改动有明确主题，但跨 runtime / frontend / gateway，适合后续作为单独收口 change，不应与 memory 簇混做。

4. **知识回答收口 / 回归包簇**
   - 已有 change：`AQ`、`AP`
   - 代表文件：
     - `D:/newwork/本地智能体/crates/runtime-core/src/executors/project.rs`
     - `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`
     - `D:/newwork/本地智能体/scripts/README.md`
     - `D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/`
     - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/`
   - 结论：主链修复与回归脚本已形成独立材料，当前问题是“输入 / 证据 / 实现目录仍未完全并账”。

### 2. 热点文件备案判断

当前 `runtime-core/src` 中超过 600 行且不属于测试目录的热点文件，至少包括：

- `D:/newwork/本地智能体/crates/runtime-core/src/observation/mod.rs`（1108）
- `D:/newwork/本地智能体/crates/runtime-core/src/planner.rs`（923）
- `D:/newwork/本地智能体/crates/runtime-core/src/knowledge.rs`（908）
- `D:/newwork/本地智能体/crates/runtime-core/src/context_builder.rs`（876）
- `D:/newwork/本地智能体/crates/runtime-core/src/events.rs`（769）
- `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`（744）
- `D:/newwork/本地智能体/crates/runtime-core/src/executors/context.rs`（737）
- `D:/newwork/本地智能体/crates/runtime-core/src/lib.rs`（731）
- `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/mod.rs`（712）
- `D:/newwork/本地智能体/crates/runtime-core/src/executors/patch.rs`（701）
- `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`（658）

本轮只做备案，不直接开拆；但要给出优先级：

1. 第一优先：`memory_router/mod.rs`、`sqlite_store/mod.rs`、`memory.rs`
   - 原因：这些文件既越线，又正处于 `AO` 相关未提交改动主簇中。
2. 第二优先：`events.rs`、`context_builder.rs`
   - 原因：当前已有改动进入这些文件，但尚不应与 memory 收口同时推进。
3. 继续观察：`observation/mod.rs`、`planner.rs`、`knowledge.rs`、`executors/context.rs`、`lib.rs`、`executors/patch.rs`
   - 原因：虽然越线，但当前未提交主簇并不以它们为核心。

### 3. `scripts / evidence / tmp` 收口原则

1. `scripts/`
   - 新增脚本 `run-knowledge-answer-eval-pack.ps1` 属于 `AP` 回归入口，应保留并通过 `scripts/README.md` 统一索引。
2. `docs/07-test/evidence/`
   - `20260506-an-browser-recovery-governance/` 与 `20260506-ap-knowledge-answer-eval-pack/` 属于有效证据目录，应保留并在对应 change 中可追溯。
3. `tmp/`
   - 本轮不直接清空；只记录：`tmp/knowledge-answer-evals/` 为当前有效回归产物目录。
   - 其它早于 7 天的实验目录和历史日志，后续如要清理，应单独建立清理型 change，避免混入当前归属判断。

### 4. 下一主推进项裁决方法

本 change 不直接进入实现；只裁决“下一把最小实现 change”应满足：

- 与当前未提交改动主簇一致；
- 有明确热点治理收益；
- 不跨 runtime / frontend / gateway 多域混写；
- 能在已有 change 基础上优先收口，而不是重新发散。

按这个标准，下一把最小实现项优先建议落到 **memory 写回治理簇的结构化收口**，而不是继续扩 browser 或知识主链。

## 风险与回退

- 风险 1：把同一文件的不同主题改动错误归到同一 change。
  - 回退：以“主行为主题 + 证据目录 + 已有 change 文档”三重交叉校验，不能确认时保持“待裁决”，不强归类。
- 风险 2：在归属梳理过程中顺手扩成真实实现。
  - 回退：AU 范围冻结为文档 / 状态 / 归属 / 备案，不做业务逻辑实现。
- 风险 3：把 `tmp/` 清理和工作区归属混在一起，造成证据丢失。
  - 回退：本轮只记录入口，不执行物理清理。
