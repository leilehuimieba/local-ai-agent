# 变更提案

## 背景

- `AK-knowledge-pack-minimal-20260505` 已把知识检索增强、knowledge pack 与 ask / learn 最小注入打通。
- `AL-verify-matrix-minimal-20260505` 已把知识回答类 verify、citation / evidence / 边界状态与最小 replan 接回主循环。
- `AP-knowledge-answer-eval-pack-20260506` 首轮真实入口回归已证明：当前问题不在回归资产，而在主链行为本身。
- 真实入口下，正常知识问答仍然收口为“本地知识检索结果”列表，`verification_task_type = generic`，没有进入 `knowledge_answer + citation-ready + 事实/推断/建议` 的回答态。

## 目标

- 产出一个只覆盖“知识问答从 SearchKnowledge 列表态收口到 citation-ready knowledge_answer 回答态”的独立实现 change。
- 只修知识问答主链在真实入口下的收口行为，不回 `AP` 扩评测资产，也不回 `AO` / `AJ` 混做记忆治理。
- 让 `AP` 的 `KA-01 ~ KA-04` 至少能够进入 `knowledge_answer` 任务分型，并具备最小 citation-ready 输出。

## 非目标

- 不在本 change 中扩新的知识源、重做检索引擎或替换存储结构。
- 不在本 change 中扩 verify 矩阵新类别。
- 不在本 change 中混做 memory 写回治理、浏览器恢复治理或 UI 面板。
- 不把本 change 扩成“回答质量全面优化”，只收口知识问答主链的回答态。

## 验收口径

- 正常知识问答不再收口为“本地知识检索结果”列表。
- `AP` 首轮失败的 `KA-01 ~ KA-04` 至少满足：
  - `verification_task_type = knowledge_answer`
  - `verification_has_citation = true`
  - 回答中能看出 `事实 / 推断 / 建议` 边界
- `KA-05` 的低证据收口路径不被破坏。
