# 技术方案

## 影响范围

- 涉及模块：
  1. `scripts/cortex/cleanup-low-quality-memories.ps1`
  2. `scripts/README.md`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/E-low-quality-scoring-upgrade/*`

## 方案

- 核心做法：
  1. 保留关键词硬拦截（失败、短噪声、无价值）作为强规则。
  2. 引入质量评分模型：
     - `confidence_score`：来自记忆字段 `confidence`（缺失时取中值）。
     - `source_trust`：按来源打分（runtime/manual 高，eval/probe 低）。
     - `duplication_score`：按规范化内容重复率扣分。
  3. 计算 `quality_score`，当低于阈值时标记 `score_low` 并进入清理候选。
  4. 报告输出 `scored_samples` 和 `deleted_samples`，支持追溯。
- 状态流转或调用链变化：
  1. 清洗脚本新增“评分 -> 判定 -> 删除”链路，不影响主写入链路。

## 风险与回退

- 主要风险：
  1. 阈值过严可能误删边界记录。
  2. 来源打分映射不合理会导致评分偏差。
- 回退方式：
  1. 保留 `-DryRun` 先评估后执行。
  2. 如误删风险升高，回退到上一版关键词规则脚本。
