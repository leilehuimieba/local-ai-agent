# 技术方案

## 影响范围

- 验收脚本：`scripts/run-stage-e-gate-batch.ps1`
- 提审文档：`docs/11-hermes-rebuild/changes/E-gate-e-signoff/*.md`
- 索引与总表：`docs/11-hermes-rebuild/changes/INDEX.md`、`docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`

## 方案

### 1. Gate-E 批量脚本

- 新增 `run-stage-e-gate-batch.ps1`，每轮顺序执行：
  1. `run-stage-e-entry1-acceptance.ps1`（E-02）
  2. `run-stage-e-consistency-acceptance.ps1`（E-04）
  3. `run-stage-e-entry-failure-acceptance.ps1`（E-05）
- 汇总每轮三类检查结果，输出：
  - `entry_rate`
  - `consistency_rate`
  - `failure_closure_rate`
  - `round_pass_rate`
- 门禁判定：
  - 默认要求 `Rounds >= 5`
  - 三个核心比率均 `>= 0.95`
  - 满足后 `gate_e.ready=true`

### 2. 提审收口

- 在 `review.md` 中固化 Gate-E 评审结论：
  - 阈值对齐
  - 样本映射
  - 风险与回退
  - 阶段切换建议（进入 F 阶段）

## 风险与回退

- 风险：当前 Gate-E 批量仍是最小样本规模，不能替代长期稳定性压测。
- 缓解：将 F 阶段前的发布候选回归作为后续稳态验证补充。
- 回退：若 Gate-E 后续复测不稳定，回退到 E-02 单入口路径，保留日志过滤和失败收口能力。
