# Day3 检索问句复核表（2026-04-10）

| 问句ID | 检索问句 | run_id | 工具 | 复核判定 |
|---|---|---|---|---|
| q1 | `read: tmp/knowledge-import-day2/20260410/import-a-summary.md` | `run-1775782084822-438` | `workspace_read` | 部分命中 |
| q2 | `read: tmp/knowledge-import-day2/20260410/import-b-summary.md` | `run-1775782086808-441` | `workspace_read` | 部分命中 |
| q3 | `list: tmp/knowledge-import-day2/20260410` | `run-1775782088799-444` | `workspace_list` | 命中 |
| q4 | `read: docs/07-test/evidence/20260410-knowledge-import-day2/day2-import-visibility-diff.json` | `run-1775782090752-447` | `workspace_read` | 部分命中 |
| q5 | `read: docs/07-test/evidence/20260410-knowledge-import-day2/day2-import-run-summary.json` | `run-1775782092721-450` | `workspace_read` | 命中 |

复核结论：

1. 命中：2 条（q3、q5）。
2. 部分命中：3 条（q1、q2、q4）。
3. 未命中：0 条。
4. 当前主要问题不是“读不到文件”，而是“中文摘要在展示层出现乱码/截断”，导致 token 匹配偏低。
