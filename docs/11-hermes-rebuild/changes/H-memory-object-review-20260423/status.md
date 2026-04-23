# H-memory-object-review-20260423（status）

最近更新时间：2026-04-23  
状态：并行 change 可提审（已完成 HMO-06~HMO-10、HMO-11、HMO-12；待主控判断；未签收）  
状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`

## 当前状态

1. 已完成：
   - 已建立 change 工作区。
   - 已完成对当前 memory 子系统与 `nocturne_memory` 的对照分析。
   - 已冻结本 change 的最小目标：`system views + memory object/version + review/rollback`。
   - 已明确本 change 当前只作为并行 change，不改写 Gate-H 主推进口径。
   - 已补实现前任务分解，明确第一批建议只启动 `system views` 只读层，不直接进入 object/version migration。
   - 已完成第一批实现：
     - `memory_recall.rs` 已接入 `select_system_view_summaries()`；
     - `lib.rs` 已声明 `mod memory_views;`；
     - 工作区内已存在 `memory_views.rs`；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core builds_five_system_views -- --nocapture`
       - `cargo test -p runtime-core recall_digest_includes_system_view_lines -- --nocapture`
       - `cargo test -p runtime-core memory_recall --lib -- --nocapture`
       - `cargo test -p runtime-core context_builder --lib -- --nocapture`
   - 已完成第二批实现：
     - `lib.rs` 已声明 `mod memory_object_store;`；
     - 工作区内已存在 `memory_object_store.rs`；
     - `sqlite_store.rs` 已新增 `memory_objects`、`memory_object_versions`、`memory_object_aliases` 三张表；
     - `write_memory_entry_sqlite()` 已在保留旧 `long_term_memory` 主路径的同时并行同步 object/version；
     - `storage_migration.rs` 已支持首次 backfill memory objects；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core sync_versions_share_object_and_update_current -- --nocapture`
       - `cargo test -p runtime-core sync_creates_alias_for_object -- --nocapture`
   - 已完成第三批实现：
     - `memory_object_store.rs` 已补 `history / diff / rollback` 最小接口；
     - `sqlite_store.rs` 已支持 `restored_from_version_id`、目标版本读取与 rollback 生成新 current version；
     - rollback 已同步写回旧 `long_term_memory` 主路径，避免 recall 新旧版本混淆；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core history_lists_versions_newest_first -- --nocapture`
       - `cargo test -p runtime-core diff_reports_summary_and_content_changes -- --nocapture`
       - `cargo test -p runtime-core rollback_creates_restored_current_version -- --nocapture`
       - `cargo test -p runtime-core rollback_keeps_aliases_and_restores_legacy_recall -- --nocapture`
   - 已完成 recall 追加接线：
     - `memory.rs` 已接入 current memory object 候选；
     - `sqlite_store.rs` 已可导出 current object 视角的 recall 条目；
     - URI / alias 已进入 `search_memory_entries()` 的命中文本；
     - `memory_recall.rs` 已为 current memory object 标记独立命中理由；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core search_includes_current_memory_object_for_duplicate_entry -- --nocapture`
       - `cargo test -p runtime-core search_can_hit_current_memory_object_uri -- --nocapture`
   - 已完成 recall 排序收紧：
     - `memory.rs` 已为 current memory object 增加独立 dedupe key；
     - 同分场景下 current memory object 已可稳定前置；
     - current memory object 与旧 `MemoryEntry` 不再因摘要相同被提前折叠；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core dedupe_keeps_distinct_memory_object_identity -- --nocapture`
       - `cargo test -p runtime-core sort_prefers_current_memory_object_on_same_score -- --nocapture`
   - 已完成 object-aware digest：
     - `memory_recall.rs` 已在 system views 后单独输出 current memory object 摘要块；
     - `sqlite_store.rs` 已支持按 limit 拉取 current object digest 候选；
     - 当前 digest 已把 current object 作为一等摘要来源，不再只依赖混合搜索结果；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core recall_digest_surfaces_current_memory_object_block -- --nocapture`
   - 已完成 C1 最小收口：
     - `context_builder.rs` 已在消费 `recall_memory_digest()` 时保留 object-aware 分层提示；
     - 当前动态上下文中的 `memory_digest` 会显式标记 `system views + current memory object` 分层，而不是再次压平成普通长期记忆摘要；
     - 当前仍保持摘要注入，不扩成 object 正文注入；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core selected_memory_digest_keeps_object_aware_marker -- --nocapture`
       - `cargo test -p runtime-core context_builder --lib -- --nocapture`
   - 已完成 C2 最小收口：
     - `DynamicPromptBlock`、`RuntimeContextSnapshot` 与运行元数据已新增 object-aware 结构化标记；
     - `prompt.rs` 当前会显式输出 `记忆分层`，不再只依赖 `memory_digest` 自身文案；
     - `events.rs` 当前可从 metadata 恢复 object-aware 标记，并在 prompt snapshot/context snapshot 中保留；
     - 当前仍保持最小透传，不扩成新的 recall 主链切换；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core render_prompt_surfaces_memory_layer_block -- --nocapture`
       - `cargo test -p runtime-core append_context_metadata_keeps_memory_layer_flags -- --nocapture`
       - `cargo test -p runtime-core context_snapshot_keeps_memory_layer_fields -- --nocapture`
       - `cargo test -p runtime-core prompt_snapshot_preserves_memory_layer_prompt -- --nocapture`
   - 已完成 C3 最小收口：
     - `memory_recalled` 事件当前会区分空召回、system-view recall 与 object-aware recall；
     - `events.rs` 已为 recall 事件补 `memory_kind`、`memory_layer_summary` 与更具体的 `reason / governance_reason`；
     - 当前 recall 事件已能表达“为什么是 object-aware recall”，不再只给出泛化标题；
     - 当前仍保持治理文案与 metadata 收紧，不切 recall 主链；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core memory_recall_event_uses_object_aware_summary_and_reason -- --nocapture`
       - `cargo test -p runtime-core memory_recall_event_keeps_empty_recall_semantics -- --nocapture`
       - `cargo test -p runtime-core events --lib -- --nocapture`
   - 已完成 C4 最小收口：
     - `executors/memory.rs` 已在 recall 执行结果中显式写出本次召回层，不再只输出泛化 recall 结果；
     - `verify.rs` 当前可在验证摘要与 evidence 中保留 object-aware recall 引用文案；
     - `tool_trace.rs` 当前可在 artifact 内容与 `ToolCallResult` 中保留 object-aware recall 引用文案；
     - 当前仍保持引用文案收紧，不新增 recall 选路或状态机分支；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core memory_recall_result_mentions_object_aware_layers -- --nocapture`
       - `cargo test -p runtime-core memory_recall_without_object_hits_mentions_system_view_layer -- --nocapture`
       - `cargo test -p runtime-core verification_summary_keeps_object_aware_recall_reasoning -- --nocapture`
       - `cargo test -p runtime-core artifact_content_keeps_object_aware_recall_reasoning -- --nocapture`
       - `cargo test -p runtime-core build_tool_result_keeps_object_aware_recall_reasoning -- --nocapture`
       - `cargo test -p runtime-core verify --lib -- --nocapture`
       - `cargo test -p runtime-core tool_trace --lib -- --nocapture`
       - `cargo test -p runtime-core executors::memory --lib -- --nocapture`
   - 已完成 C5 最小收口：
     - `run_finish_events.rs` 已补最终收口事件对 recall 层的显式可见性；
     - `run_finished` / `run_failed` 在 memory recall 场景下，当前可直接带出 `记忆召回：<layer>`；
     - finish metadata 当前已可携带 `recall_layer_summary`，便于最终结果与失败收口直接识别 object-aware recall；
     - 当前仍保持最终收口可见性增强，不扩新的结果 schema；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core run_finished_summary_surfaces_recall_layer -- --nocapture`
       - `cargo test -p runtime-core run_failed_event_surfaces_recall_layer -- --nocapture`
       - `cargo test -p runtime-core run_finish_events --lib -- --nocapture`
       - `cargo test -p runtime-core events --lib -- --nocapture`
   - 已完成 C6 最小收口：
     - 新增 `memory_layer.rs`，统一 digest / metadata / reasoning 三种 recall layer 文案与提取 helper；
     - `events.rs`、`executors/memory.rs`、`run_finish_events.rs` 当前已切到统一 helper，不再各自维护一套拼接逻辑；
     - 当前行为保持不变，目标仅为减少后续文案漂移与解析分叉；
     - 已完成最小定向验证：
       - `cargo test -p runtime-core digest_layer_helpers_keep_object_aware_labels -- --nocapture`
       - `cargo test -p runtime-core reasoning_layer_summary_extracts_recall_layer -- --nocapture`
       - `cargo test -p runtime-core memory_layer --lib -- --nocapture`
       - `cargo test -p runtime-core events --lib -- --nocapture`
       - `cargo test -p runtime-core executors::memory --lib -- --nocapture`
       - `cargo test -p runtime-core run_finish_events --lib -- --nocapture`
   - 第一批与第二批证据已分别输出到：
     - `D:/newwork/本地智能体/tmp/stage-h-memory-object-review/system-views.json`
     - `D:/newwork/本地智能体/tmp/stage-h-memory-object-review/migration-report.json`
   - 第三批证据已输出到：
     - `D:/newwork/本地智能体/tmp/stage-h-memory-object-review/rollback-drill.json`
2. 进行中：
   - 无。
3. 阻塞点：
   - 当前主推进仍为 `H-gate-h-signoff-20260416`，本 change 不应抢占主推进。
   - 当前 `runtime-core` 工作区存在其他未收口改动，本 change 暂不适合直接整体提交。
   - 当前尚未签收；主推进也尚未切到本 change。
4. 下一步：
   - 当前提审文档与三批验证证据已齐备，可交回主控判断。
   - 当前可把本 change 表述为“已完成最小范围收口，可提审，未签收”。
   - 当前 recall 已引入 current memory object 最小命中、同分前置与 object-aware digest；若主控后续继续扩展，应再判断是否进入 object-first 主链或更完整 review UI。
   - 当前已补 C1~C6：context_builder、prompt、metadata、event snapshot、recall 事件治理文案、verify/tool trace 引用文案、最终收口事件以及 recall layer helper 收敛均已完成；若主控后续继续扩展，应再判断是否进入 object-first 主链或更完整 review UI。

## 当前工作区结论

1. 当前允许的最强表述：
   - 已完成 memory object 升级方案建档；
   - 已明确最小改动路径、模块落点与后置能力；
   - `HMO-06 / HMO-07` 已完成实现与最小验证；
   - `HMO-08 / HMO-09` 已完成实现与最小验证；
   - `HMO-10` 已完成实现与最小验证；
   - current memory object 已进入 recall 最小命中链路，并具备最小 object-first 排序与 object-aware digest；
   - `context_builder.rs` 已能保留 object-aware digest 的分层提示，不在动态块中重新抹平 current memory object；
   - `prompt.rs`、`run_context_metadata.rs`、`events.rs` 已能把 object-aware 标记继续透传到 prompt 与 snapshot，不在下游再次丢失；
   - `memory_recalled` 事件已能区分 object-aware recall，并给出更具体的标题、原因与层信息；
   - `executors/memory.rs`、`verify.rs`、`tool_trace.rs` 已能继续保留 object-aware recall 的引用文案，不在验证与 trace 环节重新抹平；
   - `run_finished` / `run_failed` 当前已能显式带出 recall 层摘要，使最终收口也可直接识别 object-aware recall；
   - `memory_layer.rs` 已把 recall layer 文案与解析逻辑收敛成单一 helper，当前相关模块不再各自维护一套拼接规则；
   - `HMO-11` 已完成并给出 passed 结论；
   - `HMO-12` 已完成提审收口文档；
   - 当前仍未签收。
2. 当前不允许的表述：
   - 不表示本 change 已签收；
   - 不表示当前活跃 change 已切换；
   - 不表示主推进已切到本 change；
   - 不表示已完成前端 review UI 或图结构升级。
