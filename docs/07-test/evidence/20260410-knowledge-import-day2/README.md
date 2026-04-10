# 2026-04-10 Day2 资料导入最小闭环证据

本目录对应 7 天执行清单中的 Day2：`资料导入最小闭环`。

本轮只验证两件事：

1. 两份真实资料可被运行时读取（可见）。
2. 导入结果可被写入到目标文件并可对上运行记录（可追溯）。

## 1. 样本范围

Case A：

1. 源资料：`docs/README.md`
2. 读取 run：`run-1775781795595-426`（`workspace_read`）
3. 写入 run：`run-1775781821189-429`（`workspace_write`）
4. 目标文件：`tmp/knowledge-import-day2/20260410/import-a-summary.md`

Case B：

1. 源资料：`docs/07-test/evidence/20260409-pre-release-smoke/README.md`
2. 读取 run：`run-1775781832306-432`（`workspace_read`）
3. 写入 run：`run-1775781844456-435`（`workspace_write`）
4. 目标文件：`tmp/knowledge-import-day2/20260410/import-b-summary.md`

## 2. 证据清单

每个 case 都包含三类文件：

1. `*.run-accepted.json`
2. `*.run-events.json`
3. `*.run-finished.json`

汇总文件：

1. `day2-import-run-summary.json`（4 条 run 的状态、`result_mode`、`verification_code`、artifact 路径）
2. `day2-import-visibility-diff.json`（导入前后可见性差异）
3. `import-a-summary.preview.md`
4. `import-b-summary.preview.md`

## 3. 可见性与可追溯性结论

1. 可见性：两条写入 run 后，两个目标文件均存在（`target_exists_after_write=true`）。
2. 可追溯性：每个 case 都可由 `read run_id -> write run_id -> 目标文件` 串联，且 `run_finished` 均为 `completed + answer + verified`。
3. 口径边界：本轮导入采用“读取一步 + 写入一步”的最小闭环，不宣称是单次 run 内自动完成的重型导入流程。
