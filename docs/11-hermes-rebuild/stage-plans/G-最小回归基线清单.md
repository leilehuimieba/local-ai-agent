# 阶段 G 最小回归基线清单（G-03）

更新时间：2026-04-15

## 1. 基线目标

1. 用最小回归集快速判定“可发布稳定性”是否退化。
2. 把失败定位成本控制在单轮可回溯范围内。

## 2. 最小回归集

1. `E-01` CLI 历史切片：`scripts/run-stage-e-cli-history-acceptance.ps1`
2. `E-01` CLI 中断切片：`scripts/run-stage-e-cli-cancel-acceptance.ps1`
3. `E-04` 跨入口一致性：`scripts/run-stage-e-consistency-acceptance.ps1`
4. `F-G1` 门禁聚合：`scripts/run-stage-f-gate-acceptance.ps1 -RequireGateF`
5. `G-01` 证据保鲜聚合：`scripts/run-stage-g-gate-acceptance.ps1 -RequirePass`
6. `G-02` 告警治理聚合：`scripts/run-stage-g-warning-governance.ps1 -RequirePass`

## 3. 执行模式

1. `snapshot_verify`：默认模式，仅复核现有 `latest.json` 快照。
2. `refresh_then_verify`：`-RefreshEvidence` 模式，先刷新关键证据，再执行批量校验。
3. 推荐顺序：先跑 `Rounds=1`，确认通过后再跑 `Rounds=3` 生成批量证据。

## 4. 通过阈值

1. 回归通过率 >= 95%。
2. 任一关键门禁脚本失败即判定不通过。
3. `tmp/stage-g-regression/latest.json` 中 `summary.ready=true` 才可视为本轮达标。

## 5. 失败分流

1. `E` 侧失败：优先修复接口一致性与终态字段。
2. `F` 侧失败：优先回溯 install/doctor/rc/windows 四项证据。
3. `G` 侧失败：优先检查证据时效、warning 审计字段与治理就绪字段。
4. 报告字段：按 `runs[*].route` 与 `failed_samples[*].route` 分流定位。

## 6. 证据输出

1. 汇总报告：`tmp/stage-g-regression/latest.json`。
2. 上游引用：`tmp/stage-e-*`、`tmp/stage-f-gate/latest.json`、`tmp/stage-g-*/latest.json`。
3. 当前基线（2026-04-15）：`status=passed`，`pass_rate=100`，`rounds=3`。
