# 验证记录

## 验证方式

- 单元测试：已补一部分，覆盖 retry checkpoint 查询与 retry request 重建；`runtime-core` 侧新增了确认恢复、失败重试、恢复写回保护、handoff 路径恢复、“从 checkpoint 事件快照恢复已选动作”、“从 verification_snapshot/artifact_path 恢复验证快照摘要”，以及“从最近执行事件恢复执行中间态摘要”测试。
- 单元测试：已补一部分，覆盖 retry checkpoint 查询与 retry request 重建；`runtime-core` 侧新增了确认恢复、失败重试、恢复写回保护、handoff 路径恢复、“从 checkpoint 事件快照恢复已选动作”、“从 verification_snapshot/artifact_path 恢复验证快照摘要”、“从最近执行事件恢复执行中间态摘要”，以及“checkpoint_resumed 携带验证元数据”测试。
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
  - `tmp/stage-b-retry-acceptance/latest.json`（`retry_run.checkpoint_resume_boundary` 非空）
  - `tmp/stage-b-confirmation-acceptance/latest.json`（`after_confirmation.checkpoint_resume_boundary` 非空）
  - `tmp/stage-b-retry-acceptance/latest.json`（`retry_run.checkpoint_resume_verification_code` 非空）
  - `tmp/stage-b-retry-acceptance/latest.json`（`retry_run.checkpoint_resume_verification_summary` 非空）
  - `tmp/stage-b-retry-acceptance/latest.json`（`retry_run.checkpoint_resume_artifact_path` 非空）

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

- 时间：2026-04-11 11:19（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775877548511`
- 初始 run：`run-1775877564481-2`
- 初始 checkpoint：`run-1775877564481-2-1775877564943`
- 关键事实：
  - 确认验收脚本已收敛为“confirmation 恢复事件定向筛选”：先按 `checkpoint_resume_reason=confirmation_required` 与 `checkpoint_stage=PausedForConfirmation` 过滤，再优先匹配初始 `checkpoint_id`。
  - `tmp/stage-b-confirmation-acceptance/latest.json` 中 `after_confirmation.checkpoint_id_matched=true`，确认命中的是审批对应恢复事件，而不是后续失败重试恢复事件。
  - 同一份证据里 `after_confirmation.reason_matched=true`、`stage_matched=true`、`verification_empty=true`，与 confirmation 路径口径一致。

- 时间：2026-04-11 11:19（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775877548557`
- run：`run-1775877564473-2`
- 关键事实：
  - retry 路径回归通过且未受 confirmation 脚本筛选调整影响。
  - `tmp/stage-b-retry-acceptance/latest.json` 仍保持 `reason_matched=true`、`stage_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-11 21:50（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775915415863`
- run：`run-1775915431616-2`
- 关键事实：
  - 复跑时先暴露 `event_type_matched=false` 回归，定位为 acceptance 脚本 `checkpoint_resume_boundary` 事件提取正则转义错误（`\s` 被写成 `\\s`），导致 `checkpoint_resume_event_type` 为空。
  - 修复后，`tmp/stage-b-confirmation-acceptance/latest.json` 稳定为 `status=passed`，且 `after_confirmation.checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_empty=true`、`checkpoint_id_matched=true`。

- 时间：2026-04-11 21:50（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775915415818`
- run：`run-1775915431618-2`
- 关键事实：
  - 同轮复跑下，`tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`，且 `retry_run.checkpoint_resume_event_type=run_failed`、`event_type_matched=true`。
  - retry 原有结构化断言保持稳定：`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-11 22:00（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775915996421`
- run：`run-1775916011486-2`
- 关键事实：
  - confirmation acceptance 新增目标恢复事件唯一性断言并通过：`after_confirmation.target_resumed_unique=true`、`after_confirmation.target_resumed_count=1`。
  - 同轮字段保持一致：`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`checkpoint_id_matched=true`。

- 时间：2026-04-11 22:00（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775915996425`
- run：`run-1775916011488-2`
- 关键事实：
  - retry acceptance 新增目标恢复事件唯一性断言并通过：`retry_run.target_resumed_unique=true`、`retry_run.target_resumed_count=1`。
  - 同轮字段保持一致：`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-11 22:08（Asia/Shanghai）
- 单测：`cargo test -p runtime-core emits_retry_resume_metadata_for_acceptance_filters`
- 关键事实：
  - 新增 `runtime-core/checkpoint.rs` 最小单测 `emits_retry_resume_metadata_for_acceptance_filters` 并通过（`1 passed; 0 failed`）。
  - 单测直接验证 acceptance 对齐口径：按 `checkpoint_resume_reason=retryable_failure`、`checkpoint_stage=Execute`、`checkpoint_id` 过滤后 `checkpoint_resumed` 候选唯一（`len=1`）。
  - 同一单测断言 retry 场景边界值为 `checkpoint_resume_boundary=stage=Finish;event=run_failed`，与 acceptance 的 `checkpoint_resume_event_type=run_failed` 对齐。

- 时间：2026-04-11 22:08（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775916495578`
- run：`run-1775916510872-2`
- 关键事实：
  - 单测后回归 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`。

- 时间：2026-04-11 22:08（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775916497767`
- run：`run-1775916512242-2`
- 关键事实：
  - 单测后回归 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-11 22:13（Asia/Shanghai）
- 单测：`cargo test -p runtime-core metadata_for_acceptance_filters`
- 关键事实：
  - 新增 confirmation 侧最小单测 `emits_confirmation_resume_metadata_for_acceptance_filters` 并通过；同轮 `emits_retry_resume_metadata_for_acceptance_filters` 保持通过（`2 passed; 0 failed`）。
  - confirmation 单测直接验证 acceptance 对齐口径：按 `checkpoint_resume_reason=confirmation_required`、`checkpoint_stage=PausedForConfirmation`、`checkpoint_id` 过滤后 `checkpoint_resumed` 候选唯一（`len=1`）。
  - confirmation 单测断言边界值为 `checkpoint_resume_boundary=stage=PausedForConfirmation;event=confirmation_required;next_step=等待用户确认后再继续`。

- 时间：2026-04-11 22:13（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775916780151`
- run：`run-1775916794648-2`
- 关键事实：
  - 单测后回归 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`checkpoint_id_matched=true`。

- 时间：2026-04-11 22:13（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775916780151`
- run：`run-1775916794476-2`
- 关键事实：
  - 单测后回归 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-11 22:36（Asia/Shanghai）
- 构建与测试：
  - `cargo test -p runtime-core`：`64 passed; 0 failed`
  - `go test ./...`（gateway）：通过
  - `npm run build`（frontend）：通过
- 关键事实：
  - 工具动作快照参数 `arguments_json` 已在 runtime/gateway/frontend 三端合同对齐，字段名保持一致：`ToolCallSnapshot.arguments_json`。
  - `frontend/src/shared/contracts.ts` 已收敛为 barrel export，类型定义按 `base/settings/runtime/memory/chat` 拆分，构建通过且无类型回退告警。

- 时间：2026-04-11 22:36（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775918153874`
- run：`run-1775918170502-2`
- 关键事实：
  - 合同对齐后回归 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`checkpoint_id_matched=true`。

- 时间：2026-04-11 22:36（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775918153867`
- run：`run-1775918169753-2`
- 关键事实：
  - 合同对齐后回归 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-11 22:49（Asia/Shanghai）
- 构建与测试：
  - `cargo test -p runtime-core`：`64 passed; 0 failed`
- 关键事实：
  - 本轮持续执行恢复链路小刀拆分，覆盖 `run_resume_tests.rs`、`run_resume_event_testkit.rs`、`run_resume_testkit.rs`、`run_resume_plan.rs`、`run_resume_observation.rs`、`run_resume_hint.rs`、`run_resume_action_hint.rs`、`run_resume_verification.rs`、`run_resume_handoff.rs`、`run_resume_boundary.rs`。
  - 每次代码提交前均执行 `cargo test -p runtime-core`，未引入回归失败，恢复相关测试与既有 64 条单测均保持通过。

- 时间：2026-04-11 23:02（Asia/Shanghai）
- 构建与测试：
  - `cargo test -p runtime-core`：`64 passed; 0 failed`
- 关键事实：
  - 本轮继续执行恢复链路小刀拆分，新增覆盖 `run_resume_clear.rs`、`run_resume_state.rs`、`run_resume_artifact.rs`、`run_resume_testkit.rs`、`run_resume_event_testkit.rs`。
  - 以上改动均为无行为变更的职责下沉与重复路径收敛，且每次提交前均完成同口径回归，未出现新增失败用例。

- 时间：2026-04-11 23:06（Asia/Shanghai）
- 构建与测试：
  - `cargo test -p runtime-core`：`64 passed; 0 failed`
- 关键事实：
  - 本轮继续执行恢复链路与测试样本收敛，新增覆盖 `run_resume_event_testkit.rs`、`run_resume_testkit.rs`、`run_resume_boundary.rs`。
  - 中途一次编译失败已定位并消除（失败事件样本构造字段完整性问题），后续各次回归均恢复为 `64 passed` 且无新增失败。

- 时间：2026-04-11 23:10（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775920186891`
- run：`run-1775920201813-2`
- 关键事实：
  - 复跑 `scripts/run-stage-b-confirmation-acceptance.ps1` 后，`tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_empty=true`、`checkpoint_id_matched=true`、`boundary_recovered=true`。

- 时间：2026-04-11 23:10（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775920186874`
- run：`run-1775920201813-2`
- 关键事实：
  - 复跑 `scripts/run-stage-b-retry-acceptance.ps1` 后，`tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`verification_recovered=true`、`artifact_recovered=true`、`boundary_recovered=true`。

- 时间：2026-04-11 23:17（Asia/Shanghai）
- 构建与测试：
  - `cargo test -p runtime-core`：`64 passed; 0 failed`
- 关键事实：
  - 本轮继续执行恢复链路小刀拆分，新增覆盖 `run_resume_testkit.rs`、`run_resume_observation.rs`。
  - 上述改动均为无行为变更的职责下沉与样本字段收敛，提交前回归稳定，无新增失败测试。

- 时间：2026-04-11 23:20（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775920832612`
- run：`run-1775920847557-2`
- 关键事实：
  - 修复 retry 脚本结构化断言后回归 `scripts/run-stage-b-confirmation-acceptance.ps1`，`tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_empty=true`、`checkpoint_id_matched=true`。

- 时间：2026-04-11 23:20（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775920833871`
- run：`run-1775920848341-2`
- 关键事实：
  - `scripts/run-stage-b-retry-acceptance.ps1` 已补齐并启用 `reason_matched/stage_matched` 结构化断言；`tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段为：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-11 23:25（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775921128649`
- run：`run-1775921142763-2`
- 关键事实：
  - 在 retry 脚本补齐 `checkpoint_id_matched` 断言后复跑 `scripts/run-stage-b-confirmation-acceptance.ps1`，`tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_empty=true`。

- 时间：2026-04-11 23:25（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775921128650`
- run：`run-1775921143250-2`
- 关键事实：
  - `scripts/run-stage-b-retry-acceptance.ps1` 已补齐并启用 `checkpoint_id_matched` 结构化断言；`tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段为：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-11 23:33（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 5`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 批量统计报告为 `status=passed`，`rounds=5`。
  - 汇总字段：`confirm_pass_count=5`、`retry_pass_count=5`、`round_pass_count=5`。
  - 通过率：`confirm_pass_rate=1.0`、`retry_pass_rate=1.0`、`round_pass_rate=1.0`。

- 时间：2026-04-11 23:33（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775921573702`
- run：`run-1775921585736-2`
- 关键事实：
  - 批量脚本最后一轮 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_empty=true`。

- 时间：2026-04-11 23:33（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775921593784`
- run：`run-1775921605909-2`
- 关键事实：
  - 批量脚本最后一轮 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-12 00:09（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 50`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 批量统计报告为 `status=passed`，`rounds=50`。
  - 汇总字段：`confirm_pass_count=50`、`retry_pass_count=50`、`round_pass_count=50`。
  - 通过率：`confirm_pass_rate=1.0`、`retry_pass_rate=1.0`、`round_pass_rate=1.0`。

- 时间：2026-04-12 00:09（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775923757665`
- run：`run-1775923769740-2`
- 关键事实：
  - 50 轮批量最后一轮 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_empty=true`。

- 时间：2026-04-12 00:09（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775923777411`
- run：`run-1775923789124-2`
- 关键事实：
  - 50 轮批量最后一轮 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`target_resumed_unique=true`、`target_resumed_count=1`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-12 00:23（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 5`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 批量统计报告为 `status=passed`，`rounds=5`。
  - 通过率字段保持稳定：`confirm_pass_rate=1.0`、`retry_pass_rate=1.0`、`round_pass_rate=1.0`。
  - 新增边界覆盖率字段：`confirm_boundary_count=5`、`retry_boundary_count=5`、`confirm_boundary_rate=1.0`、`retry_boundary_rate=1.0`。

- 时间：2026-04-12 00:23（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775924574833`
- run：`run-1775924586481-2`
- 关键事实：
  - 新统计字段回归后 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`boundary_recovered=true`、`target_resumed_unique=true`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_empty=true`。

- 时间：2026-04-12 00:23（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775924594204`
- run：`run-1775924606023-2`
- 关键事实：
  - 新统计字段回归后 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`boundary_recovered=true`、`target_resumed_unique=true`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-12 00:29（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 5`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 批量统计报告为 `status=passed`，`rounds=5`。
  - 通过率字段保持稳定：`confirm_pass_rate=1.0`、`retry_pass_rate=1.0`、`round_pass_rate=1.0`。
  - 边界覆盖率字段保持稳定：`confirm_boundary_rate=1.0`、`retry_boundary_rate=1.0`。
  - 新增事件类型覆盖率字段：`confirm_event_type_count=5`、`retry_event_type_count=5`、`confirm_event_type_rate=1.0`、`retry_event_type_rate=1.0`。

- 时间：2026-04-12 00:29（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775924955615`
- run：`run-1775924968242-2`
- 关键事实：
  - 事件类型统计字段回归后 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`boundary_recovered=true`、`checkpoint_id_matched=true`。

- 时间：2026-04-12 00:29（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775924975963`
- run：`run-1775924987893-2`
- 关键事实：
  - 事件类型统计字段回归后 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`boundary_recovered=true`、`checkpoint_id_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-12 00:36（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 5`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 批量统计报告为 `status=passed`，`rounds=5`。
  - 通过率字段保持稳定：`confirm_pass_rate=1.0`、`retry_pass_rate=1.0`、`round_pass_rate=1.0`。
  - 边界与事件类型覆盖率保持稳定：`confirm_boundary_rate=1.0`、`retry_boundary_rate=1.0`、`confirm_event_type_rate=1.0`、`retry_event_type_rate=1.0`。
  - 新增 `checkpoint_id` 覆盖率字段：`confirm_checkpoint_id_count=5`、`retry_checkpoint_id_count=5`、`confirm_checkpoint_id_rate=1.0`、`retry_checkpoint_id_rate=1.0`。

- 时间：2026-04-12 00:36（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775925339255`
- run：`run-1775925351013-2`
- 关键事实：
  - `checkpoint_id` 覆盖率统计字段回归后 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_id_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`boundary_recovered=true`。

- 时间：2026-04-12 00:36（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775925358714`
- run：`run-1775925370525-2`
- 关键事实：
  - `checkpoint_id` 覆盖率统计字段回归后 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_id_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`boundary_recovered=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-12 00:42（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 5`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 批量统计报告为 `status=passed`，`rounds=5`。
  - 通过率字段保持稳定：`confirm_pass_rate=1.0`、`retry_pass_rate=1.0`、`round_pass_rate=1.0`。
  - `checkpoint_id` 覆盖率保持稳定：`confirm_checkpoint_id_rate=1.0`、`retry_checkpoint_id_rate=1.0`。
  - 新增恢复质量覆盖率字段：`confirm_verification_empty_count=5`、`retry_verification_recovered_count=5`、`retry_artifact_recovered_count=5`，对应覆盖率 `confirm_verification_empty_rate=1.0`、`retry_verification_recovered_rate=1.0`、`retry_artifact_recovered_rate=1.0`。

- 时间：2026-04-12 00:42（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775925740360`
- run：`run-1775925752571-2`
- 关键事实：
  - 恢复质量统计字段回归后 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`verification_empty=true`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`。

- 时间：2026-04-12 00:42（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775925760479`
- run：`run-1775925772491-2`
- 关键事实：
  - 恢复质量统计字段回归后 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`verification_recovered=true`、`artifact_recovered=true`、`checkpoint_id_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`。

- 时间：2026-04-12 00:50（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 5`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 批量统计报告为 `status=passed`，`rounds=5`。
  - 新增 `gate_b` 判定字段：`required_rounds=50`、`required_rate=0.95`、`rounds_ok=false`、`confirm_rate_ok=true`、`retry_rate_ok=true`、`round_rate_ok=true`、`ready=false`。
  - 在 5 轮样本下 `gate_b.ready=false` 与预期一致，避免把短样本误判为 Gate-B 达成。

- 时间：2026-04-12 00:50（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775926208710`
- run：`run-1775926220492-2`
- 关键事实：
  - Gate-B 判定字段回归后 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_id_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`verification_empty=true`。

- 时间：2026-04-12 00:50（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775926228429`
- run：`run-1775926240248-2`
- 关键事实：
  - Gate-B 判定字段回归后 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_id_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-12 01:34（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 50 -RequireGateB`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 命令返回成功（退出码 0），批量统计报告为 `status=passed`，`rounds=50`。
  - 汇总字段：`confirm_pass_count=50`、`retry_pass_count=50`、`round_pass_count=50`；通过率均为 `1.0`。
  - `gate_b` 字段全部达标：`required_rounds=50`、`required_rate=0.95`、`rounds_ok=true`、`confirm_rate_ok=true`、`retry_rate_ok=true`、`round_rate_ok=true`、`ready=true`。

- 时间：2026-04-12 01:34（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775928811131`
- run：`run-1775928823220-2`
- 关键事实：
  - Gate-B 强校验最后一轮 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_id_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`verification_empty=true`。

- 时间：2026-04-12 01:34（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775928830421`
- run：`run-1775928842809-2`
- 关键事实：
  - Gate-B 强校验最后一轮 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_id_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-12 01:39（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775929176179`
- run：`run-1775929189898-2`
- 关键事实：
  - 边界结构化精确断言回归后 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 新增字段：`checkpoint_resume_boundary_stage=PausedForConfirmation`、`boundary_stage_matched=true`、`checkpoint_resume_boundary_next_step=等待用户确认后再继续`、`boundary_next_step_matched=true`。
  - 既有关键字段保持稳定：`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`checkpoint_id_matched=true`、`verification_empty=true`。

- 时间：2026-04-12 01:39（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775929176166`
- run：`run-1775929189897-2`
- 关键事实：
  - 边界结构化精确断言回归后 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 新增字段：`checkpoint_resume_boundary_stage=Failed`、`boundary_stage_matched=true`。
  - 既有关键字段保持稳定：`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`reason_matched=true`、`stage_matched=true`、`checkpoint_id_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

- 时间：2026-04-12 01:48（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 5`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 批量统计报告为 `status=passed`，`rounds=5`。
  - 通过率字段保持稳定：`confirm_pass_rate=1.0`、`retry_pass_rate=1.0`、`round_pass_rate=1.0`。
  - 新增边界精确匹配覆盖率字段：`confirm_boundary_stage_count=5`、`confirm_boundary_next_step_count=5`、`retry_boundary_stage_count=5`，对应 `confirm_boundary_stage_rate=1.0`、`confirm_boundary_next_step_rate=1.0`、`retry_boundary_stage_rate=1.0`。

- 时间：2026-04-12 01:48（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775929654168`
- run：`run-1775929665949-2`
- 关键事实：
  - 边界精确匹配覆盖率统计回归后 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_resume_boundary_stage=PausedForConfirmation`、`boundary_stage_matched=true`、`checkpoint_resume_boundary_next_step=等待用户确认后再继续`、`boundary_next_step_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`。

- 时间：2026-04-12 01:48（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775929673246`
- run：`run-1775929685373-2`
- 关键事实：
  - 边界精确匹配覆盖率统计回归后 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_resume_boundary_stage=Failed`、`boundary_stage_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`。

- 时间：2026-04-12 02:24（Asia/Shanghai）
- 批量脚本：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-b-acceptance-batch.ps1 -Rounds 50 -RequireGateB`
- 报告：`tmp/stage-b-acceptance-batch/latest.json`
- 关键事实：
  - 命令返回成功（退出码 0），批量统计报告为 `status=passed`，`rounds=50`。
  - 汇总字段：`confirm_pass_count=50`、`retry_pass_count=50`、`round_pass_count=50`；通过率均为 `1.0`。
  - `gate_b` 字段全部达标：`rounds_ok=true`、`confirm_rate_ok=true`、`retry_rate_ok=true`、`round_rate_ok=true`、`ready=true`。
  - 边界精确匹配覆盖率字段：`confirm_boundary_stage_rate=1.0`、`confirm_boundary_next_step_rate=1.0`、`retry_boundary_stage_rate=1.0`。

- 时间：2026-04-12 02:24（Asia/Shanghai）
- 会话：`stage-b-confirmation-acceptance-1775931816841`
- run：`run-1775931828879-2`
- 关键事实：
  - 强校验最后一轮 confirmation 样本 `tmp/stage-b-confirmation-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_resume_boundary_stage=PausedForConfirmation`、`boundary_stage_matched=true`、`checkpoint_resume_boundary_next_step=等待用户确认后再继续`、`boundary_next_step_matched=true`、`checkpoint_resume_event_type=confirmation_required`、`event_type_matched=true`。

- 时间：2026-04-12 02:24（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775931836665`
- run：`run-1775931849069-2`
- 关键事实：
  - 强校验最后一轮 retry 样本 `tmp/stage-b-retry-acceptance/latest.json` 为 `status=passed`。
  - 关键字段保持稳定：`checkpoint_resume_boundary_stage=Failed`、`boundary_stage_matched=true`、`checkpoint_resume_event_type=run_failed`、`event_type_matched=true`、`verification_recovered=true`、`artifact_recovered=true`。

## Gate 映射

- 对应阶段 Gate：Gate-B
- 当前覆盖情况：
  - `50 轮任务无致命崩溃`：已有直接证据，`tmp/stage-b-acceptance-batch/latest.json` 显示 `rounds=50` 且 `round_pass_count=50`。
  - `中断恢复成功率 >= 95%`：已有直接证据，`tmp/stage-b-acceptance-batch/latest.json` 显示 `confirm_pass_rate=1.0`、`retry_pass_rate=1.0`、`round_pass_rate=1.0`。
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
  - 两条 acceptance 脚本已切到结构化断言：`boundary_recovered` 由 `checkpoint_resume_boundary` 是否为空判定
  - 本轮补齐“边界事件类型精确匹配”断言：`after_confirmation.checkpoint_resume_event_type=confirmation_required`、`retry_run.checkpoint_resume_event_type=run_failed`，两条样本均为 `event_type_matched=true`
  - 本轮补齐“目标 resumed 事件唯一性”断言：`after_confirmation.target_resumed_unique=true`、`retry_run.target_resumed_unique=true`，且两条样本 `target_resumed_count=1`
  - 本轮补齐“脚本断言与 runtime 单测口径一致性”：`checkpoint.rs` 新增最小单测，直接覆盖 `reason/stage/checkpoint_id` 过滤与 retry 边界值断言
  - 本轮补齐“confirmation 侧单测口径一致性”：`checkpoint.rs` 新增 confirmation 最小单测，直接覆盖 `reason/stage/checkpoint_id` 过滤与 confirmation 边界值断言
- 恢复事件已新增结构化验证字段：
  - `checkpoint_resumed.metadata.checkpoint_resume_verification_code` 记录恢复时可见验证状态
  - `checkpoint_resumed.metadata.checkpoint_resume_verification_summary` 记录恢复时可见验证摘要
  - `checkpoint_resumed.metadata.checkpoint_resume_artifact_path` 记录恢复时可见产物路径
  - retry acceptance 已切到结构化断言：`verification_recovered` 与 `artifact_recovered` 由上述字段是否为空判定
- 工具动作快照参数合同已三端对齐：
  - `runtime-core/contracts.rs`、`gateway/internal/contracts/contracts.go`、`frontend/src/shared/contracts/runtime.ts` 均包含 `ToolCallSnapshot.arguments_json`
  - `runtime-core/events.rs` 已按 `tool_arguments_json -> arguments_json` 映射，恢复动作反解继续可用
- 当前证据已经足以证明：
  - `after_confirmation` 与 `retryable_failure` 两条路径都能命中恢复并回到统一主循环。
  - 运行时已具备“短期状态 + 最近已选动作 + 验证快照摘要 + 执行中间态摘要”恢复能力。
  - 批量强校验口径下，Gate-B 相关指标可由结构化报告直接判定且已达标（`gate_b.ready=true`）。
- 任务口径说明：
  - 本 change 按 `tasks.md` 为收口基准，当前任务列表均已完成。
