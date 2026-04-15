# 验证记录

## 验证方式

- 文档验证：
  1. 人工核对 `G-运行手册与值守规范.md` 已覆盖频率、职责、升级、交接四类章节。
  2. 人工核对 `全路线最小任务分解总表.md` 的 `G-04` 状态与推荐顺序。
- 演练验证：
  1. 复核最新 `tmp/stage-g-*` 证据并输出值守演练记录。

## 证据位置

- 手册文档：
  1. `docs/11-hermes-rebuild/stage-plans/G-运行手册与值守规范.md`
- 演练记录：
  1. `docs/11-hermes-rebuild/changes/G-runbook-duty-closure-20260415/artifacts/G04-duty-drill-20260415.md`
- 依赖证据：
  1. `tmp/stage-g-evidence-freshness/latest.json`
  2. `tmp/stage-g-ops/latest.json`
  3. `tmp/stage-g-regression/latest.json`

## Gate 映射

- 对应阶段 Gate：
  1. Gate-G（执行中，未签收）。
- 当前覆盖情况：
  1. 已完成 `G-04`，阶段 G 当前仅剩 `G-G1` 签收任务。
