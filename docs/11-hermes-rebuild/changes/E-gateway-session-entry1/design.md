# 技术方案

## 影响范围

- 后端接口：`gateway/internal/api/chat.go`、`gateway/internal/api/router.go`、`gateway/internal/api/logs_query.go`
- 会话日志：`gateway/internal/session/bus.go`
- 验收脚本：`scripts/run-stage-e-entry1-acceptance.ps1`
- 测试：`gateway/internal/api/*_test.go`、`gateway/internal/session/*_test.go`

## 方案

### 1. 首入口会话协议字段收口

- 扩展 `ChatRunAccepted`，补齐：
  - `request_id`
  - `trace_id`
  - `entry_id`（固定 `gateway.chat.entry1`）
  - `protocol_version`（固定 `v1`）
  - `stream_endpoint`、`logs_endpoint`、`confirm_endpoint`、`retry_endpoint`
- `Run` 与 `Retry` 统一复用 `newChatRunAccepted` 构造响应，避免字段漂移。

### 2. 日志接口过滤能力

- 新增 `decodeLogsQuery/parseLogsLimit` 解析查询参数。
- `/api/v1/logs` 支持：
  - `session_id`：按会话过滤
  - `run_id`：按运行过滤
  - `limit`：范围 `[1,500]`，默认 `120`
- `EventBus` 新增 `RecentBy(limit, sessionID, runID)`，按过滤条件返回尾部记录。

### 3. 接口级验收脚本

- 新增 `scripts/run-stage-e-entry1-acceptance.ps1`：
  - 隔离端口启动 runtime/gateway
  - 调用 `chat/run` 并校验协议字段
  - 调用带过滤参数的 `logs` 并等待终态事件
  - 输出 `tmp/stage-e-entry1/latest.json`

## 风险与回退

- 主要风险：接入方仍按旧回包字段解析导致兼容疑虑。
- 缓解方式：新增字段均为追加，不移除旧字段。
- 回退方式：若异常，回退 `ChatRunAccepted` 新增字段与 `logs` 查询解析逻辑，保留旧行为。
