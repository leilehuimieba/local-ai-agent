# 验证记录

## 验证方式

- 单元测试：
  - 多步回路执行测试
  - replan trigger 触发测试
  - iteration budget 耗尽后 handoff 测试
- 集成测试：
  - 至少一条从主循环进入 2 到 4 步受控动作后收口的运行链路证据
- 人工验证：
  - 核对事件流是否出现 `plan_iteration_started`、`plan_iteration_completed`、`replan_requested`、`iteration_budget_exhausted`

## 证据位置

- 测试记录：
  - `cargo test -p runtime-core`
  - 新增覆盖：
    - `crates/runtime-core/src/query_engine.rs` 中的主循环 replan / budget 测试
    - `crates/runtime-core/src/events.rs` 中的 iteration metadata 快照测试
    - `crates/runtime-core/src/observation/tests.rs` 中的生命周期覆盖测试
- 日志或截图：
  - 当前未额外留截图；本刀以 Rust 单元测试和事件契约回归为主

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期
- 当前覆盖情况：
  - 已完成第一刀文档拆分与范围冻结
  - 已完成 `PlanEnvelope`、小步回路、replan 事件与 iteration metadata 的代码接入
  - 已完成 196 项 `runtime-core` 测试通过，包含 observation 生命周期回归
