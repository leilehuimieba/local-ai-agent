# 技术方案

## 影响范围

- 重点盘点与实现落点：
  - `D:/newwork/本地智能体/crates/runtime-core/src/planner.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/context_builder.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/knowledge.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`
- 关联验证入口：
  - `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`
  - `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`

## Harness 判断

- `AP` 已证明真实入口下的知识问答没有稳定进入回答态，因此现在最值得补的是“检索后如何收口成回答”，而不是继续扩检索、扩 verify、扩评测脚本。
- 本刀只修主链收口，不改评测包范围。

## 范围冻结

- 本刀只做三件事：
  - 让正常知识问答命中 `knowledge_answer` 而不是 `generic`
  - 让检索结果从“列表输出”升级为 citation-ready 的回答收口
  - 保持低证据问题仍能走 replan / handoff

## 当前失败模式

来自 `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json` 的已知事实：

1. `KA-01 ~ KA-04`
   - `terminal_event_type = run_finished`
   - `verification_task_type = generic`
   - `verification_has_citation = false`
2. `final_answer` 仍是“本地知识检索结果”列表，而不是回答态。
3. `knowledge_digest = 当前阶段未注入知识摘要`，说明真实入口下 ask/knowledge 装配与收口没有按预期命中。
4. `KA-05` 通过，是因为命中了 `replan_requested = true` 的低证据收口，而不是正常知识问答主链已达标。

## 最小修正方向

### 1. planner 层

- 盘点哪些用户输入会被判成：
  - `SearchKnowledge`
  - `ProjectAnswer`
  - `ContextAnswer`
- 最小目标：
  - 对定义型、关系型、工程判断型知识问题，不要在首轮直接永远停在 `SearchKnowledge`。
  - 应允许进入“先搜知识，再形成回答”的闭环。

### 2. query_engine 层

- 当前已存在：knowledge answer verify 失败时，会 replan 回 `SearchKnowledge`。
- 需要补的是另一半：
  - 当 `SearchKnowledge` 已拿到足够材料时，应推进到回答态，而不是直接把搜索结果列表当 final answer 收口。
- 最小目标：
  - `SearchKnowledge -> ProjectAnswer / ContextAnswer` 的收口条件显式化。

### 3. context_builder / knowledge 层

- 确认 ask profile 下真实入口是否稳定注入 `knowledge pack`。
- 最小目标：
  - 回答态使用 `knowledge pack` 的 citation 与 hints，而不是回退到裸检索片段列表。

### 4. verify 层

- 不扩 verify 新类别，只确保回答态命中后仍按 `knowledge_answer` 校验。
- 低证据问题仍允许：
  - `replan`
  - `handoff`
  - 或失败收口

## 风险与回退

- 风险：把所有知识检索都强行改成回答态，可能破坏低证据收口。
- 风险：回答态过早触发，反而丢掉必要的补充检索。
- 回退：
  - 保留当前 `SearchKnowledge` 作为 fallback；
  - 只在命中最小 evidence/citation 条件后进入回答态；
  - `KA-05` 必须持续保留低证据 replan 路径。
