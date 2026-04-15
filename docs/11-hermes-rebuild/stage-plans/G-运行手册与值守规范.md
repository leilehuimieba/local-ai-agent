# 阶段 G 运行手册与值守规范（G-04）

更新时间：2026-04-15

## 1. 角色与职责

1. 值守人（OnDuty）
   - 负责按频率执行阶段 G 巡检脚本并确认证据落盘。
   - 负责在发现 warning 或失败时触发分流，并在当班内完成首次回写。
2. 变更负责人（Owner）
   - 负责按分流结果修复问题并补齐 change 证据。
   - 负责在超时或阻塞时发起升级并维护追踪编号。
3. 评审人（Reviewer）
   - 负责按 Gate-G 口径复核证据完整性与时效性。
   - 负责 `G-G1` 阶段签收结论。

## 2. 值守频率与窗口

1. 日常窗口（routine）
   - 每 180 分钟至少执行 1 次：
     - `scripts/run-stage-g-evidence-freshness.ps1`
     - `scripts/run-stage-g-warning-governance.ps1 -RequirePass`
2. 发布窗口（release_window）
   - 每 30 分钟至少执行 1 次：
     - `scripts/run-stage-g-gate-acceptance.ps1 -ReleaseWindow -RefreshEvidence -RequirePass`
   - warning 场景必须提供：
     - `WarningAuditExecutor`
     - `WarningAuditTrackingId`
     - `WarningAuditDueAt`
3. 每日收口（EOD）
   - 至少执行 1 次回归基线复核：
     - `scripts/run-stage-g-regression-baseline.ps1 -Rounds 1 -RequirePass`

## 3. 值守执行清单

1. 执行脚本并确认以下报告 `status=passed`：
   - `tmp/stage-g-evidence-freshness/latest.json`
   - `tmp/stage-g-ops/latest.json`
   - `tmp/stage-g-regression/latest.json`
2. 检查关键字段：
   - `strict_gate_ready=true`
   - `checks.governance_ready=true`
   - `summary.ready=true`
3. 写入当班记录：
   - 执行时间
   - 执行人
   - 证据路径
   - 异常与分流结果

## 4. 失败分流与升级路径

1. `route=E`
   - 优先处理 CLI/网关一致性与终态字段；责任人：接口链路 owner。
2. `route=F`
   - 优先回溯 install/doctor/rc/windows 证据；责任人：发布链路 owner。
3. `route=G`
   - 优先检查证据时效、warning 审计字段、governance 就绪项；责任人：值守 owner。
4. 升级规则
   - 单次失败 30 分钟内未恢复：升级到变更负责人。
   - 同类失败连续 2 次：升级到评审人并冻结新特性。
   - 出现 P0/P1：立即暂停 Gate-G 签收流程。

## 5. 交接与产物

1. 当班必须交付：
   - 值守记录（位于当前 change `artifacts/`）
   - 关联证据路径列表
   - 未闭环问题清单（含 TrackingId 与截止时间）
2. 交接格式最小字段：
   - `shift_window`
   - `on_duty`
   - `health_summary`
   - `open_items`
   - `next_actions`

## 6. Gate-G 进入条件（供 G-G1 使用）

1. `G-01~G-04` 对应 change 均为已完成状态。
2. 最近一轮 `tmp/stage-g-regression/latest.json` 满足 `pass_rate>=95`。
3. 无未处理 P0/P1 阻塞项。
4. warning 项均具备责任人、追踪编号、到期时间三要素。
