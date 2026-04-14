# 验证记录

## 验证方式

- 单元测试：
  1. `cargo test -p runtime-core`（2026-04-14 复验：113/113 通过，含 `session::tests::keeps_compaction_boundary_hint_visible_in_session_prompt_summary`）。
  2. `go test ./...`（`gateway` 全部通过，2026-04-14 复验通过）。
- 集成测试：
  1. `npm run build`（前端 `tsc + vite build`，2026-04-14 复验通过）。
- 人工验证：
  1. 核对 `run_command` 执行链：`ActionExecution.detail_preview/raw_output -> ToolCallResult.detail_preview/raw_output_ref`。
  2. 核对 runtime 事件映射：`RunEvent.detail_preview/raw_output_ref` 从 metadata 透传，并提供 `result_summary/artifact_path` 回退。
  3. 核对 gateway 合同：`RunEvent/LogEntry` 新增字段并在事件总线中统一归一化。
  4. 核对前端合同：`frontend shared runtime contract` 新增可选字段，保证三端类型对齐。
  5. 核对网关产物读取：`GET /api/v1/artifacts/content?path=...` 仅允许 `repoRoot/data/artifacts` 范围，越界路径返回错误。
  6. 核对前端双轨展示：详情栏“命令原文输出”支持按 `raw_output_ref` 优先、`artifact_path` 回退读取完整输出。
  7. 核对时间线摘要：详情行优先展示 `detail_preview`，无预览时回退原有字段。
  8. 核对权限规则层拆分：`assess_risk` 改为 `workspace_guard -> mode_guard -> high_risk_guard` 分层判定，避免单点 if/else 漂移。
  9. 核对确认链回放字段：风险阻断与确认请求路径统一写入 `permission_decision/permission_rule_layer/permission_flow_step/confirmation_chain_step`。
  10. 核对确认防重：`approve` 路径改为 `Take` 认领，重复确认返回 `409 conflict`，避免并发双触发。
  11. 核对 ask 审计编排：`approve/reject/cancel` 三条路径统一产出确认审计字段，并对齐 `permission_* + confirmation_*` 元数据。
  12. 核对批准路径事件：新增 `confirmation_approved` 事件，明确“确认通过 -> 恢复执行”链路证据。
  13. 核对前端权限链展示：任务时间线与记录时间线统一接入 `permission_* + confirmation_*` 摘要，`ask_approved/ask_reject/ask_cancel` 不再显示“待确认”。
  14. 核对详情栏审计字段：`权限决策/权限流程/规则层/确认链步骤/决策来源` 显示统一中文词典，支持原始 metadata 回放定位。
  15. 核对日志筛选可检索性：历史页搜索可命中 `permission_decision/permission_flow_step/permission_rule_layer/confirmation_decision_source`。
  16. 核对会话压缩边界：会话摘要新增单消息聚合预算（900 字符）与最大轮次边界（最近 4 轮），预算命中后追加边界提示并省略更早轮次。
  17. 核对压缩单测：新增 compaction 用例覆盖“预算命中时仅保留最近轮次 + 边界提示”和“预算未命中时完整保留”。
  18. 核对单结果预算阈值：`run_command` 输出新增单结果预算（30,000 字符），超限时 `detail_preview` 明确“原文已外置”，避免主链路回灌大块输出。
  19. 核对预算命中回放：`verification_completed/run_finished/run_failed` metadata 可回放 `result_chars/single_result_budget_chars/single_result_budget_hit`。
  20. 核对预算单测：新增 `executors::command` 预算判定测试与 `run_verification_metadata` 预算字段写入测试。
  21. 核对会话摘要链路：`session_prompt_summary` 不再对 `compressed_summary` 做二次 `summarize_text` 截断，保证 `compaction` 的聚合预算与边界提示不被吞。
  22. 核对会话层新增测试：`session::tests::keeps_compaction_boundary_hint_visible_in_session_prompt_summary` 覆盖“边界提示可见”断言。

## 证据位置

- 测试记录：
  1. `cargo test -p runtime-core`（2026-04-14）
  2. `go test ./...`（`gateway/`，2026-04-14）
  3. `npm run build`（`frontend/`，2026-04-14）
- 日志或截图：
  1. `crates/runtime-core/src/executors/command.rs`
  2. `crates/runtime-core/src/execution.rs`
  3. `crates/runtime-core/src/capabilities/spec.rs`
  4. `crates/runtime-core/src/tool_trace.rs`
  5. `crates/runtime-core/src/artifacts.rs`
  6. `crates/runtime-core/src/run_failure_metadata.rs`
  7. `gateway/internal/contracts/contracts.go`
  8. `gateway/internal/session/bus.go`
  9. `frontend/src/shared/contracts/runtime.ts`
  10. `docs/11-hermes-rebuild/changes/E-claudecode-shell-alignment/artifacts/ClaudeCodeRev-shell-gap-matrix-20260413.md`
  11. `gateway/internal/api/artifact_content.go`
  12. `gateway/internal/api/artifact_content_test.go`
  13. `gateway/internal/api/router.go`
  14. `frontend/src/history/artifactApi.ts`
  15. `frontend/src/history/components/ArtifactOutputSection.tsx`
  16. `frontend/src/history/components/HistoryDetailRail.tsx`
  17. `frontend/src/history/components/HistoryTimelineSection.tsx`
  18. `frontend/src/index.css`
  19. `crates/runtime-core/src/risk.rs`
  20. `crates/runtime-core/src/run_risk_flow.rs`
  21. `crates/runtime-core/src/run_risk_flow_tests.rs`
  22. `gateway/internal/api/chat_confirmation_service.go`
  23. `gateway/internal/api/chat_confirmation_service_test.go`
  24. `gateway/internal/api/chat_confirmation_audit.go`
  25. `gateway/internal/api/chat_confirmation_memory.go`
  26. `gateway/internal/api/chat_confirmation_memory_test.go`
  27. `frontend/src/shared/permissionFlow.ts`
  28. `frontend/src/history/auditSignals.ts`
  29. `frontend/src/history/useHistoryReview.ts`
  30. `frontend/src/events/EventTimeline.tsx`
  31. `crates/runtime-core/src/compaction.rs`
  32. `crates/runtime-core/src/verify.rs`
  33. `crates/runtime-core/src/run_verification_metadata.rs`
  34. `crates/runtime-core/src/session.rs`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-E（执行中）
- 当前覆盖情况：
  1. 本次已覆盖 `T04-T07` 完成判据，不做 Gate-E 完成声明。
  2. `T06` 已完成三刀收口（规则分层 + ask 审计编排 + 前端/日志消费展示校验）。
  3. `T07` 已完成两刀收口（会话聚合预算边界 + 单结果预算阈值 + 预算命中 metadata 回放）。
  4. `T07` 补刀已完成（去除会话摘要二次截断 + 会话层可见性测试）。
  5. `T08` 已完成（阶段性提审包 `review.md` 已落盘，明确归档建议与回退口径）。

## 提交记录

1. 文档提交：`10d1f15`（`E-claudecode-shell-alignment` 文档与索引同步）。
2. 代码提交：`88b7172`（`T04-T07` 对应实现与测试补齐）。
3. 代码与文档补刀：`f89a401`（`T07` 会话预算链路补齐与验收记录同步）。
4. 文档推进：`cf4540f`（补齐 `T08` 任务位并同步状态验证口径）。
