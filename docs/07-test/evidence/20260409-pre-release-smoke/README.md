# 上线前 30 分钟小回归包

执行时间：`2026-04-09 21:21`  
执行范围：学习问句混合路由、provider 波动场景、页面分层抽检（结果分层信号）

## 回归结果

1. 学习问句混合路由：通过
2. provider 波动场景：通过
3. 页面分层抽检：通过

详单见：`regression-summary.json`
复跑记录见：`recheck-20260409-2124.md`
5 条真实问句质量快测见：`five-real-questions-quality-quickcheck-20260409.md`
5 条真实问句质量汇总：`five-real-questions-quality-summary-20260409.json`
5 条真实问句修补后复跑：`five-real-questions-quality-quickcheck-20260409-r2.md`
5 条真实问句前后对比：`five-real-questions-quality-compare-20260409.md`
5 条真实问句修补后汇总：`five-real-questions-quality-summary-20260409-r2.json`

## 1) 学习问句混合路由

### Case A：学习问题 + 状态词混合

1. `run_id`: `run-1775740287676-72`
2. 输入：`继续复习 Rust 所有权和借用。我现在掌握到哪了，还差什么，下一步做什么？请用学习建议回答。`
3. 期望：走 `session_context`
4. 实际：`plan_tool=session_context`，`finish_tool=session_context`，`result_mode=answer`
5. 结论：通过

### Case B：纯项目状态问句

1. `run_id`: `run-1775740313939-75`
2. 输入：`这个项目现在做到什么程度了？`
3. 期望：走 `project_answer`
4. 实际：`plan_tool=project_answer`，`finish_tool=project_answer`，`result_mode=recovery`
5. 结论：通过

## 2) provider 波动场景

### Case C：本地 provider 不可达回退

1. `run_id`: `run-1775740821016-81`
2. 配置：`provider_id=local-llama`, `model_id=Qwen3.5-9B`
3. 期望：主回答失败后走受控恢复，不崩溃
4. 实际：`verification_code=verified_with_recovery`，`result_mode=recovery`，`completion_status=completed`
5. 关键证据：`model_transport_failed: curl: (7) Failed to connect...`
6. 结论：通过

## 3) 页面分层抽检（结果信号）

### Case D：system 分层信号

1. `run_id`: `run-1775740796749-78`
2. 输入：`帮我打开计算器`（`mode=observe`）
3. 期望：被模式阻止并标记为 `system`
4. 实际：`verification_completed/run_failed/run_finished` 均带 `result_mode=system`
5. 结论：通过

### 三态覆盖检查

1. `answer`：`run-1775740287676-72`
2. `recovery`：`run-1775740313939-75`、`run-1775740821016-81`
3. `system`：`run-1775740796749-78`

前端消费点保持不变，仍是优先读取 `event.metadata.result_mode` 决定标签分层：

1. `frontend/src/chat/chatResultModel.ts` 的 `readAssistantMode` / `readExplicitResultMode`
2. `frontend/src/chat/chatResultModel.ts` 的 `readAssistantRoleLabel` / `readAssistantStatusTag` / `readAssistantSummaryLabel`

## 构建补验

1. `cargo build -p runtime-core`：通过
2. `cargo build -p runtime-host`：通过
3. `frontend npm run build`：通过

## 当前仍建议关注（非阻断）

1. provider 外部波动（429/网络不可达）仍会触发 `recovery`，但当前链路可稳定收口。
2. 学习建议内容质量仍有“可进一步更短更可执行”的优化空间。
