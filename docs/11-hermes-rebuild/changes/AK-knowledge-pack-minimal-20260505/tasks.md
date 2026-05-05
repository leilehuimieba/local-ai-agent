# 任务清单

- [x] 任务 1：冻结第四刀知识检索增强范围与落点
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只做知识检索增强与 knowledge pack”保持一致。
- [x] 任务 2：实现最小混合检索与轻量重排
  完成判据：`gateway/internal/knowledge/store.go` 与 `knowledge.rs` 能基于 title / tags / chunk 命中输出更稳定的前几条知识结果。
- [x] 任务 3：实现 knowledge pack 与 citation-ready 输出
  完成判据：`knowledge.rs` 能输出 `question_type`、`top_hits`、`supporting_hits`、`citations`、`answer_hints` 的最小 pack。
- [x] 任务 4：让 ask / learn profile 优先注入 knowledge pack
  完成判据：`context_builder.rs` 与 `events.rs` 能在 ask / learn 场景下注入并透出 knowledge pack metadata。
- [x] 任务 5：补第四刀验证证据
  完成判据：至少补齐混合检索命中测试、knowledge pack 结构测试，以及 3 个知识问答样例回归。
