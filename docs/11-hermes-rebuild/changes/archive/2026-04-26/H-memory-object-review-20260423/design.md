# H-memory-object-review-20260423（design）

更新时间：2026-04-23  
状态：草案冻结（待实现）

## 1. Harness 判断

1. 当前补的是哪层壳：
   - 记忆与知识路由；
   - 上下文装配；
   - 验证闭环。
2. 当前最明显的薄弱层：
   - 长期记忆对象缺少稳定身份与版本链；
   - 缺少面向 agent 的固定系统视图；
   - 缺少 memory object 级别的独立 review/rollback。
3. 本草案不补的壳：
   - 主循环；
   - 工具协议统一；
   - 权限与确认主线；
   - 多智能体协同；
   - 外部服务化部署。

## 2. 影响范围

1. 预计涉及模块：
   - `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/memory_recall.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/context_builder.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/memory_schema.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/storage_migration.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/paths.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/executors/memory.rs`
   - 必要时新增：
     - `D:/newwork/本地智能体/crates/runtime-core/src/memory_views.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/memory_object_store.rs`
2. 文档与证据落点：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-object-review-20260423/`
   - `D:/newwork/本地智能体/tmp/stage-h-memory-object-review/`

## 3. 最小改动路径

### 3.1 Phase 1：system views

1. 新增固定系统视图，不要求先引入 MCP 协议层：
   - `system://boot`
   - `system://recent`
   - `system://index`
   - `system://rules`
   - `system://workspace/<workspace_id>`
2. 当前数据来源保持最小复用：
   - seed memory；
   - 高优先级长期记忆；
   - 最近知识条目；
   - 最近 observation 锚点。
3. 当前目标是稳定启动与续跑入口，不提前扩为全量记忆目录浏览器。
4. 当前落点优先级：
   - `memory_views.rs` 负责 system views 的组装；
   - `memory_recall.rs` 负责在现有 recall digest 前插入 system views；
   - `context_builder.rs` 继续只消费摘要，不直接展开全量视图正文。

### 3.2 Phase 2：memory object + version

1. 引入最小对象模型：
   - `memory_object_id`
   - `memory_version_id`
2. 推荐最小表结构：
   - `memory_objects`
   - `memory_versions`
   - `memory_aliases`
3. 当前只要求支持：
   - 同一对象的当前版本；
   - 追加新版本；
   - alias/uri 绑定；
   - 基础 current 指针切换。
4. 当前不要求：
   - 图结构 `node/edge/path`；
   - 多父关系；
   - 跨 namespace 复杂拓扑。
5. 当前兼容策略：
   - 旧的 `long_term_memory` 仍保留读取能力；
   - object/version 先作为新增表结构接入 SQLite；
   - migration 先保证“新表可读写，不影响旧表 recall 主路径”。

### 3.3 Phase 3：review / rollback

1. 提供最小接口：
   - `get_memory_history(object_id)`
   - `diff_memory_versions(object_id, from, to)`
   - `rollback_memory_object(object_id, target_version_id)`
2. 当前 rollback 粒度锁定在 memory object，不做 Nocturne 式跨多表复杂因果折叠。
3. 当前 review 输出以结构化 artifact 为主，前端 UI 后置。
4. 当前落点优先级：
   - `memory_object_store.rs` 提供 history/diff/rollback 读写接口；
   - `sqlite_store.rs` 负责新增表读写；
   - `executors/memory.rs` 后续如需暴露人工入口，再做最小对接。

## 4. 召回路径调整

1. 当前推荐召回顺序：
   - system views；
   - alias / uri 命中；
   - trigger / keyword 命中；
   - 高优先级 current memory objects；
   - knowledge recall；
   - observation 辅助证据。
2. 当前不允许把全部长期记忆直接塞进 prompt。
3. 当前仍坚持摘要注入与预算限制，不回退到“多塞点记忆保安全”的做法。
4. 当前兼容要求：
   - `search_memory_entries()` 保持可用；
   - `recall_memory_digest()` 可先接入 system views，再逐步引入 object/version；
   - H-05 学习记忆路由相关证据口径不得被这轮方案直接打断。

## 5. 明确后置的高级能力

1. 图后端与路径拓扑。
2. 向量检索或语义召回主路径。
3. 新的独立 memory 服务进程。
4. 完整前端 review 面板。
5. 跨多租户/多 agent 的复杂 namespace 模型。

## 6. 风险与回退

1. 风险：
   - 旧的 `MemoryEntry` 与新的 object/version 双轨共存期可能导致口径分裂；
   - 若 system views 设计过重，可能反向扩大上下文注入；
   - rollback 若直接覆盖旧写回逻辑，可能冲击已有 H-05 证据口径。
2. 回退方式：
   - 保留现有 `MemoryEntry` 搜索与 recall 主路径；
   - system views 先作为只读派生层；
   - object/version 先走兼容写入，不立即替换全部旧记录读取。
3. 实施顺序约束：
   - 先做只读 system views；
   - 再做 SQLite 新表与兼容 migration；
   - 最后才接入 rollback；
   - 任一阶段若破坏现有 recall digest 或 H-05 证据链，立即回退到上一阶段。
