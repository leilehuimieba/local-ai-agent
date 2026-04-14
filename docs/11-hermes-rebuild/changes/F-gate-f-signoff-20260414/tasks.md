# 任务清单

- [x] T01 Gate-F 脚本路径映射修复
  完成判据：`run-stage-f-gate-acceptance.ps1` 可识别当前 `F-* -20260414` change 状态文档。
- [x] T02 Gate-F 聚合验收执行与证据落盘
  完成判据：执行 `scripts/run-stage-f-gate-acceptance.ps1 -RequireGateF`，`tmp/stage-f-gate/latest.json` 为 `passed`。
- [x] T03 提审结论与状态同步
  完成判据：`review.md`、`status.md`、`verify.md` 完成回写并给出阶段切换建议。
- [x] T04 阶段切换评审决策（F→下一阶段）
  完成判据：给出“是否切换”的明确决策，并同步 `current-state.md` 与 `review.md` 口径。
