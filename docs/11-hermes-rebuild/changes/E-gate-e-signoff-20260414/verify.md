# 验证记录

## 验证方式

- 单元测试：
  1. 本 change 默认不新增代码单测；如复核触发失败修复，再补对应单测记录。
- 集成测试：
  1. 复核脚本：`scripts/run-stage-e-gate-batch.ps1 -Rounds 5`（必要时执行）。
- 人工验证：
  1. 核对本轮阶段 E 关键 change 的提交、任务完成态、证据路径一致性。

## 证据位置

- 测试记录：
  1. `tmp/stage-e-batch/latest.json`（若执行批量复核）。
- 日志或截图：
  1. `docs/11-hermes-rebuild/changes/archive/2026-04-14/E-claudecode-shell-alignment/review.md`
  2. `docs/11-hermes-rebuild/changes/E-sensitive-pattern-expansion/verify.md`
  3. `docs/11-hermes-rebuild/changes/E-gate-e-signoff-20260414/review.md`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-E（执行中）
- 当前覆盖情况：
  1. 当前为提审收口准备阶段，不做 Gate-E 完成声明。
