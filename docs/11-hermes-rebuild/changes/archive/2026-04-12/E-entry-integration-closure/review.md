# 阶段 E 入口联调收口包（E-05）

更新时间：2026-04-12  
适用 change：`E-entry-integration-closure`  
当前结论：`E-05` 已完成，可进入 `E-G1`，尚未做 Gate-E 完成声明

## 1. 收口范围

1. 入口协议成功链收口（`E-02`）。
2. 跨入口一致性链收口（`E-04`）。
3. 失败样本与回退路径收口（`E-05`）。

## 2. 证据映射

### 2.1 成功链（首入口协议）

1. 报告：`tmp/stage-e-entry1/latest.json`
2. 结论：`status=passed`
3. 关键点：
   - `run_accepted` 协议字段完整。
   - `run_id/session_id` 过滤一致。
   - 终态为 `run_finished` 且 `completion_status=completed`。

### 2.2 一致性链（跨入口同 run_id）

1. 报告：`tmp/stage-e-consistency/latest.json`
2. 结论：`status=passed`
3. 关键点：
   - `accepted_id_matched=true`
   - `runtime_result_matched=true`
   - `all_gateway_run_matched=true`
   - `all_gateway_session_matched=true`
   - `terminal_type_matched=true`
   - `terminal_tool_matched=true`
   - `completion_status_matched=true`
   - `gateway_trace_matched=true`

### 2.3 失败链（runtime 不可达）

1. 报告：`tmp/stage-e-entry-failure/latest.json`
2. 结论：`status=passed`
3. 关键点：
   - 网关请求仍可受理（`accepted=true`）。
   - 日志出现 `run_failed`，并带 `error_code=runtime_unavailable`。
   - 日志出现 `run_finished` 终态，形成失败收口闭环。
   - `run_id/session_id` 过滤一致，可定位到单次失败样本。

## 3. 回退路径

1. 运行时不可达回退（运维级）  
   - 触发条件：失败样本出现 `runtime_unavailable`。  
   - 处理动作：恢复 runtime 进程后重试；复核 `tmp/stage-e-entry-failure/latest.json` 的错误码与收口链。

2. 多入口联调异常回退（阶段级）  
   - 触发条件：跨入口一致性不满足。  
   - 处理动作：先冻结为 `E-02` 单入口路径，保留统一协议层与日志过滤；复核 `tmp/stage-e-entry1/latest.json`。

3. 身份锚点注入异常回退（兼容级）  
   - 触发条件：外部传入 `request_id/run_id/trace_id` 造成联调偏差。  
   - 处理动作：回退到网关自生成身份（不传可选身份字段），保持协议兼容不破坏。

## 4. 风险与遗留

1. 当前失败样本覆盖的是 runtime 不可达场景，尚未覆盖所有异常类型。
2. Gate-E 的一致性指标（`>=95%`）仍需在 `E-G1` 用批量脚本做正式判定。
3. 前端窗口由用户并行推进，本收口包仅覆盖后端入口与协议链。

## 5. 阶段决议

1. `E-05` 的完成判据“失败样本与回退路径齐全”已满足。
2. 允许进入 `E-G1`，执行 Gate-E 批量验收与评审签收。
