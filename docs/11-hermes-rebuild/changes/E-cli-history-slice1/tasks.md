# 任务清单

- [x] 扩展 logs 查询视图参数
  完成判据：`decodeLogsQuery` 支持 `view=events|runs`。
- [x] 交付 run 级历史聚合能力
  完成判据：`GET /api/v1/logs?view=runs` 返回去重后的 run 历史条目。
- [x] 补齐单测
  完成判据：新增/更新单测通过，覆盖 query 解析与 runs 视图行为。
- [x] 交付 E-01 验收脚本与证据
  完成判据：`tmp/stage-e-cli-history/latest.json` 为 `status=passed`。
- [x] 回写总表与索引
  完成判据：`E-01` 状态同步为已完成，索引切换到当前变更。
