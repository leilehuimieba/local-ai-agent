# 技术方案

## 影响范围

- 涉及模块（后续实现阶段）：
  1. `crates/runtime-core/src/events.rs`（生命周期事件标准化）
  2. `crates/runtime-core/src/memory.rs`（长期记忆结构与治理）
  3. `crates/runtime-core/src/session.rs`（短期记忆注入与会话态落盘）
  4. `crates/runtime-core/src/knowledge.rs`（知识层检索补充）
  5. `crates/runtime-core/src/context_builder.rs`（分层注入与预算裁剪）
  6. `crates/runtime-core/src/sqlite_store.rs`（observation/pending 队列表）
  7. `gateway/internal/api/*`（search/timeline/get_observations 接口）
  8. `scripts/*`（验收、评测、回退演练脚本）
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`（状态裁决口径）
  2. `docs/11-hermes-rebuild/changes/INDEX.md`（change 导航）
  3. `docs/11-hermes-rebuild/changes/F-memory-progressive-disclosure-20260414/*`

## 方案

- 核心做法：
  1. 引入“生命周期采集 -> observation 持久化 -> 异步处理 -> 分层检索 -> 上下文注入”的完整主链。
  2. 采用本项目现有 Rust/Go 基础设施落地，不增加 Bun/Node 常驻 worker。
  3. 保持“SQLite 主事实源 + JSONL 审计旁路”策略，确保可回放与可回退。
  4. 检索默认关键词主路径，语义增强保持可选扩展位，不作为首轮门槛。
  5. 引入隐私治理与写入前清洗，确保敏感信息不进入持久层与日志链路。
- 状态流转或调用链变化（实施后）：
  1. `SessionStart/UserPromptSubmit/PostToolUse/Stop/SessionEnd` 均可产出统一 Observation。
  2. 主链写入失败不阻断运行，转入 pending 队列重试或降级记录。
  3. 检索采用 `search -> timeline -> get_observations` 三层披露模型，按预算注入。

## M0-M5 里程碑总览

### M0：计划冻结与合同对齐（2 天）

目标：冻结合同、指标、回退策略，避免实现期反复改口径。

交付：
  1. Observation Contract v1
  2. Retrieval Contract v1
  3. Context Injection Budget v1
  4. 风险与回退矩阵 v1

Gate-M0：
  1. 合同字段与 runtime/gateway 现有结构可映射。
  2. 每项能力都有 DoD 与回退动作。

### M1：采集与存储（5 天）

目标：打通“可采、可存、可查”最小闭环。

交付：
  1. 事件到 observation 的映射层
  2. observation schema v1（含 trace 与治理字段）
  3. SQLite 主写 + JSONL 审计旁路
  4. 去重键（content_hash + time_window）
  5. 写失败降级策略

Gate-M1：
  1. 生命周期目标事件采集覆盖率 >= 95%。
  2. observation 持久化成功率 >= 99%。
  3. 写入失败不导致 run fail。

### M2：异步处理与恢复（5 天）

目标：将 summarization/补写从主链解耦，提升稳定性。

交付：
  1. pending 队列与状态机（pending/processing/processed/failed）
  2. 重试策略（指数退避 + 次数上限）
  3. 重启恢复（处理未完成项）
  4. 健康与状态查询端点

Gate-M2：
  1. 故障注入后消息丢失率 = 0。
  2. 重复处理率 <= 1%。
  3. 重启恢复成功率 >= 95%。

### M3：检索与三层披露（5 天）

目标：实现 `search -> timeline -> get_observations`。

交付：
  1. `search` 轻量索引接口
  2. `timeline` 时序上下文接口
  3. `get_observations` 详情批量接口
  4. 统一排序融合（来源权重 + 新鲜度 + 关键词分）
  5. 固定评测脚本与结果报告

Gate-M3：
  1. 固定评测集 Top-5 命中率 >= 70%。
  2. `search` p95 < 150ms（本地环境）。
  3. `get_observations` p95 < 400ms（批量 20 条）。

### M4：上下文注入与预算治理（4 天）

目标：将检索结果可控注入 RuntimeContext。

交付：
  1. ContextBuilder 分层注入（header/summary/timeline/details）
  2. token 预算硬上限与裁剪策略
  3. 引用化输出（observation_id/artifact_ref）
  4. 全量注入 vs 分层注入对比报告

Gate-M4：
  1. 分层注入相比全量注入 token 节省 >= 50%。
  2. 关键任务回答质量不低于基线（人工评审>=90%等效）。
  3. 注入内容均可追溯到 observation_id。

### M5：隐私治理与提审收口（4 天）

目标：补齐安全、回退、证据闭环。

交付：
  1. 敏感字段脱敏与 private 片段排除
  2. 写入前治理规则（block/replace/tag）
  3. feature flag 一键回退
  4. 提审包（verify/review/risk rollback）

Gate-M5：
  1. 敏感样例泄漏数 = 0。
  2. 回退演练成功率 = 100%。
  3. 验收证据完整可审计。

## 合同冻结（实施前置）

### Observation Contract v1（冻结）

最小字段：
  1. `id`
  2. `workspace_id`
  3. `session_id`
  4. `run_id`
  5. `event_type`
  6. `tool_name`
  7. `summary`
  8. `artifact_ref`
  9. `privacy_level`
  10. `trace_id`
  11. `created_at`

规则：
  1. `summary` 不得直接存放原始大输出，必须引用 artifact。
  2. `privacy_level=private` 不写正文，仅保留最小审计信息。
  3. `event_type` 必须来自有限枚举，禁止自由文本。

### Retrieval Contract v1（冻结）

接口：
  1. `search(query, filters, limit)` -> index list
  2. `timeline(anchor_id|query, window)` -> ordered context slices
  3. `get_observations(ids[])` -> detailed entries

规则：
  1. `search` 返回轻量字段，不返回大文本正文。
  2. `get_observations` 支持批量，限制单次最大条数。
  3. 所有响应必须包含 `trace_id` 便于审计对齐。

### Context Injection Budget v1（冻结）

预算：
  1. 总预算：`B_total`
  2. summary 层：`<= 30%`
  3. timeline 层：`<= 40%`
  4. details 层：`<= 30%`

裁剪顺序：
  1. 先裁 details
  2. 再裁 timeline
  3. 最后裁 summary

## 风险与回退

- 主要风险：
  1. 事件采集过量导致存储膨胀。
  2. 检索打分不稳定导致低质量命中。
  3. 预算裁剪过激导致关键上下文丢失。
  4. 隐私规则漏检导致敏感信息入库。
- 回退方式：
  1. 通过 feature flag 关闭 memory-enhanced 注入，回退 D/E 既有主路径。
  2. 关闭异步处理，仅保留同步最小审计写入。
  3. 关闭 details 注入，仅保留 summary+timestep 索引。
  4. 对风险字段启用 hard-block，写入前直接拒绝并落审计。

## 并行执行边界（与 F-install-upgrade 协同）

1. 本 change 只允许修改以下范围：
   - `crates/runtime-core/src/*`（memory/observation/retrieval/context 注入相关）
   - `gateway/internal/api/*`（memory 三层检索接口）
   - `docs/11-hermes-rebuild/changes/F-memory-progressive-disclosure-20260414/*`
   - `tmp/stage-mem-*/*`（专项验收证据）
2. 本 change 不修改安装主线文件：
   - `scripts/install-local-agent.ps1`
   - `scripts/run-stage-f-install-acceptance.ps1`
   - `scripts/doctor.ps1`（除非主推进明确授权联动）
3. 锁文件策略：
   - `docs/11-hermes-rebuild/current-state.md`
   - `docs/11-hermes-rebuild/changes/INDEX.md`
   上述文件视为串行锁，任一时刻仅允许一条主线写入，避免状态裁决冲突。
4. 证据隔离：
   - 本 change 证据只写 `tmp/stage-mem-*`，不覆盖 `tmp/stage-f-install/*`。
5. 冲突处理：
   - 若 runtime 改动影响 install 验收脚本输入，先在本 change `status.md` 记录冲突并暂停实现，等待主推进确认。
