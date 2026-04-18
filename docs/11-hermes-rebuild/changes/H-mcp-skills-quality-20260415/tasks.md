# H-mcp-skills-quality-20260415（tasks）

更新时间：2026-04-17
状态：进行中（策略设计已闭环，H03-39 正式执行后复核与交接已完成；当前仍为 warning，仅建议主控评估是否切主推进）

| ID | 任务 | 类型 | 状态 | 验收标准 | 证据 |
|---|---|---|---|---|---|
| H03-00 | 建立 H-03 change 五件套初稿 | 文档 | done | proposal/design/tasks/status/verify 已齐备，并加入 change 索引 | `proposal.md` |
| H03-01 | 冻结 H-03 质量边界与证据口径 | 设计 | done | 质量指标、最小评测包、证据目录已固定 | `design.md` |
| H03-02a | 冻结 Skill / Memory / Evidence 边界 | 设计 | done | 三层职责、边界约束、模块落点、最小实现字段缺口已固定 | `design.md`, `tmp/stage-h-mcp-skills/architecture-freeze-h03.json` |
| H03-02b | 冻结四层上下文注入与渐进加载口径 | 设计 | done | system/run/skill/evidence 四层定义、Level 1~3 展开规则、最小可观测字段已固定 | `design.md`, `tmp/stage-h-mcp-skills/architecture-freeze-h03.json` |
| H03-02c | 冻结 Skill Guard 与信任分级口径 | 设计 | done | Guard 检查面、trust tier、allow/review/deny 结果口径、字段落点已固定 | `design.md`, `tmp/stage-h-mcp-skills/architecture-freeze-h03.json` |
| H03-03 | 生成最小 eval 证据包 | 验证 | done | `tmp/stage-h-mcp-skills/latest.json` 与 `evals/*.json` 已落盘 | `tmp/stage-h-mcp-skills/latest.json` |
| H03-04 | fallback 与失败定位收口 | 验证 | done | `fallback-cases.json` 含失败路由、等待原因、证据引用 | `tmp/stage-h-mcp-skills/fallback-cases.json` |
| H03-05 | 提审与 Gate-H 映射收口 | 文档/验证 | done | verify/review 补齐并可进入阶段提审 | `review.md` |
| H03-06 | 补一条真实 `guard downgraded -> verify` 联动样本 | 验证 | done | `verify-signals.json` 与 `fallback-cases.json` 出现同一降级样本的 `guard_downgraded=true` + `guard_decision_ref=...review` 联动证据 | `tmp/stage-h-mcp-skills/evals/verify-signals.json`, `tmp/stage-h-mcp-skills/fallback-cases.json` |
| H03-07 | 补一条最小 `skill_false_positive` 评测样本 | 验证 | done | `skill-false-positive.json` 至少包含 1 条 `skill_hit=true` 且 `skill_hit_effective=false` 的误命中样本，并回填 `latest.json` 指标 | `tmp/stage-h-mcp-skills/evals/skill-false-positive.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-08 | 补最小失败注入与人工评测证据 | 验证 | done | `failure-injection.json` 与 `manual-review.json` 落盘，且 `latest.json` 回填 `failure_injection_locatable_rate`、`manual_review_completion_rate` | `tmp/stage-h-mcp-skills/evals/failure-injection.json`, `tmp/stage-h-mcp-skills/evals/manual-review.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-B1 | 并行线程核验 H-03 剩余缺口并校正文档口径 | 验证/文档 | done | 明确“已具备最小样本”与“仍需扩样”的边界，不把最小样本误写为 ready | `tmp/stage-h-mcp-skills/latest.json`, `tmp/stage-h-mcp-skills/evals/failure-injection.json`, `tmp/stage-h-mcp-skills/evals/manual-review.json` |
| H03-B2 | 并行复核 `guard downgraded -> verify` 联动样本 | 验证/文档 | done | `verify-signals.json` 与 `fallback-cases.json` 保持同一样本引用链，`latest.json` 指向证据路径 | `tmp/stage-h-mcp-skills/evals/verify-signals.json`, `tmp/stage-h-mcp-skills/fallback-cases.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-B3 | 并行复核最小 `skill_false_positive` 样本 | 验证/文档 | done | 至少 1 条 `skill_hit=true` 且 `skill_hit_effective=false` 样本可复现，且 `latest.json` 指标一致 | `tmp/stage-h-mcp-skills/evals/skill-false-positive.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-B4 | 并行复核最小人工评测记录 | 验证/文档 | done | `manual-review.json` 覆盖 guard 降级与 false positive 两类最小样本，并保留“仍需扩样”结论 | `tmp/stage-h-mcp-skills/evals/manual-review.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-B6 | 并行补失败注入扩样（4 条） | 验证 | done | `failure-injection.json` 扩至 4 条并覆盖 `guard_denied` 与业务链 `verify_failed`，`latest.json` 同步汇总 | `tmp/stage-h-mcp-skills/evals/failure-injection.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-B7 | 并行补人工评测扩样（4 条） | 验证 | done | `manual-review.json` 扩至 4 条并覆盖 guard 拒绝与业务链 false positive，`latest.json` 同步汇总 | `tmp/stage-h-mcp-skills/evals/manual-review.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-B5 | 并行线程收口（仅 H-03 工作区） | 文档 | done | tasks/status/verify/review 口径与当前证据一致，且不触碰 H-02 与 Gate-H 聚合文档 | `tasks.md`, `status.md`, `verify.md`, `review.md` |
| H03-09 | 补最小跨技能类型扩样证据 | 验证 | done | `cross-skill-expansion.json` 覆盖至少 4 种 trust tier，且 `latest.json` 回填 `cross_skill_observable_rate` | `tmp/stage-h-mcp-skills/evals/cross-skill-expansion.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-10 | 补最小真实业务任务链路样本 | 验证 | done | `business-task-chain.json` 至少覆盖 1 条多步链路，且 `latest.json` 回填 `business_chain_observable_rate` | `tmp/stage-h-mcp-skills/evals/business-task-chain.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-11 | 扩业务链路失败分流样本到 5 条 | 验证 | done | `business-task-chain.json` 扩至 5 条，覆盖 `manual` 与 `verify` 两类路由，`latest.json` 同步汇总样本数 | `tmp/stage-h-mcp-skills/evals/business-task-chain.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-12 | 扩真实任务链路样本到 8 条 | 验证 | done | `business-task-chain.json` 扩至 8 条，补齐多步 review 与手动重试链路，`latest.json` 同步 `business_chain_samples` | `tmp/stage-h-mcp-skills/evals/business-task-chain.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-HANDOFF-20260416 | 并行线程收口交接冻结 | 文档/验证 | done | 明确“warning 已收缩到小规模扩样已完成”，并冻结“不可宣称 ready / 不可宣称 Gate-H 可签收”的交接口径 | `tasks.md`, `status.md`, `verify.md`, `review.md`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-13 | 任务8A：扩真实任务集样本到 10 条 | 验证 | done | `business-task-chain.json` 扩至 10 条，覆盖更多 manual/verify 链路，`latest.json` 同步 `business_chain_samples` | `tmp/stage-h-mcp-skills/evals/business-task-chain.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-14 | 任务8B：扩 false positive 样本到 6 条 | 验证 | done | `skill-false-positive.json` 扩至 6 条，至少含多条 `skill_hit=true` 且 `skill_hit_effective=false`，`latest.json` 同步 false_positive 指标与样本数 | `tmp/stage-h-mcp-skills/evals/skill-false-positive.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-15 | 任务10A：有界扩真实任务链样本到 14 条 | 验证 | done | `business-task-chain.json` 扩至 14 条且新增样本非换皮重复，`latest.json` 同步 `business_chain_samples=14` | `tmp/stage-h-mcp-skills/evals/business-task-chain.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-16 | 任务10B：有界扩 false positive 样本到 10 条 | 验证 | done | `skill-false-positive.json` 扩至 10 条，全部满足 `skill_hit=true` 且 `skill_hit_effective=false`，`latest.json` 同步 `false_positive_samples=10` | `tmp/stage-h-mcp-skills/evals/skill-false-positive.json`, `tmp/stage-h-mcp-skills/latest.json` |
| H03-17 | 任务10C：H-03 文档与证据收口 | 文档 | done | `tasks/status/verify/review` 与最新证据一致，并明确“warning 收缩但仍非 ready、Gate-H 仍不可签收” | `tasks.md`, `status.md`, `verify.md`, `review.md` |
| H03-18 | 任务11A：有界代表性扩样（业务链与误命中） | 验证 | done | `business-task-chain>=16`、`skill-false-positive>=12`，并新增跨域/长链/高冲突代表样本 | `tmp/stage-h-mcp-skills/evals/business-task-chain.json`, `tmp/stage-h-mcp-skills/evals/skill-false-positive.json` |
| H03-19 | 任务11B：人工评测补强到 8 条 | 验证 | done | `manual-review.json` 样本数达到 8，且覆盖 A/B/C 三类代表维度 | `tmp/stage-h-mcp-skills/evals/manual-review.json` |
| H03-20 | 任务11C：代表性覆盖说明收口 | 文档/验证 | done | `representative-coverage.json` 明确维度映射、样本引用与剩余缺口，`latest.json` 同步索引 | `tmp/stage-h-mcp-skills/evals/representative-coverage.json`, `tmp/stage-h-mcp-skills/latest.json`, `status.md`, `verify.md`, `review.md` |
| H03-21 | 任务12A：补长尾真实语境与自然复杂冲突样本 | 验证 | done | `business-task-chain.json>=18`、`skill-false-positive.json>=14`，新增样本能明确解释其长尾语境或自然冲突来源，且非换皮重复 | `tmp/stage-h-mcp-skills/evals/business-task-chain.json`, `tmp/stage-h-mcp-skills/evals/skill-false-positive.json`, `tmp/stage-h-mcp-skills/evals/long-tail-distribution.json` |
| H03-22 | 任务12B：补人工复核多样性到 10 条 | 验证 | done | `manual-review.json>=10`，且新增 2 条样本聚焦长尾复核判断，不只是重复既有结论 | `tmp/stage-h-mcp-skills/evals/manual-review.json`, `tmp/stage-h-mcp-skills/evals/long-tail-distribution.json` |
| H03-23 | 任务12C：长尾真实分布说明收口 | 文档/验证 | done | `long-tail-distribution.json` 明确长尾语境、自然复杂冲突、人工复核补强与 remaining gaps，文档口径同步 | `tmp/stage-h-mcp-skills/evals/long-tail-distribution.json`, `tmp/stage-h-mcp-skills/latest.json`, `status.md`, `verify.md`, `review.md` |
| H03-24 | 任务13A：补行业尾部长尾与恢复链样本 | 验证 | done | `business-task-chain.json>=20`、`skill-false-positive.json>=16`，新增样本覆盖食品安全/贸易融资等行业尾部语境及多轮恢复链 | `tmp/stage-h-mcp-skills/evals/business-task-chain.json`, `tmp/stage-h-mcp-skills/evals/skill-false-positive.json`, `tmp/stage-h-mcp-skills/evals/recovery-chain-distribution.json` |
| H03-25 | 任务13B：补人工复核分歧/交叉复核到 12 条 | 验证 | done | `manual-review.json>=12`，新增 2 条样本明确“为什么仍判 warning / 为什么仍不算 ready” | `tmp/stage-h-mcp-skills/evals/manual-review.json`, `tmp/stage-h-mcp-skills/evals/recovery-chain-distribution.json` |
| H03-26 | 任务13C：恢复链 / 长尾 / 复核说明收口 | 文档/验证 | done | `recovery-chain-distribution.json` 与文档口径同步，明确 remaining gaps，且本轮完成后默认停止继续扩样 | `tmp/stage-h-mcp-skills/evals/recovery-chain-distribution.json`, `tmp/stage-h-mcp-skills/latest.json`, `status.md`, `verify.md`, `review.md` |
| H03-27 | 任务15A：判断 H-03 是否进入规模化扩样策略设计 | 设计/文档 | done | 明确当前差距属于结构性问题还是继续小步补样可解决的问题，并形成策略判断结论 | `design.md`, `status.md`, `verify.md`, `review.md`, `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json` |
| H03-28 | 任务15B：形成规模化扩样策略轴与再提审最低门槛建议 | 设计/文档 | done | 明确后续策略轴、再回刷 Gate-H 的最低门槛建议，且不新增实现代码 | `design.md`, `tasks.md`, `review.md`, `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json` |
| H03-29 | 任务15C：H-03 状态切换为“策略设计判断完成/待执行” | 文档 | done | 明确当前不是 ready、不是默认继续补样，而是进入策略设计判断完成状态 | `status.md`, `verify.md`, `review.md` |

## 执行顺序

1. 主链路：H03-01 -> H03-02a/H03-02b/H03-02c -> H03-03 -> H03-04 -> H03-05
2. 当前已完成 H03-02 的架构冻结与第一版 eval / fallback 证据。
3. 当前 H03-02 冻结已补“文档口径 vs 运行时字段缺口”映射，并已补最小 verify 可观测字段。
4. 阻塞项：未补更广真实主链分布、命中有效性分布校准和正式多评审人复核前，不宣称 H-03 ready；当前已完成策略设计判断，不再默认继续补样，后续如继续推进应先执行规模化扩样策略设计任务。


## H-03 规模化扩样策略设计执行任务

| ID | 任务 | 类型 | 状态 | 顺序约束 | 验收标准 | 不代表什么 | 与 Gate-H 再提审关系 | 证据 |
|---|---|---|---|---|---|---|---|---|
| H03-30 | 冻结规模化扩样策略文档骨架 | 设计/文档 | done | 第 1 步，后续任务前置 | `design.md` 已固定策略总章、适用边界、非目标、风险与停止条件，且明确本轮只完成 H03-30 ~ H03-32 | 不代表后续六项细则已完成，不代表样本执行已开始 | 只冻结骨架，不直接提升 Gate-H 就绪度 | `design.md` |
| H03-31 | 定义样本来源分层与代表性分布规则 | 设计 | done | 第 2 步，依赖 H03-30 | 来源层定义、来源字段、覆盖要求、非换皮约束、分布检查规则已写成执行时必须检查的规则 | 不代表长尾矩阵、恢复链矩阵、校准集、复核规则已完成 | 为后续“分布已成体系”提供前提，不直接回刷 Gate-H | `design.md`, `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json` |
| H03-32 | 定义长尾语境与恢复链覆盖矩阵 | 设计 | done | 第 3 步，依赖 H03-31 | 长尾类别矩阵、每类最小要求、恢复链模式矩阵、链长要求、不得计入覆盖的情况均已明确 | 不代表长尾/恢复链样本已补齐，只代表覆盖约束已冻结 | 为后续长尾/恢复链门槛提供判定基础，不直接回刷 Gate-H | `design.md`, `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json` |
| H03-33 | 定义命中有效性校准集 | 设计 | done | 第 4 步，依赖 H03-32 | `skill_hit_effective` 五个校准桶、边界、不可以判定口径、执行检查规则均已冻结 | 不代表校准集已执行，只代表后续校准标准已明确 | 解决 H-03 当前关键结构性缺口之一，是后续再提审的必要前提 | `design.md`, `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json` |
| H03-33E1 | 命中有效性校准集第一轮执行并补证 | 验证/补证 | done | H03-33 后的首次执行化 | 已落地最小校准集，至少覆盖 `true_positive_effective`、`false_positive_noise`、`manual_assisted_effective`、`inconclusive` 四个桶，且每桶有真实样本引用与落桶理由 | 不代表校准集已完备，不代表可直接外推总体分布 | 只证明 H03-33 已开始证据化，不构成 Gate-H 回刷条件 | `tmp/stage-h-mcp-skills/evals/skill-hit-effective-calibration.json` |
| H03-33E2 | 命中有效性校准集第二轮执行并补证 | 验证/补证 | done | H03-33E1 后的稳定化补证 | `degraded_but_salvageable` 已补到至少 2 条稳定样本，五个桶均有样本数、边界说明与不可外推标记 | 不代表校准集已达最终分布代表性，不代表可回刷 Gate-H | 只证明 H03-33 的校准集已从第一轮最小落地推进到第二轮更稳定状态 | `tmp/stage-h-mcp-skills/evals/skill-hit-effective-calibration.json` |
| H03-34 | 定义人工复核轮次与角色差异规则 | 设计 | done | 第 5 步，依赖 H03-33 | 复核轮次、角色差异、分歧记录、ready blocker 与双轮复核要求均已冻结 | 不代表已经形成签收级复核证据，只代表复核治理口径已冻结 | 为“复核强度达到治理要求”提供前提，不直接回刷 Gate-H | `design.md`, `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json` |
| H03-34E1 | 人工复核轮次 / 角色差异第一轮执行并补证 | 验证/补证 | done | H03-34 后的首次执行化 | 至少形成一版含 `review_round`、`review_role`、`review_disagreement`、`cautious_reason`、`ready_blocker_flag` 的真实复核证据，且至少 1 条样本体现有分歧/有保留 | 不代表已形成正式多评审人机制，不代表 warning 已消除 | 只证明 H03-34 已开始证据化，不构成 Gate-H 回刷条件 | `tmp/stage-h-mcp-skills/evals/review-rounds-h03.json` |
| H03-34E2 | 人工复核轮次 / 角色差异第二轮执行并补证 | 验证/补证 | done | H03-34E1 后的稳定化补证 | 至少 1 条样本体现“第一轮看似可接受、第二轮仍保守判 warning / blocker”，且复核内容存在判断增量 | 不代表已形成正式多评审人制度化流程，不代表 H-03 已 ready | 只证明 H03-34 的复核证据已从第一轮差异补证推进到第二轮更稳定复核 | `tmp/stage-h-mcp-skills/evals/review-rounds-h03.json` |
| H03-35 | 冻结 Gate-H 再提审最低门槛 | 设计/文档 | done | 第 6 步，依赖 H03-34 | 数量门槛、分布门槛、校准门槛、复核门槛及阻断规则均已明确 | 不代表门槛已被满足，不代表 H-03 已 ready | 明确“何时值得再提审”，但不等于本任务完成后即可提审 | `design.md`, `review.md`, `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json` |
| H03-36 | 形成规模化扩样执行前提审包 | 文档/评审 | done | 第 7 步，依赖 H03-35 | `verify.md` 能验证 H03-30 ~ H03-36 全部完成，`review.md` 能评审“策略设计已闭环”，且 JSON 摘要同步 | 不代表 H-03 ready，不代表 Gate-H 可签收，不代表 active change 已切换 | 这是“策略设计已闭环但未执行”的文档门槛，不是 H-03 ready 或 Gate-H 可签收门槛 | `verify.md`, `review.md`, `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json` |
| H03-36E1 | 执行前提审包第一轮执行并补证 | 文档/验证 | done | H03-36 后的首次执行化 | verify/review 已可引用实际执行证据判断“哪些设计项已首次执行化”，并明确仍不足以 ready / 回刷 Gate-H | 不代表 H-03 已通过，不代表可以进入 Gate-H 提审 | 只证明 H03-36 已开始证据化，不构成 Gate-H 回刷条件 | `tmp/stage-h-mcp-skills/evals/preflight-readiness-h03.json`, `verify.md`, `review.md` |
| H03-36E2 | 执行前提审包第二轮执行并补证 | 文档/验证 | done | H03-36E1 后的稳定化补证 | preflight 证据已能区分“第二轮前进项”与“仍卡住项”，并明确当前只能继续下一轮执行，仍不能回刷 Gate-H | 不代表 H-03 已通过，不代表具备 Gate-H 再提审条件 | 只证明 H03-36 的执行前提审包已进入第二轮更稳定证据化 | `tmp/stage-h-mcp-skills/evals/preflight-readiness-h03.json`, `verify.md`, `review.md` |

### H03-30 ~ H03-36 收口说明

1. 当前这组任务的目标，是把 H-03 收口为“策略设计已闭环，且正式执行入口已明确”。
2. 当前已完成 H03-30 ~ H03-36 全部设计任务，并完成 H03-33E1/E2、H03-34E1/E2、H03-36E1/E2 的执行前证据化补证。
3. 该组任务全部完成后允许得出的最强结论只有：
   - 策略设计已闭环；
   - 正式执行入口已明确；
   - 是否启动由主控决定。
4. 该组任务全部完成后，仍不得得出以下结论：
   - H-03 ready；
   - Gate-H 可签收；
   - active change 已切换。
5. 上述 E1/E2 只证明设计规则已被证据化到“可启动正式执行前复核”的程度，不代表 H-03 正式执行已开跑。
6. 与 Gate-H 再提审的关系应固定为：
   - H03-30 ~ H03-36 负责冻结完整的“执行前规则”；
   - H03-33E1/E2、H03-34E1/E2、H03-36E1/E2 只证明这些规则已具备进入正式执行前复核的证据基础；
   - 真正影响 Gate-H 再提审的是后续按规则执行后形成的更大样本、校准与复核证据；
   - 因此当前仍属于再提审前置项，不属于再提审完成项。

### H-03 正式执行入口与交接收口

| ID | 任务 | 类型 | 状态 | 启动条件 | 验收标准 | 不代表什么 | 与 Gate-H 再提审关系 | 证据 |
|---|---|---|---|---|---|---|---|---|
| H03-37 | 正式执行起跑确认 | 文档/验证 | done | 仅在主控要求继续推进 H-03 时启动 | 已再次确认 `formal-execution-entry.md`、`latest.json`、`scale-out-strategy-h03.json` 的基线/门槛/停止点一致，并形成“是否允许进入 H03-38”的主控裁决入口 | 不代表 active change 已自动切换，不代表正式执行已完成 | 只作为是否启动 H-03 正式执行批次的入口检查 | `formal-execution-entry.md`, `tmp/stage-h-mcp-skills/latest.json`, `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json`, `tmp/stage-h-mcp-skills/h03-37-preflight-check.json` |
| H03-38 | 第一批正式执行批次 | 验证/补证 | done | 依赖 H03-37，且需主控明确授权 | 已按冻结门槛推进到 `30/24/16`，并同步补齐 4 类长尾、5 条恢复链（含 2 条三段及以上）、命中有效性校准集、2 轮/角色差异复核 | 不代表 ready，不代表 Gate-H 可回刷 | 只用于把“执行入口明确”推进到“已有正式执行批次结果” | `formal-execution-entry.md`, `tmp/stage-h-mcp-skills/evals/*.json`, `tmp/stage-h-mcp-skills/h03-38-batch1-execution.json` |
| H03-39 | 正式执行后复核与主控交接包 | 文档/评审 | done | 依赖 H03-38 | 已复核 H03-38 的 `30/24/16` 与四类结构门槛稳定落证，且 `tasks.md`、`status.md`、`verify.md`、`review.md`、`formal-execution-entry.md` 统一收口到“仍为 warning / 建议主控评估是否切主推进” | 不代表 Gate-H 已通过，不代表已经切主推进，不代表 H-03 已 ready | 只用于把 H-03 交回主控做下一步裁决 | `status.md`, `verify.md`, `review.md`, `formal-execution-entry.md`, `tmp/stage-h-mcp-skills/h03-39-handoff-check.json` |
