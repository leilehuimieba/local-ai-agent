# H-mcp-skills-quality-20260415（status）

最近更新时间：2026-04-17（H03-39 正式执行后复核与交接已完成）
状态：进行中（H03-39 正式执行后复核与交接已完成；当前仍为 warning）
状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`

## 当前状态

1. 已完成：
   - H-03 change 工作区已建立，proposal/design/tasks/status/verify 草案已落地。
   - H03-01 已冻结：质量指标、最小评测包、证据目录已固定。
   - H03-02a 已冻结：Skill / Memory / Evidence 三层职责、路由边界与模块落点已固定。
   - H03-02b 已冻结：system/run/skill/evidence 四层注入、Level 1~3 渐进加载规则已固定。
   - H03-02c 已冻结：Skill Guard 检查面、trust tier、allow/review/deny 口径已固定。
   - 已补 H03-02a/H03-02b/H03-02c 与当前代码骨架的静态映射，明确“已具备能力 / 尚缺稳定字段 / 最小实现口径”。
   - 已完成 `skill_catalog.rs` 最小实现：`trust_tier / guard_action / guard_reason` 可通过 loaded/skipped 输出观测。
   - 已完成 `context_policy.rs / context_builder.rs` 最小实现：`skill_injection_enabled / max_skill_level / injected_skill_level / injected_skill_ids / evidence_refs` 可通过上下文与 metadata 输出观测。
   - 已完成 `verify.rs` 最小实现：`skill_hit_effective / skill_hit_reason / guard_downgraded / guard_decision_ref` 可通过 outcome、metadata 与 verification snapshot 输出观测。
   - 已明确模块落点优先围绕：
     - `skill_catalog.rs`
     - `context_builder.rs`
     - `context_policy.rs`
     - `memory_router.rs`
     - `verify.rs`
   - 已补最小冻结证据：
     - `tmp/stage-h-mcp-skills/architecture-freeze-h03.json`
     - `tmp/stage-h-mcp-skills/skill-catalog-guard-sample.json`
     - `tmp/stage-h-mcp-skills/context-layer-injection-sample.json`
     - `tmp/stage-h-mcp-skills/verify-signal-sample.json`
   - H03-03 已完成：第一版 eval 证据已落盘到 `tmp/stage-h-mcp-skills/`。
   - H03-04 已完成：第一版 fallback 聚合样本已落盘到 `tmp/stage-h-mcp-skills/fallback-cases.json`。
   - 已补 H-03 扩展证据：
     - `tmp/stage-h-mcp-skills/evals/context-skill.json`
     - `tmp/stage-h-mcp-skills/evals/verify-signals.json`
   - 已确认 `skill_catalog.rs`、`context_policy.rs`、`context_builder.rs`、`verify.rs` 的最小 H-03 可观测字段已在运行时/测试侧存在；`verify_signal_observable_rate=1.0` 已进入最新证据。
   - 已把外部参考项目 `rtk-ai/rtk` 的可迁移治理原则收敛进 H-03 草案：薄适配层 + 单一裁决核心、增强层失败不阻断主链路、摘要/原始证据双轨、质量收益量化闭环。
2. 进行中：
   - H03-39 已完成正式执行后复核与交接，当前只形成“建议主控评估是否切主推进”的收口结果，而不是继续无边界补样。
3. 阻塞点：
   - 当前冻结的是架构口径，不是 runtime 完整实现。
   - 当前已具备 H03-02 最小可观测字段，并已补 1 条真实 trust tier -> guard(review) -> verify 联动样本。
   - 已扩 `skill_false_positive` 样本到 24 条，当前仍保持定向误命中校准口径，并在 H03-38 中同步补齐结构门槛证据（不代表总体真实分布）。
   - 已补失败注入与人工评测扩样证据，当前 `failure_injection_locatable_rate=1.0`（4 条）、`manual_review_completion_rate=1.0`（16 条），并新增差异复核样本用于保留 warning 边界。
   - 已补最小跨技能类型扩样证据，当前 `cross_skill_observable_rate=1.0`（4 种 trust tier）。
   - 已扩真实业务任务链路样本，当前 `business_chain_observable_rate=1.0`（30 条链路样本，已达到 H03-38 数量门槛并同步覆盖长尾/恢复链结构要求）。
   - 已新增并更新恢复链分布证据：`tmp/stage-h-mcp-skills/evals/recovery-chain-distribution.json`，当前恢复链样本>=5 且三段及以上链路>=2，交叉复核分歧说明已补至本批次要求。
   - 既有长尾与代表性证据已同步 remaining gaps，说明 warning 已从“已有更自然长尾分布覆盖”继续收缩到“关键尾部缺口已补齐最小有界证据”。
   - 外部参考项目中“WSL-first / token-first”的部分不适合作为本项目默认口径，当前仅吸收治理结构，不吸收默认平台路径。
    - 当前虽已完成 H03-38 第一批正式执行，且 H03-39 已完成复核与交接，但仍只代表 warning 下的批次推进与主控裁决前交接，不代表 H-03 已 ready。
4. 下一步：
    - H-03 当前不再继续泛化补样，也不再继续补“是否需要策略设计”的判断。
    - 当前正式执行入口已收口到 `formal-execution-entry.md`；H03-39 已完成，后续仅可交由主控单独裁决是否评估切主推进。
    - 本 change 当前能支持的最强结论是：“H03-39 已完成正式执行后复核与交接，建议主控评估是否切主推进”；这不等于已经切主推进，也不等于 H-03 ready，更不等于 Gate-H 可签收。
    - H03-38 本批次已达到 `business_chain_samples=30`、`false_positive_samples=24`、`manual_review_samples=16`，且四类结构门槛已形成对应证据；H03-39 已完成稳定性复核与文档口径统一收口。
    - 当前 warning 已从“策略设计草稿可复用”进一步收口到“制度化复核主索引最小闭环已形成、主记录 / 主台账映射闭环已补齐，且正式执行后结果已可交主控评估”；但真实主链分布仍属中小样本、命中有效性分布仍未完成可外推校准、长期正式多评审流程仍未完成，因此仍不能宣称 H-03 ready，也不能据此宣称 Gate-H 可签收。
