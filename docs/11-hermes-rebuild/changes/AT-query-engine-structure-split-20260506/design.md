# 技术方案

## 影响范围

- 当前目标文件：
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
- 预计后续拆分后涉及：
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/bootstrap.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/replan.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/browser_followup.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/knowledge_followup.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/path_followup.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/tests/`
- 可能需要最小接线的调用方：
  - `D:/newwork/本地智能体/crates/runtime-core/src/lib.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/run_runtime_state.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/run_state_builder.rs`

## 方案

### 1. 拆分原则

- 保持对外入口稳定：
  - `RuntimeEnvelope`
  - `RuntimeRunState`
  - `bootstrap_run`
  - `execute_stage`
  - `should_replan`
  - `replan_state`
  - `budget_exhausted`
- 优先做“职责搬移 + 文件瘦身”，不借机修改行为。
- 每次拆分以定向测试继续通过为前提。

### 2. 建议模块边界

1. `query_engine/mod.rs`
   - 保留公共类型与对外稳定入口。
   - 负责协调 bootstrap / replan / follow-up 逻辑。
2. `query_engine/bootstrap.rs`
   - 放 `bootstrap_run`
   - 放 runtime envelope / state 组装相关辅助
3. `query_engine/replan.rs`
   - 放 `should_replan`
   - 放 `replan_state`
   - 放 `budget_exhausted`
   - 放 `remaining_steps_for_action`
4. `query_engine/browser_followup.rs`
   - 放 browser verify failure 后的 `read_page` 补动作逻辑
   - 放 `browser_page_id` / `json_field` 等辅助
5. `query_engine/knowledge_followup.rs`
   - 放 knowledge verify failure 与 `SearchKnowledge -> ProjectAnswer` 最小收口
6. `query_engine/path_followup.rs`
   - 放 `next_read_action`
   - 放 `next_list_action`
   - 放 `extract_candidate_path`
   - 放 `clean_path`
   - 放 `join_base_path`
7. `query_engine/tests/`
   - 至少拆成 `resume`、`browser_followup`、`knowledge_followup` 三组

### 3. 当前冻结的函数映射

1. `bootstrap / execute`
   - `bootstrap_run`
   - `execute_stage`
   - `update_verification_report`
2. `replan core`
   - `should_replan`
   - `replan_state`
   - `budget_exhausted`
   - `next_action_from_trace`
   - `remaining_steps_for_action`
3. `path follow-up`
   - `next_read_action`
   - `next_list_action`
   - `extract_candidate_path`
   - `clean_path`
   - `join_base_path`
4. `knowledge follow-up`
   - `search_knowledge_followup_action`
   - `agent_knowledge_question`
   - `knowledge_answer_verify_failed`
5. `browser follow-up`
   - `browser_interaction_verify_failed`
   - `browser_interaction_needs_readback`
   - `browser_read_page_action`
   - `browser_page_id_from_action`
   - `browser_page_id_from_trace`
   - `json_field`
   - `is_browser_read_page`

### 4. 实施顺序

1. 先完成本 change 建档与状态切换。
2. 再对 `query_engine.rs` 做模块骨架确认与函数映射冻结。
3. 后续实现时优先抽：
   - `browser_followup.rs`
   - `knowledge_followup.rs`
   - `path_followup.rs`
4. 再抽：
   - `replan.rs`
   - `bootstrap.rs`
5. 最后迁测试并补最小接线修正。

### 5. 风险与回退

- 主要风险：`query_engine.rs` 与 `lib.rs`、`run_*` 模块耦合较紧，拆分时容易出现可见性与导入回归。
- 主要风险：replan 逻辑搬移后，browser / knowledge follow-up 的优先级次序被误改。
- 主要风险：测试迁移时夹具重复膨胀，再次踩到函数长度红线。
- 回退方式：
  - 优先按模块搬移粒度回退；
  - 不回退 `AS` 已完成的结构治理；
  - 如出现行为回归，先恢复单文件版本再按更小步拆。
