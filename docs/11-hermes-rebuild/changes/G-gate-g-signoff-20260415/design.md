# 技术方案

## 影响范围

- 涉及模块：
  1. 脚本层：Gate-G 聚合签收入口。
  2. 文档层：阶段签收状态与索引回写。
- 涉及文档或 contract：
  1. `scripts/run-stage-g-signoff-acceptance.ps1`
  2. `tmp/stage-g-signoff/latest.json`
  3. `docs/11-hermes-rebuild/current-state.md`
  4. `docs/11-hermes-rebuild/changes/INDEX.md`
  5. `docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`
  6. `docs/11-hermes-rebuild/changes/G-gate-g-signoff-20260415/*`

## 方案

- 核心做法：
  1. 新增 Gate-G 签收脚本，串行执行：
     - `run-stage-g-gate-acceptance.ps1`
     - `run-stage-g-warning-governance.ps1`
     - `run-stage-g-regression-baseline.ps1`
  2. 聚合检查 `g01/g02/g03/warning_audit_fields/no_open_p0_p1` 五项结果，统一落盘 `tmp/stage-g-signoff/latest.json`。
  3. 文档层回写签收结论，并将最小任务总表中的 `G-G1` 标为已完成。
- 状态流转或调用链变化：
  1. 新链路：`Gate-G signoff -> G-01/G-02/G-03 aggregate -> stage signoff verdict`。
  2. 阶段状态从 `Gate-G（执行中）` 切换到 `Gate-G（已签收）`。

## 风险与回退

- 主要风险：
  1. 上游 F 证据波动会传导到 G 聚合脚本。
  2. 多脚本串行耗时较长，可能出现超时。
- 回退方式：
  1. 若聚合失败，先按失败建议命令修复上游证据，再重跑签收脚本。
  2. 若超时，先使用 `-RequireGateG` 之外的模式完成报告落盘，再定位瓶颈后重跑。
