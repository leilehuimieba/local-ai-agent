# 技术方案

## 影响范围

- 安装脚本：`scripts/install-local-agent.ps1`
- 验收脚本：`scripts/run-stage-f-install-acceptance.ps1`
- 文档与索引：`docs/11-hermes-rebuild/changes/*`、`docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`

## 方案

### 1. 安装/升级主路径脚本

- 新增 `install-local-agent.ps1`，核心行为：
  1. 构建 `runtime-host.exe`、`gateway/server.exe`、`gateway/launcher.exe`。
  2. 组装发布目录（`releases/<version>`），包含：
     - `target/debug/runtime-host.exe`
     - `gateway/server.exe`
     - `gateway/launcher.exe`
     - `frontend/dist/*`
     - `config/app.json`
     - `start-agent.ps1`
     - `README-run.md`
  3. `install` 模式：
     - 仅在 `current` 不存在时执行。
  4. `upgrade` 模式：
     - 备份当前 `current` 到 `backups/backup-<ts>`，再切换到新版本。

### 2. F-01 验收脚本

- 新增 `run-stage-f-install-acceptance.ps1`，流程：
  1. 在 `tmp/stage-f-install/sandbox` 创建隔离安装目录。
  2. 执行一次 `install`，校验产物完整。
  3. 启动 `launcher.exe`，校验 `gateway/runtime` 健康和 `/api/v1/system/info`。
  4. 执行一次 `upgrade`，校验 `backups` 副本与 `current-version.txt`。
  5. 再次启动校验并输出 `tmp/stage-f-install/latest.json`。

## 风险与回退

- 风险：当前脚本采用“目录发布”主路径，尚未冻结 MSI/winget/scoop 渠道。
- 风险：安装包仍依赖当前仓库构建环境，不等同最终发行安装器体验。
- 回退：升级过程默认保留 `backups` 目录，可直接回切上一版本目录。
