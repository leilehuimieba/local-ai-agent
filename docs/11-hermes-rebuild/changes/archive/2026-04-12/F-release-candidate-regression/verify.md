# 验证记录

## 验证方式

- 聚合验收：
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-rc-acceptance.ps1 -Rounds 3 -RequirePass`
- 人工复核：
  - 检查 `latest.json` 的 `summary`、`release_candidate.ready`、`rounds_detail`。

## 证据位置

- 聚合报告：
  - `tmp/stage-f-rc/latest.json`
- 子链路报告：
  - `tmp/stage-f-install/latest.json`
  - `tmp/stage-f-doctor/latest.json`
  - `tmp/stage-e-entry1/latest.json`
  - `tmp/stage-e-consistency/latest.json`
  - `tmp/stage-e-entry-failure/latest.json`
- 脚本：
  - `scripts/run-stage-f-rc-acceptance.ps1`

## Gate 映射

- 对应阶段 Gate：Gate-F（已在 `F-G1` 提审签收）。
- 当前覆盖情况：
  - 已完成 `F-04` 发布候选回归与故障注入。
  - 后续 `F-05` 与 `F-G1` 已完成，见 `changes/F-windows-10min-verification/` 与 `changes/F-gate-f-signoff/`。
