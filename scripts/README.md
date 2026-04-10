# Scripts 目录说明

更新时间：2026-04-10
状态：当前有效

## 1. 当前保留脚本

1. `start-dev.ps1`
   - 用途：本地开发启动入口。
2. `run-v1-regression-check.ps1`
   - 用途：V1 固定回归检查入口。
   - 引用文档：`docs/07-test/V1回归检查入口_V1.md`

## 2. 已归档脚本

以下脚本属于阶段性验收或专项检查，不作为当前默认入口：

1. `archive/run-mainline-acceptance.ps1`
2. `archive/run-runtime-host-lock-check.ps1`
3. `archive/run-stage-d-day13-day14.ps1`

说明：

1. 归档脚本可保留用于历史复盘与证据补跑。
2. 新阶段若需复用，应先复制回根 `scripts/` 并在对应执行入口文档中登记。
3. 不建议在当前主链路文档继续引用归档路径以外的旧脚本名。
