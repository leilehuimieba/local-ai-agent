# 变更提案

## 背景

- 用户要求当前轮先暂停前端改动，优先推进后端，给后续前端服务接入提供稳定接口。
- 阶段 E 的 `E-02` 目标是“网关统一会话协议首入口”，当前缺少面向接入方的入口协议字段与可复核验收样本。

## 目标

- 在网关首入口 `chat/run|chat/retry` 回包补齐会话协议关键字段（`request_id`、`trace_id`、`entry_id`、`protocol_version` 与标准端点提示）。
- 在 `/api/v1/logs` 增加按 `session_id/run_id/limit` 的过滤能力，支撑接入方按会话与运行快速定位事件链。
- 增加 `E-02` 接口级验收脚本与证据产物，形成可复核最小闭环。

## 非目标

- 本轮不改前端页面与前端状态管理实现。
- 本轮不扩展网关第二入口，不做多入口协议编排。
- 本轮不做 Gate-E 完成声明。

## 验收口径

- `POST /api/v1/chat/run`、`POST /api/v1/chat/retry` 回包包含统一会话协议字段。
- `GET /api/v1/logs` 支持 `session_id`、`run_id`、`limit` 过滤，并通过测试。
- 执行 `scripts/run-stage-e-entry1-acceptance.ps1` 通过，生成 `tmp/stage-e-entry1/latest.json`。
