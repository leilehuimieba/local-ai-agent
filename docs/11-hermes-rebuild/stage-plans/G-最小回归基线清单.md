# 阶段 G 最小回归基线清单（G-03）

更新时间：2026-04-14

## 1. 基线目标

1. 用最小回归集快速判定“可发布稳定性”是否退化。
2. 把失败定位成本控制在单轮可回溯范围内。

## 2. 最小回归集

1. `E-01` CLI 历史切片：`scripts/run-stage-e-cli-history-acceptance.ps1`
2. `E-01` CLI 中断切片：`scripts/run-stage-e-cli-cancel-acceptance.ps1`
3. `E-04` 跨入口一致性：`scripts/run-stage-e-consistency-acceptance.ps1`
4. `F-G1` 门禁聚合：`scripts/run-stage-f-gate-acceptance.ps1 -RequireGateF`
5. `G-01` 证据保鲜聚合：`scripts/run-stage-g-evidence-freshness.ps1`

## 3. 通过阈值

1. 回归通过率 >= 95%。
2. 任一关键门禁脚本失败即判定不通过。

## 4. 失败分流

1. `E` 侧失败：优先修复接口一致性与终态字段。
2. `F` 侧失败：优先回溯 install/doctor/rc/windows 四项证据。
3. `G` 侧失败：优先检查证据时效与 warning 审计字段完整性。

## 5. 证据输出建议

1. 统一汇总到：`tmp/stage-g-regression/latest.json`。
2. 如仅单项复核，保留各脚本 `latest.json` 并在汇总中引用绝对路径。
