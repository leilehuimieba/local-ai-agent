# 技术方案

## 影响范围

- 涉及模块：
  1. `crates/runtime-core/src/knowledge.rs`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/E-cn-query-recall-optimization/*`

## 方案

- 核心做法：
  1. 在 `search_external_knowledge` 中保持“本地优先、外部补充”顺序不变。
  2. 外部主查询为空时，若检测到中文字符，则根据关键词映射生成英文锚点 fallback query 并重试一次外部 recall。
  3. fallback 仅对中文 query 生效；英文 query 返回 `None`，不改变现有行为。
- 状态流转或调用链变化：
  1. `search_knowledge -> search_external_knowledge` 路径新增一次“可选 fallback recall”分支。
  2. 原有重试、审计、降级流程保持不变。

## 风险与回退

- 主要风险：
  1. fallback 锚点词表覆盖不足，可能仍有中文 query 空结果。
  2. 锚点词过宽可能引入外部噪声结果。
- 回退方式：
  1. 回退 `knowledge.rs` 中新增 fallback 分支，恢复单次外部 recall。
  2. 保留原单测与审计逻辑，不影响本地主链路。
