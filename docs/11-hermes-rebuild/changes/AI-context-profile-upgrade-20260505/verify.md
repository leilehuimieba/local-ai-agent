# 验证记录

## 验证方式

- 单元测试：
  - `ask / act / repair / learn` profile 选择测试
  - 不同 profile 的注入差异测试
  - 恢复态 / checkpoint 情况下的 profile 选择测试
- 集成测试：
  - 至少一条回答类、执行类、恢复类链路分别命中正确 profile 的运行证据
- 人工验证：
  - 核对事件 metadata / context snapshot 中是否出现 profile 名称、选择理由与注入差异字段

## 证据位置

- 测试记录：
  - `cargo fmt --all`
  - `cargo test -p runtime-core`
- 日志或截图：
  - `crates/runtime-core/src/context_policy.rs`
  - `crates/runtime-core/src/context_builder.rs`
  - `crates/runtime-core/src/run_context_metadata.rs`
  - `crates/runtime-core/src/events.rs`
  - `gateway/internal/contracts/contracts.go`

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期
- 当前覆盖情况：
  - 已完成第二刀文档拆分与范围冻结
  - 已完成四类 profile 选择测试：`project_answer_uses_ask_profile`、`agent_resolve_uses_act_profile`、`knowledge_action_uses_learn_profile`、`recovery_override_forces_repair_profile`
  - 已完成注入差异测试：`fill_identity_fields_surfaces_injection_summary`
  - 已完成 metadata / snapshot 回归：`append_context_metadata_keeps_memory_layer_flags`、`context_snapshot_keeps_memory_layer_fields`
  - 已完成项目状态知识摘要回归：`project_answer_status_digest_prefers_current_hermes_docs`
  - 当前未引入记忆分层新结构、知识 pack 或 verify 矩阵扩 scope
