# 任务清单

- [x] 任务 1：冻结 AQ 的范围与执行入口
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只修知识问答收口态”保持一致。
- [x] 任务 2：盘点知识问答当前为何停在 SearchKnowledge 列表态
  完成判据：已记录 planner / query_engine / context_builder / knowledge / verify 在真实入口下的收口缺口。
- [x] 任务 3：收紧 SearchKnowledge -> knowledge_answer 的最小推进条件
  完成判据：正常知识问答在材料足够时能进入回答态，而不是直接输出检索列表。
- [x] 任务 4：补 citation-ready 回答收口
  完成判据：回答态能输出 citation，并体现 `事实 / 推断 / 建议` 边界。
- [x] 任务 5：保护低证据收口路径
  完成判据：`KA-05` 仍能保留 replan / handoff / 失败收口中的至少一种，不被“强行回答”覆盖。
- [x] 任务 6：复跑 AP 回归包并补证
  完成判据：复跑 `scripts/run-knowledge-answer-eval-pack.ps1`，并记录 `KA-01 ~ KA-04` 的改善结果。
- [x] 任务 7：整理 AQ 提审材料与状态收口
  完成判据：已补 `review.md`，并同步 `status.md`、`current-state.md`、`changes/INDEX.md` 的待提审口径。
