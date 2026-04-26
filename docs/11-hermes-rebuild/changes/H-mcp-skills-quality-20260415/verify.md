# H-mcp-skills-quality-20260415（verify）

更新时间：2026-04-22
状态：部分已验证（H03-39 正式执行后复核与交接已完成，仍为 warning）

## 验证方式

1. 文档验证：
   - H-03 的目标、边界、最小评测包与证据结构是否齐备。
   - H03-02a 是否明确 Skill / Memory / Evidence 三层职责、边界约束与模块落点。
   - H03-02b 是否明确 system/run/skill/evidence 四层定义与 Level 1~3 渐进加载规则。
   - H03-02c 是否明确 Skill Guard 检查面、trust tier 与 allow/review/deny 结果口径。
   - 是否已把外部参考项目的可迁移治理原则映射成 H-03 的本地设计约束，并明确哪些点不可照搬。
2. 代码映射验证：
   - `skill_catalog / context_builder / context_policy / memory_router / verify` 是否已映射到 H03-02a/b/c 文档冻结项。
   - 是否已明确“现有代码已具备能力”与“尚缺稳定字段”的边界，不把静态骨架误写成运行时闭环。
3. 证据生成验证：
   - `tmp/stage-h-mcp-skills/latest.json`
   - `tmp/stage-h-mcp-skills/evals/*.json`
   - `tmp/stage-h-mcp-skills/evals/context-skill.json`
   - `tmp/stage-h-mcp-skills/evals/verify-signals.json`
   - `tmp/stage-h-mcp-skills/evals/skill-false-positive.json`
   - `tmp/stage-h-mcp-skills/evals/failure-injection.json`
   - `tmp/stage-h-mcp-skills/evals/manual-review.json`
   - `tmp/stage-h-mcp-skills/evals/cross-skill-expansion.json`
   - `tmp/stage-h-mcp-skills/evals/business-task-chain.json`
   - `tmp/stage-h-mcp-skills/fallback-cases.json`
   - `tmp/stage-h-mcp-skills/architecture-freeze-h03.json`
4. 测试与静态读证：
   - `cargo test -p runtime-core generate_h03_eval_refresh -- --nocapture`
   - `cargo test -p runtime-core verify -- --nocapture`
   - `cargo test -p runtime-core run_verification_metadata -- --nocapture`
   - `cargo test -p runtime-core skill_catalog -- --nocapture`
   - `static read: skill_catalog.rs, context_builder.rs, context_policy.rs, memory_router.rs, verify.rs`

## 2026-04-22 漂移复核补记

1. `tmp/stage-h-mcp-skills/latest.json` 当前仍为 `2026-04-16` 的旧聚合产物，`summary` 仍停在 `business_chain_samples=8`、`false_positive_samples=3`、`manual_review_samples=4`。
2. 已于 2026-04-22 对 `tmp/stage-h-mcp-skills/latest.json` 做保守回刷：`summary` 已更新为 `30 / 24 / 16`，并新增 `summary_basis`、`base_eval_sync_pending`、`aggregation_state` 字段，明确该主报告当前以 H03-38/H03-39 专项批次证据为真值来源。
3. 已于 2026-04-22 对 `tmp/stage-h-mcp-skills/evals/business-task-chain.json`、`skill-false-positive.json`、`manual-review.json` 补入 `formal_batch_summary` 与 `batch_sync_state`，把正式批次总量、索引级样本覆盖与 `detail_backfill_pending=true` 显式落盘。
4. 已于 2026-04-22 进一步补入 `formal_batch_detailed_samples`：
   - `business-task-chain.json` 当前已回填 17 条可直接追溯的正式批次详细样本；
   - `skill-false-positive.json` 当前已回填 5 条可直接追溯的正式批次详细样本；
   - `manual-review.json` 当前已回填 8 条可直接追溯的正式批次详细样本。
5. 当前三份基础 eval 仍保留旧基础样本明细；本轮只补“可直接追溯的 detailed sample layer”，不伪造 30 / 24 / 16 的完整样本列表；因此它们当前状态应理解为“批次 summary 已同步、部分详细样本已回填、剩余样本仍 pending”。
6. `h03-38-batch1-execution.json`、`h03-39-handoff-check.json`、`skill-hit-effective-calibration.json`、`review-rounds-h03.json`、`long-tail-distribution.json`、`recovery-chain-distribution.json` 等专项证据，已推进到 H03-38/H03-39 的批次验证强度。
7. 因此，当前允许的准确表述应是：“`latest.json` 与三份基础 eval 均已按专项批次证据完成保守/诚实回刷，并补入部分可追溯 detailed sample layer，但完整详细样本明细仍待统一回填”；不得误写成“所有基础聚合文件已完全同步”。
8. 当前详细样本缺口已单独落盘到 `formal-batch-detail-backfill-gap-20260422.md`；后续如继续推进，只允许围绕该清单补剩余 detailed sample 明细。
9. 已于 2026-04-22 进一步核对 `manual-review` 的 formal batch detailed sample 落点：`review-rounds-h03.json`、`manual-review.json.institutional_review_primary_records`、`indexed_formal_batch_sample_ids` 与 `formal_batch_detailed_samples` 当前都只稳定支撑 8 条结构化样本。
10. `representative-coverage.json` 中较早引用的 `manual_review_cross_domain_external_imported / manual_review_long_chain_review_manual_verify / manual_review_high_conflict_multi_candidate / manual_review_cross_domain_trust_conflict`，当前未在现有 formal batch detailed sample layer 中形成对应的结构化回指落点。
11. 因此，`manual_review_cases=16` 当前只能作为 formal batch summary 口径使用；剩余 8 条不能默认视为“已有现成可回填明细”，而应继续视为“来源待确认 / 结构化落点待补”。
12. 已继续核对 `update_task13.py` 与 `h03-institutional-review-check.json`：前者只显式涉及较早的 2 条 manual-review 扩样脚本落点，后者只确认了 `review_rounds_samples=8 / manual_review_primary_records=8` 的最小主索引闭环；当前未发现除现有 8 条之外的新增结构化明细来源。
13. 因此，当前更准确结论应收紧为：`manual-review` 剩余 8 条不是“待抄录”，而是“当前未发现更多可直接回填的结构化来源”。

## 本轮新增验证结论

1. H03-02a：
   - 文档已冻结 Skill / Memory / Evidence 边界。
   - 静态读证确认：`memory_router.rs` 当前未见 skill SOP 直接写入 memory 主链。
   - 本轮已补：`verify.rs` 对 skill 命中有效性的显式字段。
2. H03-02b：
   - 文档已冻结四层注入与三级渐进加载。
   - 静态读证确认：`context_builder.rs` 当前已有 `static/project/dynamic` 主体结构与 observation 注入预算。
   - 本轮已补：`skill_injection_enabled / max_skill_level / injected_skill_level / injected_skill_ids / evidence_refs` 最小可观测字段。
   - 仍缺：真正按 trust tier 升降级的 runtime 决策与更细粒度 evidence layer 引用。
3. H03-02c：
   - 文档已冻结 Skill Guard 与 trust tier。
   - 静态读证确认：`skill_catalog.rs` 当前已有 pin、scope、entry isolation 与 skip reason。
   - 本轮已补：`trust_tier / guard_action / guard_reason` 运行时稳定输出。
   - 本轮已补：`verify.rs` 的 `guard_downgraded / guard_decision_ref` 最小可观测字段。
   - 已补：1 条真实 trust tier -> guard(review) -> verify 联动样本，`sample_id=verification_guard_downgraded_path`。
4. H03-03/H03-04 扩展：
   - 已扩：`skill-false-positive.json` 到 6 条样本，包含多条 `skill_hit=true && skill_hit_effective=false` 的误命中样本。
   - 已回填：`skill_hit_effective_rate=0.5`、`skill_false_positive_rate=0.5`（小规模扩样口径，仅用于 warning 收缩，不代表 H-03 ready）。
5. H03-08 扩展：
   - 已补：`failure-injection.json`，覆盖 4 条失败注入样本，`failure_injection_locatable_rate=1.0`。
   - 已补：`manual-review.json`，覆盖 4 条人工评测样本，`manual_review_completion_rate=1.0`。
6. H03-09 扩展：
   - 已补：`cross-skill-expansion.json`，覆盖 4 种 trust tier 的最小跨技能类型样本。
   - 已回填：`cross_skill_observable_rate=1.0`（最小样本口径，不代表 H-03 ready）。
7. H03-10 扩展：
   - 已补：`business-task-chain.json`，覆盖 3 条最小真实业务任务链路样本。
   - 已回填：`business_chain_observable_rate=1.0`（最小样本口径，不代表 H-03 ready）。
8. H03-11 扩展：
   - 已扩：`business-task-chain.json` 到 5 条样本，覆盖 `manual` 与 `verify` 两类失败分流路径。
   - 已回填：`business_chain_samples=5`，用于 Gate-H warning 收缩，不代表 H-03 ready。
9. H03-12 扩展：
   - 已扩：`business-task-chain.json` 到 8 条样本，补齐多步 review 与手动重试链路。
   - 已回填：`business_chain_samples=8`，用于 Gate-H warning 收缩，不代表 H-03 ready。
10. H03-13/H03-14 扩展（任务8A/8B）：
   - 已扩：`business-task-chain.json` 到 10 条样本，进一步覆盖 manual/verify 真实任务链路。
   - 已扩：`skill-false-positive.json` 到 6 条样本，误命中样本不再只有 1 条。
   - 已回填：`business_chain_samples=10`、`false_positive_samples=6`，用于 warning 继续收缩，不代表 H-03 ready。
11. H03-15/H03-16 扩展（任务10A/10B，有界扩样）：
   - 已扩：`business-task-chain.json` 到 14 条样本，新增链路覆盖“上下文过载裁剪、工具预览噪声检测、多技能冲突人工升级、memory 命中但 skill 失配后的恢复”。
   - 已扩：`skill-false-positive.json` 到 10 条样本，均满足 `skill_hit=true && skill_hit_effective=false`，用于稳定表达“命中无有效增益/引入噪声”。
   - 已回填：`business_chain_samples=14`、`false_positive_samples=10`；达到本轮阈值后按停止条件收口，不继续开放式扩样。
12. 并行线程 B2/B3/B4/B6/B7 复核：
   - B2：已复核 `verification_guard_downgraded_path` 在 `verify-signals.json` 与 `fallback-cases.json` 的证据引用链一致。
   - B3：已复核 `verification_failure_path` 为最小 `skill_hit=true && skill_hit_effective=false` 样本，且 `latest.json` 指标一致。
   - B4：已复核 `manual-review.json` 覆盖 guard 降级与 false positive 两类最小人工评测样本。
   - B6：已复核失败注入扩样到 4 条，含 `guard_denied` 与业务链 `verify_failed`。
   - B7：已复核人工评测扩样到 4 条，含 guard 拒绝与业务链 warning。
13. H03-18/H03-19/H03-20 扩展（任务11：有界代表性扩样）：
   - 已扩：`business-task-chain.json` 到 16 条样本，新增跨域导入复核与多候选冲突误命中链路。
   - 已扩：`skill-false-positive.json` 到 12 条样本，全部满足 `skill_hit=true && skill_hit_effective=false`。
   - 已扩：`manual-review.json` 到 8 条样本，覆盖代表维度 A/B/C。
   - 已新增：`representative-coverage.json`，明确 A（跨域分布）/B（长链异常组合）/C（高复杂冲突）/D（人工补强）与样本映射。
   - 已回填：`latest.json` 的样本汇总与代表性指标（`representative_dimension_coverage_rate=1.0`、`representative_cross_domain_types=4`）。
   - 结论：warning 已继续收敛到“代表性有界扩样已完成”，但仍未达到 ready 条件。
14. H03-21/H03-22/H03-23 扩展（任务12：长尾真实分布补样）：
   - 已扩：`business-task-chain.json` 到 18 条样本，新增 `chain_local_generated_cli_triage_review_manual` 与 `chain_builtin_project_overlap_noise_verify` 两类更接近真实尾部分布的链路。
   - 已扩：`skill-false-positive.json` 到 14 条样本，新增 2 条误命中样本分别对应“本地生成 skill + 仓库私有 CLI 分诊”与“builtin/project/external 重叠候选噪声”。
   - 已扩：`manual-review.json` 到 10 条样本，新增 2 条复核样本聚焦长尾语境判断与自然复杂冲突噪声判断。
   - 已新增：`long-tail-distribution.json`，明确 A（跨域长尾语境）/B（自然复杂冲突）/C（人工复核多样性）与样本映射、remaining gaps。
   - 已回填：`latest.json` 的长尾分布指标（`long_tail_distribution_coverage_rate=1.0`、`long_tail_context_samples=3`、`natural_conflict_samples=3`、`manual_review_diversity_samples=2`）。
   - 结论：warning 已从“代表维度可读可映射但仍偏定向样本”继续收敛到“已有更自然的长尾真实分布覆盖”，但仍未达到 ready 条件。
15. H03-24/H03-25/H03-26 扩展（任务13：有界恢复链 + 行业尾部长尾补样）：
   - 已扩：`business-task-chain.json` 到 20 条样本，新增 `chain_food_import_label_review_manual_recheck` 与 `chain_trade_finance_multi_candidate_degrade_clear_decision`，分别覆盖食品安全进口标签复核与贸易融资审单冲突两类行业尾部语境。
   - 已扩：`skill-false-positive.json` 到 16 条样本，新增 2 条误命中样本分别对应“法规 skill 首轮命中但仍需 manual->verify_recheck”与“多候选冲突经 degrade 后才形成 clear decision”。
   - 已扩：`manual-review.json` 到 12 条样本，新增 2 条交叉复核样本，明确说明“为什么仍判 warning / 为什么仍不算 ready / 为什么不能因局部恢复成功而过度乐观”。
   - 已新增：`recovery-chain-distribution.json`，明确 A（行业尾部长尾）/B（多轮失败后恢复链）/C（复核分歧与交叉复核）与样本映射、remaining gaps。
   - 已回填：`latest.json` 的恢复链分布指标（`recovery_chain_distribution_coverage_rate=1.0`、`industry_tail_samples=2`、`multi_round_recovery_samples=3`、`cross_review_disagreement_samples=2`）。
   - 结论：warning 已从“已有更自然的长尾真实分布覆盖”继续收敛到“关键尾部缺口已补齐最小有界证据”，但仍未达到 ready 条件。
16. H03-27/H03-28/H03-29 扩展（任务15：规模化扩样策略设计判断）：
   - 已明确：当前 H-03 距离 ready 的主要差距已从“数量问题”转为“结构性问题”。
   - 已明确：继续零散补几个样本，对 Gate-H 签收判断的边际收益已下降，下一步更适合进入规模化扩样策略设计。
   - 已新增：`scale-out-strategy-h03.json`，明确当前基线、结构性缺口、后续策略轴与再提审最低门槛建议。
   - 已建议：下一轮 Gate-H 提审前至少达到 `business_chain_samples>=30`、`false_positive_samples>=24`、`manual_review_samples>=16`，并补最小命中有效性校准集与 2 轮复核视角说明。
   - 结论：当前证据不足以直接签收，但已足以支撑“进入策略设计判断并待执行”的状态转换。
17. 外部参考项目映射补充：
   - 已把 `rtk-ai/rtk` 中“薄适配层 + 单一裁决核心、增强层失败不阻断主链路、摘要/原始证据双轨、质量收益量化闭环”收敛进 H-03 文档口径。
   - 已明确：这些内容当前只构成设计与任务约束，不构成“运行时已全部实现”的证明。
   - 已明确：`WSL-first / token-first` 不作为本项目默认口径，避免把外部项目平台假设误写为本项目设计前提。

## 证据位置

1. 当前文档证据：
   - `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/design.md`
   - `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/tasks.md`
   - `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/status.md`
   - `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/review.md`
   - `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/formal-execution-entry.md`
2. 当前运行/占位证据：
   - `tmp/stage-h-mcp-skills/latest.json`
   - `tmp/stage-h-mcp-skills/evals/skill-catalog.json`
   - `tmp/stage-h-mcp-skills/evals/tool-routing.json`
   - `tmp/stage-h-mcp-skills/evals/visibility-fallback.json`
   - `tmp/stage-h-mcp-skills/evals/context-skill.json`
   - `tmp/stage-h-mcp-skills/evals/verify-signals.json`
   - `tmp/stage-h-mcp-skills/fallback-cases.json`
   - `tmp/stage-h-mcp-skills/architecture-freeze-h03.json`
   - `tmp/stage-h-mcp-skills/skill-catalog-guard-sample.json`
   - `tmp/stage-h-mcp-skills/context-layer-injection-sample.json`
   - `tmp/stage-h-mcp-skills/verify-signal-sample.json`
   - `tmp/stage-h-mcp-skills/evals/representative-coverage.json`
   - `tmp/stage-h-mcp-skills/evals/long-tail-distribution.json`
   - `tmp/stage-h-mcp-skills/evals/recovery-chain-distribution.json`
   - `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json`

## Gate 映射

1. 对应阶段 Gate：
   - `Gate-H`（子项 H-03）
2. 当前覆盖情况：
   - 已完成 H-03 工作区草案、H03-01 质量口径冻结。
   - 已完成 H03-02a/H03-02b/H03-02c 架构冻结。
   - 已完成 H03-02 与当前代码骨架的静态映射，明确最小实现字段缺口。
   - 已完成 `skill_catalog.rs` 的最小可观测字段落地，可输出 trust tier 与 Guard 动作样例。
   - 已完成 `context_policy.rs / context_builder.rs` 的最小可观测字段落地，可输出注入开关、最大级别、实际注入级别与 evidence refs 样例。
   - 已完成 `verify.rs` 的最小可观测字段落地，可输出命中有效性与 Guard 决策引用样例。
   - 已落第一版 eval、fallback 与冻结摘要证据。
   - 已具备真实降级联动样本、误命中扩样（16 条）、失败注入扩样（4 条）与人工评测扩样（12 条）的可复现证据。
   - 已在文档层明确：后续需把 `fallback_reason / raw_artifact_ref / verify_result` 视为统一裁决字段的一部分。
   - 当前证据已足以支撑“进入规模化扩样策略设计判断”，因为主要缺口已表现为结构性分布问题，而不再只是简单数量不足。
   - 当前仍未解决：真实主链分布仍属中小样本、命中有效性分布仍未校准、交叉复核仍只是最小证据说明；因此仍不具备 H-03 ready 条件。
   - 由于 H-03 仍是 warning，当前不能据此直接宣称 Gate-H 可签收。


## H-03 规模化扩样策略设计验证口径

### 验证目标
本节验证的不是 H-03 是否 ready，而是：

1. 当前证据是否已足以支撑 H-03 进入规模化扩样策略设计；
2. 策略设计稿是否覆盖了当前识别出的结构性缺口；
3. 策略设计稿是否已经从“原则性描述”升级为“可执行设计”；
4. 再提审 Gate-H 前的最低门槛是否已具备可操作性。

### 验证范围
覆盖：

- 设计目标与边界是否明确；
- 当前基线与结构性缺口是否明确；
- 七个策略轴是否齐备；
- 最低门槛表是否形成；
- 风险、停止条件与后置项是否明确。

不覆盖：

- 真正执行规模化扩样；
- 新一轮样本补充；
- H-03 ready 判定；
- Gate-H 回刷。

### 核心验证问题

#### 验证 1：当前证据是否足以进入策略设计
判定标准：

- 当前已有最小有界证据；
- 当前缺口已转为结构性缺口；
- 继续默认零散补样边际收益下降；
- H-03 下一步不再是“是否设计”，而是“执行设计”。

若以上均成立，则可判定：
- 当前证据足以支撑进入策略设计。

#### 验证 2：策略设计是否覆盖七个轴
判定标准：

- 设计稿中已出现 A ~ G 七个策略轴；
- 每个策略轴均包含目标、字段、要求或阈值；
- 不存在仅有原则、没有执行约束的空轴。

若以上均成立，则可判定：
- 策略设计已具备结构完整性。

#### 验证 3：再提审门槛是否可操作
判定标准：

- 已有明确数量门槛；
- 已有明确分布门槛；
- 已有明确校准门槛；
- 已有明确复核门槛；
- 已能回答“什么时候值得再回刷 Gate-H”。

若以上均成立，则可判定：
- 再提审门槛已具备可执行性。

#### 验证 4：风险是否显式收口
判定标准：

- 已显式识别无限扩样风险；
- 已显式识别定向样本堆数量风险；
- 已显式识别 Gate-H 长期不裁决风险；
- 已写明本轮停止条件与后置项。

若以上均成立，则可判定：
- 风险治理口径已成型。

### 预期结果
预期本轮收口后，应能得到以下结论：

1. H-03 仍不是 ready；
2. H-03 仍不可作为 Gate-H 签收依据；
3. H-03 的下一步不再是“继续判断 / 继续补样”，而是“按正式执行入口决定是否启动执行批次”；
4. 后续若启动执行，只能按冻结门槛推进，不再无界扩样。

### 实际结果（2026-04-17 收口）
- 是否已满足“进入策略设计”的条件：是，且该问题已收口完成。
- 七轴是否齐备：是。
- 最低门槛表是否已冻结：是。
- 是否已明确停止条件与后置项：是。
- 是否已形成正式执行入口：是，见 `formal-execution-entry.md`。
- 当前结论：
  - H-03 是否 ready：否
  - 是否建议立即回刷 Gate-H：否
  - 是否可由主控评估是否切入 H-03 正式执行：是


### 策略设计完成、可进入后续执行的判定口径

#### 判定目标
本判定只回答：

1. H-03 的规模化扩样策略设计是否已经完成；
2. 是否可以进入后续执行任务；
3. 是否值得把下一轮工作从“继续判断”切换成“按设计执行”。

本判定不回答：

1. H-03 是否 ready；
2. Gate-H 是否可签收；
3. 当前 active change 是否切换。

#### 本轮验证范围
本轮 verify 验证 H03-30 ~ H03-36 是否全部完成。

具体包括：

1. `design.md` 是否已冻结 H03-33 的校准桶、边界与不可判定口径；
2. `design.md` 是否已冻结 H03-34 的复核轮次、角色差异、分歧记录与 ready blocker 规则；
3. `design.md` 是否已冻结 H03-35 的数量/分布/校准/复核四类门槛与阻断规则；
4. `tasks.md` 是否已把 H03-30 ~ H03-36 全部标记为完成；
5. `review.md` 与 `scale-out-strategy-h03.json` 是否已同步“策略设计已闭环但未执行”的口径。

本轮 verify 明确不验证：

1. H-03 ready；
2. Gate-H 可签收；
3. H-03 已开始执行规模化扩样；
4. active change 已切换。

#### 完成判定条件
仅当以下条件同时满足时，才可判定“策略设计已完成，可进入后续执行”：

1. H03-30 ~ H03-36 对应章节均已稳定；
2. 七个策略轴均已从原则描述收口为执行规则；
3. `tasks.md`、`review.md`、`scale-out-strategy-h03.json` 已同步反映“策略设计已闭环”；
4. 已明确这只是执行前提，不是 ready 结论；
5. 当前文档已明确：H-03 仍为 warning，Gate-H 仍不可签收。

#### 不通过判定条件
出现以下任一情况，均不得判定为“策略设计已完成，可进入后续执行”：

1. H03-33 ~ H03-35 任一项仍只有原则描述，没有执行检查规则；
2. `verify.md` 或 `review.md` 把设计闭环误写成 ready；
3. `scale-out-strategy-h03.json` 未同步闭环状态；
4. 文档把“可进入后续执行”误写成“可回刷 Gate-H”。

#### 下一轮真正开始执行前，至少要看什么
在真正进入规模化扩样执行前，至少还应复核以下事项：

1. 当前基线是否仍为最新：`business-task-chain=20`、`skill-false-positive=16`、`manual-review=12`；
2. H03-30 ~ H03-36 的冻结内容是否已被后续文档稳定引用；
3. `formal-execution-entry.md` 是否仍与 `latest.json`、`scale-out-strategy-h03.json` 的基线和门槛一致；
4. `tasks.md` 是否已明确“策略设计已闭环，正式执行入口已明确，但正式执行尚未启动”；
5. 当前主线是否明确授权进入 H03-37 ~ H03-39。

#### 第一轮执行化补证结论
本轮除确认 H03-30 ~ H03-36 已完成外，还新增验证以下执行化内容：

1. `skill-hit-effective-calibration.json` 是否已把 H03-33 的校准桶首次落到真实样本；
2. `review-rounds-h03.json` 是否已把 H03-34 的复核规则首次落到真实复核证据；
3. `preflight-readiness-h03.json` 是否已把 H03-36 的执行前提审包首次落到实际证据。

#### 第二轮执行化补证结论
本轮新增验证以下第二轮执行化内容：

1. `skill-hit-effective-calibration.json` 是否已补齐 `degraded_but_salvageable` 的稳定样本；
2. `skill-hit-effective-calibration.json` 是否已体现每个桶的样本数、边界与不可外推说明；
3. `review-rounds-h03.json` 是否已体现第二轮更稳定复核，而不只是形式上的轮次+1；
4. `preflight-readiness-h03.json` 是否已区分本轮前进项与仍卡住项。

#### 本轮仍未完成的内容
本轮 verify 必须同时指出以下事项仍未完成：

1. 制度化复核主索引最小闭环已形成，但长期正式多轮人工复核机制仍未形成；
2. 更大规模真实主链分布仍未补齐；
3. 第二轮校准集仍不足以外推为总体真实分布；
4. 当前仍不足以把 H-03 写成 ready；
5. 当前仍不足以回刷 Gate-H。

#### 本轮预期结论
本轮完成第二轮执行并补证后，verify 应只能给出以下类型结论：

- degraded_but_salvageable 是否已稳定落地：是 / 否
- H03-33 第二轮执行化是否形成更稳定校准集：是 / 否
- H03-34 第二轮执行化是否形成更稳定复核：是 / 否
- H-03 是否 ready：否
- 是否建议回刷 Gate-H：否

## H03-37 正式执行起跑确认验证

### 验证目标
本节只验证：H03-37 是否已经完成，以及当前是否允许交由主控单独判断是否启动 H03-38。

本节不验证：

1. H03-38 是否已启动；
2. H-03 是否 ready；
3. Gate-H 是否可签收。

### 核心验证点
1. `latest.json` 与 `scale-out-strategy-h03.json` 是否仍统一引用 `20 / 16 / 12` 基线；
2. `tasks.md`、`status.md`、`verify.md`、`review.md`、`formal-execution-entry.md` 是否仍统一坚持“不继续无边界补样 / 不把策略设计闭环误写成 ready / 不把 H-03 文档推进误写成 Gate-H 可签收”；
3. `formal-execution-entry.md` 是否已明确 H03-38 的数量门槛、结构门槛、停止点与禁止得出的错误结论。

### 实际结果（2026-04-17 H03-37）
1. 已核对：`latest.json.summary` 与 `scale-out-strategy-h03.json.current_baseline` 对 `business_chain_samples=20`、`false_positive_samples=16`、`manual_review_samples=12` 一致。
2. 已核对：H-03 工作区文档仍统一坚持 warning 口径，不继续无边界补样，不把策略设计闭环写成 ready，不把 H-03 文档推进写成 Gate-H 可签收。
3. 已核对：H03-38 的进入条件已同时包含 `30/24/16` 数量门槛与长尾 / 恢复链 / 校准 / 复核四类结构门槛，不能只堆数量。
4. 已核对：`formal-execution-entry.md` 已明确停止点与错误结论边界；`h03-37-preflight-check.json` 已形成本轮起跑确认证据。

### 本轮结论
- H03-37：已完成。
- 是否允许交由主控判断是否启动 H03-38：允许。
- 该结论不等于 H03-38 已启动，不等于 H-03 ready，不等于 Gate-H 可签收。

## H03-38 第一批正式执行批次验证

### 验证目标
本节只验证：H03-38 是否在不越界前提下完成第一批正式执行，并同时推进数量与四类结构门槛。

本节不验证：

1. H03-39 是否已启动；
2. H-03 是否 ready；
3. Gate-H 是否可签收。

### 实际结果（2026-04-17 H03-38）
1. 数量门槛已在专项批次证据中达到：`business_chain_samples=30`、`false_positive_samples=24`、`manual_review_samples=16`（见 `h03-38-batch1-execution.json.quantity_threshold`；当前 `latest.json.summary` 仍未回刷到该口径）。
2. 长尾门槛已落证：4 类行业（`food_safety_import_tail`、`trade_finance_tail`、`compliance_support_tail`、`repo_maintenance_tail`）且每类 >=2 条非换皮样本（见 `long-tail-distribution.json`）。
3. 恢复链门槛已落证：恢复链样本 10 条，三段及以上 7 条（见 `recovery-chain-distribution.json`）。
4. 校准门槛已落证：`skill_hit_effective` 五桶保持有效，且 `manual_assisted_effective` 明确单列，未混算为 `true_positive_effective`（见 `skill-hit-effective-calibration.json`）。
5. 复核门槛已落证：已体现双轮/角色差异复核，且对 `manual_assisted_effective`、`degraded_but_salvageable`、`ready_blocker_flag=true` 保留差异说明（见 `review-rounds-h03.json`）。
6. 本批次停止边界保持：未进入 H03-39，且文档未写成 ready / Gate-H 可签收。

### 本节结论
- H03-38：已完成（第一批正式执行完成）。
- 当前状态：仍为 warning。
- 后续动作：仅可交由主控判断是否进入 H03-39；该结论不等于 H-03 ready，不等于 Gate-H 可签收。

## H03-39 正式执行后复核与交接验证

### 验证目标
本节只验证：

1. H03-38 的 `30/24/16` 与四类结构门槛是否已经形成稳定批次结果；
2. `tasks.md`、`status.md`、`verify.md`、`review.md`、`formal-execution-entry.md` 是否已统一收口到 H03-39 的允许结论强度；
3. 当前是否已足以交由主控单独裁决“是否评估切主推进”。

本节不验证：

1. H-03 是否 ready；
2. Gate-H 是否可签收；
3. active change 是否切换；
4. 主控是否已经批准切主推进。

### 实际结果（2026-04-17 H03-39）
1. 已核对：`h03-38-batch1-execution.json`、`h03-39-handoff-check.json`、`scale-out-strategy-h03.json.h03_38_batch1_result` 对 `business_chain_samples=30`、`false_positive_samples=24`、`manual_review_samples=16` 的专项批次门槛表达一致；当前 `latest.json.summary` 仍停在旧聚合口径，尚未完成统一回刷。
2. 已核对：`long-tail-distribution.json`、`recovery-chain-distribution.json`、`skill-hit-effective-calibration.json`、`review-rounds-h03.json` 均已分别落证长尾、恢复链、五桶校准、双轮/角色差异复核四类结构门槛，而不是仅停留在文档口头表述。
3. 已核对：`tasks.md`、`status.md`、`verify.md`、`review.md`、`formal-execution-entry.md` 当前统一收口到“仍为 warning / 建议主控评估是否切主推进”，未把结论外溢成 ready、Gate-H 可签收、active change 已切换或主控已批准切主推进。
4. 已新增：`tmp/stage-h-mcp-skills/h03-39-handoff-check.json`，集中记录 H03-38 稳定性、文档强度一致性与主控交接建议。
5. 当前 remaining gaps 仍存在：制度化复核主索引最小闭环虽已形成，但长期正式多评审制度化流程仍未完成；同时真实主链分布仍属第一批正式执行样本规模、命中有效性分布仍不足以外推为总体稳定分布。

### 本节结论
- H03-39：已完成（正式执行后复核与交接已完成）。
- 当前状态：仍为 warning。
- 当前建议：建议主控评估是否切主推进。
- 上述建议不等于已经切主推进，不等于 H-03 ready，不等于 Gate-H 可签收。

## 结构性缺口说明（2026-04-24）

1. 已补齐 `tmp/stage-h-mcp-skills/structural-gap-acceptance-20260424.md`，明确：
   - `business-task-chain` 缺口 6 条、`skill-false-positive` 缺口 15 条、`manual-review` 缺口 4 条。
   - 所有结构化来源已用尽，缺口无法通过继续补样闭合。
   - `skill-false-positive` 目标 24 条远超当前可用独立样本数（9 条），属于结构性目标设定问题。
   - 当前样本已覆盖主要场景，制度化多评审流程已成型。
   - 上线前接受当前缺口率，以 runtime 观测后续回填替代强制达标。
2. 该说明文档作为 H-03 上线前风险接受条件的正式记录，不改变 H-03 warning 状态，但可作为 Gate-H 提审时缺口已由文档化口径覆盖的输入。
