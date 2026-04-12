# 技术方案

## 影响范围

- `crates/runtime-core/`（工具调用 contract、错误与事件落点）
- `gateway/internal/`（请求适配、风险确认链、审计字段透传）
- `frontend/src/`（确认交互与审计可读字段）
- `docs/11-hermes-rebuild/stage-plans/` 与 `docs/11-hermes-rebuild/changes/`

## 方案

- 先做合同收口，不先大改实现：
  - 盘点 runtime/gateway/frontend 的工具入参、出参、错误字段与 trace 字段差异。
  - 产出阶段 C 的“最小统一字段表”和“兼容映射表”。
- 按最小闭环推进实现：
  - 第一步：工具调用合同统一（参数、错误码、耗时、trace id）。
  - 第二步：风险分级与确认链统一（先覆盖高风险动作）。
  - 第三步：审计日志补齐可追溯字段并给出查询口径。
- 验证口径直接映射 Gate-C：
  - 高风险动作拦截准确率 >= 99%。
  - 工具调用失败可定位率 >= 95%。
  - 审计字段齐全且可追溯。

## 跨端字段对齐表（2026-04-12）

| 对象 | Runtime | Gateway | Frontend | 结论 |
| --- | --- | --- | --- | --- |
| `ErrorInfo` | `error_code/message/summary/retryable/source/stage/metadata` | 同名同义 | 同名同义 | 已对齐，无需改口径 |
| `RunEvent` 工具相关字段 | `tool_name/tool_display_name/tool_category/output_kind/result_summary/artifact_path/risk_level/confirmation_id` | 同名同义 | 同名同义 | 已对齐，无需改口径 |
| `RunEvent.tool_call_snapshot` | `tool_name/display_name/category/risk_level/input_schema/output_kind/requires_confirmation/arguments_json` | 同名同义 | 同名同义 | 已对齐，可作为阶段 C 最小基线 |
| `RunEvent.metadata` | `map<string,string>`，承载 `error_code` 与工具扩展字段 | 同口径透传 | 同口径消费 | 已对齐，后续只补字段治理规则 |
| `RunEvent.metadata.tool_elapsed_ms` | 已在 runtime 成功/失败链路写出（来自工具执行耗时采集） | 透传 | 可直接消费 | 已对齐，并有接口级样本 |
| `RunEvent.metadata.confirmation_*`（恢复链） | `checkpoint_resumed` 输出 `confirmation_id/confirmation_decision/confirmation_decision_note/confirmation_chain_step/confirmation_resume_strategy/confirmation_decision_source` | 透传 | 可直接消费 | 已对齐，可串联 `confirmation_required -> checkpoint_resumed -> run_finished` |
| `RunEvent.metadata.checkpoint_id`（确认收口） | 在确认恢复与关闭链路输出 | gateway 关闭事件补齐同名字段 | 可直接消费 | 已对齐，可定位确认对应 checkpoint |
| `RuntimeContextSnapshot` | 含 `phase_label/selection_reason/prefers_artifact_context/artifact_hint` | 已补齐 | 已补齐 | 已对齐，进入后续联调阶段 |
| `RunRequest.confirmation_decision.decision` | `string` | `string` | `"approve" \| "reject" \| "cancel"` | 前端更严格，后端需保持枚举值约束，避免脏值 |

本轮收口结论：

- 阶段 C 可直接复用现有 `ToolCallSnapshot` 与 `RunEvent` 工具字段作为“统一合同第一版”，不需要重写模型。
- `RuntimeContextSnapshot` 缺口已收口，可按统一字段继续做联调与回归。
- `tool_elapsed_ms` 成功/失败链路已收口，下一批实现聚焦风险确认链细化。
- 风险确认恢复链已补齐 `confirmation_*` 审计键，可稳定检索确认决策与恢复动作。
- 已有接口级样本证明 `chat/run -> logs` 可稳定检索 `metadata.tool_elapsed_ms`。

## 风险与回退

- 主要风险：跨端字段改动不同步导致联调失败。
- 主要风险：确认链改动影响现有可用路径。
- 回退方式：保留 PowerShell 主路径，其他后端能力降级为可选；合同字段采用兼容映射，不做破坏性删除。
