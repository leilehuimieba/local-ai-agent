# Scripts 目录说明

更新时间：2026-04-12
状态：当前有效

## 1. 当前保留脚本

1. `start-dev.ps1`
   - 用途：本地开发启动入口。
2. `run-v1-regression-check.ps1`
   - 用途：V1 固定回归检查入口。
   - 引用文档：`docs/07-test/V1回归检查入口_V1.md`
3. `sync-to-server.ps1`
   - 用途：将本地仓库完整同步到服务器 `/opt/hermes`。
   - 默认参数：`175.178.90.193` + `root` + `~/.ssh/labsafe_new.pem`。
4. `sync-from-server.ps1`
   - 用途：将服务器 `/opt/hermes` 回拉到本地仓库。
   - 默认行为：回拉完成后删除本地备份；如需保留，使用 `-KeepLocalBackup`。
5. `run-stage-d-migrate-acceptance.ps1`
   - 用途：阶段 D `D-02` 记忆迁移最小脚本验收（legacy JSONL -> SQLite）。
   - 证据输出：`tmp/stage-d-migrate-acceptance/latest.json`。
6. `run-stage-d-day1-acceptance.ps1`
   - 用途：阶段 D `D-06` 跨会话连续性首轮样本（会话 A 写记忆 -> 会话 B 召回）。
   - 证据输出：`tmp/stage-d-day1/latest.json`。
7. `run-stage-d-gate-batch.ps1`
   - 用途：阶段 D `D-G1` 7 天 Gate-D 批量验收（连续性与召回命中率阈值校验）。
   - 证据输出：`tmp/stage-d-batch/latest.json`。

## 2. 同步命令示例

1. 本地 -> 服务器
   - `powershell -NoProfile -File scripts/sync-to-server.ps1`
2. 服务器 -> 本地（保留本地备份）
   - `powershell -NoProfile -File scripts/sync-from-server.ps1 -KeepLocalBackup`
3. 服务器 -> 本地（回拉后自动删除本地备份）
   - `powershell -NoProfile -File scripts/sync-from-server.ps1`

## 3. 已归档脚本

以下脚本属于阶段性验收或专项检查，不作为当前默认入口：

1. `archive/run-mainline-acceptance.ps1`
2. `archive/run-runtime-host-lock-check.ps1`
3. `archive/run-stage-d-day13-day14.ps1`

说明：

1. 归档脚本可保留用于历史复盘与证据补跑。
2. 新阶段若需复用，应先复制回根 `scripts/` 并在对应执行入口文档中登记。
3. 不建议在当前主链路文档继续引用归档路径以外的旧脚本名。
