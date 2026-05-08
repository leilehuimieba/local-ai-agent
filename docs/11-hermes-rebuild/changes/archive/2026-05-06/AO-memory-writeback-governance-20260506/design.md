# 技术方案

## 影响范围

- Runtime：
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_schema.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/session.rs`
  - 视情况涉及 `run_finish_events.rs` / `events.rs` 的 metadata 透出
- Gateway：
  - 当前阶段原则上不做主链改动，除非实现时发现学习记忆 API 缺少最小治理字段

## Harness 判断

- 读取侧已经有 `AJ` 最小路由，当前最薄弱的是“写回是否可控”。
- 因此本刀优先补“什么能进长期记忆、进哪一层、留下什么治理信号”，而不是继续扩 recall 或知识检索。

## 范围冻结

- 本刀只做四件事：
  - 最小长期记忆写回分层
  - 最小写回准入/拒绝规则
  - 治理 metadata 与事件透出
  - 最小测试与留痕

## 最小写回分层

建议先只收口 3 类：

1. `working_only`
   - 仅停留在当前 session / 当前 run，不进入长期层
2. `episodic_memory`
   - 记录一次任务的过程性经验、失败/恢复过程
3. `semantic_or_procedural_memory`
   - 记录可跨任务复用的规则、模式、工作流或原则

## 现有写回函数到三层的映射

当前实现主入口是 `crates/runtime-core/src/memory_router/mod.rs::evaluate_finish_memory_writes(...)`。
AO 这一刀先不重排入口顺序，只收紧“每条写回结果最终归哪层、能不能进入长期层”。

### 明确归层

1. `working_memory_outcome(...)`
   - 固定归层：`working_only`
   - 说明：只表示当前 run/session 的短期状态已落盘，不进入长期记忆治理主线

2. `write_failure_lesson_memory(...)`
   - 固定归层：`episodic_memory`
   - 说明：失败、恢复、重试、阻塞这类经验首先是“过程性教训”，不是通用知识规则

3. `write_preference_memory(...)`
   - 固定归层：`semantic_or_procedural_memory`
   - 说明：用户偏好、回答风格、流程偏好本质上属于可跨任务复用规则

4. `write_long_term_memory(...)`
   - 不再等同于统一的 `long_term_memory`
   - 需要先经过一个最小决策函数，再决定：
     - 留在 `working_only`
     - 进入 `episodic_memory`
     - 进入 `semantic_or_procedural_memory`

5. `write_knowledge_record(...)`
   - 继续归 `knowledge_base`
   - 说明：知识层保留在 `AK` 既有主线，AO 不把它并入“三层普通长期记忆”治理

### `write_long_term_memory(...)` 的最小判层规则

AO 第一版不引入复杂评分器，只基于现有 `kind / tool_name / verification` 做最小判层：

1. 命中 `workspace_summary`
   - 默认归 `working_only`
   - 默认原因：更像项目状态回显，跨任务复用价值低

2. 命中 `lesson_learned`、`task_outcome`
   - 归 `episodic_memory`
   - 默认原因：保留任务过程、失败恢复与结果教训

3. 命中 `preference`、`workflow_preference`、`project_rule`、`workflow_pattern`
   - 归 `semantic_or_procedural_memory`
   - 默认原因：具备跨任务复用的规则或工作流价值

4. 未通过验证的自动长期写回候选
   - 默认退回 `working_only`
   - 除非已经被 `write_failure_lesson_memory(...)` 单独吸收为失败教训

5. `project_answer` 这类运行结果
   - 即使当前 `report.outcome.passed = true`
   - 若产物仍主要是一次性项目状态摘要，也默认停在 `working_only`

## 写回决策最小模型

AO 第一刀建议把“是否写长期层”拆成两个连续决策，而不是直接 `append_memory_entry(...)`：

1. 先判 `memory_write_layer`
   - 候选值：`working_only / episodic_memory / semantic_or_procedural_memory`

2. 再判 `memory_write_decision`
   - 候选值最小集：
     - `accepted`
     - `rejected`
     - `duplicate_skipped`

3. `working_only`
   - 并不等同失败
   - 表示该结果可以保留短期留痕，但不应进入长期层

4. `accepted`
   - 仅用于真正进入 `episodic_memory` 或 `semantic_or_procedural_memory`

5. `duplicate_skipped`
   - 保留现有重复保护语义
   - 但需要显式说明是“命中重复而跳过”，不是普通拒绝

## 写回准入规则

至少同时满足下面条件才允许进入长期层：

1. 已通过验证，或失败但形成了明确可复用教训
2. 摘要不是简单项目状态回显
3. 具备跨任务复用价值
4. 内容长度与信息密度达到最小阈值

### 准入规则与三层映射的关系

1. `working_only`
   - 不走长期层准入
   - 只要求生成最小治理留痕

2. `episodic_memory`
   - 允许两类来源进入：
     - 已验证通过且能形成过程经验
     - 未完全成功，但已形成明确失败教训或恢复策略

3. `semantic_or_procedural_memory`
   - 必须同时满足：
     - 已验证或用户明确给出长期偏好
     - 可抽象成规则、流程、偏好或模式
     - 不是单次项目状态摘要

## 拒绝条件

下面情况应默认拒绝长期写回：

1. 当前只是短期上下文回显
2. 结果未经验证且没有明确恢复结论
3. 内容过短、过泛或高度重复
4. 仅描述一次偶发命令输出、无稳定复用价值

## 治理字段

建议最小补齐：

- `memory_write_layer`
- `memory_write_decision`
- `memory_write_reason`
- `memory_reuse_value`
- `memory_verification_gate`
- `memory_duplicate_strategy`

## 第一批字段落点

AO 先分两层落点：持久化字段 + 事件透出字段。

### 持久化第一批

第一批只强制落 4 个字段，避免一开始把 schema 扩得过宽：

1. `memory_write_layer`
2. `memory_write_decision`
3. `memory_write_reason`
4. `memory_duplicate_strategy`

建议持久化落点：

1. `crates/runtime-core/src/memory.rs::MemoryEntry`
   - 作为内存态与 SQLite / JSONL 写入前的标准承载

2. `crates/runtime-core/src/memory_schema.rs::StructuredMemoryEntry`
   - 作为 JSONL / 结构化记录的兼容 schema

说明：

- 这 4 个字段属于“条目自身的治理解释”，应跟随长期记忆条目一起落盘。
- `working_only` 若不进入长期存储，可不强制写入 `StructuredMemoryEntry`，但事件侧仍要能看到。

### 事件透出第一批

建议同步出现在：

1. `crates/runtime-core/src/memory_router/mod.rs::MemoryAuditTrail`
2. `crates/runtime-core/src/memory_router/mod.rs::MemoryWriteOutcome`

说明：

- `MemoryWriteOutcome` 负责给 `memory_written / memory_write_skipped` 一类事件提供可读结果。
- `MemoryAuditTrail` 负责把治理解释稳定透出给事件消费侧和后续验收。

### 第二批先保留为事件侧可选字段

下面两个字段先不强制持久化，可优先作为事件 metadata 或审计字段透出：

1. `memory_reuse_value`
2. `memory_verification_gate`

原因：

- 它们更偏“本次决策为何成立”的上下文，而不是长期条目最小主键语义。
- 等 AO 第一版实现稳定后，再判断是否需要进入持久化 schema。

## 重复策略最小口径

AO 先不重写现有 `same_memory(...)` 与 `has_memory_duplicate(...)` 主逻辑，只补可观测决策：

1. 命中重复时
   - `memory_write_decision = duplicate_skipped`

2. `memory_duplicate_strategy`
   - 第一版建议最小候选值：
     - `same_kind_title_summary`
     - `knowledge_record_duplicate`
     - `none`

3. 事件与状态文档中需要能区分：
   - 因低价值被拒绝
   - 因未验证被拒绝
   - 因重复被跳过

## 验证路径

- Rust 测试覆盖：
  - 写回分层判断
  - 准入/拒绝判断
  - 事件 metadata 写入
- 最小留痕：
  - 至少一条“通过验证写入长期层”的样例
  - 至少一条“被拒绝，仅停留 working”的样例
