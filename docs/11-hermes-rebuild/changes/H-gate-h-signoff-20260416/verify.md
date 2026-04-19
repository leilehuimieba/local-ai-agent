# 验证记录

## 验证方式

- 文档验证：
  1. 以 `docs/11-hermes-rebuild/current-state.md` 作为唯一主推进状态源，核对 Gate-H 工作区文档是否与当前 active change=`H-gate-h-signoff-20260416` 保持一致。
  2. 核对 H-02 的 `status.md`、`verify.md`，确认其当前口径是否已收紧为“并行观察 / 冻结观察、仍为 warning、当前无新的合格受限样本”。
  3. 核对 H-03 的 `status.md`、`verify.md`、`review.md` 与 `formal-execution-entry.md`，确认其当前最强结论是否已更新为“`H03-39` 已完成，建议主控评估是否切主推进”，且仍为 `warning`。
- 一致性验证：
  1. 检查 Gate-H 文档是否明确：H-02 / H-03 当前都仍是 `warning`。
  2. 检查 Gate-H 文档是否明确：不把 H-02 写成 ready、当前主推进或可签收输入。
  3. 检查 Gate-H 文档是否明确：不把 H-03 写成 ready，不把“建议主控评估是否切主推进”外溢成“已经切主推进”。
  4. 检查 Gate-H 文档是否明确：当前虽已接手主推进中的聚合复核，但仍不把 Gate-H 写成可签收。
  5. 检查 Gate-H 文档是否明确：本工作区收紧不等于阶段完成，不等于全局状态修改。
- 收紧验证：
  1. 检查 Gate-H 文档是否已移除“非当前主推进 / 聚合复核候选”等旧口径。
  2. 检查 Gate-H 文档是否仍坚持：主推进虽已切到 Gate-H，但 Gate-H 仍不可签收。
  3. 检查 Gate-H 文档是否已把 H-03 从“策略设计闭环 + 两轮执行并补证完成”更新到 `H03-39` 完成后的当前权威强度。

## 证据位置

- 当前文档证据：
  1. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/proposal.md`
  2. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/design.md`
  3. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/tasks.md`
  4. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/status.md`
  5. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/verify.md`
  6. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/review.md`
- H-02 状态证据：
  1. `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/status.md`
  2. `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/verify.md`
- H-03 状态证据：
  1. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/status.md`
  2. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/verify.md`
  3. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/review.md`
  4. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/formal-execution-entry.md`
- 唯一主推进状态源：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`

## Gate 映射

- 对应阶段 Gate：
  1. `Gate-H`
- 当前覆盖情况：
  1. H-01 已签收。
  2. H-02 当前口径已收紧为“并行观察 / 冻结观察”，仍为 `warning`，且当前无新的合格受限样本；旧的第二窗口 `aborted_manual_takeover` 记录不构成新的成功验证结论。
  3. H-03 当前已完成 `H03-39` 正式执行后复核与交接，当前最强结论只到“建议主控评估是否切主推进”，但仍为 `warning`。
  4. H-04/H-05 已签收。
  5. Gate-H 当前已承接主推进中的聚合复核，但本轮最强结论仍只到“已完成当前轮次聚合复核判断，仍不可签收”。

## 本轮为何只做最小收紧

1. 本轮目标是在当前主推进已切到 Gate-H 的前提下，完成正式聚合复核判断并统一工作区口径，不延伸到签收裁决。
2. 当前唯一权威状态已显示活跃 change 为 `H-gate-h-signoff-20260416`；因此 Gate-H 工作区需要同步到“执行中、未签收”的当前事实，而不是继续保留候选口径。
3. H-02 / H-03 的最新权威结论都仍为 `warning`，且都没有形成 Gate-H 可签收结论；因此本轮只能完成聚合判断，不能把主推进切换误写成 signoff。

## 当前仍不可签收的原因

1. H-02 当前仍是“并行观察 / 冻结观察”，且当前无新的合格受限样本；第二窗口的 `aborted_manual_takeover` 不能回抬成新的成功验证。
2. H-03 虽已完成 `H03-39`，且制度化复核主索引最小闭环已形成，但当前最强结论也只到“建议主控评估是否切主推进”；真实主链分布、命中有效性分布与长期正式多轮复核机制仍不足以支撑 ready。
3. Gate-H 作为阶段聚合判断，不能在 H-02 / H-03 仍为 `warning` 的情况下改写为可签收。
4. 因此，本轮最多只能把 Gate-H 工作区收紧到“已完成当前轮次聚合复核判断，仍为 warning / 执行中 / 未签收 / 不可签收”的强度，不能推进为可签收。

## 若主控后续接手时的入口边界

1. H-02：只能按“冻结观察、仍为 warning、当前无新的合格受限样本”的口径继续引用；在出现新的合格样本前，不得回抬为主推进。
2. H-03：只能按“`H03-39` 已完成、建议主控评估是否切主推进、仍为 warning”的口径继续引用；不得误写成 ready。
3. Gate-H：当前已承接主推进中的聚合复核，但在 H-02 / H-03 仍为 `warning` 且主控未形成更强裁决前，只能维持 `warning / 执行中 / 未签收 / 不可签收`，不得回刷可签收结论。
