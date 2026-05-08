# 任务清单

- [x] 任务 1：冻结 AO 的范围与执行入口
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只做记忆写回治理”保持一致。
- [x] 任务 2：盘点当前记忆写回链缺口
  完成判据：已记录当前长期写回入口、写回类型判断、去重/拒绝规则、事件 metadata 当前有哪些字段，哪些还不足。
- [x] 任务 3：补最小写回分层与准入规则
  完成判据：至少能区分 `working_only / episodic_memory / semantic_or_procedural_memory`；并完成 `working_memory_outcome / write_failure_lesson_memory / write_preference_memory / write_long_term_memory` 的最小归层决策。
- [x] 任务 4：补治理 metadata 与最小事件透出
  完成判据：`memory_write_layer / memory_write_decision / memory_write_reason / memory_duplicate_strategy` 能在 `MemoryWriteOutcome / MemoryAuditTrail` 读取，并在长期条目 schema 中具备落点。
- [x] 任务 5：补测试与留痕
  完成判据：至少补齐分层判断测试、准入/拒绝测试和一条最小写回治理留痕。
- [x] 任务 6：整理 AO 提审材料
  完成判据：已补 `review.md`，并让 `status.md`、`verify.md` 与待提审口径保持一致。
