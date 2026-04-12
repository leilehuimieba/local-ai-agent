# 验证记录

## 验证方式

- 文档验证：`design.md` 已冻结三层记忆 schema v1（字段表、类型口径、读写检索规则）。
- 文档验证：`design.md` 已补齐迁移策略草案（分步迁移、兼容策略、回退策略）。
- 源码映射验证：已按以下命令复核字段来源与存储表结构。  
  `rg -n "struct (ShortTermMemory|SessionMemory|MemoryEntry|KnowledgeRecord)|create table if not exists (long_term_memory|knowledge_base)" crates/runtime-core/src gateway/internal -S`
- 单元测试：`cargo test -p runtime-core` 通过（`70 passed; 0 failed`）。
- D-04 治理规则单测：`memory::tests::dedupe_keeps_first_entry_per_memory_key`、`memory::tests::score_prefers_higher_priority_for_same_query`、`memory::tests::append_blocks_low_value_runtime_fallback_memory` 全部通过。
- 落盘证据：`tmp/stage-d-writeback-governance/latest.txt` 可复核以上三条测试名与总通过数。
- D-05 技能治理单测：`skill_catalog::tests::loads_skill_when_version_and_scope_match`、`skill_catalog::tests::skips_skill_when_pin_version_mismatch`、`skill_catalog::tests::blocks_skill_when_entry_path_escapes_workspace` 全部通过。
- D-05 接入验证：`query_engine::bootstrap_run` 已接入 `load_skill_catalog`，运行态 `RuntimeEnvelope` 已携带技能目录快照。
- 落盘证据：`tmp/stage-d-skill-catalog/latest.txt` 可复核 D-05 三条单测名与总通过数。
- D-06 跨会话样本验收：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-d-day1-acceptance.ps1` 通过（`status=passed`）。
  - 样本判据：`cross_session=true`、`write_has_memory_written=true`、`recall_has_memory_recalled=true`、`recall_hit=true`、`token_hits=2`。
  - 复核口径：会话 A 与会话 B 不同，且会话 B 的 `memory_recalled` 已命中会话 A 写入 token。
- D-G1 批量验收准备：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-d-gate-batch.ps1 -Days 7 -RequireGateD` 通过（`status=passed`，`gate_d.ready=true`）。
  - 阈值映射：`min_continuity_days=7`、`min_pass_rate=0.9`、`min_recall_hit_rate=0.9`。
  - 批量结果：`days=7`、`pass_days=7`、`pass_rate=1`、`recall_hit_days=7`、`recall_hit_rate=1`。
- 迁移脚本验收：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-d-migrate-acceptance.ps1` 通过（`status=passed`）。
  - 迁移判据：`sqlite_memory_before=0 -> sqlite_memory_after=4`，`sqlite_knowledge_before=0 -> sqlite_knowledge_after=1`。
  - 平滑性判据：`memory_imported=true`、`knowledge_imported=true`、`legacy_duplicate_removed=true`、`memory_api_has_token=true`。
- 接口样本：通过隔离端口运行 `chat/run`，使用 `query=recall: 项目 文档 主入口`，命中 `memory_recalled` 事件并校验可解释字段。
  - 命中判据：`memory_recalled.detail` 包含 `来源=`、`类型=`。
  - 可解释判据：`metadata.reason`、`metadata.governance_reason` 非空，`metadata.includes_memory=true`。
- 索引验证：`changes/INDEX.md` 已把本 change 置为当前活跃第一位。
- 后续验证：阶段 E 继续补前端字段消费联调证据，并保持与 C/D 审计字段口径一致。

## 证据位置

- `docs/11-hermes-rebuild/changes/D-memory-skill-foundation/proposal.md`
- `docs/11-hermes-rebuild/changes/D-memory-skill-foundation/design.md`
- `docs/11-hermes-rebuild/changes/D-memory-skill-foundation/tasks.md`
- `docs/11-hermes-rebuild/changes/D-memory-skill-foundation/status.md`
- `docs/11-hermes-rebuild/changes/D-memory-skill-foundation/verify.md`
- `docs/11-hermes-rebuild/changes/INDEX.md`
- `scripts/run-stage-d-migrate-acceptance.ps1`
- `scripts/run-stage-d-day1-acceptance.ps1`
- `scripts/run-stage-d-gate-batch.ps1`
- `tmp/stage-d-migrate-acceptance/latest.json`
- `tmp/stage-d-migrate-acceptance/logs/runtime.stdout.log`
- `tmp/stage-d-migrate-acceptance/logs/runtime.stderr.log`
- `tmp/stage-d-migrate-acceptance/logs/gateway.stdout.log`
- `tmp/stage-d-migrate-acceptance/logs/gateway.stderr.log`
- `tmp/stage-d-migrate-acceptance/sandbox/data/storage/main.db`
- `tmp/stage-d-memory-recall-acceptance/latest.json`
- `tmp/stage-d-memory-recall-acceptance/logs/runtime.stdout.log`
- `tmp/stage-d-memory-recall-acceptance/logs/runtime.stderr.log`
- `tmp/stage-d-memory-recall-acceptance/logs/gateway.stdout.log`
- `tmp/stage-d-memory-recall-acceptance/logs/gateway.stderr.log`
- `tmp/stage-d-writeback-governance/latest.txt`
- `tmp/stage-d-skill-catalog/latest.txt`
- `tmp/stage-d-day1/latest.json`
- `tmp/stage-d-day1/logs/runtime.stdout.log`
- `tmp/stage-d-day1/logs/runtime.stderr.log`
- `tmp/stage-d-day1/logs/gateway.stdout.log`
- `tmp/stage-d-day1/logs/gateway.stderr.log`
- `tmp/stage-d-batch/latest.json`
- `tmp/stage-d-batch/logs/runtime.stdout.log`
- `tmp/stage-d-batch/logs/runtime.stderr.log`
- `tmp/stage-d-batch/logs/gateway.stdout.log`
- `tmp/stage-d-batch/logs/gateway.stderr.log`
- `crates/runtime-core/src/session.rs`
- `crates/runtime-core/src/memory.rs`
- `crates/runtime-core/src/skill_catalog.rs`
- `crates/runtime-core/src/knowledge_store.rs`
- `crates/runtime-core/src/sqlite_store.rs`
- `gateway/internal/memory/store.go`

## Gate 映射

- 对应阶段 Gate：Gate-D。
- 当前覆盖情况：阶段 D 已完成执行入口准备，`D-01`、`D-02`、`D-03`、`D-04`、`D-05`、`D-06`、`D-G1` 已完成；可切换阶段 E 推进前端与网关一致性任务。
