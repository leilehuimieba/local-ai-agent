# 阶段性提审包（AO-memory-writeback-governance-20260506）

更新时间：2026-05-06  
提审类型：阶段子项提审  
评审状态：已签收

## 1. 提审范围

本次提审仅覆盖记忆写回治理，不包含 recall 重排、知识检索增强、verify 矩阵扩面、Browser / MCP / UI 链路改动。

覆盖项：

1. `working_only / episodic_memory / semantic_or_procedural_memory` 的最小写回分层。
2. 写回准入、拒绝与重复跳过的最小决策口径。
3. `memory_write_layer / memory_write_decision / memory_write_reason / memory_duplicate_strategy` 的持久化落点。
4. `MemoryAuditTrail / MemoryWriteOutcome / run_memory_metadata` 的治理字段透出。
5. 记忆写回分层、拒绝样例、accepted 样例与全量 `runtime-core` 回归证据。

## 2. 前置依赖与口径

1. 当前状态裁决文件：`D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. 对应阶段计划：`D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
3. 对应 change 文档：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/proposal.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/design.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/tasks.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/status.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/verify.md`

## 3. 核心证据

### 3.1 聚合证据

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/verify.md`

### 3.2 子证据

1. `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`
2. `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`
3. `D:/newwork/本地智能体/crates/runtime-core/src/memory_schema.rs`
4. `D:/newwork/本地智能体/crates/runtime-core/src/session.rs`
5. `D:/newwork/本地智能体/crates/runtime-core/src/run_finish_events.rs`

### 3.3 构建/测试记录

1. `cargo test -p runtime-core memory_`
2. `cargo test -p runtime-core run_finish_events`
3. `cargo test -p runtime-core memory_write_`
4. `cargo test -p runtime-core context_snapshot_keeps_memory_layer_fields`
5. `cargo test -p runtime-core`

## 4. 指标判定

| 指标 | 阈值 | 实测 | 结论(PASS/WARN/FAIL) | 证据 |
|---|---|---|---|---|
| 三层写回分层已落地 | = true | true | PASS | `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/status.md` |
| 第一批治理字段已持久化 | = 4 个 | 4 个 | PASS | `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/status.md` |
| 事件治理字段已透出 | = true | true | PASS | `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/status.md` |
| `working_only + rejected` 留痕样例 | >= 1 | 1 | PASS | `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/verify.md` |
| accepted 长期层样例 | >= 1 | 2 | PASS | `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/verify.md` |
| `runtime-core` 全量回归 | = 通过 | 240 passed / 0 failed | PASS | `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/status.md` |

## 5. 评审结论

1. 本次提审结果：`status=passed`
2. 就绪度判定：`ao.ready=true`
3. 签收结论：`ao.signed_off=true`
4. 阻塞项统计：`p0=0, p1=0, warning=1`
5. 结论说明：
   - 当前已完成 AO 既定范围内的最小写回治理闭环，写回不再只是“按来源和 kind 写入”，而是具备了明确的分层、准入和拒绝解释。
   - `working_memory_outcome / write_failure_lesson_memory / write_preference_memory / write_long_term_memory` 已具备最小归层决策，`workspace_summary` 会停在 `working_only`，失败教训进入 `episodic_memory`，偏好进入 `semantic_or_procedural_memory`。
   - 第一批治理字段已经进入条目 schema、迁移与事件透出链路，并有定向测试与全量 `runtime-core` 回归作为支撑。
   - 当前 warning 仅在于：`preference_summary(...)` 仍有存量超长函数问题，但属于历史工程护栏项，不影响 AO 本刀的治理闭环成立。

## 6. 风险与回退

1. 风险：
   - 当前写回治理仍是最小决策模型，尚未引入更细粒度的复用价值打分或验证等级细分。
   - `knowledge_base` 仍保留在 AK 主线，不纳入 AO 的普通长期记忆三层治理。
2. 回退触发条件：
   - 新的写回分层导致长期记忆污染升高，或 accepted / rejected / duplicate 的语义出现明显误判。
   - 事件消费侧无法正确读取新增治理字段，导致审计链退化。
3. 回退动作：
   - 回退 `write_long_term_memory(...)` 的最小判层逻辑，恢复为保守的短期留痕路径。
   - 保留 schema 兼容字段，优先在事件消费侧降级读取旧字段。

## 7. 后续动作

1. 若 `passed`：
   - AO 当前已签收，进入待归档状态。
   - 若后续继续深化记忆治理，单独新开 change 处理复用价值评分、验证等级或更严格工程护栏，不回 AO 扩 scope。
2. 若 `warning`：
   - 责任人：`待定`
   - 追踪编号：`AO-preference-summary-guardrail`
   - 到期时间：`待下一主推进项确定后回填`
   - 补证动作：决定是否单开工程护栏 change，处理 `preference_summary(...)` 的存量超长函数。
3. 若 `failed`：
   - 回到 AO 的定向测试与事件透出链重新定位。
   - 暂停下游记忆治理扩展，先修复写回分层或去重语义退化。

## 8. Gate 映射

1. 对应 Gate：`Gate-I（自由迭代期，不新增阶段 Gate）`
2. 覆盖项：
   - 自由迭代期下的记忆写回治理最小闭环
   - 持久化字段、事件透出与回归证据
   - 提审前范围冻结与风险说明
3. 未覆盖项（如有）：
   - recall 重排与知识检索增强（原因：不在 AO 范围）
   - 更复杂的长期记忆评分与多智能体反思（原因：后续如需推进应拆新 change）

## 9. 签收记录（评审后回填）

1. 评审人：`当前对话裁决`
2. 评审时间：`2026-05-06T13:45:26+08:00`
3. 最终结论：`passed`
4. 签收备注：按 AO 既定范围签收；warning 仅保留为后续独立工程护栏候选，不阻塞归档准备。
