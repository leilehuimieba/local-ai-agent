# 任务清单

- [x] T01 G-03 回归脚本稳定化
  完成判据：`run-stage-g-regression-baseline.ps1` 完成模式拆分与失败分流字段补齐。
- [x] T02 最小回归首轮验收（1 轮）
  完成判据：执行 `-Rounds 1 -RequirePass` 通过。
- [x] T03 最小回归批量验收（3 轮）
  完成判据：执行 `-RefreshEvidence -Rounds 3 -RequirePass` 通过，`pass_rate>=95`。
- [x] T04 文档回写与状态同步
  完成判据：`current-state/INDEX/总表/G-最小回归基线清单` 与本 change 五件套一致。
- [x] T05 G-03 提审结论
  完成判据：`review.md` 补齐结论、边界与下一步（G-04）。
