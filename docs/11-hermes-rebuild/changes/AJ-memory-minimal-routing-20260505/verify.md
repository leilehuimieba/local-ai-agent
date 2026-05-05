# 验证记录

## 验证方式

- 单元测试：
  - memory route 选择测试
  - 不同 route 的最小 digest 差异测试
  - 恢复态 / checkpoint 情况下的 route 选择测试
- 运行验证：
  - `cargo fmt --all`
  - `cargo test -p runtime-core`
- 人工验证：
  - 核对事件 metadata / context snapshot 中是否出现 `memory_route`、层选择理由和最小 digest 字段

## 证据位置

- 测试记录：
  - `crates/runtime-core/src/memory_router/read_route.rs`
  - `crates/runtime-core/src/memory_recall.rs`
  - `crates/runtime-core/src/memory_layer.rs`
  - `crates/runtime-core/src/context_builder.rs`
  - `crates/runtime-core/src/events.rs`
- 命令记录：
  - 2026-05-05：`cargo fmt --all`
  - 2026-05-05：`cargo test -p runtime-core`
  - 结果：205 个测试全部通过

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期
- 当前覆盖情况：
  - 已完成第三刀文档拆分、范围冻结、代码实现与 route 字段贯通
  - 已补 route 选择测试、digest 差异测试、恢复态回归验证
  - 未覆盖知识 pack、知识检索增强、verify 矩阵与写侧治理重构，这些内容继续留在后续 change
