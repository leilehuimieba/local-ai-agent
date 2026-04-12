# 验证记录

## 验证方式

- doctor 验收：
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-doctor-acceptance.ps1`
- 命令直跑：
  - `powershell -ExecutionPolicy Bypass -File scripts/doctor.ps1 -RepoRoot <repo> -GatewayPort <port> -RuntimePort <port>`
- 人工复核：
  - 检查报告中的 `checks`、`versions`、`artifacts` 字段完整性。

## 证据位置

- 命令与脚本：
  - `scripts/doctor.ps1`
  - `scripts/run-stage-f-doctor-acceptance.ps1`
- 报告与日志：
  - `tmp/stage-f-doctor/latest.json`
  - `tmp/stage-f-doctor/logs/runtime.log`
  - `tmp/stage-f-doctor/logs/gateway.log`

## Gate 映射

- 对应阶段 Gate：Gate-F（已在 `F-G1` 提审签收）。
- 当前覆盖情况：
  - 已完成 `F-02` 的核心诊断命令接入与验收。
  - 后续 `F-03`~`F-G1` 已完成，见 `changes/F-gate-f-signoff/review.md`。
