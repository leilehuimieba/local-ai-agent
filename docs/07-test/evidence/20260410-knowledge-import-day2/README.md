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

## 4. Day3 检索问句快测（5 条）

原始样本：

1. `day3-q1.run-accepted.json` / `day3-q1.run-events.json` / `day3-q1.run-finished.json`
2. `day3-q2.run-accepted.json` / `day3-q2.run-events.json` / `day3-q2.run-finished.json`
3. `day3-q3.run-accepted.json` / `day3-q3.run-events.json` / `day3-q3.run-finished.json`
4. `day3-q4.run-accepted.json` / `day3-q4.run-events.json` / `day3-q4.run-finished.json`
5. `day3-q5.run-accepted.json` / `day3-q5.run-events.json` / `day3-q5.run-finished.json`

判定文件：

1. 自动判定：`day3-retrieval-results-20260410.json`、`day3-retrieval-table-20260410.md`
2. 人工复核：`day3-retrieval-reviewed-20260410.json`、`day3-retrieval-reviewed-20260410.md`

复核结论：

1. 命中：2 条（q3、q5）。
2. 部分命中：3 条（q1、q2、q4）。
3. 未命中：0 条。
4. 残余风险：检索链路可达，但中文摘要存在乱码/截断，影响 token 匹配与可读性。

## 5. Day4 真实问句回答可用性快测（5 条）

原始样本：

1. `day4-q1.run-accepted.json` / `day4-q1.run-events.json` / `day4-q1.run-finished.json`
2. `day4-q2.run-accepted.json` / `day4-q2.run-events.json` / `day4-q2.run-finished.json`
3. `day4-q3.run-accepted.json` / `day4-q3.run-events.json` / `day4-q3.run-finished.json`
4. `day4-q4.run-accepted.json` / `day4-q4.run-events.json` / `day4-q4.run-finished.json`
5. `day4-q5.run-accepted.json` / `day4-q5.run-events.json` / `day4-q5.run-finished.json`

判定文件：

1. `day4-answer-quality-results-20260410.json`
2. `day4-answer-quality-table-20260410.md`

复核结论：

1. 可用：3 条（q1、q2、q4）。
2. 不可用：2 条（q3、q5）。
3. 不可用主因：`agent_resolve` 路径在英文自然问句下出现最大执行轮次失败，返回系统失败日志而不是最终可用回答。

## 6. Day5 学习记录回写与追溯检查

写回样本：

1. `day5-write.run-accepted.json` / `day5-write.run-events.json` / `day5-write.run-finished.json`
2. `day5-readback.run-accepted.json` / `day5-readback.run-events.json` / `day5-readback.run-finished.json`

汇总文件：

1. `day5-learning-writeback-summary-20260410.json`
2. `day5-learning-records.preview.md`

复核结论：

1. 写回 run：`run-1775782519902-468`，状态 `completed + answer + verified`。
2. 读回 run：`run-1775782536288-471`，状态 `completed + answer + verified`。
3. 目标记录文件：`tmp/knowledge-import-day2/20260410/day5-learning-records.md`。
4. 追溯检查通过：记录内已保留 3 条来源 `source_run_id`（q1/q2/q4）。

## 7. Day6 端到端演练（导入 -> 摘要 -> 回写）

样本文件：

1. `day6-source-read-failed.run-*.json`（中文路径读取失败样本）
2. `day6-source-read.run-*.json`（ASCII 路径重试成功样本）
3. `day6-summary-write.run-*.json`
4. `day6-record-write.run-*.json`
5. `day6-record-readback.run-*.json`

汇总文件：

1. `day6-e2e-summary-20260410.json`
2. `day6-e2e-evaluation-20260410.json`
3. `day6-summary.preview.md`
4. `day6-learning-record.preview.md`

复核结论：

1. 端到端主链路已跑通（读取 -> 写摘要 -> 写记录 -> 读回）。
2. 暴露瓶颈：中文路径在显式 `read:` 指令链路里易被编码成问号，触发 `os error 123`。
3. 最小修补建议：仅补“显式文件路径 UTF-8 归一化 + 非法字符拒绝提示”，不扩接口、不改共享合同、不改主循环。
