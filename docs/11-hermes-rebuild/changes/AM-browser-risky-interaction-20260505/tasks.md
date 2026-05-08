# 任务清单

- [x] 任务 1：冻结 AM 的范围与执行入口
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只做浏览器高风险交互增量”保持一致。
- [x] 任务 2：盘点 `select / submit / upload` 当前实现缺口
  完成判据：已记录 Runtime、Gateway、Browser MCP、verify 链各自已有能力与待补点。
- [x] 任务 3：落最小交互 contract 与风险分层
  完成判据：三类动作在 request-scoped spec、risk、confirmation 口径中一致可见。
- [x] 任务 4：补最小 verify 与 recovery/handoff
  完成判据：三类动作 verify 失败时先单次回读，再失败则停止自动续跑并进入 handoff。
- [x] 任务 5：补测试与联调证据
  完成判据：至少补齐 contract 测试、verify/replan 测试和一条最小联调证据。
