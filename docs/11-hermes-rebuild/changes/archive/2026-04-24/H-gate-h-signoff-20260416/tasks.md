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
- [x] T09 补 Gate-H 聚合证据入口
  完成判据：新增 `scripts/run-stage-h-gate-acceptance.ps1` 并生成 `tmp/stage-h-gate/latest.json`，其中 `status=warning` 且 `gate_h.ready=false`。
- [x] T10 补 Gate-H 提审证据入口
  完成判据：新增 `scripts/run-stage-h-signoff-acceptance.ps1` 并生成 `tmp/stage-h-signoff/latest.json`，其中 `status=warning` 且 `gate_h_signoff.signoff_ready=false`。
- [x] T11 为 Gate-H 聚合 JSON 补中文说明字段
  完成判据：`tmp/stage-h-gate/latest.json` 与 `tmp/stage-h-signoff/latest.json` 在保留英文结构字段的同时，新增 `summary_zh`、`status_zh` 及各阻塞项中文说明字段。

## 口径注记（2026-04-19 最小收紧）

1. 上述任务记录保留历史执行痕迹，不改写 `current-state.md` 或 `changes/INDEX.md`。
2. 当前权威状态以 `current-state.md` 与 H-02 / H-03 各自工作区为准：
   - 当前活跃 change：`H-gate-h-signoff-20260416`
   - H-03 当前最强结论：`H03-39 已完成；建议主控评估是否切主推进`，但仍为 `warning`
   - H-02 当前口径：`并行观察 / 冻结观察`，仍为 `warning`，且当前无新的合格受限样本
3. `tmp/stage-h-gate/latest.json` 与 `tmp/stage-h-signoff/latest.json` 当前只允许固化“聚合复核入口 / 提审入口”，不允许把当前 `warning` 强行改写成 `passed`。
4. 因此，Gate-H 当前允许的最强口径是：已完成当前轮次聚合复核判断；当前仍为 `warning` / `执行中` / `未签收` / `不可签收`，不等于 Gate-H 可签收，不等于阶段 H 已完成。
- [x] T12 落盘主控裁决并刷新 Gate-H 签收证据
  完成判据：H-02 人工接管手册替代验收、H-03 结构性缺口风险接受均写入 status/verify，`scripts/run-stage-h-signoff-acceptance.ps1 -RequireSignoff` 生成 `signoff_ready=true`。

## 口径注记（2026-04-24 主控裁决）

1. 早期口径注记中的 `warning`、`不可签收`、`不允许改写成 passed` 保留为历史执行痕迹，不再作为当前裁决。
2. 当前权威状态以 `status.md`、`verify.md` 与 `tmp/stage-h-signoff/latest.json` 为准：Gate-H 已签收。
3. 签收依据不是 H-02 高风险场景自动修复通过，也不是 H-03 长期校准取消；签收依据是主控接受 H-02 永久人工接管手册与 H-03 结构性缺口说明作为替代验收输入。
