# 验证记录

## 验证方式

- 单元测试：
  1. M1~M5 代码落地后按模块补齐（events/memory/context/sqlite_store/gateway）单测。
- 集成测试：
  1. 通过阶段脚本验证采集、存储、检索、注入、治理、回退全链路。
- 人工验证：
  1. 核对观察记录可追溯性、预算裁剪可解释性、隐私治理有效性。

## 验收矩阵（冻结）

| 维度 | 指标 | 阈值 | 证据文件 |
| --- | --- | --- | --- |
| 功能完整性 | 生命周期事件采集覆盖率 | >= 95% | `tmp/stage-mem-m1/events-mapping.json` |
| 功能完整性 | 三层检索接口可用率 | = 100% | `tmp/stage-mem-m3/*.json` |
| 稳定性 | 持久化成功率 | >= 99% | `tmp/stage-mem-m1/storage.json` |
| 稳定性 | 重启恢复成功率 | >= 95% | `tmp/stage-mem-m2/recovery.json` |
| 稳定性 | 消息丢失率 | = 0 | `tmp/stage-mem-m2/queue-flow.json` |
| 质量 | Top-5 命中率 | >= 70% | `tmp/stage-mem-eval/latest.json` |
| 质量 | 分层注入 token 节省率 | >= 50% | `tmp/stage-mem-m4/ab-test.json` |
| 安全 | 敏感样例泄漏数 | = 0 | `tmp/stage-mem-m5/privacy-redact.json` |
| 安全 | private 片段入库数 | = 0 | `tmp/stage-mem-m5/private-skip.json` |
| 回退 | feature flag 回退成功率 | = 100% | `tmp/stage-mem-m5/rollback.json` |

## Gate 映射

- Gate-M0：
  1. 合同冻结完成，任务拆解与回退矩阵完成。
- Gate-M1：
  1. 采集、存储、降级链路指标满足阈值。
- Gate-M2：
  1. 队列、重试、恢复链路指标满足阈值。
- Gate-M3：
  1. 检索与评测指标满足阈值。
- Gate-M4：
  1. 注入预算与质量对比指标满足阈值。
- Gate-M5：
  1. 隐私与回退指标满足阈值，提审材料完整。

## 证据位置

- 阶段产物目录：
  1. `tmp/stage-mem-m1/`
  2. `tmp/stage-mem-m2/`
  3. `tmp/stage-mem-m3/`
  4. `tmp/stage-mem-m4/`
  5. `tmp/stage-mem-m5/`
  6. `tmp/stage-mem-eval/`
- 文档证据：
  1. `docs/11-hermes-rebuild/changes/F-memory-progressive-disclosure-20260414/proposal.md`
  2. `docs/11-hermes-rebuild/changes/F-memory-progressive-disclosure-20260414/design.md`
  3. `docs/11-hermes-rebuild/changes/F-memory-progressive-disclosure-20260414/tasks.md`
  4. `docs/11-hermes-rebuild/changes/F-memory-progressive-disclosure-20260414/status.md`
  5. `docs/11-hermes-rebuild/changes/F-memory-progressive-disclosure-20260414/verify.md`

## 当前覆盖情况

1. 已完成 M0 规划冻结、M1 全量、M2 全量、M3 全量、M4 全量，M0~M5 全量完成。
2. 当前按并行执行策略推进，不切主推进项；锁文件仅做串行维护。

## M0 验收记录（已完成）

### MEM-01 验收（五件套冻结）

- 执行动作：
  1. 新建专项目录并补齐 `proposal/design/tasks/status/verify`。
  2. 校验五件套文件存在且可读。
- 验证结果：
  1. 五件套均已落盘且可正常访问。
  2. `changes/INDEX.md` 已增加并行规划入口。
- 结论：
  1. `MEM-01` 通过。

### MEM-02 验收（Observation Contract v1）

- 执行动作：
  1. 在 `design.md` 冻结 observation 最小字段集合与治理规则。
- 验证结果：
  1. 字段、必填、隐私处理与枚举约束均已定义。
- 结论：
  1. `MEM-02` 通过。

### MEM-03 验收（Retrieval Contract v1）

- 执行动作：
  1. 在 `design.md` 冻结 `search/timeline/get_observations` 三层检索合同。
- 验证结果：
  1. 三类接口入参、出参与返回粒度约束已定义。
  2. 批量拉取限制与 trace_id 审计要求已定义。
- 结论：
  1. `MEM-03` 通过。

### MEM-04 验收（Context Injection Budget v1）

- 执行动作：
  1. 在 `design.md` 冻结注入预算配比与裁剪顺序。
- 验证结果：
  1. `summary/timeline/details` 分层预算与超预算裁剪顺序已定义。
- 结论：
  1. `MEM-04` 通过。

### Gate-M0 结论

1. Gate-M0 通过（规划层）。
2. 当前保持并行规划状态，未切主推进，不进入 M1 代码实现声明。

### MEM-05 验收（生命周期事件映射）

- 执行动作：
  1. 新增 `crates/runtime-core/src/observation.rs`，实现生命周期事件到 observation 的有限枚举映射。
  2. 在 `crates/runtime-core/src/lib.rs` 暴露映射入口，供后续存储链路复用。
  3. 新增 `crates/runtime-core/examples/export_lifecycle_mapping.rs`，导出映射快照。
  4. 生成证据：`tmp/stage-mem-m1/events-mapping.json`。
- 验证命令：
  1. `cargo test -p runtime-core observation::tests`
  2. `cargo run -p runtime-core --example export_lifecycle_mapping > tmp/stage-mem-m1/events-mapping.json`
- 验证结果：
  1. 单测通过（2/2）。
  2. 证据文件覆盖目标事件 6/6，`coverage_percent=100.0`，`missing_targets=[]`。
- 结论：
  1. `MEM-05` 通过。

### MEM-06 验收（observation 持久化）

- 执行动作：
  1. 在 `crates/runtime-core/src/sqlite_store.rs` 新增 `runtime_observations` 表及写入入口。
  2. 在 `crates/runtime-core/src/observation.rs` 新增持久化流程：SQLite 主写 + JSONL 审计旁路。
  3. 在 `crates/runtime-core/src/paths.rs` 增加 observation 审计文件路径。
  4. 新增 `crates/runtime-core/examples/export_observation_storage.rs` 产出存储验收报告。
  5. 生成证据：`tmp/stage-mem-m1/storage.json`。
- 验证命令：
  1. `cargo test -p runtime-core observation::tests`
  2. `cargo run -p runtime-core --example export_observation_storage > tmp/stage-mem-m1/storage.json`
- 验证结果：
  1. 单测通过（2/2）。
  2. 存储报告显示：`mapped_event_count=6`、`sqlite_written_count=6`、`audit_written_count=6`、`errors=[]`。
- 结论：
  1. `MEM-06` 通过。

### MEM-07 验收（去重策略）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 增加去重键计算：`event_type + content_hash + time_window_bucket`。
  2. 在持久化流程中引入去重预处理，仅将去重后 observation 写入 SQLite 与 JSONL。
  3. 新增 `crates/runtime-core/examples/export_observation_dedupe.rs` 导出去重报告。
  4. 生成证据：`tmp/stage-mem-m1/dedupe.json`。
- 验证命令：
  1. `cargo test -p runtime-core observation::tests`
  2. `cargo run -p runtime-core --example export_observation_dedupe > tmp/stage-mem-m1/dedupe.json`
- 验证结果：
  1. 单测通过（3/3，包含去重用例）。
  2. 去重报告显示：`total_incoming=7`、`unique_count=6`、`dropped_count=1`。
- 结论：
  1. `MEM-07` 通过。

### MEM-08 验收（写入失败降级）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 增加强制失败开关（`force_observation_sqlite_fail` / `force_observation_audit_fail`）。
  2. 在持久化报告中新增 `fallback_applied` 字段，用于标记是否触发降级。
  3. 新增 `crates/runtime-core/examples/export_observation_fallback.rs`，模拟 SQLite 主写失败场景。
  4. 生成证据：`tmp/stage-mem-m1/fallback.json`。
- 验证命令：
  1. `cargo test -p runtime-core observation::tests`
  2. `cargo run -p runtime-core --example export_observation_fallback > tmp/stage-mem-m1/fallback.json`
- 验证结果：
  1. 单测通过（3/3）。
  2. 降级报告显示：`sqlite_written_count=0`、`audit_written_count=6`、`fallback_applied=true`、`errors=["sqlite:forced_failure:observation_write"]`。
- 结论：
  1. `MEM-08` 通过。

### MEM-09 验收（pending 队列表与状态流转）

- 执行动作：
  1. 在 `crates/runtime-core/src/sqlite_store.rs` 增加 `observation_pending_queue` 表与索引。
  2. 在 `crates/runtime-core/src/observation.rs` 增加队列流转实现：`pending -> processing -> processed/failed`。
  3. 新增 `crates/runtime-core/examples/export_observation_queue_flow.rs` 导出队列状态流转报告。
  4. 生成证据：`tmp/stage-mem-m2/queue-flow.json`。
- 验证命令：
  1. `cargo test -p runtime-core observation::tests`
  2. `cargo run -p runtime-core --example export_observation_queue_flow > tmp/stage-mem-m2/queue-flow.json`
- 验证结果：
  1. 单测通过（3/3）。
  2. 队列报告显示：`queued_count=6`、`processing_count=6`、`processed_count=5`、`failed_count=1`。
  3. 状态序列符合预期：`pending -> processing -> failed -> processed`。
- 结论：
  1. `MEM-09` 通过。

### MEM-10 验收（重试策略）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 增加 failed 队列重试流程与退避时间计算（指数退避）。
  2. 新增 `ObservationRetryReport`，记录重试次数、退避时间、成功率。
  3. 新增 `crates/runtime-core/examples/export_observation_retry.rs` 导出重试报告。
  4. 生成证据：`tmp/stage-mem-m2/retry.json`。
- 验证命令：
  1. `cargo test -p runtime-core observation::tests`
  2. `cargo run -p runtime-core --example export_observation_retry > tmp/stage-mem-m2/retry.json`
- 验证结果：
  1. 单测通过（3/3）。
  2. 重试报告显示：`initial_failed_count=1`、`retried_count=1`、`remaining_failed_count=0`。
- 结论：
  1. `MEM-10` 通过。

### MEM-11 验收（重启恢复）

- 执行动作：
  1. 复用队列状态流转逻辑，构建“首次运行 + 再次运行”恢复演练样例。
  2. 新增 `crates/runtime-core/examples/export_observation_recovery.rs` 导出恢复报告。
  3. 生成证据：`tmp/stage-mem-m2/recovery.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_recovery > tmp/stage-mem-m2/recovery.json`
- 验证结果：
  1. `first_run` 与 `recovery_run` 均无错误。
  2. `recovered=true`，恢复后 `failed_count=0`。
- 结论：
  1. `MEM-11` 通过。

### MEM-12 验收（健康与状态接口）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 增加 `observation_queue_health` 健康查询接口。
  2. 新增 `ObservationQueueHealthReport` 输出总量与各状态计数。
  3. 新增 `crates/runtime-core/examples/export_observation_health.rs` 导出健康报告。
  4. 生成证据：`tmp/stage-mem-m2/health.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_health > tmp/stage-mem-m2/health.json`
- 验证结果：
  1. 健康报告显示：`total_count=6`、`processed_count=6`、`failed_count=0`、`healthy=true`。
- 结论：
  1. `MEM-12` 通过。

### MEM-13 验收（search 接口）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 新增 `search_observations` 与轻量返回结构。
  2. 在 `crates/runtime-core/src/lib.rs` 导出 search 相关接口。
  3. 新增 `crates/runtime-core/examples/export_observation_search.rs` 导出检索报告。
  4. 生成证据：`tmp/stage-mem-m3/search.json`。
- 验证命令：
  1. `cargo test -p runtime-core observation::tests`
  2. `cargo run -p runtime-core --example export_observation_search > tmp/stage-mem-m3/search.json`
- 验证结果：
  1. 单测通过（7/7）。
  2. 报告显示返回字段为轻量索引：`observation_id/event_type/summary_preview`，不含大文本正文。
- 结论：
  1. `MEM-13` 通过。

### MEM-14 验收（timeline 接口）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 新增 `observation_timeline` 与时间窗查询逻辑。
  2. 新增 `crates/runtime-core/examples/export_observation_timeline.rs` 导出 timeline 报告。
  3. 生成证据：`tmp/stage-mem-m3/timeline.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_timeline > tmp/stage-mem-m3/timeline.json`
- 验证结果：
  1. 报告含 `anchor_source=query` 与有序时序片段。
  2. `is_anchor=true` 项存在，符合 anchor/query 语义。
- 结论：
  1. `MEM-14` 通过。

### MEM-15 验收（get_observations 接口）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 新增 `get_observations` 批量详情拉取。
  2. 新增 `crates/runtime-core/examples/export_get_observations.rs` 导出详情报告。
  3. 生成证据：`tmp/stage-mem-m3/get-observations.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_get_observations > tmp/stage-mem-m3/get-observations.json`
- 验证结果：
  1. 报告显示 `requested_count=3`、`returned_count=3`，批量拉取生效。
  2. 返回详情包含 `summary/tool_name/artifact_ref/trace_id`。
- 结论：
  1. `MEM-15` 通过。

### MEM-16 验收（检索排序融合）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 新增 `rank_observations`。
  2. 评分拆解包含 `source_weight + freshness_score + keyword_score`。
  3. 新增 `crates/runtime-core/examples/export_observation_rank.rs` 导出排序报告。
  4. 生成证据：`tmp/stage-mem-m3/rank.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_rank > tmp/stage-mem-m3/rank.json`
- 验证结果：
  1. 报告字段包含 `source_weight/freshness_score/keyword_score/total_score`。
  2. `total_score` 按融合权重排序输出。
- 结论：
  1. `MEM-16` 通过。

### MEM-17 验收（固定评测集回归）

- 执行动作：
  1. 新增 `crates/runtime-core/examples/export_observation_eval.rs` 构建固定评测集。
  2. 对 `analysis/plan/action/verification/finish` 五类查询执行 Top-5 命中评估。
  3. 生成证据：`tmp/stage-mem-eval/latest.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_eval > tmp/stage-mem-eval/latest.json`
- 验证结果：
  1. 报告显示 `top5_hit_rate=100.0`，超过阈值 `70.0`。
  2. `passed=true`。
- 结论：
  1. `MEM-17` 通过。

### MEM-18 验收（ContextBuilder 分层注入接入）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 新增 `build_layered_injection`。
  2. 在 `crates/runtime-core/src/context_builder.rs` 将分层注入结果接入 `DynamicPromptBlock`。
  3. 新增 `crates/runtime-core/examples/export_observation_injection.rs` 导出注入报告。
  4. 生成证据：`tmp/stage-mem-m4/injection.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_injection > tmp/stage-mem-m4/injection.json`
- 验证结果：
  1. 报告包含 `summary/timeline/details` 三层注入文本。
  2. 注入结构可直接用于 prompt 组装。
- 结论：
  1. `MEM-18` 通过。

### MEM-19 验收（预算裁剪实现）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 实现总预算拆分（30/40/30）与裁剪。
  2. 通过示例输出预算指标并复用为 `budget` 证据。
  3. 生成证据：`tmp/stage-mem-m4/budget.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_injection > tmp/stage-mem-m4/budget.json`
- 验证结果：
  1. 报告显示 `budget_total_chars=800`、`used_chars=780`，在预算内。
  2. 各层预算字段完整：`summary_budget_chars/timeline_budget_chars/details_budget_chars`。
- 结论：
  1. `MEM-19` 通过。

### MEM-20 验收（引用化输出）

- 执行动作：
  1. 在分层注入中追加 `references`（`observation_id`）。
  2. 详情层输出保留 `artifact_ref`。
  3. 新增 `crates/runtime-core/examples/export_observation_reference.rs` 导出引用报告。
  4. 生成证据：`tmp/stage-mem-m4/reference.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_reference > tmp/stage-mem-m4/reference.json`
- 验证结果：
  1. 报告包含 `references=["observation_id=..."]`。
  2. `details_section` 含 `artifact=artifacts/action_completed.json`。
- 结论：
  1. `MEM-20` 通过。

### MEM-21 验收（全量 vs 分层对比）

- 执行动作：
  1. 在 `crates/runtime-core/src/observation.rs` 新增 `compare_layered_vs_full`。
  2. 在示例中输出 `ab_test` 指标。
  3. 生成证据：`tmp/stage-mem-m4/ab-test.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_injection > tmp/stage-mem-m4/ab-test.json`
- 验证结果：
  1. 报告显示 `saved_percent=58.3778...`，超过阈值 50%。
  2. `quality_preserved=true`。
- 结论：
  1. `MEM-21` 通过。

### Gate-M3 结论

1. Gate-M3 通过：`MEM-13`~`MEM-17` 证据齐全且阈值达标。

### Gate-M4 结论

1. Gate-M4 通过：`MEM-18`~`MEM-21` 证据齐全，预算节省率达标（58.37%）。

### MEM-22 验收（敏感字段脱敏）

- 执行动作：
  1. 在 `crates/runtime-core/src/sensitive_data.rs` 新增脱敏函数与私有标记识别。
  2. 在 `crates/runtime-core/src/observation.rs` 将脱敏规则接入 observation 写入前治理。
  3. 新增 `crates/runtime-core/examples/export_observation_privacy_redact.rs` 导出脱敏报告。
  4. 生成证据：`tmp/stage-mem-m5/privacy-redact.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_privacy_redact > tmp/stage-mem-m5/privacy-redact.json`
- 验证结果：
  1. 报告显示 `redacted_count=3`。
  2. 样例摘要包含 `[REDACTED]`，敏感值未原样落盘。
- 结论：
  1. `MEM-22` 通过。

### MEM-23 验收（private 片段排除）

- 执行动作：
  1. 在 observation 治理中增加 private marker 排除逻辑。
  2. 新增 `crates/runtime-core/examples/export_observation_private_skip.rs` 导出排除报告。
  3. 生成证据：`tmp/stage-mem-m5/private-skip.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_private_skip > tmp/stage-mem-m5/private-skip.json`
- 验证结果：
  1. 报告显示 `private_marker_count=3`、`stored_count=3`、`incoming_count=6`。
  2. private 样例已进入 `skipped_ids`。
- 结论：
  1. `MEM-23` 通过。

### MEM-24 验收（feature flag 回退演练）

- 执行动作：
  1. 在 observation 侧增加 `memory_enhanced_enabled` 开关判定。
  2. 新增 `crates/runtime-core/examples/export_observation_rollback.rs` 同时演练 enabled/disabled 双态。
  3. 生成证据：`tmp/stage-mem-m5/rollback.json`。
- 验证命令：
  1. `cargo run -p runtime-core --example export_observation_rollback > tmp/stage-mem-m5/rollback.json`
- 验证结果：
  1. `enabled.fallback_to_legacy=false`；
  2. `disabled.fallback_to_legacy=true` 且 `feature_enabled=false`。
- 结论：
  1. `MEM-24` 通过。

### MEM-25 验收（提审包收口）

- 执行动作：
  1. 补齐 `review.md` 与 `risk-rollback-register.md`。
  2. 在本文件补齐 M3~M5 验收与 Gate 结论。
  3. 对齐任务清单、状态、验证三件套。
- 验证结果：
  1. 提审文档齐全：`verify.md`、`review.md`、`risk-rollback-register.md`。
  2. M0~M5 证据路径均可复现。
- 结论：
  1. `MEM-25` 通过。

### Gate-M5 结论

1. Gate-M5 通过：隐私治理、回退演练、提审文档收口均完成。

