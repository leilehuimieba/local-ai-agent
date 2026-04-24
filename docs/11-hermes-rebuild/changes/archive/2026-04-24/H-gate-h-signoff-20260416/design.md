# 技术方案

## 影响范围

- 涉及模块：
  1. 本次以文档与证据聚合为主，不改运行时代码。
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`（只读引用）
  2. `docs/11-hermes-rebuild/changes/INDEX.md`（只读引用）
  3. `docs/11-hermes-rebuild/stage-plans/H-产品差异化与透明执行路线.md`
  4. `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/*`
  5. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/*`
  6. `docs/11-hermes-rebuild/changes/H-learning-mode-browser-20260415/*`
  7. `docs/11-hermes-rebuild/changes/H-memory-routing-kb-20260415/*`
  8. `docs/11-hermes-rebuild/changes/archive/2026-04-15/H-visibility-runtime-20260415/*`
  9. `tmp/stage-h-*/latest.json`
  10. `scripts/run-stage-h-gate-acceptance.ps1`
  11. `scripts/run-stage-h-signoff-acceptance.ps1`
  12. `tmp/stage-h-gate/latest.json`
  13. `tmp/stage-h-signoff/latest.json`

## 方案

- 核心做法：
  1. 以 `current-state.md` 为唯一权威，聚合 H-01 ~ H-05 的 `status/review/verify/latest.json`。
  2. 将每个子项归类为 `signed_off / warning / blocked / stale` 四类中的一种。
  3. 只在本 change 中记录 Gate-H 缺口，不反向篡改子项结论。
  4. 若发现文档口径漂移，只记为 warning，由对应子项后续回补。
  5. 新增 `run-stage-h-gate-acceptance.ps1` 作为 Gate-H 聚合入口，统一输出 `tmp/stage-h-gate/latest.json`。
  6. 新增 `run-stage-h-signoff-acceptance.ps1` 作为 Gate-H 提审入口，统一输出 `tmp/stage-h-signoff/latest.json`。
  7. Gate-H 聚合结论优先级固定为：`current-state.md` -> Gate-H 工作区文档 -> 子项 `status/verify` -> 子项 `latest.json`；子项 `latest.json` 只作为证据引用与样本摘要来源，不单独覆盖当前裁决结论。
- 状态流转或调用链变化：
  1. 当前活跃 change 已切为 `H-gate-h-signoff-20260416`；Gate-H 工作区当前承接主推进中的聚合复核判断，但不改写 H-02 / H-03 子项事实，不直接给出签收结论。
  2. 本轮只基于 H-02 / H-03 最新权威结论完成 Gate-H 聚合复核判断；在 H-02 / H-03 仍为 `warning` 前，不把 Gate-H 改写为可签收，也不把阶段 H 改写为已完成。
  3. 新增脚本入口只负责把当前聚合判断结构化落盘，不触发 active change 切换，不回刷 Gate-H ready，不把 `warning` 改写为 `passed`。

## 风险与回退

- 主要风险：
  1. 子项文档状态与聚合结论不一致，导致阶段裁决口径漂移。
  2. 使用旧证据或最小样本过度乐观，导致 Gate-H 误判。
- 回退方式：
  1. 若聚合信息不一致，以 `current-state.md + review.md + latest.json` 的交集为准，保持 `warning`。
  2. 若后续补证推翻当前判断，仅更新本 change，不覆盖历史 review。
