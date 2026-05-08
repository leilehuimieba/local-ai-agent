# 任务清单

- [x] 任务 1：建立 `AR-state-realignment-and-modularity-closeout-20260506` 正式 change 工作区
  完成判据：已补齐 `proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 五件套。
- [x] 任务 2：对齐当前执行入口口径
  完成判据：`D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md` 与 `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md` 已改为由 AR 承接当前活跃 change。
- [x] 任务 3：盘点真实工作区未收口改动
  完成判据：已记录 `git status --short`、`git diff --stat` 的核心结论，并确认“当前不能写无活跃 change”。
- [x] 任务 4：盘点结构性债务与待收口目录
  完成判据：已记录越线热点文件，以及 `scripts / evidence / tmp` 当前需要收口的目录与入口。
- [x] 任务 5：形成未提交改动归属矩阵
  完成判据：已把 runtime / browser / gateway / docs / scripts / evidence 改动按“已完成待入库 / 待补文档 / 需新开实现 change”三类分清，并写入 `design.md`。
- [x] 任务 6：形成热点文件拆分备案
  完成判据：已完成 `verify.rs`、`query_engine.rs` 的拆分备案与优先顺序说明，且没有直接落实现。
- [x] 任务 7：形成聚合回归与证据入口收口单
  完成判据：已明确 `scripts/README.md`、`docs/07-test/evidence/20260506-*`、`tmp/knowledge-answer-evals/` 的保留口径、引用方式与后续清理边界。
- [x] 任务 8：确定下一步最小实现闭环
  完成判据：已确定下一步最小实现项为新开 `AS-verify-structure-split-20260506`，并已把前置依赖与后置项写回 `design.md`、`status.md`。
