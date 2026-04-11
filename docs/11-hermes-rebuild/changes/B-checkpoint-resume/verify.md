# 验证记录

## 验证方式

- 单元测试：已补一部分，覆盖 retry checkpoint 查询与 retry request 重建；`runtime-core` 侧新增了确认恢复、失败重试、恢复写回保护、handoff 路径恢复、“从 checkpoint 事件快照恢复已选动作”、“从 verification_snapshot/artifact_path 恢复验证快照摘要”，以及“从最近执行事件恢复执行中间态摘要”测试。
- 集成测试：已完成 `retryable_failure` 与 `after_confirmation` 两条恢复闭环样本。
- 人工验证：已确认 retry 与 confirm 两条路径都不是单纯插入恢复事件，而是会继续进入统一主循环并产生后续执行事件；并新增“恢复边界已回填”断言。

## 证据位置

- 测试记录：
  - `gateway/internal/state/runtime_checkpoint_store_test.go`
  - `gateway/internal/api/chat_retry_test.go`
  - `crates/runtime-core/src/query_engine.rs`
  - `crates/runtime-core/src/checkpoint.rs`
  - `crates/runtime-core/src/run_resume.rs`
  - `crates/runtime-core/src/session.rs`
  - `cargo test -p runtime-core`
  - `go test ./internal/api ./internal/state`
- 日志或截图：
  - `scripts/run-stage-b-retry-acceptance.ps1`
  - `scripts/run-stage-b-confirmation-acceptance.ps1`
  - `tmp/stage-b-retry-acceptance/latest.json`
  - `tmp/stage-b-confirmation-acceptance/latest.json`
  - `tmp/stage-b-retry-acceptance/logs/runtime.log`
  - `tmp/stage-b-retry-acceptance/logs/gateway.log`
  - `tmp/stage-b-confirmation-acceptance/logs/runtime.log`
  - `tmp/stage-b-confirmation-acceptance/logs/gateway.log`
  - `tmp/stage-b-retry-acceptance/latest.json`（`retry_run.boundary_recovered=true`）
  - `tmp/stage-b-confirmation-acceptance/latest.json`（`after_confirmation.boundary_recovered=true`）

## 联调样本

- 时间：2026-04-10 23:17（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775834258001`
- 初始 run：`run-1775834266276-2`
- 初始 checkpoint：`run-1775834266276-2-1775834266891`
- retry checkpoint：`run-1775834266276-2-1775834269201`
- 关键事实：
  - 初始 run 日志包含 `checkpoint_written`。
  - retry run 与初始 run 保持同一个 `run_id`。
  - retry 日志包含 `checkpoint_resumed`。
  - retry 日志未出现 `checkpoint_resume_skipped`。
  - retry 恢复后继续出现 `analysis_ready`、`plan_ready`、`action_requested`、`action_completed`、`verification_completed`、`checkpoint_written` 等后续事件，说明恢复后回到了统一主循环，而不是只插入恢复提示事件。
  - retry 最终落到 `run_failed`，失败原因符合故意构造的命令失败样本，证明当前脚本已经拿到可重复的失败恢复闭环证据。

- 时间：2026-04-10 23:36（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775835358718`
- 初始 run：`run-1775835368799-2`
- 初始 checkpoint：`run-1775835368799-2-1775835369182`
- confirmation：`confirm-risk-run-1775835368799-2`
- 恢复后 checkpoint：`run-1775835368799-2-1775835371115`
- 关键事实：
  - 初始 run 事件链包含 `run_started -> analysis_ready -> plan_ready -> memory_recalled -> confirmation_required -> checkpoint_written`，其中 `confirmation_kind=high_risk_action`。
  - `POST /api/v1/chat/confirm` 使用同一个 `run_id` 审批通过后，恢复日志包含 `checkpoint_resumed`。
  - confirm 恢复日志未出现 `checkpoint_resume_skipped`。
  - confirm 恢复后继续出现 `analysis_ready`、`plan_ready`、`memory_recalled`、`action_requested`、`action_completed`、`verification_completed`、`memory_written`、`knowledge_write_skipped`、`checkpoint_written`、`run_finished` 等后续事件，说明审批通过后已重新接回统一主循环。
  - 恢复后的 `context_snapshot.session_summary` 已写入 `当前计划：从 checkpoint 恢复：confirmation_required -> PausedForConfirmation` 与 `当前阶段：confirmation_resume`，证明短期状态恢复口径已生效。
  - 当前稳定样本命令为 `cmd: Remove-Item AGENTS.md -WhatIf`，既能稳定触发 `high_risk_action`，又不会真的改动工作区文件。
  - confirm 路径最终落到 `run_finished`，工具输出为 `WhatIf` 预演结果，说明当前样本可稳定复现“先确认、再恢复、再继续执行”的闭环。

## Gate 映射

- 对应阶段 Gate：Gate-B
- 当前覆盖情况：
  - `50 轮任务无致命崩溃`：当前没有直接证据。
  - `中断恢复成功率 >= 95%`：当前已有 checkpoint 写入、retry 查询、retry request 重建、保留原 `run_id` 约束验证、短期状态恢复测试，以及 `retryable_failure`、`after_confirmation` 两条接口级闭环样本；仍缺批量成功率统计。
  - `关键事件链路完整可追溯`：当前已拿到两条接口级证据：
    `checkpoint_written -> checkpoint_resumed -> 后续执行事件 -> terminal event`
    两条样本均未出现 `checkpoint_resume_skipped`。

## 当前结论

- checkpoint 最小字段集合已通过单元测试固定：
  `checkpoint_id / run_id / session_id / trace_id / workspace_id / status / final_stage / resumable / resume_reason / resume_stage / event_count / request / response`
- `resume_matches` 当前已通过单元测试固定为：
  `run_id + session_id + workspace_id` 同时一致才允许恢复；scope 不一致时退化为 `checkpoint_resume_skipped`
- 恢复短期状态写回保护已通过单元测试固定：
  - `after_confirmation` 命中恢复后，规划写回不会覆盖 `confirmation_resume`、`awaiting_confirmation` 和恢复计划
  - `retryable_failure` 命中恢复后，规划写回不会清空失败摘要或把 `recovery` 刷回普通 planning 态
- 恢复输入已新增一层稳定接回：
  - `retryable_failure` 命中恢复后，会从 checkpoint 响应事件里回填上一轮失败运行的 `handoff_artifact_path`
  - 这让恢复主线不仅保留失败摘要，也保留最近一次执行产物引用，且在 planning 写回阶段不会被立即清空，便于后续继续往动作边界恢复收口
- 已选动作恢复已通过单元测试固定：
  - `tool_call_snapshot.arguments_json` 已进入运行时事件合同
  - `bootstrap_run` 命中 checkpoint 时，会优先从最近一条 `tool_call_snapshot` 反解 `PlannedAction`
  - 当前已证明恢复后可以沿用最近一次已选动作，而不是必然重新规划
- 恢复计划已新增一层失败提示接回：
  - `retryable_failure` 命中恢复后，会优先从最近一条 `run_failed` 事件里读取 `failure_recovery_hint`
  - 当前已证明恢复计划不仅知道“继续哪个动作”，也知道“建议如何继续”
- 恢复输入已新增验证快照摘要接回：
  - 命中恢复后，会优先从最近一条带 `verification_snapshot` 的事件回填 `verification_snapshot.summary`
  - 若事件里存在 `artifact_path` 或 `verification_snapshot.evidence` 中的 `artifact=...`，会同步回填到短期观察，保留产物引用
  - 当前已证明恢复上下文不再只依赖失败摘要，还可带回最近一次验证结论与产物路径
- 恢复计划已新增执行中间态摘要接回：
  - 命中恢复后，会优先从最近执行相关事件（`action_requested/action_completed/verification_completed/run_failed`）提取 `stage/event_type/next_step`
  - 恢复计划会追加“恢复边界：阶段=...，事件=...，下一步=...”摘要，减少恢复后阶段漂移
  - 对 `after_confirmation` 场景，若 checkpoint 里尚无执行事件，则回退提取 `confirmation_required` 边界，确保恢复计划仍有边界信息
  - 当前已证明恢复计划不再只有“恢复原因 + 目标阶段”，而是带回最近执行边界或确认边界
- 恢复事件已新增结构化边界字段：
  - `checkpoint_resumed.metadata.checkpoint_resume_boundary` 已写入结构化边界（`stage=...;event=...;next_step=...`）
  - `checkpoint_resume_skipped` 场景不写该字段，避免误判
  - 当前已可直接按事件元数据检索“恢复边界”，不再依赖 `context_snapshot.session_summary` 文本匹配
- 当前证据已经足以证明：
  - `after_confirmation` 与 `retryable_failure` 两条路径都能命中恢复
  - 恢复后会重新回到统一主循环
- 当前证据已经足以证明：
  - 运行时已具备“短期状态 + 最近已选动作 + 验证快照摘要 + 执行中间态摘要”四层恢复能力
- 当前证据还不能证明：
  - 运行时已经恢复到“工具执行中的精确中断边界”
  - 执行阶段中间态或验证前快照已经被完整复用
