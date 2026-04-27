# H-03 正式执行入口（formal execution entry）

更新时间：2026-04-17
状态：H-03 工作区执行入口，H03-39 正式执行后复核与交接已完成（warning 未消除，仅建议主控评估是否切主推进）

## 当前定位

1. H-03 已完成策略设计闭环与执行前证据化补证，本轮不再继续泛化补样。
2. 当前最强结论是：“H03-39 正式执行后复核与交接已完成，建议主控评估是否切主推进”。
3. 本文只定义正式执行入口，不改写 active change，不改写 Gate-H 结论，也不把 H-03 写成 ready。

## 启动前必须再次确认的基线

1. 当前聚合基线仍为：`business_chain_samples=20`、`false_positive_samples=16`、`manual_review_samples=12`。
2. 当前结构性缺口仍为：真实主链分布不足、命中有效性分布仍未完成可外推校准、制度化复核主索引最小闭环虽已形成但长期正式多轮复核机制仍未形成。
3. 若以上基线或缺口发生变化，必须先回写 H-03 工作区，再决定是否启动正式执行批次。

## 正式执行顺序（只作为待启动入口）

### Step 1：H03-37 正式执行起跑确认

1. 复核 `latest.json` 与 `scale-out-strategy-h03.json` 是否仍使用同一基线。
2. 复核后续执行是否仍受以下约束：
   - 不继续无边界补样；
   - 不得把策略设计闭环误写成 ready；
   - 不得把 H-03 文档推进误写成 Gate-H 可签收。
3. 只有当上述确认成立时，才允许进入 H03-38。

#### H03-37 本轮确认结果（2026-04-17）

1. `latest.json` 与 `scale-out-strategy-h03.json` 已再次核对，当前仍统一引用 `business_chain_samples=20`、`false_positive_samples=16`、`manual_review_samples=12`。
2. H-03 工作区当前仍统一坚持以下边界：不继续无边界补样、不把策略设计闭环误写成 ready、不把 H-03 文档推进误写成 Gate-H 可签收、H03-38 必须同步推进数量门槛与结构门槛。
3. 因此本轮只得出一条结论：允许交由主控判断是否启动 H03-38；这不等于 H03-38 已启动，不等于 H-03 ready，不等于 Gate-H 可签收。

### Step 2：H03-38 第一批正式执行批次（已完成）

1. 数量门槛只按冻结口径推进到：`business-task-chain>=30`、`skill-false-positive>=24`、`manual-review>=16`。
2. 分布门槛必须同步推进：
   - 长尾行业类别至少 4 类，且每类至少 2 条非换皮样本；
   - 恢复链至少 5 条，其中至少 2 条为三段及以上链路。
3. 校准门槛必须同步推进：
   - `skill_hit_effective` 五个校准桶保持有效；
   - 不得只补 `false_positive_noise`，必须同步检查 `true_positive_effective`、`manual_assisted_effective`、`degraded_but_salvageable`、`inconclusive`。
4. 复核门槛必须同步推进：
   - 至少体现 2 轮复核视角或等价角色差异；
   - 对 `manual_assisted_effective`、`degraded_but_salvageable`、`ready_blocker_flag=true` 的样本保留差异复核说明。



#### H03-38 本批次执行结果（2026-04-17）

1. 数量门槛已达到：`business_chain_samples=30`、`false_positive_samples=24`、`manual_review_samples=16`。
2. 结构门槛已在本批次同步推进：
   - 长尾：已形成 4 类行业类别，且每类至少 2 条非换皮样本；
   - 恢复链：已覆盖 10 条恢复链，其中 7 条为三段及以上链路；
   - 校准：`skill_hit_effective` 五桶保持有效，且 `manual_assisted_effective` 全程单列，未混算为 `true_positive_effective`；
   - 复核：已体现双轮/角色差异复核，并对 `manual_assisted_effective`、`degraded_but_salvageable`、`ready_blocker_flag=true` 保留差异说明。
3. 本批次结论只能是：H03-38 已完成并保持 warning；不等于 H-03 ready，不等于 Gate-H 可签收，不等于 H03-39 已启动。

### Step 3：H03-39 正式执行后复核与交接

1. 只允许形成两类结论：
   - 仍为 warning，继续留在 H-03 工作区补证；
   - 已达到“可由主控评估是否切主推进”的条件。
2. 仍然禁止形成以下结论：
   - H-03 ready；
   - Gate-H 可签收；
   - active change 已自动切换。

#### H03-39 本轮复核结果（2026-04-17）

1. 已核对 H03-38 的数量门槛与四类结构门槛均有独立证据落地：`30 / 24 / 16`、4 类长尾行业、10 条恢复链（其中 7 条三段及以上）、五桶命中有效性校准、双轮/角色差异复核。
2. 已核对 `tasks.md`、`status.md`、`verify.md`、`review.md`、`formal-execution-entry.md` 当前结论强度一致，只收口到“仍为 warning / 建议主控评估是否切主推进”。
3. 已新增 `h03-39-handoff-check.json` 作为本轮复核与交接证据。
4. 因此本轮可得出的最强结论只有：建议主控评估是否切主推进；这不等于已经切主推进，不等于 H-03 ready，不等于 Gate-H 可签收。

## 再提审最低门槛（仍不是 ready 门槛）

1. 数量门槛：`30 / 24 / 16`。
2. 分布门槛：4 类长尾行业、5 条恢复链、至少 2 条三段及以上恢复链。
3. 校准门槛：`skill_hit_effective` 五桶均保持可用，且不可把人工辅助样本混算为纯自动有效。
4. 复核门槛：至少 2 轮复核视角或等价角色差异说明。
5. 任一结构门槛未达标时，即使数量达标，也不得回刷 Gate-H。

## 停止点

1. H03-39 已完成；后续仅可交由主控判断是否评估切主推进。
2. 若新增证据只会继续堆数量、不能改善分布/校准/复核结构，则停止，不继续扩样。
3. 若文档表述开始逼近“已经切主推进”或 ready / Gate-H 签收语义，则立即回退到 warning 口径。

## 证据引用

- `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/latest.json`
- `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/scale-out-strategy-h03.json`
- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/tasks.md`
- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/verify.md`
- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/review.md`
- `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/h03-37-preflight-check.json`
- `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/h03-38-batch1-execution.json`
- `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/h03-39-handoff-check.json`
