# 任务清单

- [x] 任务 1：冻结第二刀上下文 profile 范围与落点
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只做上下文 profile 升级”保持一致。
- [x] 任务 2：实现 `ask / act / repair / learn` profile 选择
  完成判据：`context_policy.rs` 能根据动作类型、恢复态、问题类型和 mode 选出四类 profile。
- [x] 任务 3：实现不同 profile 的最小包装配
  完成判据：`context_builder.rs` 能让四类 profile 输出不同注入差异，并把 profile 名称与选择理由挂到 metadata。
- [x] 任务 4：补第二刀验证证据
  完成判据：至少补齐 profile 选择测试、不同 profile 注入差异证据，以及恢复态不会误注入无关大段知识摘要的回归。
