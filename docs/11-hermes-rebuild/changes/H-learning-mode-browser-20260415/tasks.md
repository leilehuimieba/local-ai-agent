# H-learning-mode-browser-20260415（tasks）

更新时间：2026-04-15
状态：草案

| ID | 任务 | 类型 | 状态 | 验收标准 | 证据 |
|---|---|---|---|---|---|
| H04-01 | 冻结学习模式协议 v1 | 设计 | todo | 接口字段固定并评审通过 | `design.md` |
| H04-02 | 页面抽取 MVP | 实现 | todo | 样本页解析成功率>=95% | `tmp/stage-h-learning/extract.json` |
| H04-03 | 解释/翻译卡片 | 实现 | todo | 输出可读且上下文相关 | `tmp/stage-h-learning/explain-translate.json` |
| H04-04 | 价值判断引擎 v1 | 实现 | todo | 输出 score/reason/next_action | `tmp/stage-h-learning/value-score.json` |
| H04-05 | 个性化学习建议 | 实现 | todo | 给出“该学什么/关注什么” | `tmp/stage-h-learning/recommend.json` |
| H04-06 | 记忆路由联动 | 实现 | todo | 写入/读取/注入策略生效 | `tmp/stage-h-learning/memory-routing.json` |
| H04-07 | 审计与回放 | 验证 | todo | trace_id 全链路可串联 | `tmp/stage-h-learning/audit-trace.json` |
| H04-08 | 回退开关验证 | 验证 | todo | 关闭学习模式可回退 | `tmp/stage-h-learning/rollback.json` |

## 执行顺序

1. 主链路：H04-01 -> H04-02 -> H04-03 -> H04-04 -> H04-05 -> H04-06 -> H04-07 -> H04-08
2. 可并行项：H04-03 与 H04-04 在 H04-02 后并行
3. 阻塞项：H04-01 未冻结前不开发接口
