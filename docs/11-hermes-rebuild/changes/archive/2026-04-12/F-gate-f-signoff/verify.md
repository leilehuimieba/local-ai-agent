# 验证记录

## 验证方式

- Gate-F 聚合验收：
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-gate-acceptance.ps1 -RequireGateF`
- 后端回归：
  - `go test ./...`（`gateway/`）

## 证据位置

- Gate-F 报告：
  - `tmp/stage-f-gate/latest.json`
- 输入证据：
  - `tmp/stage-f-install/latest.json`
  - `tmp/stage-f-doctor/latest.json`
  - `tmp/stage-f-rc/latest.json`
  - `tmp/stage-f-windows/latest.json`
- 脚本：
  - `scripts/run-stage-f-gate-acceptance.ps1`

## Gate 映射

- 对应阶段 Gate：Gate-F。
- 当前覆盖情况：
  - 安装/升级链路通过（`install_ready=true`）。
  - doctor 核心检查通过（`doctor_ready=true`）。
  - 发布候选回归与故障注入通过（`release_candidate_ready=true`）。
  - Windows 新机 10 分钟验证通过（`windows_10min_ready=true`）。
  - F 阶段无未关闭阻塞（`no_open_p0_p1=true`）。
  - 门禁判定通过（`gate_f.ready=true`）。
