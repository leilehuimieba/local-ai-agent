# 技术方案

## 影响范围

- 涉及模块：
  1. `crates/runtime-core/src/memory_router.rs`
  2. `crates/runtime-core/src/knowledge_store.rs`（仅复用既有拦截规则，不改 schema）
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/*`

## 方案

- 核心做法：
  1. `knowledge_type` 扩展：`agent_resolve` 且验证通过时，归类为 `workflow_pattern` 进入知识写入链路。
  2. `knowledge_summary` 收口：优先用 `result.summary`；若过短则回退到 `result.final_answer` 摘要，再进入低价值拦截。
  3. 新增针对上述两点的单测，确保可持续回归。
- 状态流转或调用链变化：
  1. `action_completed -> verification_completed -> write_knowledge_record` 链路在 `agent_resolve` 成功场景下可触发 `knowledge_written`。
  2. 失败结果仍由 `report.outcome.passed=false` 阻断，不会进入知识层。

## 风险与回退

- 主要风险：
  1. 放量后可能引入低质量知识条目，污染知识层。
  2. 某些成功但价值低的结果可能被写入，增加复盘噪声。
- 回退方式：
  1. 回退 `memory_router.rs` 中新增来源映射即可恢复原策略。
  2. 若噪声偏高，可仅保留摘要回退策略并移除 `agent_resolve` 入库映射。
