# 变更提案

## 背景

- 总表中 `E-01` 长期处于待办，当前缺少“CLI/TUI 交互切片 1”的最小后端闭环证据。
- 现有 `GET /api/v1/logs` 只提供事件明细视图，不利于 CLI 快速查看“按 run 聚合”的历史。
- 用户要求继续后端推进，本轮不改前端。

## 目标

- 为 `GET /api/v1/logs` 增加 `view=runs` 历史视图，返回去重后的最近 run 历史条目。
- 补齐对应单测与验收脚本，产出 `tmp/stage-e-cli-history/latest.json`。
- 完成 `E-01` 文档与状态收口。

## 非目标

- 不改前端页面与交互样式。
- 不改 Runtime Host 协议与执行链路。
- 不新增网关入口，仅在现有日志接口上扩展视图。

## 验收口径

- `GET /api/v1/logs?view=runs` 可返回 run 级历史。
- `go test ./...`（`gateway/`）通过。
- `run-stage-e-cli-history-acceptance.ps1` 通过并输出 `status=passed`。
