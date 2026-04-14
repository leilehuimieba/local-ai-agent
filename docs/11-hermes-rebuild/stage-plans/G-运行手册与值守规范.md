# 阶段 G 运行手册与值守规范（G-04）

更新时间：2026-04-14

## 1. 值守职责

1. 值守人：按时执行证据保鲜巡检并确认落盘。
2. 变更负责人：处理失败分流、修复并回写 change 验证。
3. 评审人：按 Gate-G 口径进行阶段评审与签收决策。

## 2. 日常值守流程

1. 执行 `scripts/run-stage-g-evidence-freshness.ps1`。
2. 检查 `tmp/stage-g-evidence-freshness/latest.json`。
3. 若有 warning，补齐责任字段并复跑。
4. 将结果回写对应 change 的 `status.md` 与 `verify.md`。

## 3. 发布窗口流程

1. 使用 `-ReleaseWindow -RefreshEvidence` 执行保鲜脚本。
2. warning 场景必须携带 `WarningAuditExecutor/TrackingId/DueAt`。
3. 仅当 `ready_for_release=true` 才可进入发布判定。

## 4. 例外处理

1. 若脚本执行失败，先保存错误输出与报告路径。
2. 无法在单轮修复时，标记阻塞并升级给评审人。
3. 不得在缺证据状态下直接声明 Gate-G 通过。
