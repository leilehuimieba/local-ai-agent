# 验证记录

## 验证方式

- 单元测试：
  1. 本 change 默认不新增代码单测；如复核触发失败修复，再补对应单测记录。
- 集成测试：
  1. 复核脚本：`scripts/run-stage-e-gate-batch.ps1 -Rounds 5`（已执行，最新复核时间 `2026-04-14T20:57:15+08:00`）。
- 人工验证：
  1. 核对本轮阶段 E 关键 change 的提交、任务完成态、证据路径一致性。

## 证据位置

- 测试记录：
  1. `scripts/run-stage-e-gate-batch.ps1 -Rounds 5`
  2. `tmp/stage-e-batch/latest.json`（`checked_at=2026-04-14T20:57:15.5045443+08:00`）
- 日志或截图：
  1. `docs/11-hermes-rebuild/changes/archive/2026-04-14/E-claudecode-shell-alignment/review.md`
  2. `docs/11-hermes-rebuild/changes/E-sensitive-pattern-expansion/verify.md`
  3. `docs/11-hermes-rebuild/changes/E-gate-e-signoff-20260414/review.md`
  4. `tmp/stage-e-entry1/latest.json`
  5. `tmp/stage-e-consistency/latest.json`
  6. `tmp/stage-e-entry-failure/latest.json`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-E（已签收）
- 当前覆盖情况：
  1. `entry_rate/consistency_rate/failure_closure_rate` 本轮均为 `1.0`，`gate_e.ready=true`。
  2. 已完成 Gate-E 阶段切换评审并签收，通过结论见 `review.md`；状态切换已写入 `docs/11-hermes-rebuild/current-state.md`（阶段 F / Gate-F）。
