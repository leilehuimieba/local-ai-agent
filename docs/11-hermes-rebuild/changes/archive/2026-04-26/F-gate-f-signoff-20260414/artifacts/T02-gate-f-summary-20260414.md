# T02 Gate-F 聚合验收摘要（2026-04-14）

更新时间：2026-04-14  
范围：`F-gate-f-signoff-20260414` / `F-G1`

## 1. 验收命令

1. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-gate-acceptance.ps1 -RequireGateF`

## 2. 结果摘要

1. 报告文件：`tmp/stage-f-gate/latest.json`
2. `checked_at`：`2026-04-14T22:40:25.7301978+08:00`
3. `status`：`passed`
4. Gate 字段：
   - `install_ready=true`
   - `doctor_ready=true`
   - `release_candidate_ready=true`
   - `windows_10min_ready=true`
   - `no_open_p0_p1=true`
   - `ready=true`

## 3. blocker 检查结果

1. `F-install-upgrade-20260414/status.md`：`exists=true`，`no_blocker=true`
2. `F-doctor-core-checks-20260414/status.md`：`exists=true`，`no_blocker=true`
3. `F-release-candidate-regression-20260414/status.md`：`exists=true`，`no_blocker=true`
4. `F-windows-10min-verification-20260414/status.md`：`exists=true`，`no_blocker=true`

## 4. 证据路径

1. `tmp/stage-f-gate/latest.json`
2. `tmp/stage-f-install/latest.json`
3. `tmp/stage-f-doctor/latest.json`
4. `tmp/stage-f-rc/latest.json`
5. `tmp/stage-f-windows/latest.json`
