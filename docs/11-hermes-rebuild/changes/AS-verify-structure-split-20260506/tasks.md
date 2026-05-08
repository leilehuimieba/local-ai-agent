# 任务清单

- [x] 任务 1：建立 `AS-verify-structure-split-20260506` 正式 change 工作区
  完成判据：已补齐 `proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 五件套。
- [x] 任务 2：冻结拆分范围与边界
  完成判据：已明确只处理 `verify.rs`，且不扩 `query_engine.rs`、knowledge scope、browser scope。
- [x] 任务 3：完成模块骨架设计
  完成判据：已确定 `mod.rs / policy.rs / evidence.rs / knowledge_answer.rs / browser_interaction.rs / file_command_memory.rs / tests/` 的职责边界。
- [x] 任务 4：实施 `verify.rs` 结构拆分
  完成判据：主文件显著瘦身，核心逻辑已迁入新模块，且外部入口保持稳定。
- [x] 任务 5：整理测试模块与最小接线
  完成判据：测试文件完成分组，最小引用修正已完成，不引入范围外逻辑修改。
- [x] 任务 6：执行验证并补证据
  完成判据：verify 相关定向测试、`run_verification_metadata` 相关测试通过，并写回 `verify.md`。
