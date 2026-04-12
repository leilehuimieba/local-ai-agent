# 任务清单

- [x] 交付安装/升级主路径脚本
  完成判据：`install-local-agent.ps1` 支持 `install/upgrade` 两种模式。
- [x] 交付 F-01 验收脚本
  完成判据：`run-stage-f-install-acceptance.ps1` 可自动执行安装与升级校验。
- [x] 产出 F-01 证据样本
  完成判据：`tmp/stage-f-install/latest.json` 状态为 `passed`。
- [x] 后端回归验证
  完成判据：`go test ./...`（`gateway/`）通过。
- [x] 回写索引与总表状态
  完成判据：`INDEX.md` 和 `全路线最小任务分解总表.md` 与当前推进一致。
