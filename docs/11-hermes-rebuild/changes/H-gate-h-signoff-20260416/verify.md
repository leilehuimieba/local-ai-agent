# 验证记录

## 验证方式

- 文档验证：
  1. 以 `docs/11-hermes-rebuild/current-state.md` 作为唯一主推进状态源，核对 Gate-H 工作区文档是否与当前 active change=`H-gate-h-signoff-20260416` 保持一致。
  2. 核对 H-02 的 `status.md`、`verify.md`，确认其当前口径是否已收紧为“并行观察 / 冻结观察、仍为 warning、当前无新的合格受限样本”，并已同步 `baijiacms` 的“环境恢复 -> MySQL 启动 -> Host 匹配 -> 首页装修初始化”高质量多层人工接管样本。
  3. 核对 H-03 的 `status.md`、`verify.md`、`review.md` 与 `formal-execution-entry.md`，确认其当前最强结论是否已更新为“`H03-39` 已完成，建议主控评估是否切主推进”，且仍为 `warning`。
- 脚本验证：
  1. 执行 `scripts/run-stage-h-gate-acceptance.ps1`，确认生成 `tmp/stage-h-gate/latest.json`。
  2. 执行 `scripts/run-stage-h-signoff-acceptance.ps1`，确认生成 `tmp/stage-h-signoff/latest.json`。
  3. 检查两个 JSON 当前都保持 `status=warning`，且不把 Gate-H 错写成 `passed` 或 `ready=true`。
  4. 检查两个 JSON 在保留英文结构字段的同时，包含 `summary_zh`、`status_zh` 以及阻塞项中文说明字段。
  5. 检查两份 JSON 的双语输出约定是否稳定：英文结构字段作为机器可读主结构，中文说明字段仅作人工复核与提审说明，不替代英文结构字段。
  6. 已于 2026-04-22 重新执行上述两份脚本，确认聚合结论未漂移：`tmp/stage-h-gate/latest.json` 仍为 `warning / ready=false`，`tmp/stage-h-signoff/latest.json` 仍为 `warning / signoff_ready=false`。
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
  7. `scripts/run-stage-h-gate-acceptance.ps1`
  8. `scripts/run-stage-h-signoff-acceptance.ps1`
  9. `tmp/stage-h-gate/latest.json`
  10. `tmp/stage-h-signoff/latest.json`
- H-02 状态证据：
  1. `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/status.md`
  2. `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/verify.md`
  3. `tmp/stage-h-remediation/h02-baijiacms-db-prereq-takeover-20260421.json`
  4. `tmp/stage-h-remediation/h02-baijiacms-db-prereq-guide-20260421.json`
  5. `tmp/stage-h-remediation/manual-guides/baijiacms-db-prereq-missing.md`
  6. `tmp/stage-h-remediation/h02-baijiacms-siteid-host-check-20260421.json`
  7. `tmp/stage-h-remediation/h02-baijiacms-homepage-check-20260421.json`
  8. `tmp/stage-h-remediation/h02-baijiacms-sample-pass-summary-20260421.json`
- H-03 状态证据：
  1. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/status.md`
  2. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/verify.md`
  3. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/review.md`
  4. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/formal-execution-entry.md`
  5. `tmp/stage-h-mcp-skills/latest.json`
  6. `tmp/stage-h-mcp-skills/h03-38-batch1-execution.json`
  7. `tmp/stage-h-mcp-skills/h03-39-handoff-check.json`
- 唯一主推进状态源：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`

## Gate 映射

- 对应阶段 Gate：
  1. `Gate-H`
- 当前覆盖情况：
  1. H-01 已签收。
  2. H-02 当前口径已收紧为“并行观察 / 冻结观察”，仍为 `warning`，且当前无新的合格受限样本；旧的第二窗口 `aborted_manual_takeover` 记录不构成新的成功验证结论。另已新增 `baijiacms` 的人工接管样本总结，明确该样本已稳定收口为“环境恢复 -> MySQL 启动 -> Host 匹配 -> 首页装修初始化”的高质量多层接管链，且当前业务层直接缺的是商城首页装修记录。
  3. H-03 当前已完成 `H03-39` 正式执行后复核与交接，当前最强结论只到“建议主控评估是否切主推进”，但仍为 `warning`；另已确认 `tmp/stage-h-mcp-skills/latest.json` 已按 H03-38/H03-39 专项批次证据保守回刷到 `30 / 24 / 16`，且三份基础 eval 已补入 summary 层诚实回填与部分 detailed sample layer，但完整详细样本明细仍未统一回填。
  4. H-04/H-05 已签收。
5. Gate-H 当前已承接主推进中的聚合复核，但本轮最强结论仍只到“已完成当前轮次聚合复核判断，仍不可签收”。
6. 当前 Gate-H 已具备机器可读聚合入口与提审入口；且已于 2026-04-22 复跑验证，入口输出仍严格维持 `warning / ready=false / signoff_ready=false` 的真实状态。

## 本轮为何只做最小收紧

1. 本轮目标是在当前主推进已切到 Gate-H 的前提下，完成正式聚合复核判断并统一工作区口径，不延伸到签收裁决。
2. 当前唯一权威状态已显示活跃 change 为 `H-gate-h-signoff-20260416`；因此 Gate-H 工作区需要同步到“执行中、未签收”的当前事实，而不是继续保留候选口径。
3. H-02 / H-03 的最新权威结论都仍为 `warning`，且都没有形成 Gate-H 可签收结论；因此本轮只能完成聚合判断，不能把主推进切换误写成 signoff。

## 当前仍不可签收的原因

1. H-02 当前仍是“并行观察 / 冻结观察”，且当前无新的合格受限样本；第二窗口的 `aborted_manual_takeover` 不能回抬成新的成功验证。`baijiacms` 的新增高质量多层接管样本只能证明环境链路已恢复、接管边界更清楚，并且站点ID需依赖正确 Host 解析、业务层还缺首页装修记录，不能把 H-02 推进到 ready。
2. H-03 虽已完成 `H03-39`，且制度化复核主索引最小闭环已形成，但当前最强结论也只到“建议主控评估是否切主推进”；真实主链分布、命中有效性分布与长期正式多轮复核机制仍不足以支撑 ready。另当前 H-03 仍存在“主报告已保守回刷、基础 eval 仅部分 detailed sample layer 已补、完整明细仍待统一回填”的聚合漂移，进一步说明此时不能把 H-03 误写为已完成全部聚合收口。
3. 进一步就 `manual-review` 细项核对可确认：当前只有 8 条样本在 `review-rounds-h03.json`、`institutional_review_primary_records` 与 `formal_batch_detailed_samples` 之间形成稳定结构化回指；剩余 8 条不能默认按“已有现成明细”处理，这进一步说明 H-03 仍停留在“summary 已同步、部分 detailed sample layer 已回填”的 warning 强度。
4. 已继续核对 `update_task13.py` 与 `h03-institutional-review-check.json`，当前仍未发现除这 8 条之外的新增结构化明细来源；因此剩余 8 条更准确应表述为“当前未发现更多可直接回填的结构化来源”。
5. Gate-H 作为阶段聚合判断，不能在 H-02 / H-03 仍为 `warning` 的情况下改写为可签收。
6. 因此，本轮最多只能把 Gate-H 工作区收紧到“已完成当前轮次聚合复核判断，仍为 warning / 执行中 / 未签收 / 不可签收”的强度，不能推进为可签收。

## 若主控后续接手时的入口边界

1. H-02：只能按“冻结观察、仍为 warning、当前无新的合格受限样本”的口径继续引用；在出现新的合格样本前，不得回抬为主推进。
2. H-03：只能按“`H03-39` 已完成、建议主控评估是否切主推进、仍为 warning”的口径继续引用；不得误写成 ready。若要继续推进，应先收口 H-03 的聚合证据漂移，而不是直接重复 Gate-H 复核。
3. Gate-H：当前已承接主推进中的聚合复核，但在 H-02 / H-03 仍为 `warning` 且主控未形成更强裁决前，只能维持 `warning / 执行中 / 未签收 / 不可签收`，不得回刷可签收结论。
