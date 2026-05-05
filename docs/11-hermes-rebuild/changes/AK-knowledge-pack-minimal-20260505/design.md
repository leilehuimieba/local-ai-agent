# 技术方案

## 影响范围

- 涉及模块：
  - `gateway/internal/knowledge/store.go`
  - `crates/runtime-core/src/knowledge.rs`
  - `crates/runtime-core/src/knowledge_store.rs`
  - `crates/runtime-core/src/context_builder.rs`
  - `crates/runtime-core/src/events.rs`
- 涉及文档或 contract：
  - `docs/11-hermes-rebuild/changes/AG-agent-loop-memory-knowledge-20260505/design.md`
  - Runtime knowledge digest / context snapshot / citation-ready 输出口径

## Harness 判断

- 第一刀已经把“多步回路”立起来，第二刀已经把“按任务装配上下文”立起来，第三刀已经把“记忆最小路由”立起来。
- 第四刀现在最值得补的，是把“能搜到知识”升级成“能稳定支撑回答”的最小知识主链。
- 因此本刀只补知识检索增强与 `knowledge pack`，不提前混入 verify 矩阵。

## 范围冻结

- 本刀只做四件事：
  - 收紧本地知识检索排序，让结果更像可回答材料而不是命中列表
  - 定义最小 `knowledge pack`
  - 让 Runtime 输出 citation-ready 的知识摘要与用途标签
  - 让 ask / learn profile 优先注入 `knowledge pack`

## 最小检索口径

### 检索目标

- 本刀不引入新存储引擎，先基于现有 `knowledge_items` / `knowledge_chunks` 做逻辑增强：
  - `title/category/tags` 命中
  - chunk 文本命中
  - `updated_at`、引用密度、命中位置做轻量重排

### 输出目标

- `knowledge.rs` 不再只返回 `path + snippet`。
- 本刀应至少补下面这些读侧结果字段：
  - `match_reason`
  - `citation_ready`
  - `use_for`
  - `source_kind`
- `use_for` 最小先支持：
  - `definition`
  - `relation`
  - `workflow`
  - `risk`
  - `evidence`

## Knowledge Pack 改造

- 新增最小 `knowledge pack` 概念，供 Runtime 回答时使用。
- 建议结构：
  - `question_type`
  - `top_hits`
  - `supporting_hits`
  - `citations`
  - `answer_hints`
- 最小 pack 目标：
  - 先说明“本次为什么选这几条”
  - 再输出 1 到 3 条主命中
  - 最后补少量 supporting hit 与 citation 列表

## Knowledge Store 改造

- `gateway/internal/knowledge/store.go`
  - 保留当前数据结构
  - 搜索从单纯 `LIKE` 升级为：
    - title / category / tags 优先
    - chunk 命中辅助
    - `updated_at` 和命中密度轻量重排
- 本刀不做：
  - 向量库替换
  - 复杂 embedding 检索
  - 大规模索引迁移

## Context / Event 出口

- `context_builder.rs`
  - 对 ask / learn profile，优先注入 `knowledge pack`
  - act / repair profile 仍保持“知识最小注入”，避免执行类上下文再次被知识堆料
- `events.rs`
  - 让知识 pack 结果进入 `context_snapshot`
  - 至少保留：
    - `knowledge_digest`
    - `knowledge_pack_question_type`
    - `knowledge_pack_citations`
    - `knowledge_pack_match_reason`

## 风险与回退

- 主要风险：
  - 轻量重排过宽，导致稳定资料被低价值 chunk 挤掉
  - citation-ready 字段写了，但回答仍然只给摘要不给来源
  - ask / learn profile 注入 pack 后，旧知识摘要与新 pack 重复堆叠
- 回退方式：
  - 保留当前 `knowledge_digest` 作为 fallback
  - 当 pack 构造失败时，退回当前稳定知识摘要路径
  - 先补混合检索测试和 pack 结构测试，再决定是否缩减旧摘要出口
