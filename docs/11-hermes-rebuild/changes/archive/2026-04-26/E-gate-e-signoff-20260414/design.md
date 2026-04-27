# 技术方案

## 影响范围

- 涉及模块：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/E-gate-e-signoff-20260414/*`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
  2. `docs/11-hermes-rebuild/changes/archive/2026-04-14/E-claudecode-shell-alignment/review.md`
  3. `docs/11-hermes-rebuild/changes/E-sensitive-pattern-expansion/verify.md`
  4. `tmp/stage-e-batch/latest.json`（若重跑 Gate-E 批量脚本）

## 方案

- 核心做法：
  1. 先做状态对齐：切换当前活跃 change 到本目录，并同步 `changes/INDEX.md`。
  2. 再做证据收口：整合本轮 E 阶段关键 change（敏感治理 + shell 对齐）的提交、测试、验证证据。
  3. 进行最小 Gate-E 复核：优先复用既有 `tmp/stage-e-*` 证据；若证据过旧或缺失，再执行 `scripts/run-stage-e-gate-batch.ps1`。
  4. 输出阶段切换评审结论：按 Gate-E 指标给出“签收通过/不通过”决策，并明确是否切换到阶段 F。
- 状态流转或调用链变化：
  1. 本 change 只做文档治理与验收复核，不改运行时调用链。
  2. 如果 Gate-E 复核失败，保留阶段 E 并回到失败样本对应 change 处理；复核通过则切换 `current-state.md` 到阶段 F。

## 风险与回退

- 主要风险：
  1. 批量验收脚本可能受本地环境波动影响，导致误报失败。
  2. 历史 evidence 与当前代码状态时间戳不一致，造成提审证据争议。
- 回退方式：
  1. 若复核脚本失败，先落失败样本与日志，不做阶段结论变更。
  2. 若证据冲突，回退到“仅记录事实，不给切阶段建议”的保守结论。
