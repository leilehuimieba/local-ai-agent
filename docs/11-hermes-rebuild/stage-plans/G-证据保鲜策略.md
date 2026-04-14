# 阶段 G 证据保鲜策略（G-01）

更新时间：2026-04-14  
适用范围：`docs/11-hermes-rebuild/` 阶段 G 主线

## 1. 目标

1. 让 Gate 关键证据保持在时效阈值内，避免“历史通过、当前失效”。
2. 固化发布窗口与日常巡检两套复跑策略。
3. 把 warning 审计记录收敛到统一链路，避免发布判定口径分叉。

## 2. 证据来源与入口

1. 后端复核聚合：`scripts/run-stage-backend-reverify-pack.ps1`
2. 阶段 G 证据保鲜入口：`scripts/run-stage-g-evidence-freshness.ps1`
3. 证据落盘：
   - `tmp/stage-g-evidence-freshness/latest.json`
   - `tmp/stage-g-evidence-freshness/warning-audit-latest.json`

## 3. 时效阈值与复跑频率

1. 日常巡检（`routine`）：
   - 证据最大年龄：180 分钟
   - 建议复跑频率：每 180 分钟
2. 发布窗口（`release_window`）：
   - 证据最大年龄：30 分钟
   - 建议复跑频率：每 30 分钟

## 4. 执行命令

1. 日常巡检（不强制刷新底层证据）：
   - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-evidence-freshness.ps1`
2. 日常巡检（强制刷新底层证据）：
   - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-evidence-freshness.ps1 -RefreshEvidence`
3. 发布窗口巡检（带 warning 责任字段）：
   - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-g-evidence-freshness.ps1 -ReleaseWindow -RefreshEvidence -WarningAuditExecutor <owner> -WarningAuditTrackingId <tracking-id> -WarningAuditDueAt <ISO8601>`

## 5. 通过判据

1. `latest.json.status = passed`
2. `checks.backend_reverify_ready = true`
3. `checks.strict_gate_ready = true`
4. 发布窗口下如存在 warning，`warning-audit-latest.json.ready_for_release = true`

## 6. 失败分流

1. `failed_checks_count > 0`：按 `failed_checks.reason_codes` 与 `recommended_commands_minimal` 定向修复。
2. `non_blocking_warning_count > 0`：补齐 `Executor/TrackingId/DueAt` 并复跑。
3. 证据超时：启用 `-RefreshEvidence` 重新取证。

## 7. 责任分工

1. 值守人：执行保鲜脚本并确认报告落盘。
2. 变更负责人：处理失败分流与 warning 审计字段补齐。
3. Gate 评审人：以 `tmp/stage-g-evidence-freshness/latest.json` 为阶段 G 证据主入口之一。
