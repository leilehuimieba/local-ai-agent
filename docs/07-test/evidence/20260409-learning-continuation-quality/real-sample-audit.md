# 学习续接真实样本审计

## 本轮边界

本轮做了两件事：

1. 继续保留并复核真实负样本，确保项目说明类记录不会误触发学习续接。
2. 通过模拟真实学习提问补一条真实正样本，并做“系统回答 vs 人工基准回答”效果对比。
3. 修补并复验学习续推路由，避免“学习问题带状态词”被误分到 `project_answer`。

本轮不改后端接口，不扩共享合同。

## 审计范围

1. `logs/run-logs.jsonl`
2. `data/daily/daily-rollup.jsonl`
3. 网关实时日志接口：`/api/v1/logs`
4. 新增真实运行样本：
   `run-1775739568970-69`
5. 新增路由复验样本：
   `run-1775740287676-72`、`run-1775740313939-75`

## 真实负样本（保留）

### 样本身份

1. `run_id`: `run-1775271798483-5`
2. `log_id`: `run-1775271798483-5-10-1775271798643`
3. `tool_name`: `project_answer`
4. `task_title`: `项目问答: 这个项目现在做到什么程度了`

### 判定

该记录虽然包含“长期学习”等词，但本质是项目说明，不是学习过程记录，仍应排除。

## 真实正样本（新增）

### 样本身份

1. `run_id`: `run-1775739568970-69`
2. `session_id`: `session-learning-positive-20260409d`
3. `tool_name`: `session_context`
4. `completion_status`: `completed`
5. `result_mode`: `answer`

### 样本提问

`继续复习 Rust 所有权和借用，请按已掌握、待巩固、建议先做三点给我学习建议。`

### 样本文件

1. `positive-sample-run-1775739568970-69.json`
2. `positive-signal-check-run-1775739568970-69.json`
3. `answer-quality-comparison.md`

### 关键观察

1. 真实 `run_finished` 的 `detail/final_answer` 已出现明确学习型三段结构：
   `已掌握 / 待巩固 / 建议先做`。
2. `tool_name` 为 `session_context`，不是 `project_answer`，不属于项目说明误判。
3. 在补入 `detail/final_answer` 作为学习证据来源后，该样本的学习信号判定为：
   `has_intent=true, has_evidence=true, learning_signal=true`。

## 回答效果对比结论

1. 系统真实回答已经达到“可用学习建议”标准，结构完整、方向基本正确。
2. 人工基准回答在“动作颗粒度”上更短更直接，执行门槛更低。
3. 当前最务实判断：学习型表达残余问题已从“无真实正样本”收敛到“可继续优化表达精炼度”。

## 路由风险复验结论

### 风险说明

历史风险是：当学习提问中出现“还差什么 / 下一步做什么”等状态词时，可能被 planner 误判为 `project_answer`。

### 修补点

1. `crates/runtime-core/src/planner.rs`
2. 新增学习续推识别分支，学习语义优先走 `ContextAnswer`。
3. 保留项目状态问题走 `ProjectAnswer` 的既有路径。

### 真实复验样本

1. 学习问题样本：
   `run-1775740287676-72`，路由到 `session_context`，`run_finished.result_mode = answer`。
2. 项目问题样本：
   `run-1775740313939-75`，仍路由到 `project_answer`，`run_finished.result_mode = recovery`。
3. 留证文件：
   `routing-check-learning-run-1775740287676-72.json`
   `routing-check-project-run-1775740313939-75.json`

## 本轮结论

1. 真实负样本继续成立，误判排除口径有效。
2. 真实正样本已补齐，不再是阻断项。
3. 学习续推路由误判风险已完成最小修补并通过真实样本复验。
4. 学习续接表达质量可以进入下一阶段持续优化，不需要再以“正样本缺失”阻断验收。
