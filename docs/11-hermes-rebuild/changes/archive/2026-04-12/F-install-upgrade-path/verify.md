# 验证记录

## 验证方式

- 安装/升级验收：
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-install-acceptance.ps1`
- 后端回归：
  - `go test ./...`（工作目录：`gateway/`）
- 人工复核：
  - 检查 `latest.json` 的安装产物、启动健康、升级备份与版本文件匹配结果。

## 证据位置

- 脚本：
  - `scripts/install-local-agent.ps1`
  - `scripts/run-stage-f-install-acceptance.ps1`
- 报告：
  - `tmp/stage-f-install/latest.json`
- 验收沙箱：
  - `tmp/stage-f-install/sandbox/*`

## Gate 映射

- 对应阶段 Gate：Gate-F（已在 `F-G1` 提审签收）。
- 当前覆盖情况：
  - 已完成 `F-01` 的安装/升级主路径与验收证据。
  - 后续 `F-02`~`F-G1` 已完成，见 `changes/F-gate-f-signoff/review.md`。
