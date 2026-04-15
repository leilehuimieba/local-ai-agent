# 技术方案

## 影响范围

- 涉及模块：
  1. 脚本层：阶段 G 证据保鲜与聚合验收入口。
- 涉及文档或 contract：
  1. `scripts/run-stage-g-evidence-freshness.ps1`
  2. `scripts/run-stage-g-gate-acceptance.ps1`
  3. `docs/11-hermes-rebuild/stage-plans/G-证据保鲜策略.md`
  4. `docs/11-hermes-rebuild/stage-plans/G-发布后巡检与告警治理.md`
  5. `docs/11-hermes-rebuild/stage-plans/G-最小回归基线清单.md`
  6. `docs/11-hermes-rebuild/stage-plans/G-运行手册与值守规范.md`
  7. `docs/11-hermes-rebuild/changes/G-evidence-freshness-policy-20260414/*`

## 方案

- 核心做法：
  1. 以 `run-stage-backend-reverify-pack.ps1` 为上游，新增 `run-stage-g-evidence-freshness.ps1` 统一封装阶段 G 的策略参数与输出口径。
  2. 新增 `run-stage-g-gate-acceptance.ps1` 作为 `G-01` 聚合验收入口，输出 `tmp/stage-g-gate/latest.json`。
  3. 文档侧明确两种模式：`routine(180分钟)` 与 `release_window(30分钟)`，并固定 warning 审计责任字段规则。
- 状态流转或调用链变化：
  1. 新调用链：`G gate acceptance -> evidence freshness -> backend reverify -> warning audit`。
  2. 证据链从单一 `stage-backend-reverify` 扩展为阶段 G 专属聚合口径。

## 风险与回退

- 主要风险：
  1. 上游复核脚本失败会导致阶段 G 入口整体失败。
  2. warning 责任字段传参缺失可能造成发布窗口判定歧义。
- 回退方式：
  1. 保留上游脚本不变，阶段 G 入口仅做封装，可快速回退到原脚本直跑。
  2. 若聚合口径争议，直接引用 `tmp/stage-backend-reverify/latest.json` 做临时仲裁。
