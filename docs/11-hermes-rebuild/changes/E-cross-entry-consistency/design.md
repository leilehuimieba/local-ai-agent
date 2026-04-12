# 技术方案

## 影响范围

- 网关请求构造：`gateway/internal/api/chat.go`、`gateway/internal/api/chat_context_resolver.go`
- 网关测试：`gateway/internal/api/chat_context_ids_test.go`
- 验收脚本：`scripts/run-stage-e-consistency-acceptance.ps1`

## 方案

### 1. 入口身份字段透传

- 为 `ChatRunRequest` 增加可选字段：
  - `request_id`
  - `run_id`
  - `trace_id`
- `buildRunRequest` 使用 `pickRunIdentity`：
  - 有输入值则复用输入
  - 无输入值则按原逻辑生成新 ID

### 2. 一致性验收脚本

- 新增 `run-stage-e-consistency-acceptance.ps1`，流程：
  1. 隔离端口启动 runtime 与 gateway。
  2. 以固定 `request_id/run_id/session_id/trace_id` 直调 runtime（模拟 CLI 入口）。
  3. 以相同身份字段调 `chat/run`（gateway 入口）。
  4. 按 `session_id/run_id` 过滤拉取 gateway 日志。
  5. 校验 run 身份、终态事件类型、终态工具名、完成状态一致性并落盘。

## 风险与回退

- 风险：同 `run_id` 二次执行会引入历史上下文差异，导致细粒度事件序列不完全一致。
- 缓解：E-04 只将“身份一致性 + 终态一致性 + 可过滤定位”作为必过条件。
- 回退：若后续接入不需要外部注入身份，可移除可选字段，仅保留网关内部生成。
