# 验证记录

## 验证方式

- 文档验证：
  - 核对 `proposal.md`、`design.md`、`tasks.md`、`status.md` 是否都只覆盖记忆写回治理。
- 测试验证：
  - 写回分层判断测试
  - 准入/拒绝规则测试
  - 治理 metadata 输出测试
- 留痕验证：
  - 至少 1 条长期写回成功样例
  - 至少 1 条被拒绝、仅停留 working 的样例

## 证据位置

- 已完成：
  - Rust: `cargo test -p runtime-core`
  - Rust: `cargo test -p runtime-core memory_`
  - Rust: `cargo test -p runtime-core run_finish_events`
  - Rust: `cargo test -p runtime-core memory_write_`
  - Rust: `cargo test -p runtime-core context_snapshot_keeps_memory_layer_fields`
  - 样例：`memory_router::tests::rejected_workspace_summary_marks_working_only`
  - 样例：`memory_router::tests::preference_entry_goes_semantic_or_procedural`
  - 样例：`memory_router::tests::failure_lesson_entry_goes_episodic`
  - 事件：`run_finish_events::tests::memory_event_keeps_write_governance_metadata`
- 待补：
  - 暂无

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期
- 当前覆盖情况：
  - 已完成独立 change 建档
  - 已完成第一批字段、最小判层、事件 metadata 与定向测试
  - 已补 `working_only + rejected` 与长期层 accepted 最小留痕样例
  - 已补全量 `runtime-core` 回归证据
  - 已补 `review.md`，可进入提审裁决
