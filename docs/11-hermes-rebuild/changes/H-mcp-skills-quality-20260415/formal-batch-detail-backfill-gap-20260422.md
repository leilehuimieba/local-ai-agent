# H-03 formal batch 详细样本回填缺口清单（2026-04-22）

更新时间：2026-04-22  
状态：部分已回填（仅补可直接追溯的 detailed sample layer）

## 当前结论

1. `tmp/stage-h-mcp-skills/latest.json` 已按 H03-38 / H03-39 专项批次证据保守回刷到 `30 / 24 / 16`。
2. 三份基础 eval 已补入：
   - `formal_batch_summary`
   - `batch_sync_state`
   - `formal_batch_detailed_samples`
3. 当前 detailed sample layer 只包含“可被现有结构化专项证据直接追溯”的样本，不包含推测、补写或伪造样本。

## 缺口汇总

| 文件 | formal batch 目标 | 已回填 detailed samples | 当前缺口 |
|---|---:|---:|---:|
| `tmp/stage-h-mcp-skills/evals/business-task-chain.json` | 30 | 17 | 13 |
| `tmp/stage-h-mcp-skills/evals/skill-false-positive.json` | 24 | 5 | 19 |
| `tmp/stage-h-mcp-skills/evals/manual-review.json` | 16 | 8 | 8 |

## 当前已回填范围

### 1. business-task-chain

已回填 17 条可直接追溯样本，主要来自：

- `skill-hit-effective-calibration.json`
- `long-tail-distribution.json`
- `recovery-chain-distribution.json`

当前已覆盖的结构化维度包括：

- `calibration_bucket`
- `why_bucket`
- `domain`
- `long_tail`
- `recovery_chain_sample`
- `three_plus_stage_recovery`

### 2. skill-false-positive

已回填 5 条可直接追溯样本，当前全部来自：

- `skill-hit-effective-calibration.json.false_positive_noise`

当前仍缺少其余 19 条正式批次误命中样本的结构化明细落点。

### 3. manual-review

已回填 8 条可直接追溯样本，主要来自：

- `review-rounds-h03.json`
- `manual-review.json.institutional_review_primary_records`
- `h03-institutional-review-check.json`

当前已形成：

- 主记录 / 主台账最小闭环
- 双轮 / 角色差异复核样本的稳定明细层

但距离完整 16 条正式批次人工复核详细样本仍差 8 条。

补充核对结论（2026-04-22）：

1. 当前 `review-rounds-h03.json` 只存在 8 条可直接回指到 `manual-review.json.institutional_review_primary_records` 的结构化样本。
2. `manual-review.json.formal_batch_detailed_samples` 与 `indexed_formal_batch_sample_ids` 当前也只落了这 8 条。
3. `representative-coverage.json` 中曾引用的以下 4 条旧 manual 样本：
   - `manual_review_cross_domain_external_imported`
   - `manual_review_long_chain_review_manual_verify`
   - `manual_review_high_conflict_multi_candidate`
   - `manual_review_cross_domain_trust_conflict`
   当前仅能作为较早代表性覆盖口径中的引用，尚未在现有 formal batch detailed sample layer 中形成可直接回指的结构化落点。
4. 因此，`manual_review=16` 当前只能稳定表述为“formal batch summary 已成立”，不能默认推导为“剩余 8 条都已存在现成可回填明细”。
5. 在找到新的结构化来源前，`manual-review` 应继续维持“已回填 8 条、剩余 8 条来源待确认”的口径。
6. 已继续核对 `update_task13.py` 与 `h03-institutional-review-check.json`：当前未发现除现有 8 条之外的新增结构化 manual-review 明细来源。
7. 因此，当前更准确的边界应是：剩余 8 条不是“待抄录”，而是“当前未发现更多可直接回填的结构化来源”。

## 当前不能做什么

1. 不能把尚未落盘的剩余样本直接伪写进 `samples`。
2. 不能把 `formal_batch_summary=30/24/16` 误读为“三份基础 eval 已拥有完整 30/24/16 的详细样本列表”。
3. 不能把当前状态误写成：
   - `H-03 ready`
   - `Gate-H signoff`
   - `基础 eval 明细已全部同步`

## 下一步只允许做什么

1. 继续围绕这三份基础 eval 补“剩余 detailed sample 明细”。
2. 每补一批，都必须满足：
   - 样本有明确来源文件；
   - 样本字段可由现有专项证据直接映射；
   - 不凭口头结论倒推样本明细。
3. 在详细样本未补齐前，继续维持：
   - `detail_backfill_pending=true`
   - H-03=`warning`
   - Gate-H=`warning / 不可签收`

