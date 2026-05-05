# 任务清单

- [x] 任务 1：冻结第五刀 verify 矩阵范围与实现入口
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只做 verify 矩阵”保持一致。
- [x] 任务 2：先实现知识回答类 verify
  完成判据：`verify.rs` 能对 knowledge answer 给出独立 policy、通过标准、失败标准和 next step。
- [x] 任务 3：补最小 verification metadata 扩展
  完成判据：`run_verification_metadata.rs`、`events.rs` 能透出 task type、citation、evidence count、风险边界状态。
- [x] 任务 4：把知识 verify 结果接回主循环决策
  完成判据：knowledge verify 失败时，系统能优先给出 replan / handoff，而不是继续硬跑。
- [x] 任务 5：补第五刀验证证据
  完成判据：至少补齐知识回答 verify 的单元测试、metadata 测试，以及 3 个知识问答回归样例。
- [x] 任务 6：标记其余任务类型为后续增量
  完成判据：文件修改 / 命令执行 / 记忆写入 / 浏览器交互的 verify 扩展顺序与后置边界已记录清楚。
