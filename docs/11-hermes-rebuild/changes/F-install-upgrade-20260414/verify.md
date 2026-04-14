# 验证记录

## 验证方式

- 单元测试：
  1. 本刀默认不新增单测；如涉及脚本辅助模块再补充。
- 集成测试：
  1. `scripts/run-stage-f-install-acceptance.ps1`
- 人工验证：
  1. 核对安装后首任务可执行与关键路径可回退。

## 证据位置

- 测试记录：
  1. `tmp/stage-f-install/latest.json`
- 日志或截图：
  1. `scripts/install-local-agent.ps1`
  2. `scripts/run-stage-f-install-acceptance.ps1`
  3. `docs/11-hermes-rebuild/changes/F-install-upgrade-20260414/review.md`（后续提审时补）

## Gate 映射

- 对应阶段 Gate：
  1. Gate-F（执行中）
- 当前覆盖情况：
  1. 当前仅完成阶段切换与变更初始化，不做 Gate-F 完成声明。
