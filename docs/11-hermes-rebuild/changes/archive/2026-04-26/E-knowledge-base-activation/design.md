# 技术方案

## 影响范围

- 涉及模块：
  1. `crates/runtime-core/src/memory_router.rs`
  2. `crates/runtime-core/src/knowledge_store.rs`（仅复用既有拦截规则，不改 schema）
  3. `crates/runtime-host/*`（外部记忆服务配置与路由接入，按最小改动实施）
  4. `scripts/*`（知识导出、图谱构建与评测脚本，按需新增）
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/*`

## 方案

- 核心做法：
  1. `knowledge_type` 扩展：`agent_resolve` 且验证通过时，归类为 `workflow_pattern` 进入知识写入链路。
  2. `knowledge_summary` 收口：优先用 `result.summary`；若过短则回退到 `result.final_answer` 摘要，再进入低价值拦截。
  3. 保持 `SQLite/主数据库` 为唯一事实源（SoT），外部系统仅做可插拔增强，不反向覆盖主库。
  4. 引入“外部记忆服务适配层”（优先 `cortex`）：通过 feature flag 控制，默认可回退。
  5. 引入“知识消费导出层”：从主库导出 Markdown + Frontmatter + Wikilink，接 Obsidian/graphify 做关系可视化。
  6. 新增固定评测集与指标脚本，按 Top-K 命中率、写入成功率、延迟口径持续回归。
- 状态流转或调用链变化：
  1. `action_completed -> verification_completed -> write_knowledge_record` 链路在 `agent_resolve` 成功场景下可触发 `knowledge_written`。
  2. 失败结果仍由 `report.outcome.passed=false` 阻断，不会进入知识层。
  3. 外部记忆增强链路采用旁路：`write_knowledge_record -> adapter(可选) -> external_ingest`。
  4. 查询链路优先本地：`query -> local_recall -> external_recall(可选) -> merge/rank`。

## 执行约束

1. 同一时刻只允许一个 in-progress task，避免并行漂移。
2. 每个 task 必须具备“输入、产出、完成判据、回退动作”四元组，否则不执行。
3. 未补齐 `verify.md` 证据前，不得标记任务完成。
4. 任何外部服务接入必须默认鉴权，禁止未鉴权端口暴露。
5. 未通过 Gate 子门禁前，不做跨范围代码重构或 schema 迁移。

## 阶段门禁（本 change 内）

1. `G1 基础可运行`：完成外部服务最小可运行、鉴权可用、可回退。
2. `G2 接入主链路`：完成 ingest/recall 适配与单测，主流程不回退。
3. `G3 可视化消费`：完成导出与图谱链路，业务样例可直观看关系。
4. `G4 质量治理`：完成评测、清洗、回退演练与安全核查。
5. `G5 提审收口`：完成文档证据、遗留项登记与下一步 backlog 冻结。

## 风险与回退

- 主要风险：
  1. 放量后可能引入低质量知识条目，污染知识层。
  2. 某些成功但价值低的结果可能被写入，增加复盘噪声。
  3. 外部服务接入失败可能影响主链路稳定性。
  4. 图谱抽取策略不稳定可能引入关系噪声，影响可读性。
- 回退方式：
  1. 回退 `memory_router.rs` 中新增来源映射即可恢复原策略。
  2. 若噪声偏高，可仅保留摘要回退策略并移除 `agent_resolve` 入库映射。
  3. 关闭外部记忆 feature flag，全部回退至主库检索与写入路径。
  4. 保留导出脚本但关闭图谱构建步骤，不影响主业务流程。
