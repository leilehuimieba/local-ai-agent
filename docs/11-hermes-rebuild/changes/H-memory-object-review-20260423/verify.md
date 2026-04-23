# H-memory-object-review-20260423（verify）

更新时间：2026-04-23  
状态：验证已收口（前三批 + 兼容性复核已完成）

## 验证方式

### 1. 文档验证

1. 核对本 change 是否明确：
   - 当前只为并行 change；
   - 不切主推进；
   - 最小范围为 `system views + object/version + review/rollback`。
2. 核对本 change 是否明确后置能力：
   - 图结构；
   - 向量检索主路径；
   - 新的独立 memory 服务；
   - 完整前端 UI。

### 2. 当前已执行的最小复核

1. 已确认工作区存在 `system views` 初步实现：
   - `D:/newwork/本地智能体/crates/runtime-core/src/memory_views.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/memory_recall.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/lib.rs`
2. 已执行定向单测：
   - `cargo test -p runtime-core builds_five_system_views --lib -- --nocapture`
   - `cargo test -p runtime-core recall_digest_includes_system_view_lines --lib -- --nocapture`
3. 当前结论：
   - `system views` 五类视图已具备最小生成能力；
   - recall digest 已能包含 `system://*` 视图摘要；
   - `cargo test -p runtime-core memory_recall --lib -- --nocapture` 与 `cargo test -p runtime-core context_builder --lib -- --nocapture` 当前已可通过；
   - 当前仍未完成更大范围兼容性回归，不可表述为“已验证通过”。
4. 已确认第二批最小实现与验证：
   - 工作区存在：
     - `D:/newwork/本地智能体/crates/runtime-core/src/memory_object_store.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store.rs`
     - `D:/newwork/本地智能体/crates/runtime-core/src/storage_migration.rs`
   - 已执行定向单测：
     - `cargo test -p runtime-core sync_versions_share_object_and_update_current -- --nocapture`
     - `cargo test -p runtime-core sync_creates_alias_for_object -- --nocapture`
   - 当前结论：
     - object/version/alias 最小存储层已可工作；
     - 旧 `long_term_memory` 主路径仍保留；
     - 当前 rollback 实现前置条件已满足。

### 3. 当前已执行的 rollback 复核（HMO-10）

1. 已执行定向单测：
   - `cargo test -p runtime-core history_lists_versions_newest_first -- --nocapture`
   - `cargo test -p runtime-core diff_reports_summary_and_content_changes -- --nocapture`
   - `cargo test -p runtime-core rollback_creates_restored_current_version -- --nocapture`
   - `cargo test -p runtime-core rollback_keeps_aliases_and_restores_legacy_recall -- --nocapture`
2. 当前结论：
   - object history 已可按版本链返回，当前版本位于最新位置；
   - diff 已可输出 summary/content 的结构化差异；
   - rollback 采用“恢复为新 current version”策略，不直接覆盖旧版本；
   - rollback 后旧 `long_term_memory` 主路径也能返回恢复后的版本，不出现 recall 混淆。
3. 当前追加复核：
   - `search_memory_entries()` 已能纳入 current memory object 候选；
   - query 命中 URI 或 alias 时，可命中 current object 视角条目；
   - current memory object 已具备独立 dedupe key，与旧 `MemoryEntry` 不因相同摘要被提前折叠；
   - 同分场景下 current memory object 已可稳定前置；
   - `recall_memory_digest()` 已在 system views 后单独纳入 current object 摘要块；
   - 当前仍保持旧 `MemoryEntry` recall 主链兼容，不做激进切主。

### 4. 当前已执行的兼容性复核（HMO-11）

1. Rust 主链已执行：
   - `cargo test -p runtime-core memory_recall --lib -- --nocapture`
   - `cargo test -p runtime-core context_builder --lib -- --nocapture`
   - `cargo test -p runtime-core dedupe_keeps_first_entry_per_memory_key --lib -- --nocapture`
   - `cargo test -p runtime-core score_prefers_higher_priority_for_same_query --lib -- --nocapture`
   - `cargo test -p runtime-core append_blocks_low_value_runtime_fallback_memory --lib -- --nocapture`
2. H-05 Go 侧证据入口已尝试执行：
   - `go test -run TestGenerateH05MemoryRoutingEvidence ./internal/api`
   - `go test -run TestGenerateLearningMemoryRoutingEvidence ./internal/api`
3. 当前阻塞：
   - 无。
4. 当前结论：
   - runtime-core 的 recall/context 最小兼容性通过；
   - `search_memory_entries()` 相关去重、优先级排序与低价值 runtime fallback 拦截能力当前可通过抽样回归；
   - current memory object 的最小 recall 接线未破坏现有摘要主链；
   - H-05 的 Go 侧证据入口已可重跑；
   - 当前未见 memory schema/recall 行为回归，因此 HMO-11 当前可给出 `passed`。

### 4.1 当前已执行的 C1 复核（context_builder object-aware digest）

1. 已执行定向单测：
   - `cargo test -p runtime-core selected_memory_digest_keeps_object_aware_marker -- --nocapture`
   - `cargo test -p runtime-core context_builder --lib -- --nocapture`
2. 当前结论：
   - `context_builder.rs` 当前在消费 `recall_memory_digest()` 时，已保留 `system views + current memory object` 的分层提示；
   - 当前动态块中的 `memory_digest` 不会把 current memory object 再次压平成普通长期记忆；
   - 当前仍保持摘要注入，未扩大为 object 正文注入，也未切换 recall 主链。

### 4.2 当前已执行的 C2 复核（prompt / metadata / event snapshot 透传）

1. 已执行定向单测：
   - `cargo test -p runtime-core render_prompt_surfaces_memory_layer_block -- --nocapture`
   - `cargo test -p runtime-core append_context_metadata_keeps_memory_layer_flags -- --nocapture`
   - `cargo test -p runtime-core context_snapshot_keeps_memory_layer_fields -- --nocapture`
   - `cargo test -p runtime-core prompt_snapshot_preserves_memory_layer_prompt -- --nocapture`
2. 当前结论：
   - `prompt.rs` 当前已显式输出 `记忆分层`，不再只依赖 `memory_digest` 文案本身；
   - `run_context_metadata.rs` 当前可把 object-aware 标记写入运行 metadata；
   - `events.rs` 当前可从 metadata 恢复 object-aware 标记，并保留到 `context_snapshot` / `prompt_snapshot`；
   - 当前仍保持最小结构化透传，未引入新的 recall 切换策略，也未扩大上下文预算。

### 4.3 当前已执行的 C3 复核（memory_recalled 事件治理文案）

1. 已执行定向单测：
   - `cargo test -p runtime-core memory_recall_event_uses_object_aware_summary_and_reason -- --nocapture`
   - `cargo test -p runtime-core memory_recall_event_keeps_empty_recall_semantics -- --nocapture`
   - `cargo test -p runtime-core events --lib -- --nocapture`
2. 当前结论：
   - `memory_recalled` 事件当前已能区分空召回、system-view recall 与 object-aware recall；
   - recall 事件 metadata 当前已包含 `memory_kind` 与 `memory_layer_summary`；
   - recall 事件的 `reason / governance_reason` 当前会显式说明本次注入的是哪一层记忆摘要；
   - 当前仍保持治理文案与 metadata 收紧，未切换 recall 主链，也未增加正文注入。

### 4.4 当前已执行的 C4 复核（verify / tool trace / run result 引用文案）

1. 已执行定向单测：
   - `cargo test -p runtime-core memory_recall_result_mentions_object_aware_layers -- --nocapture`
   - `cargo test -p runtime-core memory_recall_without_object_hits_mentions_system_view_layer -- --nocapture`
   - `cargo test -p runtime-core verification_summary_keeps_object_aware_recall_reasoning -- --nocapture`
   - `cargo test -p runtime-core artifact_content_keeps_object_aware_recall_reasoning -- --nocapture`
   - `cargo test -p runtime-core build_tool_result_keeps_object_aware_recall_reasoning -- --nocapture`
   - `cargo test -p runtime-core verify --lib -- --nocapture`
   - `cargo test -p runtime-core tool_trace --lib -- --nocapture`
   - `cargo test -p runtime-core executors::memory --lib -- --nocapture`
2. 当前结论：
   - `executors/memory.rs` 当前已在 recall 执行结果中显式保留真实召回层说明；
   - `verify.rs` 当前可在验证摘要与 evidence 中继续保留 object-aware recall 引用文案；
   - `tool_trace.rs` 当前可在 trace artifact 与 `ToolCallResult` 中继续保留 object-aware recall 引用文案；
   - 当前仍保持引用文案收紧，未新增 recall 选路、状态机分支或更大范围的 schema 扩展。

### 4.5 当前已执行的 C5 复核（最终收口事件可见性）

1. 已执行定向单测：
   - `cargo test -p runtime-core run_finished_summary_surfaces_recall_layer -- --nocapture`
   - `cargo test -p runtime-core run_failed_event_surfaces_recall_layer -- --nocapture`
   - `cargo test -p runtime-core run_finish_events --lib -- --nocapture`
   - `cargo test -p runtime-core events --lib -- --nocapture`
2. 当前结论：
   - `run_finish_events.rs` 当前已能在 memory recall 场景下为 `run_finished / run_failed` 显式补 recall 层摘要；
   - 最终收口事件 metadata 当前可携带 `recall_layer_summary`；
   - 当前查看最终收口事件时，已可直接判断是否发生 object-aware recall，而不必回翻中间 recall 事件；
   - 当前仍保持最终收口可见性增强，未扩新的结果 schema。

### 4.6 当前已执行的 C6 复核（recall layer helper 收敛）

1. 已执行定向单测：
   - `cargo test -p runtime-core digest_layer_helpers_keep_object_aware_labels -- --nocapture`
   - `cargo test -p runtime-core reasoning_layer_summary_extracts_recall_layer -- --nocapture`
   - `cargo test -p runtime-core memory_layer --lib -- --nocapture`
   - `cargo test -p runtime-core events --lib -- --nocapture`
   - `cargo test -p runtime-core executors::memory --lib -- --nocapture`
   - `cargo test -p runtime-core run_finish_events --lib -- --nocapture`
2. 当前结论：
   - `memory_layer.rs` 当前已统一 digest / metadata / reasoning 三种 recall layer helper；
   - `events.rs`、`executors/memory.rs`、`run_finish_events.rs` 当前已改用统一 helper；
   - 当前行为未变化，主要收益是减少 recall layer 文案漂移与解析分叉。

### 5. 实现后验证（当前已补齐）

1. system views：
   - 验证 `boot/recent/index/rules/workspace` 是否都能生成稳定摘要；
   - 验证输出是否保持预算受控，不发生全量记忆注入。
2. object/version：
   - 验证新增对象、追加版本、切换 current version 是否可复现；
   - 验证 alias/uri 能稳定命中目标对象。
3. review / rollback：
   - 已验证 history 查询、diff 产物、rollback 恢复；
   - 已验证 rollback 后 recall 不出现旧新版本混淆。
4. 兼容性：
   - 验证旧 `MemoryEntry` recall 主链在兼容阶段仍可工作；
   - 验证 H-05 现有记忆路由证据不被破坏。

### 6. 分批验证顺序（已执行）

1. 第一批（system views）：
   - 针对 `memory_views.rs` 做单元测试；
   - 针对 `memory_recall.rs` 与 `context_builder.rs` 做摘要预算回归；
   - 证据输出到 `system-views.json`。
   - 若编译被 object/version 相关未收口代码阻塞，先解除阻塞，再继续第一批回归。
2. 第二批（object/version）：
   - 针对 `sqlite_store.rs` 新表做 migration 测试；
   - 针对 `memory_object_store.rs` 做 current version 切换测试；
   - 证据输出到 `migration-report.json`。
   - 当前已追加最小 recall 接线验证：
     - `cargo test -p runtime-core search_includes_current_memory_object_for_duplicate_entry -- --nocapture`
     - `cargo test -p runtime-core search_can_hit_current_memory_object_uri -- --nocapture`
     - `cargo test -p runtime-core dedupe_keeps_distinct_memory_object_identity -- --nocapture`
     - `cargo test -p runtime-core sort_prefers_current_memory_object_on_same_score -- --nocapture`
     - `cargo test -p runtime-core recall_digest_surfaces_current_memory_object_block -- --nocapture`
     - `cargo test -p runtime-core selected_memory_digest_keeps_object_aware_marker -- --nocapture`
     - `cargo test -p runtime-core render_prompt_surfaces_memory_layer_block -- --nocapture`
     - `cargo test -p runtime-core append_context_metadata_keeps_memory_layer_flags -- --nocapture`
     - `cargo test -p runtime-core context_snapshot_keeps_memory_layer_fields -- --nocapture`
     - `cargo test -p runtime-core prompt_snapshot_preserves_memory_layer_prompt -- --nocapture`
     - `cargo test -p runtime-core memory_recall_event_uses_object_aware_summary_and_reason -- --nocapture`
     - `cargo test -p runtime-core memory_recall_event_keeps_empty_recall_semantics -- --nocapture`
     - `cargo test -p runtime-core memory_recall_result_mentions_object_aware_layers -- --nocapture`
     - `cargo test -p runtime-core memory_recall_without_object_hits_mentions_system_view_layer -- --nocapture`
     - `cargo test -p runtime-core verification_summary_keeps_object_aware_recall_reasoning -- --nocapture`
     - `cargo test -p runtime-core artifact_content_keeps_object_aware_recall_reasoning -- --nocapture`
     - `cargo test -p runtime-core build_tool_result_keeps_object_aware_recall_reasoning -- --nocapture`
     - `cargo test -p runtime-core run_finished_summary_surfaces_recall_layer -- --nocapture`
     - `cargo test -p runtime-core run_failed_event_surfaces_recall_layer -- --nocapture`
     - `cargo test -p runtime-core digest_layer_helpers_keep_object_aware_labels -- --nocapture`
     - `cargo test -p runtime-core reasoning_layer_summary_extracts_recall_layer -- --nocapture`
3. 第三批（rollback）：
   - 已针对 history/diff/rollback 做演练；
   - 已验证 rollback 后 recall 不出现旧新版本混淆；
   - 证据输出到 `rollback-drill.json`。

## 证据位置

1. 当前文档证据：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-object-review-20260423/proposal.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-object-review-20260423/design.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-object-review-20260423/tasks.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-object-review-20260423/status.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-object-review-20260423/verify.md`
2. 预留实现证据目录：
   - `D:/newwork/本地智能体/tmp/stage-h-memory-object-review/`
3. 建议证据文件：
   - `D:/newwork/本地智能体/tmp/stage-h-memory-object-review/system-views.json`
   - `D:/newwork/本地智能体/tmp/stage-h-memory-object-review/migration-report.json`
   - `D:/newwork/本地智能体/tmp/stage-h-memory-object-review/compatibility-report.json`
   - `D:/newwork/本地智能体/tmp/stage-h-memory-object-review/rollback-drill.json`

## Gate 映射

1. 当前映射：
   - 作为阶段 H 的并行 change，不改写 `Gate-H` 当前裁决。
2. 后续若继续推进实现，建议独立产出：
   - system views 证据；
   - object/version 迁移证据；
   - compatibility report；
   - review/rollback drill 证据；
   - 与 H-05 兼容性复核证据。

## 当前结论

1. 当前已完成第一批、第二批、第三批与 HMO-11 兼容性复核，当前范围内可表述为“已完成验证收口”。
2. 当前最强结论是：
   - `HMO-06 / HMO-07` 已完成最小实现与回归；
   - `HMO-08 / HMO-09` 已完成最小实现与回归；
   - `HMO-10` 已完成最小 history/diff/rollback 与 rollback drill；
   - current memory object 已进入 recall 最小命中链路，并具备最小 object-first 排序与 object-aware digest；
   - `context_builder.rs` 已能保留 object-aware digest 的分层提示，不在动态块中重新抹平 current memory object；
   - `prompt.rs`、`run_context_metadata.rs`、`events.rs` 已能把 object-aware 标记继续透传到 prompt、metadata 与 snapshot；
   - `memory_recalled` 事件已能区分 object-aware recall，并给出更具体的标题、原因与层信息；
   - `executors/memory.rs`、`verify.rs`、`tool_trace.rs` 已能继续保留 object-aware recall 的引用文案；
   - `run_finished` / `run_failed` 当前已能显式带出 recall 层摘要，使最终收口也可直接识别 object-aware recall；
   - `memory_layer.rs` 已把 recall layer 文案与解析逻辑收敛成单一 helper，减少后续 drift；
   - `HMO-11` 当前为 `passed`：Rust 与 Go 侧兼容性入口均可通过；
   - 当前 change 已可提审，但仍未签收。
