# 技术方案

## 影响范围

- 涉及模块：
  1. 脚本层：G-02 告警治理聚合。
- 涉及文档或 contract：
  1. `scripts/run-stage-g-warning-governance.ps1`
  2. `tmp/stage-g-ops/latest.json`
  3. `tmp/stage-g-ops/warning-tracker.json`
  4. `docs/11-hermes-rebuild/stage-plans/G-发布后巡检与告警治理.md`
  5. `docs/11-hermes-rebuild/changes/G-warning-governance-closure-20260415/*`

## 方案

- 核心做法：
  1. 在 `run-stage-g-warning-governance.ps1` 中复用 `run-stage-g-evidence-freshness.ps1` 结果，读取上游 `backend reverify` 与 warning audit 报告。
  2. 维护 warning tracker：记录 warning_code 计数、最近快照与历史窗口（最近 30 次）。
  3. 设定升级阈值（默认 2 次）：同一 warning_code 连续出现达到阈值即进入 escalated 列表。
  4. 产出 `latest.json`，统一返回 `freshness_ready/audit_fields_ready/tracker_updated/governance_ready`。
- 状态流转或调用链变化：
  1. 新链路：`warning governance -> evidence freshness -> backend reverify -> warning audit`。
  2. G-02 从文档规则升级为可执行验收入口。

## 风险与回退

- 主要风险：
  1. 上游报告路径缺失会导致 G-02 脚本失败。
  2. tracker 文件损坏会影响历史计数。
- 回退方式：
  1. tracker 损坏时重建空 tracker 并保留当前轮结果。
  2. 上游失败时直接回退到 `run-stage-g-evidence-freshness.ps1` 重新取证。
