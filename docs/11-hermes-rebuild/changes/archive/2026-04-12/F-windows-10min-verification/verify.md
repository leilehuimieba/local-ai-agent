# 验证记录

## 验证方式

- 新机 10 分钟验证：
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-windows-acceptance.ps1 -RequirePass`
- 人工复核：
  - 检查 `latest.json` 的耗时、健康状态、首任务终态字段。

## 证据位置

- 报告：
  - `tmp/stage-f-windows/latest.json`
  - `tmp/stage-f-windows/latest.md`
- 脚本：
  - `scripts/run-stage-f-windows-acceptance.ps1`

## Gate 映射

- 对应阶段 Gate：Gate-F（已在 `F-G1` 提审签收）。
- 当前覆盖情况：
  - 已完成 `F-05` Windows 10 分钟验证。
  - Gate-F 已完成评审签收，见 `changes/F-gate-f-signoff/review.md`。
