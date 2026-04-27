# 验证记录

## 验证方式

- 单元测试：
  1. 本刀默认不新增单测；如涉及脚本辅助模块再补充。
- 集成测试：
  1. `scripts/run-stage-f-install-acceptance.ps1`（已执行，最新复核时间 `2026-04-14T21:42:36.6967594+08:00`）。
- 人工验证：
  1. 核对安装后首任务可执行与关键路径可回退。

## 证据位置

- 测试记录：
  1. `tmp/stage-f-install/latest.json`（`checked_at=2026-04-14T21:42:36.6967594+08:00`，`status=passed`）
- 日志或截图：
  1. `scripts/install-local-agent.ps1`
  2. `scripts/run-stage-f-install-acceptance.ps1`
  3. `docs/11-hermes-rebuild/changes/F-install-upgrade-20260414/artifacts/T02-install-chain-inventory-20260414.md`
  4. `docs/11-hermes-rebuild/changes/F-install-upgrade-20260414/review.md`（后续提审时补）

## Gate 映射

- 对应阶段 Gate：
  1. Gate-F（执行中）
- 当前覆盖情况：
  1. `F-01` 安装/升级验收通过：`artifact_ok=true`、`backup_ok=true`、`version_file_matched=true`。
  2. install/upgrade 启动检查通过：`gateway_ready=true`、`runtime_ready=true`、`system_info_ok=true`。
  3. 当前变更完成 `T01-T03`，但不做 Gate-F 完成声明。
