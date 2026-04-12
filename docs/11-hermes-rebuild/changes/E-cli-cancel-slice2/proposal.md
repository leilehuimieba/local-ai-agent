# 变更提案

## 背景

- `E-01` 的历史视图切片已完成，但 CLI/TUI 交互中“中断”动作还缺少后端接口闭环。
- 当前 `chat/run`、`chat/retry`、`chat/confirm` 已可用，缺少统一的运行中取消入口。
- 用户要求继续后端推进，本轮不改前端。

## 目标

- 新增 `POST /api/v1/chat/cancel`，支持按 `session_id + run_id` 取消运行中任务。
- 在事件流中补齐取消终态（`completion_status=cancelled`）并保持可追溯。
- 产出 `tmp/stage-e-cli-cancel/latest.json` 验收证据。

## 非目标

- 不改前端页面与交互行为。
- 不改 Runtime 协议。
- 不调整已有确认链与重试链行为。

## 验收口径

- `chat/cancel` 对运行中任务返回 `202 accepted`。
- 取消后日志终态可见 `run_finished` 且 `completion_status=cancelled`。
- `run-stage-e-cli-cancel-acceptance.ps1` 执行通过。
