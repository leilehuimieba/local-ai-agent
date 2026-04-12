# 任务清单

- [x] 新增 `chat/cancel` 接口
  完成判据：`POST /api/v1/chat/cancel` 可接收 `session_id + run_id` 并返回受理结果。
- [x] 补齐运行注册与取消收口
  完成判据：取消后产生日志终态 `completion_status=cancelled`。
- [x] 补齐单测
  完成判据：`TestCancelStopsRunningRun` 通过。
- [x] 交付验收脚本与证据
  完成判据：`tmp/stage-e-cli-cancel/latest.json` 为 `status=passed`。
