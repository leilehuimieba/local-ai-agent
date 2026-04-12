# 技术方案

## 影响范围

- `crates/runtime-core/src/session.rs`
- `crates/runtime-core/src/execution.rs`
- `crates/runtime-core/src/events.rs`
- `crates/runtime-core/src/checkpoint.rs`
- `crates/runtime-core/src/query_engine.rs`
- `crates/runtime-core/src/contracts.rs`
- `docs/11-hermes-rebuild/stage-plans/`

## 方案

- 当前仓库已经具备 checkpoint 持久化和 resume 事件骨架：
  - `checkpoint.rs` 会把完整 `RunRequest` 与 `RuntimeRunResponse` 写入 SQLite。
  - `simulate_run_with_runtime_events` 会在响应中注入 `checkpoint_written` 和 `checkpoint_resumed` 事件。
  - `RunResult` 已暴露 `checkpoint_id` 与 `resumable` 字段。
- 当前缺口不在“有没有 checkpoint”，而在“resume 是否真正恢复执行状态”：
  - `checkpoint_resume_event` 目前只负责读取 checkpoint 并插入事件。
  - `bootstrap_run` 仍然回到统一主循环，但当前已经可以优先复用 checkpoint 响应事件里最近一次 `tool_call_snapshot` 对应的动作。
  - 运行时仍然没有恢复工具执行中的临时产物、验证前快照或更深的执行阶段边界。
- 本 change 的最小目标是先把恢复范围收紧，再把恢复主线补成立：
  - 第一类：`awaiting_confirmation` 后审批通过，恢复到统一主循环。
  - 第二类：`retryable_failure` 后重试，恢复到受控重试入口。
  - 本轮不做任意阶段的全量恢复，不做跨机器恢复，不做多 checkpoint 回放。

## 当前代码判断

- `checkpoint_resume_profile` 当前把可恢复场景收敛到两类：
  - `awaiting_confirmation -> PausedForConfirmation`
  - `retryable_failure -> Execute`
- `resume_matches` 目前要求 `run_id + session_id + workspace_id` 同时一致。
- `context_policy.rs` 已具备 `confirmation_resume`、`handoff_resume`、`recovery` 等上下文标签，说明上下文装配层已经为恢复做过准备。
- `session.rs` 会记录 `pending_confirmation`、`last_run_status`、`handoff_artifact_path`，但这些短期状态还没有和 checkpoint 恢复主线真正闭合。
- `events.rs` 已能把 `tool_call_snapshot` 序列化进事件流；补上 `arguments_json` 后，事件快照已经足够反解大多数本地动作。

## 收口方案

### 1. 先冻结恢复边界

- 恢复只覆盖 `after_confirmation` 和 `retryable_failure` 两条主路径。
- 恢复目标不是“还原整个运行时对象”，而是“恢复到统一主循环可继续执行的最小状态”。
- 恢复必须回到主循环，不做旁路执行。

### 2. 明确 checkpoint 最小字段集合

- 保留当前已落库的基础字段：
  - `checkpoint_id`
  - `run_id`
  - `session_id`
  - `trace_id`
  - `workspace_id`
  - `status`
  - `final_stage`
  - `resumable`
  - `resume_reason`
  - `resume_stage`
  - `event_count`
  - `request`
  - `response`
- 这些字段当前分别承担的最小职责已经明确：
  - `checkpoint_id`：恢复请求与日志证据的唯一锚点。
  - `run_id + session_id + workspace_id`：恢复 scope 锁，避免误命中其他运行或其他工作区。
  - `trace_id`：追溯单次请求链路，不参与 scope 判定。
  - `status + final_stage`：区分 `awaiting_confirmation`、`failed` 与 terminal 状态。
  - `resumable + resume_reason + resume_stage`：声明“能不能恢复、为什么恢复、恢复后先落到哪个统一阶段标签”。
  - `event_count`：用于 checkpoint 事件证据和调试，不承担恢复判定。
  - `request + response`：承载恢复时需要复用的原请求语义、确认信息和最近一次结果摘要。
- 本轮不新增大块冗余 payload，优先复用现有 `request/response` 与事件元数据。
- 如果恢复主线仍缺关键状态，再增量补字段，而不是先扩表。

### 3. 建立恢复入口

- 在 `bootstrap_run` 前后增加 checkpoint 恢复判定，优先决定：
  - 本次是否是恢复请求。
  - 属于哪种恢复策略。
  - 应该恢复到哪个统一阶段标签。
- 对于 `after_confirmation`：
  - 读取 checkpoint。
  - 校验确认决策与 checkpoint 的 `resume_reason` 匹配。
  - 清空短期 `pending_confirmation`，把 `current_plan/current_phase/last_run_status/recent_tool_result` 写回 session。
  - 如果 checkpoint 响应事件中存在最近一次 `tool_call_snapshot`，优先反解成 `PlannedAction` 并复用原工具规格。
  - 再走统一的 `prepare_run_state -> execute_stage -> verify` 主链，而不是旁路执行。
- 对于 `retryable_failure`：
  - 读取上次失败结果。
  - 把失败摘要写回 `open_issue`，并把 `current_phase` 标记为 `recovery`。
  - 如果 checkpoint 响应事件中存在最近一次 `tool_call_snapshot`，优先反解成 `PlannedAction` 并复用失败前最后一次已选动作。
  - 以受控重试方式回到统一执行入口。
  - 保留失败原因与重试依据，避免“像新任务一样重跑”。
- 当前联调结论已经明确：
  - `gateway` 在构建 retry request 时必须保留原 `run_id`。
  - 如果 retry 请求重建新的 `run_id`，`runtime-core/checkpoint.rs` 中的 `resume_matches` 会判定 scope 不一致，并退化为 `checkpoint_resume_skipped`。
  - 因此，在当前实现口径下，`retryable_failure` 的恢复应保持“原 run_id + 原 session_id + 原 workspace_id”。
- 当前恢复入口的模块级顺序已经收口为：
  - `gateway/internal/api/chat_retry_service.go` 与 `gateway/internal/api/chat_confirmation_service.go`
    负责构造恢复请求，并写入 `resume_from_checkpoint_id` 与 `resume_strategy`。
  - `crates/runtime-core/src/checkpoint.rs`
    负责读取 checkpoint、执行 `resume_matches` scope 校验、注入 `checkpoint_resumed/checkpoint_resume_skipped` 事件。
  - `crates/runtime-core/src/query_engine.rs`
    负责把 checkpoint 结果写回短期 session，并把恢复请求接回统一主循环。
  - `crates/runtime-core/src/session.rs` 与 `crates/runtime-core/src/context_policy.rs`
    负责消费恢复后的短期状态和阶段标签。
  - `scripts/run-stage-b-retry-acceptance.ps1` 与 `scripts/run-stage-b-confirm-acceptance.ps1`
    负责接口级闭环验收。

### 4. 事件与验证同步收口

- 恢复路径要补齐最小事件证据：
  - 读取了哪个 checkpoint。
  - 为什么允许恢复。
  - 恢复后回到了哪个阶段。
  - 恢复后是否复用了最近一次已选动作。
  - 后续执行是否成功完成。
- `verify` 不能只看“有恢复事件”，还要看“是否真的恢复了执行主线”。

## 当前收口结论

- `resume_matches` 当前继续绑定 `run_id + session_id + workspace_id`：
  - 这是当前 runtime 与 gateway 已联调通过的唯一稳定口径。
  - 两条接口级样本都证明了“保留原 run_id”可以命中恢复，而 scope 不一致会退化为 `checkpoint_resume_skipped`。
  - 在补齐审计语义之前，不放宽 `run_id` 约束。
- `retryable_failure` 本轮不新增“重试运行”持久字段：
  - 当前 `resume_strategy=retry_failure`、`checkpoint_resumed` 事件和后续 checkpoint 已足够支撑阶段 B 的恢复证据。
- 本轮新增的恢复载体不是新表字段，而是事件快照补强：
  - `tool_call_snapshot.arguments_json` 作为最小动作参数载体，跟随 `response.events` 一起落入 checkpoint。
  - 恢复时优先从最近一条 `tool_call_snapshot` 反解 `PlannedAction`，避免重新规划漂移。
- 审批通过后的恢复继续复用原请求，并只覆盖：
  - `confirmation_decision`
  - `resume_from_checkpoint_id`
  - `resume_strategy`
  - `trace_id/request_id`
  - 其余字段保持原 run 语义不变。

## 风险与回退

- 风险：如果继续只做“恢复事件提示”而不恢复主线，阶段 B 会产生“看起来可恢复、实际上不可恢复”的假象。
- 风险：`tool_call_snapshot` 当前只覆盖最近一次已选动作，还不能表达工具执行过程中的中间产物与部分完成状态。
- 风险：`run_id` 绑定过严会让恢复只能发生在极窄路径，影响 Gate-B 的实际通过率。
- 风险：恢复入口如果绕开主循环，会进一步放大状态分叉。
- 回退方式：如果恢复主线不稳定，保留当前 checkpoint 持久化能力与恢复事件能力，暂不放开“已支持恢复执行”的表述。
