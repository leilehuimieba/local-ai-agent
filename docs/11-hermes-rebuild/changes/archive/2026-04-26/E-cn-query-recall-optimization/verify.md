# 验证记录

## 验证方式

- 单元测试：
  1. `cargo test -p runtime-core knowledge::tests -- --nocapture`
- 集成测试：
  1. 本刀无新增接口，未新增集成测试。
- 人工验证：
  1. 代码走查 `search_external_knowledge` 的主查询与 fallback 分支条件。
  2. 代码走查 `chinese_recall_fallback_query` 的中英文分流逻辑。

## 证据位置

- 测试记录：
  1. `knowledge::tests` 共 10 条通过。
  2. 新增通过用例：
     - `chinese_query_builds_fallback_aliases`
     - `english_query_has_no_fallback_aliases`
- 日志或截图：
  1. 代码证据：`crates/runtime-core/src/knowledge.rs`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-E（执行中）
- 当前覆盖情况：
  1. 本 change 仅覆盖中文 query 直召回优化，不做 Gate-E 完成声明。
