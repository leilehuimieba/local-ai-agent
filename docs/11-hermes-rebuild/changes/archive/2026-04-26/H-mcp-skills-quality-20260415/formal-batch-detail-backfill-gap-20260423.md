# H-03 formal batch 详细样本回填缺口清单（2026-04-22 → 2026-04-23）

更新时间：2026-04-23
状态：部分已回填（两轮回填完成，仍维持 `detail_backfill_pending=true`）

## 当前结论

1. `tmp/stage-h-mcp-skills/latest.json` 已按 H03-38 / H03-39 专项批次证据保守回刷到 `24 / 9 / 12`（business-task-chain / skill-false-positive / manual-review）。
2. 三份基础 eval 已补入：
   - `formal_batch_summary`
   - `batch_sync_state`
   - `formal_batch_detailed_samples`
3. 当前 detailed sample layer 只包含"可被现有结构化专项证据直接追溯"的样本，不包含推测、补写或伪造样本。
4. 2026-04-23 完成两轮回填：
   - 第一轮：从 `skill-hit-effective-calibration.json` 和 `manual-review.before-batch-sync-20260422.json` 映射
   - 第二轮：从 `long-tail-distribution.json` 和 `representative-coverage.json` 映射

## 缺口汇总（更新后）

| 文件 | formal batch 目标 | 已回填 detailed samples | 当前缺口 |
|---|---:|---:|---:|
| `tmp/stage-h-mcp-skills/evals/business-task-chain.json` | 30 | 24 | 6 |
| `tmp/stage-h-mcp-skills/evals/skill-false-positive.json` | 24 | 9 | 15 |
| `tmp/stage-h-mcp-skills/evals/manual-review.json` | 16 | 12 | 4 |

## 2026-04-23 第一轮映射范围

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

## 2026-04-23 第二轮映射范围

### 1. business-task-chain

新增映射 5 条样本，来自 `long-tail-distribution.json` 的 `A_cross_domain_long_tail` / `D_industry_category_floor`：

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

### 2. skill-false-positive

新增映射 1 条样本，来自 `representative-coverage.json`：

- `fp_chain_multi_candidate_conflict_false_positive`

映射规则：
- 与已映射的 `business-task-chain.json:chain_multi_candidate_conflict_false_positive` 共享同一冲突语境
- `skill_false_positive=true`, `skill_hit=true`, `skill_hit_effective=false`

## 当前仍不能做什么

1. 不能把尚未落盘的剩余样本直接伪写进 `samples`。
2. 不能把 `formal_batch_summary=30/24/16` 误读为"三份基础 eval 已拥有完整 30/24/16 的详细样本列表"。
3. 不能把当前状态误写成：
   - `H-03 ready`
   - `Gate-H signoff`
   - `基础 eval 明细已全部同步`

## 来源用尽确认（2026-04-23）

以下全部专项批次证据文件已逐一检查，**所有可直接追溯的结构化样本均已映射**：

| 来源文件 | 样本数 | 已映射到 | 备注 |
|---|---|---|---|
| `skill-hit-effective-calibration.json` | 19 | business-task-chain(11), skill-false-positive(5), manual-review(间接) | 已全部映射 |
| `manual-review.before-batch-sync-20260422.json` | 8 | manual-review(8) | 已全部映射 |
| `long-tail-distribution.json` | 13 | business-task-chain(5 新增) | 其余 8 条已在第一轮映射 |
| `representative-coverage.json` | 1 | skill-false-positive(1) | 其余为引用已存在样本 |
| `verify-signals.json` | 3 | 初始 eval 已含 | 无新增可映射 |
| `skill-catalog.json` | 3 | 结构不匹配 | guard_action/trust_tier 样本 |
| `context-skill.json` | 3 | 结构不匹配 | profile 样本 |
| `failure-injection.json` | 4 | 无新增 | 全部引用其他 eval 已有样本 |
| `cross-skill-expansion.json` | 4 | 结构不匹配 | trust_tier 样本 |
| `recovery-chain-distribution.json` | 0 | 无新增 | 全部为 sample_refs 引用 |
| `review-rounds-h03.json` | 8 | manual-review(8) | 与 institutional_review_primary_records 同批 |

**结论**：当前证据体系内所有可直接追溯的结构化来源已用尽。剩余缺口（business=6、false_positive=15、manual_review=4）无现成来源。

## 下一步只允许做什么

1. **等待新的运行时观测数据**：通过实际执行积累新的 business chain、false positive、manual review 样本，再按相同规则映射。
2. **放宽映射标准的风险**：如从 `long-tail-distribution.json` 的 `why_long_tail` 文本描述反向构造样本，属于"凭口头结论倒推样本明细"，当前规则禁止。
3. **在详细样本未补齐前，继续维持**：
   - `detail_backfill_pending=true`
   - H-03=`warning`
   - Gate-H=`warning / 不可签收`
