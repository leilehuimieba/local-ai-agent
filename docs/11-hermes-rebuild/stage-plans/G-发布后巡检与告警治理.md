# 阶段 G 发布后巡检与告警治理（G-02）

更新时间：2026-04-15

## 1. 巡检目标

1. 固化阶段 G 的日常巡检节奏，保证证据持续有效。
2. 把 warning 从“可忽略提示”收敛为“可追踪事项”。

## 2. 巡检项

1. 证据时效：`strict_gate.evidence_age_minutes` 全项不超过阈值。
2. 严格门禁：`summary.strict_gate_ready=true`。
3. 非阻断告警：`non_blocking_warnings.count` 与详情完整。
4. 审计记录：warning 场景具备 `Executor/TrackingId/DueAt`。

## 3. 执行入口

1. 主入口：`scripts/run-stage-g-evidence-freshness.ps1`
2. 上游复核：`scripts/run-stage-backend-reverify-pack.ps1`
3. 告警审计：`scripts/run-stage-f-warning-audit-record.ps1`（由主入口透传）

## 4. 告警治理规则

1. warning 不是失败，但必须登记责任人与到期时间。
2. warning 审计记录缺字段时，`ready_for_release=false`。
3. 同一 warning_code 连续出现两次及以上，升级为重点跟踪项。

## 5. 证据落点

1. `tmp/stage-g-evidence-freshness/latest.json`
2. `tmp/stage-g-evidence-freshness/warning-audit-latest.json`
3. 关联上游：`tmp/stage-backend-reverify/latest.json`
4. 治理聚合：`tmp/stage-g-ops/latest.json`
5. warning 追踪器：`tmp/stage-g-ops/warning-tracker.json`

## 6. 升级阈值

1. 默认升级阈值：同一 `warning_code` 连续出现 >= 2 次。
2. 达到阈值后写入 `escalated_codes`，进入重点跟踪。
