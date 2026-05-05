# 任务清单

- [x] 任务 1：冻结第一刀主循环范围与改造落点
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只做主循环最小升级”保持一致。
- [x] 任务 2：实现 `PlanEnvelope` 与小步回路
  完成判据：Runtime 主链支持 `PlanEnvelope` 和最多 2 到 4 步的受控迭代，且保留单步兼容回退。
- [x] 任务 3：补齐 replan 与 iteration 事件
  完成判据：`query_engine.rs` 支持最小 replan trigger，`events.rs` 能输出 iteration 相关事件和必要 metadata。
- [x] 任务 4：补第一刀验证证据
  完成判据：至少补齐多步回路单元测试、replan 触发测试，以及 budget 耗尽后 handoff 的证据。
