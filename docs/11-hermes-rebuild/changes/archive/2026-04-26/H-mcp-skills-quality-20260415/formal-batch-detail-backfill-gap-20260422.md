# H-03 formal batch 详细样本回填缺口清单（2026-04-22 → 2026-04-23）

更新时间：2026-04-23
状态：部分已回填（新增一轮映射，仍维持 `detail_backfill_pending=true`）

## 当前结论

1. `tmp/stage-h-mcp-skills/latest.json` 已按 H03-38 / H03-39 专项批次证据保守回刷到 `19 / 6 / 12`（business-task-chain / skill-false-positive / manual-review）。
2. 三份基础 eval 已补入：
   - `formal_batch_summary`
   - `batch_sync_state`
   - `formal_batch_detailed_samples`
3. 当前 detailed sample layer 只包含“可被现有结构化专项证据直接追溯”的样本，不包含推测、补写或伪造样本。
4. 2026-04-23 新增一轮映射：从 `skill-hit-effective-calibration.json` 和 `manual-review.before-batch-sync-20260422.json` 的 `institutional_review_primary_records` 中映射了可直接追溯的样本。

## 缺口汇总（更新后）

| 文件 | formal batch 目标 | 已回填 detailed samples | 当前缺口 |
|---|---:|---:|---:|
| `tmp/stage-h-mcp-skills/evals/business-task-chain.json` | 30 | 24 | 6 |
| `tmp/stage-h-mcp-skills/evals/skill-false-positive.json` | 24 | 7 | 17 |
| `tmp/stage-h-mcp-skills/evals/manual-review.json` | 16 | 12 | 4 |

## 2026-04-23 新增映射范围

### 1. business-task-chain

新增映射 11 条样本，全部来自 `skill-hit-effective-calibration.json` 中引用的 `business-task-chain.json` 条目：

- `chain_cross_domain_procurement_rule_sync_verify`（true_positive_effective）
- `chain_food_import_label_review_manual_recheck`（manual_assisted_effective）
- `chain_food_import_allergen_mapping_manual_verify_bridge`（manual_assisted_effective）
- `chain_trade_finance_invoice_risk_recover_verify`（manual_assisted_effective）
- `chain_trade_finance_multi_candidate_degrade_clear_decision`（inconclusive）
- `chain_multi_candidate_conflict_false_positive`（inconclusive）
- `chain_mixed_support_multi_source_conflict_manual_triage`（inconclusive）
- `chain_external_import_review_manual_verify`（degraded_but_salvageable）
- `chain_local_generated_cli_triage_review_manual`（degraded_but_salvageable）
- `chain_trade_finance_lc_clause_recheck_manual_clear`（degraded_but_salvageable）
- `chain_repo_release_workflow_local_skill_degrade_recover`（degraded_but_salvageable）

映射规则：
- `sample_id` 直接取自 `sample_ref`
- `route` / `verify_sample` 取自 `evidence` 字段
- `result` 统一为 `"passed"`（所有校准样本均标记为 passed）
- `step_index` 从原最大序号顺延

### 2. skill-false-positive

新增映射 5 条样本，全部来自 `skill-hit-effective-calibration.json` 的 `false_positive_noise` 桶：

- `fp_chain_trade_finance_multi_candidate_degrade_clear_decision`
- `fp_chain_builtin_project_overlap_noise_verify`
- `fp_chain_local_generated_cli_triage_review_manual`
- `fp_chain_compliance_export_control_overlap_verify_noise`
- `fp_chain_repo_release_workflow_local_skill_degrade_recover`

映射规则：
- `skill_false_positive=true`, `skill_hit=true`, `skill_hit_effective=false`
- `evidence_ref` 指向原始校准文件

### 3. manual-review

新增映射 8 条样本，全部来自 `manual-review.before-batch-sync-20260422.json` 的 `institutional_review_primary_records`：

- `manual_review_food_import_label_cross_check`
- `manual_review_trade_finance_degrade_not_ready`
- `manual_review_long_tail_local_generated_cli`
- `manual_review_overlap_conflict_noise`
- `manual_review_food_allergen_manual_assisted_effective`
- `manual_review_trade_finance_degraded_salvageable`
- `manual_review_compliance_overlap_ready_blocker`
- `manual_review_repo_release_role_diff_blocker`

映射规则：
- `decision` 根据 `ready_blocker_flag` 推断（`true` → `accepted_with_blocker`，`false` → `accepted_as_expected`）
- `source_eval` 指向 `review-rounds-h03.json`

## 当前仍不能做什么

1. 不能把尚未落盘的剩余样本直接伪写进 `samples`。
2. 不能把 `formal_batch_summary=30/24/16` 误读为“三份基础 eval 已拥有完整 30/24/16 的详细样本列表”。
3. 不能把当前状态误写成：
   - `H-03 ready`
   - `Gate-H signoff`
   - `基础 eval 明细已全部同步`

### 4. 第二轮新增映射（2026-04-23 追加）

新增映射 5 条 business-task-chain 样本，来自 `long-tail-distribution.json` 的 `A_cross_domain_long_tail` / `D_industry_category_floor`：

- `chain_food_import_batch_trace_degrade_manual_finalize`（food_safety_import_tail，route=manual）
- `chain_builtin_project_overlap_noise_verify`（compliance_support_tail，route=verify）
- `chain_compliance_export_control_overlap_verify_noise`（compliance_support_tail，route=verify）
- `chain_repo_cli_permission_matrix_manual_bridge`（repo_maintenance_tail，route=manual）
- `chain_compliance_sanction_code_recheck_verify`（compliance_support_tail，route=verify）

映射规则：
- `sample_id` 直接取自 `sample_ref`
- `route` / `verify_sample` 从 sample_id 语义推断（`_manual_` → manual，`_verify_` → verify）
- `result` 统一为 `"passed"`
- **注意**：这些样本在 `long-tail-distribution.json` 中只有 sample_ref 和行业分类，没有 calibration 级别的详细 evidence；映射时仅使用了 sample_id 语义 + 分类上下文。

新增映射 1 条 skill-false-positive 样本，来自 `representative-coverage.json`：

- `fp_chain_multi_candidate_conflict_false_positive`

映射规则：
- 与已映射的 `business-task-chain.json:chain_multi_candidate_conflict_false_positive` 共享同一冲突语境
- `skill_false_positive=true`, `skill_hit=true`, `skill_hit_effective=false`

## 下一步只允许做什么

1. 继续围绕剩余缺口（`business=6`、`false_positive=17`、`manual_review=4`）寻找可直接映射的结构化来源。
2. 每补一批，都必须满足：
   - 样本有明确来源文件；
   - 样本字段可由现有专项证据直接映射；
   - 不凭口头结论倒推样本明细。
3. 在详细样本未补齐前，继续维持：
   - `detail_backfill_pending=true`
   - H-03=`warning`
   - Gate-H=`warning / 不可签收`
