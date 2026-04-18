# 任务清单

- [x] T01 建立 Gate-H 签收草案工作区
  完成判据：proposal/design/tasks/status/verify 五件套齐备，并加入 `changes/INDEX.md`。
- [x] T02 完成 H-01 ~ H-05 状态与证据盘点
  完成判据：每条主线均有 `status/review/verify/latest.json` 的聚合结论。
- [x] T03 输出 Gate-H 缺口与阻塞项清单
  完成判据：明确哪些 warning 为硬阻塞、哪些可后置。
- [x] T04 形成 Gate-H 提审草案
  完成判据：能支持后续阶段签收或继续补证决策。
- [x] T05 基于 H-02/H-03 新证据刷新聚合判断（首轮）
  完成判据：H-02/H-03 warning 收敛口径、唯一下一优先级任务与 Gate-H 不可签收原因同步到 status/verify/review。
- [x] T06 基于 H-03 最新扩样结果刷新聚合判断（本轮）
  完成判据：将 `business-task-chain: 10`、`skill-false-positive: 6` 纳入 Gate-H 聚合，并明确 Gate-H 仍不可签收、唯一下一优先级仍为 H-03 扩样。
- [x] T07 基于任务13结果再次刷新 Gate-H 聚合判断
  完成判据：将 H-03 的 `business-task-chain: 20`、`skill-false-positive: 16`、`manual-review: 12` 及恢复链/行业尾部/交叉复核证据纳入聚合，并重新判断 H-02/H-03 状态、Gate-H 不可签收原因与唯一下一优先级任务。
- [x] T08 基于任务15结果刷新 Gate-H 聚合判断
  完成判据：将 H-03 的“策略设计判断已完成，可进入正式执行入口”纳入 Gate-H 聚合，并明确 Gate-H 仍不可签收。

## 口径注记（2026-04-19 最小收紧）

1. 上述任务记录保留历史执行痕迹，不改写 `current-state.md` 或 `changes/INDEX.md`。
2. 当前权威状态以 `current-state.md` 与 H-02 / H-03 各自工作区为准：
   - 当前活跃 change：`H-mcp-skills-quality-20260415`
   - H-03 当前最强结论：`H03-39 已完成；建议主控评估是否切主推进`，但仍为 `warning`
   - H-02 当前口径：`并行观察 / 冻结观察`，仍为 `warning`，且当前无新的合格受限样本
3. 因此，Gate-H 当前只允许保留为聚合复核候选 / 待主控接手的聚合复核入口，不等于已切主推进，不等于 Gate-H 可签收。
