# 任务清单

- [x] 任务 1：冻结 AN 的范围与执行入口
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只做浏览器恢复治理细化”保持一致。
- [x] 任务 2：盘点浏览器失败路径当前输出缺口
  完成判据：已记录 verify 失败、单次回读、handoff / failure metadata 当前有哪些字段，哪些还不足。
- [x] 任务 3：补最小失败分型与 recovery 停止条件
  完成判据：至少能区分目标未命中、状态未变化、边界信号不足、recovery 用尽。
- [x] 任务 4：补 handoff artifact 与失败提示
  完成判据：失败后可留下可读、可交接的最小 artifact / metadata。
- [x] 任务 5：补测试与联调证据
  完成判据：至少补齐 failure typing 测试、handoff 输出测试和一条失败联调证据。
