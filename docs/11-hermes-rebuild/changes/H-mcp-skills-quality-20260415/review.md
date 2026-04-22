# 阶段性提审包（H-mcp-skills-quality-20260415）

更新时间：2026-04-22  
提审类型：阶段 H 子项提审草案（H-03 MCP + Skills 执行质量体系）  
评审状态：草案（H03-39 正式执行后复核与交接已完成；当前仍为 warning，仅建议主控评估是否切主推进）

## 2026-04-22 聚合漂移补记

1. `tmp/stage-h-mcp-skills/latest.json` 已按 H03-38/H03-39 专项批次证据保守回刷到 `30 / 24 / 16`。
2. `evals/business-task-chain.json`、`skill-false-positive.json`、`manual-review.json` 已补入 `formal_batch_summary / batch_sync_state / formal_batch_detailed_samples`，但当前只允许表述为“summary 已同步、部分 detailed sample layer 已回填”。
3. 其中 `manual-review` 当前只有 8 条样本在 `review-rounds-h03.json`、`institutional_review_primary_records` 与 `formal_batch_detailed_samples` 之间形成稳定结构化回指；formal batch 目标中的剩余 8 条仍应视为“来源待确认 / 结构化落点待补”。
4. 因此，当前不能再把 H-03 写成“基础 eval 明细已全部同步”或“formal batch 16 条人工复核明细已完整落盘”；更准确口径应是：`30 / 24 / 16` 已在 summary 层与专项批次证据层成立，但 detailed sample layer 仍是 partial backfill。
5. 已继续核对 `update_task13.py` 与 `h03-institutional-review-check.json`，当前未发现除现有 8 条之外的新增结构化 manual-review 明细来源；因此剩余 8 条不能按“继续整理即可补齐”理解。

## 1. 提审范围

本次提审草案覆盖 H-03 change 工作区初始化、H03-01 冻结、H03-02a/H03-02b/H03-02c 架构冻结，以及第一版 eval / fallback 聚合证据；本轮新增 verify 最小可观测字段补证，但不包含扩展样本与 Gate-H 签收。

覆盖项：

1. H-03 目标、边界、验收阈值草案。
2. H-03 最小链路设计：skills 装载、四层注入、Skill Guard、fallback 可观测、eval 包。
3. H03-01 冻结项：质量指标、最小评测包、证据目录。
4. H03-02a 冻结项：Skill / Memory / Evidence 边界与模块落点。
5. H03-02b 冻结项：system/run/skill/evidence 四层注入与渐进加载规则。
6. H03-02c 冻结项：Skill Guard 检查面、trust tier 与 Guard 动作口径。
7. H03-03/H03-04 第一版 eval 与 fallback 证据。
8. H03-02 与当前代码骨架的静态映射：已具备能力、尚缺字段、最小实现口径。
9. `skill_catalog.rs` 最小可观测字段：`trust_tier / guard_action / guard_reason`。
10. `context_policy.rs / context_builder.rs` 最小可观测字段：`skill_injection_enabled / max_skill_level / injected_skill_level / injected_skill_ids / evidence_refs`。
11. `verify.rs` 最小可观测字段：`skill_hit_effective / skill_hit_reason / guard_downgraded / guard_decision_ref`。

## 2. 前置依赖与口径

1. 当前状态裁决文件：`D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. 对应阶段计划：`D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
3. 对应 change 文档：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/proposal.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/design.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/tasks.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/status.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/verify.md`

## 3. 核心证据

### 3.1 聚合报告

1. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/latest.json`

### 3.2 子证据

1. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/skill-catalog.json`
2. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/tool-routing.json`
3. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/visibility-fallback.json`
4. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/context-skill.json`
5. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/verify-signals.json`
6. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/skill-false-positive.json`
7. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/failure-injection.json`
8. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/manual-review.json`
9. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/cross-skill-expansion.json`
10. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/business-task-chain.json`
11. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/fallback-cases.json`
12. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/architecture-freeze-h03.json`
13. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/representative-coverage.json`
14. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/long-tail-distribution.json`
15. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/evals/recovery-chain-distribution.json`
16. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/scale-out-strategy-h03.json`

### 3.3 构建/测试记录

1. `cargo test -p runtime-core generate_h03_eval_refresh -- --nocapture`
2. `cargo test -p runtime-core verify -- --nocapture`
3. `cargo test -p runtime-core run_verification_metadata -- --nocapture`
4. `cargo test -p runtime-core skill_catalog -- --nocapture`
5. `static read: skill_catalog.rs, context_builder.rs, context_policy.rs, memory_router.rs, verify.rs`

## 4. 指标判定

| 指标 | 阈值 | 实测 | 结论(PASS/WARN/FAIL) | 证据 |
|---|---|---|---|---|
| H03 工作区完整性 | = 100% | 100% | PASS | `proposal.md` |
| H03-01 质量口径冻结完整性 | = 100% | 100% | PASS | `design.md` |
| H03-02a 边界冻结完整性 | = 100% | 100% | PASS | `design.md`, `architecture-freeze-h03.json` |
| H03-02b 四层注入冻结完整性 | = 100% | 100% | PASS | `design.md`, `architecture-freeze-h03.json` |
| H03-02c Skill Guard 冻结完整性 | = 100% | 100% | PASS | `design.md`, `architecture-freeze-h03.json` |
| H03-02 模块缺口映射完整性 | = 100% | 100% | PASS | `design.md`, `verify.md` |
| skill_catalog Guard 字段最小落地 | = 100% | 100% | PASS | `skill_catalog.rs`, `skill-catalog-guard-sample.json` |
| context 注入字段最小落地 | = 100% | 100% | PASS | `context_policy.rs`, `context_builder.rs`, `context-layer-injection-sample.json` |
| verify 信号字段最小落地 | = 100% | 100% | PASS | `verify.rs`, `verify-signal-sample.json` |
| eval 聚合证据 | = 100% | 100% | PASS | `tmp/stage-h-mcp-skills/latest.json` |
| MCP/Skills 主链成功率 | >= 92% | 100%（当前最小样本） | PASS | `tmp/stage-h-mcp-skills/latest.json` |
| 失败可定位率 | >= 95% | 100%（当前最小样本） | PASS | `tmp/stage-h-mcp-skills/latest.json` |
| 关键技能评测通过率 | >= 95% | 100%（3/3） | PASS | `tmp/stage-h-mcp-skills/evals/skill-catalog.json` |
| trust tier / Guard 可观测性 | = 100%（最小样本） | 100%（3/3） | PASS | `tmp/stage-h-mcp-skills/evals/skill-catalog.json` |
| context skill 可观测性 | = 100%（最小样本） | 100%（3/3） | PASS | `tmp/stage-h-mcp-skills/evals/context-skill.json` |
| verify 信号可观测性 | = 100%（最小样本） | 100%（3/3） | PASS | `tmp/stage-h-mcp-skills/evals/verify-signals.json` |
| guard 降级联动可观测性 | = 100%（最小样本） | 100%（1/1） | PASS | `tmp/stage-h-mcp-skills/evals/verify-signals.json`, `tmp/stage-h-mcp-skills/fallback-cases.json` |
| skill 命中有效率 | 冻结口径 | 0%（0/16，定向误命中样本） | WARN | `tmp/stage-h-mcp-skills/evals/skill-false-positive.json` |
| skill 误命中率 | 冻结口径 | 100%（16/16，定向误命中样本） | WARN | `tmp/stage-h-mcp-skills/evals/skill-false-positive.json` |
| 失败注入可定位率 | 冻结口径 | 100%（4/4，小样本） | PASS | `tmp/stage-h-mcp-skills/evals/failure-injection.json` |
| 人工评测完成率 | 冻结口径 | 100%（12/12，中小样本） | PASS | `tmp/stage-h-mcp-skills/evals/manual-review.json` |
| 跨技能类型可观测率 | 冻结口径 | 100%（4/4，最小样本） | PASS | `tmp/stage-h-mcp-skills/evals/cross-skill-expansion.json` |
| 业务任务链路可观测率 | 冻结口径 | 100%（20/20，中小样本） | PASS | `tmp/stage-h-mcp-skills/evals/business-task-chain.json` |
| 代表性维度覆盖率 | 冻结口径 | 100%（A/B/C/D 四维均有样本映射） | PASS | `tmp/stage-h-mcp-skills/evals/representative-coverage.json` |
| 长尾真实分布覆盖率 | 冻结口径 | 100%（已补到 5 条长尾语境映射） | PASS | `tmp/stage-h-mcp-skills/evals/long-tail-distribution.json` |
| 恢复链/复核分歧覆盖率 | 冻结口径 | 100%（行业尾部 2 条、恢复链 3 条、交叉复核 2 条） | PASS | `tmp/stage-h-mcp-skills/evals/recovery-chain-distribution.json` |

## 5. 评审结论

1. 本次提审结果：`status=warning`
2. 就绪度判定：`h03.ready=false`
3. 阻塞项统计：`p0=0, p1=0, warning=3`
4. 结论说明（必填）：
   - 当前已完成 H-03 change 草案、H03-01 冻结、H03-02a/H03-02b/H03-02c 架构冻结，以及第一版 eval/fallback 证据。
   - 当前已明确主骨架无需推倒重来，升级重点是 `Skills 闭环 / 技能资产治理层`。
   - 当前已明确 H03-02 先从“文档与静态代码映射”推进为“部分运行时最小可观测”，但不等于完整 runtime 闭环与扩展样本已完成。
   - 当前已把 `skill_catalog` 的 trust tier / Guard 动作推进到最小可观测实现。
   - 当前已把 `context_policy/context_builder` 的注入开关与级别字段推进到最小可观测实现。
   - 当前已把 `verify` 的命中有效性与 Guard 决策引用推进到最小可观测实现，并已补 1 条真实 `guard downgraded -> verify` 联动样本。
   - 当前 warning 已继续收缩到“策略设计判断完成，可进入规模化扩样设计”：已补失败注入扩样（4 条）、人工评测扩样（12 条）、跨技能类型样本（4 条）、真实业务任务链路样本（20 条）与 false positive 扩样（16 条），并形成代表性维度 + 长尾分布 + 恢复链/复核分歧三层证据映射。
   - 当前仍未解决：真实主链分布仍属中小样本、命中有效性分布仍未校准、交叉复核尚不是正式评审机制；因此不能宣称 H-03 ready，也不能据此宣称 Gate-H 可签收。
   - 当前还需额外保留一个收紧边界：`manual-review=16` 目前只成立在 formal batch summary 层；当前只有 8 条样本形成了稳定结构化 detailed sample 回指，剩余 8 条不能默认按“已有现成明细待抄录”处理。

## 6. 风险与回退

1. 风险：
   - 当前最小字段虽已补齐，且代表性覆盖、长尾分布与恢复链分布已有显式映射；但仍需避免把“中小规模尾部样本可观测”误当成“真实分布已达标”。
   - 若继续沿用零散补样模式，容易无限扩样、继续用定向样本堆数量，并把 H-03 变成新的工程债；因此更合理的下一步是先完成规模化扩样策略设计。
2. 回退触发条件：
   - 在未补扩展样本前直接宣称 H-03 ready。
   - 证据文件与源码行为出现漂移。
3. 回退动作：
   - 保持 H-03 为“部分已验证”状态，不切主推进。
   - 本轮已完成任务15策略设计判断；建议将 H-03 的下一步正式切换为“规模化扩样策略设计”，待形成成体系分布方案后，再决定是否回刷 Gate-H。

## 7. 后续动作

1. 若 `passed`：
   - 不适用，本提审包当前不以签收为目标。
2. 若 `warning`：
   - 责任人：`待定`
   - 追踪编号：`H03-DRAFT-001`
   - 到期时间：`2026-04-18T18:00:00+08:00`
   - 补证动作：已完成任务15策略设计判断，确认 H-03 后续应进入规模化扩样策略设计；当前基线为 `business_chain_samples=20`、`false_positive_samples=16`、`manual_review_samples=12`，下一轮 Gate-H 提审前建议至少达到 `30/24/16` 并补命中有效性校准集。
3. 若 `failed`：
   - 暂停 H-03 推进。
   - 回到阶段 H 路线重新收紧范围。

## 8. Gate 映射

1. 对应 Gate：`Gate-H`
2. 覆盖项：
   - H-03 change 工作区初始化
   - H03-01 质量口径冻结
   - H03-02a 边界冻结
   - H03-02b 四层注入冻结
   - H03-02c Skill Guard 冻结
   - H03-03/H03-04 第一版 eval / fallback 证据
3. 未覆盖项（如有）：
   - 更广主链成功率验证（原因：样本覆盖仍有限）
   - skill 命中有效性 / 误命中验证的分布代表性校准（原因：当前 false positive 仍以定向样本集为主）
   - Guard 降级/拒绝真实联动扩样（原因：当前高复杂度联动样本仍少）
   - 人工评测进一步扩样（原因：虽已到 8 条，但评审人多样性与复核轮次仍不足）

## 9. 签收记录（评审后回填）

1. 评审人：`待定`
2. 评审时间：`待定`
3. 最终结论：`warning`
4. 签收备注：当前已完成 H03-02 架构冻结与第一版 H-03 证据，但不代表 H-03 已通过。


## H-03 规模化扩样策略设计评审结论

### 评审目的
本节用于评审：
H-03 当前是否应正式从“策略设计判断已完成”切换到“策略设计执行”，以及本轮设计稿是否已足以作为后续扩样执行的前提。

### 当前评审结论

#### 结论 1：建议收口为正式执行入口，交由主控判断是否启动
评审结论：**建议**

原因如下：

1. 当前最小有界证据与策略设计已足以回答“正式执行应如何开跑”；
2. H-03 当前不再适合继续重复策略判断，也不适合继续零散补样；
3. H-03 当前仍缺的，是主控是否授权启动正式执行，而不是再写更多策略；
4. 因此本轮最合适的收口，是把 H-03 固定为“正式执行入口已明确、等待主控判断”。

#### 结论 2：H-03 当前仍不可改判为 ready
评审结论：**不可**

原因如下：

1. business-task-chain 仍属中小样本；
2. `skill_hit_effective` 尚未形成正式校准集；
3. 长尾语境覆盖尚未形成正式类别约束；
4. 恢复链覆盖尚未形成正式分布要求；
5. 人工复核轮次与角色差异尚未具备签收级治理强度。

#### 结论 3：当前策略设计稿的合格标准
本轮策略设计稿只有在满足以下条件时，才可视为“足以进入下一轮执行”：

1. 七个策略轴齐备；
2. 每个策略轴均包含字段、范围、最低要求或阈值；
3. 最低门槛表已明确；
4. verify 已能验证策略设计是否完整；
5. review 已能明确说明“为什么下一步应进入执行，而不是继续判断”。

若仅有原则描述、没有字段与阈值，则不能视为合格设计稿。

### 对 Gate-H 的影响
本轮策略设计完成后，对 Gate-H 的影响应表述为：

1. Gate-H 仍不可签收；
2. H-03 仍为 warning；
3. 但 H-03 的下一步不再模糊；
4. 后续可以从“继续补样”正式切换到“执行策略设计 + 按门槛扩样”。

### 评审通过标准
本轮评审通过，并不意味着 H-03 ready。
本轮评审通过仅意味着：

- H-03 已完成从判断层到设计层的切换；
- 后续扩样可以按正式策略推进；
- Gate-H 的后续判断将建立在更明确、更可复核的门槛之上。

### 当前建议
建议后续继续留在：

- `H-mcp-skills-quality-20260415`

内推进，且下一步应为：

- **按 `formal-execution-entry.md` 收口为正式执行入口**
- **由主控判断是否把 H-03 纳入下一轮正式执行候选**
- 而不是继续重复策略设计判断
- 也不是继续无边界补样


## H-03 策略设计执行前评审口径

### 评审目标
本节评审的不是 H-03 是否 ready，而是：

1. H03-30 ~ H03-36 是否已完成策略设计闭环；
2. 是否可以把下一步从“继续判断”切换为“可进入后续执行”；
3. 是否已明确与 Gate-H 再提审门槛之间的关系。

### 当前接受范围
当前评审只接受 H-03 进入“策略设计已闭环，但尚未执行”的状态。

这表示：

1. H03-30 ~ H03-36 已完成；
2. 后续执行顺序已固定；
3. 当前仍未开始实际扩样执行；
4. 当前仍不是 ready，仍不能回刷 Gate-H。

### 评审通过条件
仅当以下条件同时满足时，才可认定策略设计闭环成立：

1. `design.md` 已把 H03-33 ~ H03-35 收口为执行规则；
2. `verify.md` 已能验证 H03-30 ~ H03-36 全部完成；
3. `tasks.md` 已明确 H03-30 ~ H03-36 完成；
4. `scale-out-strategy-h03.json` 已同步闭环状态；
5. 文档已明确：策略设计闭环不等于 H-03 ready，不等于 Gate-H 可签收。

### 不接受的表述
当前评审明确不接受以下表述：

1. 把 H-03 写成 ready；
2. 把 H-03 写成 Gate-H 可签收依据；
3. 把 H-03 写成 active change 已自动切换；
4. 把“策略设计已闭环”写成“规模化扩样已执行完成”。

### 评审结论用语约束
本节后续只能使用以下结论语义：

- H-03 策略设计已闭环；
- 可进入后续执行；
- 仍非 ready；
- 仍不足以支撑 Gate-H 可签收。

不得使用以下误导性语义：

- H-03 已 ready；
- H-03 已通过；
- Gate-H 可签收；
- active change 已切换。

### 第二轮执行并补证评审结论
1. 当前可接受的新增结论是：H03-33 / H03-34 / H03-36 的执行前证据化补证已完成第二轮收口。
2. 该新增结论只意味着：
   - `degraded_but_salvageable` 已不再处于空缺；
   - 校准集与复核证据比第一轮更稳定；
   - 已足以整理出正式执行入口，是否启动仍由主控决定。
3. 该新增结论不意味着：
   - H-03 warning 已消除；
   - Gate-H 已具备签收条件；
   - active change 已自动切换。

### 与 Gate-H 的关系
1. 本轮评审通过后，只能接受“H03-30 ~ H03-36 已闭环，执行前证据化补证已收口，且正式执行入口已明确”。
2. 该评审通过只意味着：
   - H-03 后续正式执行已有完整入口；
   - 主控可以据此判断是否切入 H-03；
   - 下一轮 Gate-H 回刷条件更加明确。
3. 该评审通过不意味着：
   - H-03 warning 已消除；
   - Gate-H 已具备签收条件；
   - active change 已自动切换。


## H03-37 正式执行起跑确认评审结论

### 评审目标
本节只评审：H03-37 是否已经完成，以及当前是否允许把“是否启动 H03-38”交由主控单独裁决。

### 评审结论
1. H03-37 起跑确认已完成。
2. 当前建议：允许交由主控判断是否进入 H03-39。
3. 上述建议仅代表 H03-38 已完成第一批正式执行且保持 warning，不代表 H-03 ready，也不代表 Gate-H 可签收。

### 评审依据
1. `latest.json` 与 `h03-38-batch1-execution.json` 已体现 H03-38 执行结果：`30 / 24 / 16`；
2. `formal-execution-entry.md` 已明确 H03-38 必须同步满足数量门槛与结构门槛，不能只堆数量；
3. `tasks.md`、`status.md`、`verify.md`、`review.md` 当前口径一致：H03-38 已完成但仍为 warning，不把结果误写成 ready，不把 H-03 文档推进误写成 Gate-H 可签收。

### 评审停止点
1. 若主控未授权，不启动 H03-39；
2. 若后续新增证据只会堆数量、不改善结构门槛，则继续停在 warning；
3. 若后续文档出现 ready / Gate-H 可签收语义外溢，必须回退到当前 warning 口径。


## H03-38 第一批正式执行评审结论

### 评审结论
1. H03-38 第一批正式执行已完成。
2. 数量门槛已达：`business_chain_samples=30`、`false_positive_samples=24`、`manual_review_samples=16`。
3. 四类结构门槛均已形成对应证据：
   - 长尾：4 类行业且每类>=2条非换皮样本；
   - 恢复链：>=5 条且三段及以上>=2条；
   - 校准：五桶有效且人工辅助不混算为纯自动有效；
   - 复核：双轮/角色差异复核已体现，并保留 manual_assisted/degraded/blocker 差异说明。
4. 本结论仍限定在 warning 口径，不等于 H-03 ready，不等于 Gate-H 可签收。

### 评审后的主控接口
- 仅可交由主控判断是否进入 H03-39。
- 绝不等同于 H03-39 已启动。

## H03-39 正式执行后复核与交接评审结论

### 评审目标
本节只评审：H03-39 是否已经完成，以及当前是否已足以把“是否评估切主推进”交由主控单独裁决。

### 评审结论
1. H03-39 正式执行后复核与交接已完成。
2. 当前建议：建议主控评估是否切主推进。
3. 上述建议仅代表 H03-38 的正式执行结果已稳定落证，且 H03-39 已完成 warning 口径下的交接收口；不代表已经切主推进，不代表 H-03 ready，也不代表 Gate-H 可签收。

### 评审依据
1. `latest.json`、`h03-38-batch1-execution.json` 与 `scale-out-strategy-h03.json` 对 `30 / 24 / 16` 与四类结构门槛的表达一致。
2. `skill-hit-effective-calibration.json`、`review-rounds-h03.json`、`long-tail-distribution.json`、`recovery-chain-distribution.json` 已分别形成校准、复核、长尾、恢复链的独立证据。
3. `tasks.md`、`status.md`、`verify.md`、`review.md`、`formal-execution-entry.md` 当前口径一致：只允许得出“仍为 warning / 建议主控评估是否切主推进”，不把结果误写成 ready、Gate-H 可签收、active change 已切换或主控已批准切主推进。
4. `tmp/stage-h-mcp-skills/h03-39-handoff-check.json` 已形成本轮主控交接证据。
5. 当前 `manual-review` 的 formal batch detailed sample layer 只稳定落了 8 条；`representative-coverage.json` 中较早引用的旧 manual 样本当前尚未形成对应的 structured detailed sample 落点，因此不能把 `manual_review=16` 误读成“16 条明细已完整同步”。

### 评审停止点
1. H03-39 的交接完成，不等于主控已经做出切主推进裁决。
2. 制度化复核主索引最小闭环虽已形成，但在长期正式多评审制度化与更长期分布稳定性证据未补齐前，H-03 继续保持 warning。
3. 若后续文档把“建议主控评估是否切主推进”外溢成“已经切主推进 / ready / Gate-H 可签收”，必须回退到当前收口口径。
