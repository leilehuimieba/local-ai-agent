# 任务清单

- [x] T01 中文 query fallback 生成逻辑落地
  完成判据：中文 query 可生成 fallback 锚点；英文 query 不触发。
- [x] T02 外部 recall 空结果二次召回接入
  完成判据：主查询空结果时执行一次 fallback recall，主查询有结果时不触发。
- [x] T03 单测与验证证据补齐
  完成判据：`knowledge::tests` 通过，新增中文 fallback 相关测试。
