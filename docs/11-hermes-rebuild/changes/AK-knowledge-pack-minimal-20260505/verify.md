# 验证记录

## 验证方式

- 单元测试：
  - 混合检索命中与轻量重排测试
  - `knowledge pack` 结构稳定性测试
  - ask / learn profile 的知识注入差异测试
- 运行验证：
  - 至少 3 个知识问答样例回归
- 人工验证：
  - 核对事件 metadata / context snapshot 中是否出现 `knowledge_pack_question_type`、`knowledge_pack_citations`、`knowledge_pack_match_reason`

## 证据位置

- 测试记录：
  - Rust: `cargo test -p runtime-core`
  - Go: `go test ./internal/knowledge/...`（`workdir=gateway/`）
- 命令记录：
  - 2026-05-05：`cargo fmt --all`
  - 2026-05-05：`cargo test -p runtime-core`
  - 2026-05-05：`go test ./internal/knowledge/...`

## 本次结果

- 混合检索命中与轻量重排：
  - `gateway/internal/knowledge/store_test.go`
  - `TestStore_SearchReranksTitleTagsAndCitationCount` 通过
  - 验证标题、tags、citation_count 共同参与重排，弱命中项不会被 SQL `%整句%` 过滤提前截断
- `knowledge pack` 结构稳定性：
  - `crates/runtime-core/src/knowledge.rs`
  - `build_knowledge_pack_collects_citations_and_supporting_hits` 通过
  - 验证 `question_type`、`top_hits`、`supporting_hits`、`citations`、`answer_hints`
- ask / learn profile 注入差异：
  - `crates/runtime-core/src/context_builder.rs`
  - `ask_profile_prefers_knowledge_pack_digest` 通过
  - `learn_profile_prefers_knowledge_pack_digest` 通过
- metadata / snapshot 透出：
  - `crates/runtime-core/src/events.rs`
  - `crates/runtime-core/src/run_context_metadata.rs`
  - 相关测试已验证 `knowledge_pack_question_type`、`knowledge_pack_citations`、`knowledge_pack_match_reason`

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期
- 当前覆盖情况：
  - 已完成第四刀文档拆分、范围冻结与代码实现
  - 已补混合检索、knowledge pack、ask / learn 注入、snapshot metadata 的测试证据
  - verify 矩阵仍明确后置，不在本 change 内继续扩 scope
