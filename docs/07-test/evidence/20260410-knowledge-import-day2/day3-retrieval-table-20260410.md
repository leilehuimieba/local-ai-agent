# Day3 检索可用性结果（2026-04-10）

| 问句ID | 检索问句 | run_id | 工具 | 结果 | 命中判定 |
|---|---|---|---|---|---|
| q1 | read: tmp/knowledge-import-day2/20260410/import-a-summary.md | run-1775782084822-438 | workspace_read | 0/2 | 未命中 |
| q2 | read: tmp/knowledge-import-day2/20260410/import-b-summary.md | run-1775782086808-441 | workspace_read | 0/2 | 未命中 |
| q3 | list: tmp/knowledge-import-day2/20260410 | run-1775782088799-444 | workspace_list | 2/2 | 命中 |
| q4 | read: docs/07-test/evidence/20260410-knowledge-import-day2/day2-import-visibility-diff.json | run-1775782090752-447 | workspace_read | 0/2 | 未命中 |
| q5 | read: docs/07-test/evidence/20260410-knowledge-import-day2/day2-import-run-summary.json | run-1775782092721-450 | workspace_read | 2/2 | 命中 |

判定口径：命中=预期 token 全命中；部分命中=命中至少 1 个；未命中=0 命中。
