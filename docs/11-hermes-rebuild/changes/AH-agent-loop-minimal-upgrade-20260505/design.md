# 技术方案

## 影响范围

- 涉及模块：
  - `crates/runtime-core/src/planner.rs`
  - `crates/runtime-core/src/query_engine.rs`
  - `crates/runtime-core/src/lib.rs`
  - `crates/runtime-core/src/run_state_builder.rs`
  - `crates/runtime-core/src/events.rs`
- 涉及文档或 contract：
  - `docs/11-hermes-rebuild/changes/AG-agent-loop-memory-knowledge-20260505/design.md`
  - Runtime 事件元数据与 handoff 行为口径

## 方案

### 范围冻结

- 本刀只做四件事：
  - 在 planner 层引入最小 `PlanEnvelope`
  - 让单次运行支持最多 2 到 4 步的小步循环
  - 在 `query_engine.rs` 增加最小 replan trigger
  - 在事件流中补齐 iteration 相关事件

### PlanEnvelope 最小结构

- 建议字段：
  - `goal`
  - `current_step`
  - `remaining_steps`
  - `stop_condition`
  - `max_iterations`
  - `iteration_index`
  - `needs_verification`
- `PlannedAction` 继续承担“当前动作”职责，但不再等同整轮计划。
- 第一刀不做复杂子任务树，只保留线性 step 视图。

### 小步回路

- `lib.rs` 中把当前单次 `execute_stage()` 收口扩展为受控循环：
  - 单轮最多执行 2 到 4 次动作
  - 每次迭代都保留 stage metadata
  - 达到预算上限后进入 handoff，而不是继续硬跑
- `run_state_builder.rs` 负责把 `iteration_index`、`max_iterations`、`stop_condition` 等元数据挂回运行态。

### Replan trigger

- `query_engine.rs` 至少补齐以下触发条件：
  - 工具执行成功，但结果信息不足以完成当前目标
  - verify 未通过，且允许继续补一步观察或修正
  - confirmation 恢复后，需要继续原计划
  - 读取类动作返回结果后，显示下一步应追加观察或执行
- 第一刀只定义“何时要求重规划”，不实现复杂策略学习。

### 事件与可观测性

- `events.rs` 新增建议事件：
  - `plan_iteration_started`
  - `plan_iteration_completed`
  - `replan_requested`
  - `iteration_budget_exhausted`
- 事件元数据至少应带：
  - `iteration_index`
  - `max_iterations`
  - `goal`
  - `current_step`
  - `reason`

## 状态流转或调用链变化

- 预期主链由：
  - `Analyze -> Plan -> Execute -> Observe -> Verify -> Finish`
- 收紧为：
  - `Analyze -> PlanEnvelope -> Execute/Observe -> Replan or Verify -> Next Iteration or Handoff -> Finish`
- 关键变化不是新增阶段，而是允许主链在受控预算内完成 2 到 4 次连续动作。

## 风险与回退

- 主要风险：
  - 单动作假设可能散落在 planner、query engine、事件消费方，改造后容易出现 stage metadata 不一致。
  - 如果 budget 与停止条件不清晰，可能导致“多跑一步”和“过早 handoff”两类回归。
  - 事件新增后，现有 UI 或日志消费方可能需要兼容未知事件类型。
- 回退方式：
  - 保留单动作 fallback 开关或兼容路径，确保多步回路异常时可退回原有单步收口。
  - 先以测试和事件证据冻结行为，再决定是否删除旧路径。
