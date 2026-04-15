# H-visibility-runtime-20260415（design）

更新时间：2026-04-15
状态：草案

## 1. 设计概览

目标是建立“可理解、可追踪、可接管”的执行可视化协议，不改变主状态机，仅增强状态表达与消费展示。

## 2. 状态合同（最小字段）

1. stage：Analyze/Plan/Act/Verify/Finish
2. activity_state：running/waiting/retrying/blocked/completed
3. task_title：当前任务标题
4. active_tool：当前工具名（如 run_command）
5. heartbeat_at：最近心跳时间
6. stall_seconds：无进展持续秒数
7. waiting_reason：等待原因（确认/外部输入/重试窗口）
8. next_action_hint：下一步建议
9. trace_id：链路追踪ID
10. evidence_ref：证据路径引用（artifact/log）

## 3. 展示层结构

1. 顶部状态条：stage + activity_state + heartbeat
2. 当前任务卡：task_title + active_tool + waiting_reason + stall_seconds
3. 执行轨道：关键事件时间线（摘要）
4. 详情抽屉：raw_output_ref / artifact_path / trace_id
5. 失败分流条：retry/manual/stop 三选一

## 4. 卡住检测策略（初版）

1. 30s 无进展：提示“处理中”
2. 60s 无进展：提示“可能卡住”，给出建议动作
3. 120s 无进展：标记 blocked，触发人工接管建议

## 5. 兼容与回退

1. 新字段均为可选字段，前端按“新字段优先，旧字段回退”。
2. 若状态字段缺失，回退到现有 timeline 展示。
3. 保留 feature flag：`visibility_v1_enabled`。

## 6. 验证设计

1. 合同测试：runtime/gateway/frontend 字段一致性。
2. 链路测试：运行成功、等待确认、失败重试、人工接管四条样本。
3. 卡住测试：模拟无进展时间窗口，验证提示逻辑。
