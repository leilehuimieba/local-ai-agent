# H-memory-routing-kb-20260415（design）

更新时间：2026-04-16
状态：已冻结（H05-01）

## 影响范围

1. 涉及模块：
   - `gateway/internal/api/learning_memory.go`
   - `gateway/internal/api/learning_memory_test.go`
   - `gateway/internal/memory/`
   - `tmp/stage-h-memory-routing/`
2. 涉及文档或 contract：
   - `docs/11-hermes-rebuild/stage-plans/H-产品差异化与透明执行路线.md`
   - `docs/11-hermes-rebuild/changes/H-memory-routing-kb-20260415/`

## 方案

1. 核心做法：
   - 以当前学习文章写入链路为基线，冻结“分类 -> 写入判据 -> 命中解释 -> 注入预算 -> 回退演练”最小治理口径。
   - 当前写入阈值沿用 `score >= 70`；低于阈值时输出 `route=skip`、`write_status=skipped_low_score`，不进入长期记忆。
   - 当前记忆实体保持 `kind=learning_article`、`scope=learning_mode`，不提前扩到多类知识路由。
   - 当前 recall 匹配口径保持最小规则版：优先按 `article_id` 或 `title` 命中，不引入语义召回。
   - 当前注入预算保持最小上限：最多取 3 条 recall 结果，只输出 `memory_digest` 与 `injection_preview`。
   - 聚合输出 `tmp/stage-h-memory-routing/latest.json`，用于 H-05 提审与 Gate-H 汇总。
2. 状态流转或调用链变化：
   - `extract -> value_score -> memory_write -> recall_preview -> injection_audit -> rollback_drill`
   - 当前主链仍保持 gateway 侧闭环，先证明策略稳定，再决定是否下沉到 runtime 主上下文装配。

## 冻结结论

1. H05-01 设计冻结只收紧治理边界，不引入新存储层、新召回策略或前端协议。
2. H05-02 允许的代码增量仅限：
   - `learning_memory` 路由解释字段补强
   - 注入预算/误注入证据输出
   - 回退演练辅助入口或测试
3. H05-02 不允许扩项到：
   - 向量检索
   - 多站点统一记忆抽象
   - runtime 主上下文自动注入改造

## 风险与回退

1. 主要风险：
   - 学习文章摘要长度与主题粒度不稳定，导致注入质量波动。
   - 记忆命中评估若只看“命中”不看“有效”，会产生虚高结论。
   - 当前 recall 仅按同文文章命中，尚未覆盖“同主题不同文章”的复用场景。
2. 回退方式：
   - 关闭自动写入或将写入降级为 `skip/manual_confirm`。
   - 注入侧保留只读预览，不把学习记忆自动并入主执行上下文。
