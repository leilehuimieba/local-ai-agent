# H-visibility-runtime-20260415（design）

更新时间：2026-04-15
状态：已冻结（H01-01）

## 1. 设计概览

目标是建立“可理解、可追踪、可接管”的执行可视化协议，不改变主状态机，仅增强状态表达与消费展示。

## 2. 透明执行合同 v1（冻结）

| 字段 | 类型 | 必填 | 生产端 | 消费端 | 说明 |
|---|---|---|---|---|---|
| `stage` | enum | 是 | runtime | gateway/frontend | `Analyze/Plan/Act/Verify/Finish` |
| `activity_state` | enum | 是 | runtime | gateway/frontend | `running/waiting/retrying/blocked/completed` |
| `task_title` | string | 是 | runtime | frontend | 当前任务标题 |
| `active_tool` | string | 否 | runtime | frontend | 当前执行工具名（如 `run_command`） |
| `heartbeat_at` | string(ISO8601) | 是 | runtime | gateway/frontend | 最近进展心跳时间 |
| `stall_seconds` | integer | 是 | runtime | frontend | 连续无进展秒数 |
| `waiting_reason` | string | 否 | runtime | frontend | 等待原因（确认/输入/重试窗口） |
| `next_action_hint` | string | 否 | runtime | frontend | 建议下一步动作 |
| `trace_id` | string | 是 | runtime/gateway | frontend | 链路追踪 ID |
| `evidence_ref` | object | 否 | runtime/gateway | frontend | 证据引用（`artifact_path/raw_output_ref/log_id`） |
| `failure_route` | enum | 否 | runtime | frontend | `none/retry/manual/stop` |
| `updated_at` | string(ISO8601) | 是 | runtime/gateway | frontend | 该状态更新时间 |

## 3. 事件到可视化状态映射 v1（冻结）

| 事件类型 | stage | activity_state | waiting_reason | failure_route |
|---|---|---|---|---|
| `run_started` | Analyze | running | - | none |
| `analysis_ready` | Analyze | running | - | none |
| `plan_ready` | Plan | running | - | none |
| `action_requested` | Act | running | - | none |
| `confirmation_required` | Act | waiting | confirmation | manual |
| `checkpoint_resumed` | Act | retrying | retry_window | retry |
| `action_completed` | Act | running | - | none |
| `verification_completed` | Verify | running | - | none |
| `run_finished` | Finish | completed | - | none |
| `run_failed` | Finish | blocked | failed | manual |

## 4. 展示层结构

1. 顶部状态条：`stage + activity_state + heartbeat_at`。
2. 当前任务卡：`task_title + active_tool + waiting_reason + stall_seconds`。
3. 执行轨道：关键事件时间线（摘要）。
4. 详情抽屉：`evidence_ref/raw_output_ref/artifact_path/trace_id`。
5. 失败分流条：`failure_route`（retry/manual/stop）。

## 5. 卡住检测策略（初版）

1. `stall_seconds >= 30`：提示“处理中”。
2. `stall_seconds >= 60`：提示“可能卡住”，展示 `next_action_hint`。
3. `stall_seconds >= 120`：标记 `activity_state=blocked`，触发人工接管建议。

## 6. 兼容与回退

1. 新字段均为可选增量兼容，消费端按“新字段优先，旧字段回退”。
2. 若状态字段缺失，回退到现有 timeline 展示。
3. 保留 feature flag：`visibility_v1_enabled`。

## 7. 验证设计

1. 合同测试：runtime/gateway/frontend 字段一致性。
2. 链路测试：成功、等待确认、失败重试、人工接管四条样本。
3. 卡住测试：模拟无进展窗口，验证 30/60/120 秒阈值行为。
4. 回退测试：关闭 `visibility_v1_enabled` 后仍可用旧展示链路。
