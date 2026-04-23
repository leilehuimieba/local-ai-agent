# H-memory-object-review-20260423（tasks）

更新时间：2026-04-23  
状态：并行 change 进行中

| ID | 任务 | 类型 | 状态 | 验收标准 | 证据 |
|---|---|---|---|---|---|
| HMO-01 | 冻结 memory 升级边界与模块落点 | 设计 | done | `proposal.md`、`design.md` 明确最小范围、模块落点与后置能力 | `proposal.md`、`design.md` |
| HMO-02 | 补 system views 只读层设计与接线点 | 设计 | done | 明确 `memory_views.rs -> memory_recall.rs -> context_builder.rs` 的最小接线顺序 | `design.md` |
| HMO-03 | 补 object/version/alias 最小模型与兼容策略 | 设计 | done | 明确新增表、旧表兼容读写与 migration 顺序 | `design.md` |
| HMO-04 | 补 review/rollback 最小接口与 artifact 口径 | 设计 | done | 明确 history/diff/rollback 的接口边界，不引入复杂因果折叠 | `design.md`、`verify.md` |
| HMO-05 | 冻结验证与回退原则 | 验证设计 | done | 明确 system views、migration、rollback 与 H-05 兼容性证据 | `verify.md` |
| HMO-06 | 实现阶段 1：新增 `memory_views.rs` 与 system views 摘要装配 | 实现 | done | `boot/recent/index/rules/workspace` 五类视图可生成受控摘要 | 代码变更、单元测试、`tmp/stage-h-memory-object-review/system-views.json` |
| HMO-07 | 实现阶段 1：将 system views 接入 `memory_recall.rs` 与 `context_builder.rs` | 实现 | done | recall digest 可优先消费 system views，且不扩大上下文预算 | 代码变更、回归测试、`tmp/stage-h-memory-object-review/system-views.json` |
| HMO-08 | 实现阶段 2：新增 object/version/alias SQLite 表与最小 migration | 实现 | done | `sqlite_store.rs`、`storage_migration.rs` 支持新表建表与兼容导入 | 代码变更、迁移测试、`tmp/stage-h-memory-object-review/migration-report.json` |
| HMO-09 | 实现阶段 2：抽离 `memory_object_store.rs` 最小读写接口 | 实现 | done | 支持创建对象、追加版本、切 current、查 alias | 代码变更、单元测试、`tmp/stage-h-memory-object-review/migration-report.json` |
| HMO-10 | 实现阶段 3：补 memory object history/diff/rollback | 实现 | done | rollback 可恢复目标版本，且 recall 不混淆旧新版本 | 代码变更、回退演练、`tmp/stage-h-memory-object-review/rollback-drill.json` |
| HMO-11 | 兼容性复核：H-05 记忆路由与现有 recall 主链回归 | 验证 | done | 不破坏 H-05 证据口径；现有 `search_memory_entries()` 与 `recall_memory_digest()` 仍可用 | 回归记录、`verify.md`、`tmp/stage-h-memory-object-review/compatibility-report.json` |
| HMO-12 | 提审收口 | 验证 | done | proposal/design/tasks/status/verify 与实现证据完整 | `status.md`、`verify.md`、提审包 |

## 执行顺序

1. 设计冻结：HMO-01 -> HMO-02 -> HMO-03 -> HMO-04 -> HMO-05
2. 实现主链：HMO-06 -> HMO-07 -> HMO-08 -> HMO-09 -> HMO-10
3. 收口主链：HMO-11 -> HMO-12
4. 当前已完成第一批 `HMO-06 / HMO-07`、第二批 `HMO-08 / HMO-09`、第三批 `HMO-10` 与 `HMO-11 / HMO-12` 收口。
5. 若后续继续推进，仍应先复核 Gate-H 主推进状态，再决定是否扩大到 recall 主链切换或 review UI。

## 当前最小实施建议

1. 第一批已完成：
   - `system views` 已作为只读派生层接入 recall digest；
   - 未扩大为全量正文注入。
2. 第二批已完成：
   - 引入 object/version/alias；
   - 保留旧 `MemoryEntry` 主路径。
3. 第三批已完成 HMO-10：
   - history/diff/rollback 已补齐；
   - rollback 采用“恢复为新 current version”策略，并同步旧 `long_term_memory` 主路径。
4. 当前追加优化（不另起主 change）：
   - current memory object 已以最小方式进入 `search_memory_entries()`；
   - 支持 URI / alias 参与命中；
   - 仍保留旧 `MemoryEntry` 主路径兼容；
   - `context_builder.rs` 已在消费 `recall_memory_digest()` 时保留 object-aware 分层标记，不把 current memory object 重新压平成普通记忆摘要；
   - `prompt.rs`、`run_context_metadata.rs`、`events.rs` 已补最小结构化透传，使 object-aware 标记可进入 prompt、metadata 与 event context snapshot；
   - `memory_recalled` 事件已补 object-aware 标题、原因与结构化层信息，不再只输出泛化“已完成记忆召回”；
   - `executors/memory.rs`、`verify.rs`、`tool_trace.rs` 当前已能在 recall 执行结果、验证摘要与 trace artifact 中显式保留 object-aware recall 引用文案；
   - `run_finish_events.rs` 与 `run_finished / run_failed` 收口事件当前已能显式带出 recall 层摘要，使最终结果可直接看出是否发生 object-aware recall；
   - 已新增 `memory_layer.rs`，把 digest / metadata / reasoning 三处的 recall layer 文案与提取逻辑收敛为单一 helper，减少后续漂移。
