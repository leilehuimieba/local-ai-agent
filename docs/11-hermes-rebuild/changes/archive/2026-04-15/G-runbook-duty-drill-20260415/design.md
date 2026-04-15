# 技术方案

## 影响范围

- 涉及模块：
  1. 脚本层：`scripts/run-stage-g-evidence-freshness.ps1`、`scripts/run-stage-g-gate-acceptance.ps1`、`scripts/run-stage-g-warning-governance.ps1`、`scripts/run-stage-g-regression-baseline.ps1`。
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/stage-plans/G-运行手册与值守规范.md`
  2. `docs/11-hermes-rebuild/stage-plans/G-证据保鲜策略.md`
  3. `docs/11-hermes-rebuild/changes/G-runbook-duty-drill-20260415/*`
  4. `tmp/stage-g-evidence-freshness/latest.json`
  5. `tmp/stage-g-gate/latest.json`
  6. `tmp/stage-g-ops/latest.json`
  7. `tmp/stage-g-regression/latest.json`

## 方案

- 核心做法：
  1. 按 runbook 流程先执行 routine 回归演练，确认基线持续通过。
  2. 执行 release_window + refresh 演练，验证 30 分钟阈值与 warning 审计责任字段链路。
  3. 再执行 G-02 治理聚合，确认 `governance_ready=true` 与 tracker 落盘。
  4. 回写 change 五件套与阶段文档，明确下一步进入 `G-G1` 评审。
- 状态流转或调用链变化：
  1. 新增 `G-04` 主推进 change：`G-runbook-duty-drill-20260415`。
  2. 证据链按 `freshness -> gate -> governance -> regression` 串联，并保持可复跑。

## 风险与回退

- 主要风险：
  1. 上游 F 阶段脚本在 refresh 过程偶发并发冲突（日志文件占用、runtime 二进制缺失）导致 stderr 噪声。
  2. 长时 refresh 演练耗时较高，可能影响单轮窗口稳定性。
- 回退方式：
  1. 若 refresh 链路出现并发冲突，先保留已落盘通过证据，再以 snapshot 模式复核稳定态。
  2. 保留 `G-03` 回归脚本作为兜底验收入口，确保 `pass_rate` 与 route 分流不退化。
