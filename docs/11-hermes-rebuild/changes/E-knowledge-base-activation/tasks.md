# 任务清单

- [x] 任务 1：扩展知识入库来源到 `agent_resolve`
  完成判据：`memory_router` 在 `agent_resolve + verified` 时生成知识类型并进入写入判定链路。
- [x] 任务 2：收口短摘要回退策略
  完成判据：知识摘要过短时可回退 `final_answer` 摘要，避免因短摘要误触发低价值拦截。
- [x] 任务 3：补齐单测与验证证据
  完成判据：新增单测通过，并在 `verify.md` 记录命令、结果与风险结论。
