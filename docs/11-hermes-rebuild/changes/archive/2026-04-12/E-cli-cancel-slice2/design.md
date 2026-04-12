# 技术方案

## 影响范围

- 接口与执行注册：
  - `gateway/internal/api/chat.go`
  - `gateway/internal/api/chat_cancel_service.go`
  - `gateway/internal/api/chat_execution_registry.go`
  - `gateway/internal/api/router.go`
- 单测：
  - `gateway/internal/api/chat_cancel_test.go`
- 验收脚本：
  - `scripts/run-stage-e-cli-cancel-acceptance.ps1`

## 方案

### 1. 运行中任务注册

- 在 `ChatHandler` 内维护运行注册表（`run_id -> cancel handle`）。
- `execute` 启动时注册，结束时清理。

### 2. 取消接口

- 新增 `POST /api/v1/chat/cancel`：
  - 入参：`session_id`、`run_id`。
  - 若命中运行中任务，触发 cancel 并返回 `202`。
  - 未命中返回 `404 run not running`。

### 3. 取消终态事件

- 取消后补发：
  - `run_failed`（`error_code=run_cancelled`）
  - `run_finished`（`completion_status=cancelled`）
- 保持与现有日志查询链路兼容。

## 风险与回退

- 风险：取消触发与运行完成存在并发窗口。
- 缓解：以运行注册表保证仅取消运行中任务，结束即清理。
- 回退：若出现异常，可移除 `chat/cancel` 路由与注册逻辑，恢复原执行路径。
