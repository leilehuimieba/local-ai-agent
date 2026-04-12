# 任务清单

- [x] 建立阶段 E 后端并行 change 并切主推进索引
  完成判据：`changes/INDEX.md` 将本 change 置为当前主推进项。
- [x] `chat/run|retry` 回包补齐首入口会话协议字段
  完成判据：回包包含 `request_id`、`trace_id`、`entry_id`、`protocol_version` 与标准端点提示。
- [x] `/api/v1/logs` 支持会话与运行过滤
  完成判据：支持 `session_id/run_id/limit` 参数，非法 `limit` 返回 `400`。
- [x] 补齐后端单测
  完成判据：`go test ./...` 通过，覆盖协议字段和日志过滤关键路径。
- [x] 增加接口级验收脚本并生成证据
  完成判据：`scripts/run-stage-e-entry1-acceptance.ps1` 通过并产出 `tmp/stage-e-entry1/latest.json`。
