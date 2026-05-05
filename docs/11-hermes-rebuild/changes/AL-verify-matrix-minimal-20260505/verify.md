# 验证记录

## 验证方式

- 单元测试：
  - knowledge answer verify policy 测试
  - verification metadata 写入测试
  - knowledge verify 失败后的 next step / replan 建议测试
- 运行验证：
  - 至少 3 个知识问答样例回归
- 人工验证：
  - 核对 verification metadata 中是否出现：
    - `verification_task_type`
    - `verification_evidence_count`
    - `verification_has_citation`
    - `verification_fact_inference_split`
    - `capability_risk_checked`
    - `permission_boundary_respected`

## 证据位置

- 测试记录：
  - Rust: `cargo test -p runtime-core`
- 命令记录：
  - 2026-05-05：`cargo fmt --all`
  - 2026-05-05：`cargo test -p runtime-core`

## 本次结果

- knowledge answer verify policy：
  - `crates/runtime-core/src/verify.rs`
  - `knowledge_answer_requires_citation_and_fact_boundary` 通过
  - `knowledge_answer_passes_with_citation_and_fact_boundary` 通过
- file change verify policy：
  - `crates/runtime-core/src/verify.rs`
  - `file_change_requires_path_and_effect_signal` 通过
  - `file_change_passes_with_path_and_effect_signal` 通过
- verification metadata 写入：
  - `crates/runtime-core/src/run_verification_metadata.rs`
  - `writes_single_result_budget_fields_into_metadata` 已同时验证新字段写入
- verification snapshot 透出：
  - `crates/runtime-core/src/events.rs`
  - 已通过编译与全量测试验证新的 snapshot 字段可被读取
- 主循环收口：
  - `crates/runtime-core/src/query_engine.rs`
  - 已把知识回答 verify 失败接回最小 replan 路径，不再默认直接硬收口

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期
- 当前覆盖情况：
  - 已完成第五刀文档拆分、知识回答 verify、文件变更 verify 与 metadata 证据
  - 命令执行 / 记忆写入 / 浏览器交互的 verify 扩展仍后置
