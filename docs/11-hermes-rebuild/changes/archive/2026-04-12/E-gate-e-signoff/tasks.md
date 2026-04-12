# 任务清单

- [x] 交付 Gate-E 批量验收脚本
  完成判据：`run-stage-e-gate-batch.ps1` 可执行并输出批量报告。
- [x] 运行 Gate-E 批量验收
  完成判据：`tmp/stage-e-batch/latest.json` 生成且 `status=passed`。
- [x] 完成 Gate-E 判定
  完成判据：报告中 `gate_e.ready=true`，关键通过率满足阈值。
- [x] 输出阶段 E 提审收口包
  完成判据：`review.md` 含门禁判定、证据映射、风险与回退、阶段切换建议。
- [x] 回写索引与总表
  完成判据：`INDEX.md` 与 `全路线最小任务分解总表.md` 状态同步到 `E-G1`。
