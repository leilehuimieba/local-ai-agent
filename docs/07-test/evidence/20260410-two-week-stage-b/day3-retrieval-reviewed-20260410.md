# Day3 检索问句人工复核（2026-04-10）

| qid | prompt | run_id | 手工判定 | 说明 |
|---|---|---|---|---|
| q1 | `read: tmp/knowledge-import-day2/20260410-two-week/import-a-summary.md` | `run-1775805765214-137` | 命中 | 命中“导入样本 A + docs/README.md”。 |
| q2 | `read: tmp/knowledge-import-day2/20260410-two-week/import-b-summary.md` | `run-1775805767513-140` | 命中 | 命中“导入样本 B + 产品定义”。 |
| q3 | `list: tmp/knowledge-import-day2/20260410-two-week` | `run-1775805769768-143` | 命中 | 命中两个文件名。 |
| q4 | `read: docs/07-test/evidence/20260410-two-week-stage-b/day2-import-run-summary-20260410.json` | `run-1775805771895-146` | 部分命中 | 摘要命中 a-read，b-write 受摘要截断影响。 |
| q5 | `read: docs/07-test/evidence/20260410-two-week-stage-b/day2-import-visibility-diff-20260410.json` | `run-1775805774058-149` | 命中 | 命中 target_exists_after_write=true。 |

复核结论：

1. 命中：4 条（q1、q2、q3、q5）。
2. 部分命中：1 条（q4）。
3. 未命中：0 条。
4. 当前主要风险：长 JSON 文件在摘要输出时易截断，影响关键词完整覆盖。
