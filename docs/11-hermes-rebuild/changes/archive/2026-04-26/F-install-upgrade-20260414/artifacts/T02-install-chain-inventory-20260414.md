# T02 安装链路现状盘点与最小修复清单（2026-04-14）

更新时间：2026-04-14  
范围：`F-install-upgrade-20260414` / `F-01`

## 1. 当前链路（源码对照）

1. 验收入口：`scripts/run-stage-f-install-acceptance.ps1`
2. 安装执行：`Invoke-Install -Mode install` 调用 `scripts/install-local-agent.ps1`
3. 升级执行：`Invoke-Install -Mode upgrade` 调用 `scripts/install-local-agent.ps1`
4. 启动验证：`Start-AndCheckSystem` 启动 `gateway/launcher.exe`，检查：
   - `http://127.0.0.1:{gateway}/health`
   - `http://127.0.0.1:{runtime}/health`
   - `GET /api/v1/system/info` 返回的 `repo_root`
5. 通过判定：
   - 安装产物齐全（launcher/server/runtime/frontend/config/start-agent/readme）
   - install/upgrade 启动都通过
   - upgrade 生成 backup
   - `current-version.txt` 与 upgrade 版本一致

## 2. 盘点出的失败点

1. 依赖前置检查缺失（`npm/cargo/go`）时，失败信息不够前置，定位成本高。
2. `Invoke-LoggedCommand` 失败路径未保证执行 `Pop-Location`，存在工作目录泄露风险。
3. `README-run.md` 写入固定占位符 `{gateway_port}`，与实际端口不一致。

## 3. 本刀最小修复（已落地）

1. 新增 `Assert-CommandAvailable`，在触发构建前检查必要命令：
   - `Prepare-FrontendDist`：检查 `npm`
   - `Build-ReleaseBinaries`：检查 `cargo`、`go`
2. 调整 `Invoke-LoggedCommand` 为 `try/finally`，保证 `Pop-Location` 必执行。
3. `Write-RunReadme` 增加 `GatewayPort` 参数，写入真实访问地址。

## 4. 边界与后续

1. 本刀不引入 `doctor` 新检查项，仅收口安装/升级主路径的可诊断性。
2. 本刀不调整 `run-stage-f-install-acceptance.ps1` 判定门槛。
3. 下一步进入 `T03`，执行 `scripts/run-stage-f-install-acceptance.ps1` 回写证据。
