# 阶段 C 提审收口包（Gate-C）

更新时间：2026-04-12  
适用 change：`C-tool-contract-unification`  
当前结论：满足 Gate-C，已签收，可切换至阶段 D

## 1. 提审范围

1. 工具合同最小统一：参数、错误、trace、耗时字段收口。
2. 风险分级确认链最小闭环：高风险动作确认、审批恢复、终态收口可追溯。
3. 审计字段完善：成功/失败/确认恢复链路关键字段可检索。

## 2. Gate-C 判定结果

判定依据来自批量验收报告：`tmp/stage-c-gate-c-batch/latest.json`。

1. 高风险动作拦截率：`1.0`（阈值 `>= 0.99`）`PASS`
2. 工具调用失败可定位率：`1.0`（阈值 `>= 0.95`）`PASS`
3. 审计字段完整率：`1.0`（阈值 `= 1.0`）`PASS`
4. 额外样本校验：`tool_elapsed_ms` 成功链路字段完整 `PASS`
5. 综合判定：`gate_c.ready=true`

## 3. 证据映射

### 3.1 核心报告

1. `tmp/stage-c-gate-c-batch/latest.json`
2. `tmp/stage-c-risk-audit-acceptance/latest.json`
3. `tmp/stage-c-tool-elapsed-acceptance/latest.json`
4. `tmp/stage-b-retry-acceptance/latest.json`（失败可定位率统计输入）

### 3.2 关键实现落点

1. `crates/runtime-core/src/checkpoint.rs`  
   - `checkpoint_resumed` 新增：`confirmation_decision`、`confirmation_chain_step`、`confirmation_resume_strategy`、`confirmation_decision_source` 等。
2. `gateway/internal/api/chat_confirmation_memory.go`  
   - 确认收口事件新增：`checkpoint_id`、`confirmation_chain_step`、`confirmation_resume_strategy`、`confirmation_decision_source`。
3. `scripts/run-stage-c-risk-audit-acceptance.ps1`  
   - 接口级确认链验收：`chat/run -> confirmation_required -> checkpoint_resumed -> run_finished`。
4. `scripts/run-stage-c-gate-batch.ps1`  
   - Gate-C 批量统计与阈值判定。

### 3.3 回归验证

1. `cargo test -p runtime-core` 通过（64 passed）。
2. `go test ./...`（`gateway/`）通过。
3. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-c-gate-batch.ps1 -Rounds 5 -RequireGateC` 通过。

## 4. 风险与回退

1. 风险：批量样本目前为 5 轮，属于最小门禁样本，不代表长时稳定性极限。
2. 风险：前端若未消费新增确认审计键，展示层可能出现“可追溯但未展示”。
3. 回退：确认链字段均为兼容性新增，不涉及破坏性删除；若联调异常，可先降级为旧字段读取路径。

## 5. 阶段切换判定

1. 阶段 C 的 Gate-C 指标已满足，建议进入下一阶段开发准备。
2. 按总路线顺序，下一主阶段应为阶段 D；阶段 E 前端窗口可继续保持“方案并行，不抢主线实现”。
3. 批准条件：评审通过本提审包后，将 `C-tool-contract-unification` 从“当前活跃 change”转入“最近收口”，并启动下一主推进项。

## 6. 签收记录

1. 签收时间：2026-04-12
2. 签收依据：Gate-C 指标报告 `tmp/stage-c-gate-c-batch/latest.json`，`gate_c.ready=true`
3. 签收动作：按总表任务 `C-G1` 执行评审签收与索引切换
4. 阶段决议：阶段 C 收口，切换到阶段 D 主推进
