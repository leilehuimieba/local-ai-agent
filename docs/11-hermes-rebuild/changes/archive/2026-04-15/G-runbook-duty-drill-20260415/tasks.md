# 任务清单

- [x] T01 建立 G-04 变更工作区并切换主推进项
  完成判据：`G-runbook-duty-drill-20260415` 五件套创建完成，`current-state/INDEX` 指向该 change。
- [x] T02 完成值守演练取证（routine + release_window）
  完成判据：`tmp/stage-g-evidence-freshness/latest.json`、`tmp/stage-g-gate/latest.json`、`tmp/stage-g-ops/latest.json` 均为 `passed`，且 release_window 策略为 30 分钟。
- [x] T03 完成 G-04 回归基线复核
  完成判据：`tmp/stage-g-regression/latest.json` 为 `passed` 且 `summary.pass_rate>=95`。
- [x] T04 回写阶段文档与 change 状态
  完成判据：`status/verify/review` 与 `阶段计划总表/全路线最小任务分解总表` 同步到 G-04 口径。
