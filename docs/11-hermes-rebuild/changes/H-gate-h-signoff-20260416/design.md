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

## 方案

- 核心做法：
  1. 以 `current-state.md` 为唯一权威，聚合 H-01 ~ H-05 的 `status/review/verify/latest.json`。
  2. 将每个子项归类为 `signed_off / warning / blocked / stale` 四类中的一种。
  3. 只在本 change 中记录 Gate-H 缺口，不反向篡改子项结论。
  4. 若发现文档口径漂移，只记为 warning，由对应子项后续回补。
- 状态流转或调用链变化：
  1. 当前活跃 change 仍以 `H-mcp-skills-quality-20260415` 为准；Gate-H 工作区当前只作为聚合复核候选，不接手主推进。
  2. 若主控后续决定切入 Gate-H，再由本 change 汇总 H-02 / H-03 最新权威结论并评估是否进入阶段签收；在此之前不以本 change 改写主推进状态。

## 风险与回退

- 主要风险：
  1. 子项文档状态与聚合结论不一致，导致阶段裁决口径漂移。
  2. 使用旧证据或最小样本过度乐观，导致 Gate-H 误判。
- 回退方式：
  1. 若聚合信息不一致，以 `current-state.md + review.md + latest.json` 的交集为准，保持 `warning`。
  2. 若后续补证推翻当前判断，仅更新本 change，不覆盖历史 review。
