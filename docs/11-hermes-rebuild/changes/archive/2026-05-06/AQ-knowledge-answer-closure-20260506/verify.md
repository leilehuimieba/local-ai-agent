# 验证方案

## 文档级验收

本 change 第一阶段先以“失败模式与实现边界是否收敛清楚”为验收口径。

必须满足：

1. 已明确 `AP` 首轮失败模式。
2. 已明确只修知识问答主链收口，不扩检索、不扩 verify 矩阵。
3. 已明确代码落点与复跑入口。

## 实现后验证要求

### 预检

- `cargo test -p runtime-core verify -- --nocapture`
- 视改动补：
  - `cargo test -p runtime-core planner -- --nocapture`
  - `cargo test -p runtime-core query_engine -- --nocapture`
  - `cargo test -p runtime-core knowledge -- --nocapture`

### 真实入口回归

- `powershell -NoProfile -File D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`

## 通过标准

至少满足：

1. `KA-01 ~ KA-04` 不再输出“本地知识检索结果”列表作为最终收口。
2. `verification_task_type = knowledge_answer`
3. `verification_has_citation = true`
4. 回答中可见 `事实 / 推断 / 建议`
5. `KA-05` 仍保留低证据收口能力。

## 证据位置

- 回归报告：`D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`
- 固定问题集：`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/fixtures/knowledge-answer-cases.json`
- 失败基线：同一 `latest.json` 中 2026-05-06 首轮结果

## 本轮验证结果

### 代码级验证

- `cargo test -p runtime-core query_engine -- --nocapture`
  - 结果：通过
  - 新增闭环测试：
    - `search_knowledge_replans_to_project_answer_for_agent_questions`
    - `search_knowledge_stays_put_for_non_agent_questions`
- `cargo test -p runtime-core verify -- --nocapture`
  - 结果：通过
  - 说明：知识回答 verify 仍保持 `citation + 事实/推断/建议边界` 的原有门槛，没有因本刀放松。

### 真实入口回归

- 执行命令：
  - `powershell -NoProfile -File D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`
- 最新报告：
  - `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`
- 最新 run：
  - `knowledge-answer-eval-20260506-132409`

### 结果摘录

- 汇总结果：
  - `total_cases = 5`
  - `passed_cases = 5`
  - `failed_cases = 0`
- `KA-01 ~ KA-04`
  - 已全部变为 `verification_task_type = knowledge_answer`
  - 已全部变为 `verification_has_citation = true`
  - 已全部出现 `事实 / 推断 / 建议`
  - `final_answer` 不再是“本地知识检索结果”列表态
- `KA-05`
  - 仍保留 `replan_requested = true`
  - 未被错误提升为知识回答强收口

### 本刀实现证据

- `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
  - 已把 `SearchKnowledge` 成功后的最小 follow-up 收紧到 agent 工程问答场景
  - replan 时已重建执行态上下文，保证回答阶段拿到 `project_answer` prompt profile
- `D:/newwork/本地智能体/crates/runtime-core/src/run_state_builder.rs`
  - 已暴露执行态 context rebuild 入口，供主循环 replan 使用
- `D:/newwork/本地智能体/crates/runtime-core/src/executors/project.rs`
  - 已补本地稳定的 agent 工程回答模板，输出 citation-ready 摘要与最终回答
